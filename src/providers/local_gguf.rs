//! Local LLM integration using llama.cpp
//!
//! Provides direct GGUF model inference without external servers.
//! Supports persistent KV cache, streaming, and context window management.

use anyhow::{Context, Result};
use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::context::LlamaContext;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::{AddBos, LlamaChatMessage, LlamaModel, Special};
use llama_cpp_2::sampling::LlamaSampler;
use llama_cpp_2::token::LlamaToken;
use std::num::NonZeroU32;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;

// Configuration constants
const DEFAULT_MODEL_PATH: &str = "models/model.gguf";
const INFERENCE_CONTEXT_TOKENS: u32 = 32_768;
const PROMPT_BATCH_SIZE: usize = 512;
const MAX_GENERATION_TOKENS: usize = 4096;
const SENTENCE_BOUNDARY_GRACE: usize = 50;

// Sampler parameters
const SAMPLER_TEMPERATURE: f32 = 0.7;
const SAMPLER_TOP_P: f32 = 0.92;
const SAMPLER_TOP_K: i32 = 40;
const SAMPLER_MIN_P: f32 = 0.05;
const SAMPLER_REPEAT_LAST_N: i32 = 256;
const SAMPLER_REPEAT_PENALTY: f32 = 1.10;

/// When history token count exceeds this fraction of context, evict oldest turns.
const CONTEXT_USAGE_EVICTION_THRESHOLD: f32 = 0.85;

const SYSTEM_PROMPT: &str = "\
# IDENTITY
You are DX-Agent — an AI assistant built for developers. \
You run locally, you're fast, and you're precise.

# VOICE
- Direct and technically accurate
- Short, clear sentences
- No corporate buzzwords or filler
- First word should be substantive content
";

#[derive(Clone, Debug)]
struct ChatMessage {
    role: String,
    content: String,
}

/// Persistent inference state — context and KV cache survive across calls.
struct InferenceState {
    backend: LlamaBackend,
    model: LlamaModel,
    ctx: LlamaContext<'static>,
    history: Vec<ChatMessage>,
    /// Number of tokens currently in the KV cache (all prior turns).
    kv_cursor: i32,
    /// The token IDs currently in the KV cache (for sampler penalty tracking).
    cached_tokens: Vec<LlamaToken>,
    /// Physical core count, computed once.
    n_threads: i32,
}

// SAFETY: LlamaBackend, LlamaModel, LlamaContext are single-threaded C++ objects.
// We protect all access behind a tokio::Mutex and only ever touch them inside
// spawn_blocking, so at most one thread accesses them at a time.
unsafe impl Send for InferenceState {}

#[derive(Clone)]
pub struct LocalGgufProvider {
    state: Arc<Mutex<Option<InferenceState>>>,
}

impl LocalGgufProvider {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(None)),
        }
    }

    /// Resolve model path: env var DX_MODEL_PATH > config > default
    fn model_path() -> String {
        std::env::var("DX_MODEL_PATH").unwrap_or_else(|_| DEFAULT_MODEL_PATH.to_string())
    }

    /// Compute optimal thread count once (physical cores - 1, min 1).
    fn compute_thread_count() -> i32 {
        let sys = sysinfo::System::new_with_specifics(
            sysinfo::RefreshKind::nothing().with_cpu(sysinfo::CpuRefreshKind::nothing()),
        );
        let physical = sys.physical_core_count().unwrap_or(1).max(1);
        if physical > 4 {
            (physical - 1) as i32
        } else {
            physical as i32
        }
    }

    fn sampler_seed() -> u32 {
        rand::random::<u32>()
    }

    /// Create a context with flash attention, falling back to standard if unsupported.
    fn create_context<'a>(
        model: &'a LlamaModel,
        backend: &'a LlamaBackend,
        n_threads: i32,
    ) -> Result<LlamaContext<'a>> {
        let base = LlamaContextParams::default()
            .with_n_ctx(NonZeroU32::new(INFERENCE_CONTEXT_TOKENS))
            .with_n_batch(PROMPT_BATCH_SIZE as u32)
            .with_n_threads(n_threads)
            .with_n_threads_batch(n_threads);

        // Try flash attention first
        let fa_params = base.clone().with_flash_attention_policy(1);
        model
            .new_context(backend, fa_params)
            .or_else(|_e| {
                let std_params = base.with_flash_attention_policy(0);
                model.new_context(backend, std_params)
            })
            .context("Failed to create inference context")
    }

    /// One-time initialization. Loads model and creates persistent context.
    pub async fn initialize(&self) -> Result<()> {
        let mut guard = self.state.lock().await;
        if guard.is_some() {
            return Ok(());
        }

        let state = tokio::task::spawn_blocking(|| -> Result<InferenceState> {
            let mut backend =
                LlamaBackend::init().context("Failed to initialize llama backend")?;
            backend.void_logs();

            let model_path = Self::model_path();
            let model_params = LlamaModelParams::default().with_n_gpu_layers(999);
            let model = LlamaModel::load_from_file(&backend, &model_path, &model_params)
                .context(format!("Failed to load model from: {}", model_path))?;

            let n_threads = Self::compute_thread_count();
            let ctx = Self::create_context(&model, &backend, n_threads)?;

            // SAFETY: We're transmuting the lifetime to 'static because we store
            // backend, model, and ctx together in InferenceState and never move
            // the backend/model while ctx exists. The Mutex ensures single-threaded access.
            let ctx: LlamaContext<'static> = unsafe { std::mem::transmute(ctx) };

            Ok(InferenceState {
                backend,
                model,
                ctx,
                history: Vec::new(),
                kv_cursor: 0,
                cached_tokens: Vec::new(),
                n_threads,
            })
        })
        .await??;

        *guard = Some(state);
        Ok(())
    }

    /// Build the prompt string using the model's embedded chat template.
    fn build_prompt(model: &LlamaModel, history: &[ChatMessage]) -> Result<String> {
        let mut messages = Vec::with_capacity(history.len() + 1);

        messages.push(
            LlamaChatMessage::new("system".to_string(), SYSTEM_PROMPT.to_string())
                .context("Failed to create system chat message")?,
        );

        for msg in history {
            messages.push(
                LlamaChatMessage::new(msg.role.clone(), msg.content.clone())
                    .context("Failed to create chat message")?,
            );
        }

        // Try model's embedded template first
        match model.apply_chat_template(None, &messages, true) {
            Ok(prompt) => Ok(prompt),
            Err(_) => {
                // Fallback: manual ChatML construction
                let mut prompt = String::with_capacity(4096);
                prompt.push_str("<|im_start|>system\n");
                prompt.push_str(SYSTEM_PROMPT);
                prompt.push_str("<|im_end|>\n");

                for msg in history {
                    prompt.push_str("<|im_start|>");
                    prompt.push_str(&msg.role);
                    prompt.push('\n');
                    prompt.push_str(&msg.content);
                    prompt.push_str("<|im_end|>\n");
                }

                prompt.push_str("<|im_start|>assistant\n");
                Ok(prompt)
            }
        }
    }

    /// Evict oldest conversation turns when approaching context limit.
    fn maybe_evict_history(state: &mut InferenceState) {
        let limit = (INFERENCE_CONTEXT_TOKENS as f32 * CONTEXT_USAGE_EVICTION_THRESHOLD) as usize;

        if state.cached_tokens.len() < limit {
            return;
        }

        let target = (INFERENCE_CONTEXT_TOKENS as f32 * 0.5) as usize;

        while state.cached_tokens.len() > target && state.history.len() > 2 {
            state.history.drain(0..2.min(state.history.len()));
        }

        state.ctx.clear_kv_cache();
        state.kv_cursor = 0;
        state.cached_tokens.clear();
    }

    /// Core inference engine. Streams tokens via `on_token` callback.
    fn run_inference(
        state: &mut InferenceState,
        cancel: &CancellationToken,
        on_token: &dyn Fn(&str),
    ) -> Result<String> {
        let full_prompt = Self::build_prompt(&state.model, &state.history)?;

        let all_tokens = state
            .model
            .str_to_token(&full_prompt, AddBos::Never)
            .context("Tokenization failed")?;

        let new_tokens = if state.kv_cursor > 0 {
            let cached = state.kv_cursor as usize;
            if cached < all_tokens.len()
                && all_tokens[..cached] == state.cached_tokens[..cached.min(state.cached_tokens.len())]
            {
                &all_tokens[cached..]
            } else {
                state.ctx.clear_kv_cache();
                state.kv_cursor = 0;
                state.cached_tokens.clear();
                &all_tokens[..]
            }
        } else {
            &all_tokens[..]
        };

        let mut pos = state.kv_cursor;
        let total = new_tokens.len();
        let mut offset = 0;

        while offset < total {
            if cancel.is_cancelled() {
                anyhow::bail!("Generation cancelled during prompt evaluation");
            }

            let end = (offset + PROMPT_BATCH_SIZE).min(total);
            let chunk = &new_tokens[offset..end];
            let is_last_chunk = end == total;

            let mut batch = LlamaBatch::new(chunk.len(), 1);
            for (i, &token) in chunk.iter().enumerate() {
                let logits = is_last_chunk && i == chunk.len() - 1;
                batch.add(token, pos, &[0], logits)?;
                pos += 1;
            }
            state.ctx.decode(&mut batch)?;
            offset = end;
        }

        state.kv_cursor = all_tokens.len() as i32;
        state.cached_tokens = all_tokens.clone();

        let mut sampler = LlamaSampler::chain_simple([
            LlamaSampler::penalties(SAMPLER_REPEAT_LAST_N, SAMPLER_REPEAT_PENALTY, 0.0, 0.0),
            LlamaSampler::top_k(SAMPLER_TOP_K),
            LlamaSampler::top_p(SAMPLER_TOP_P, 1),
            LlamaSampler::min_p(SAMPLER_MIN_P, 1),
            LlamaSampler::temp(SAMPLER_TEMPERATURE),
            LlamaSampler::dist(Self::sampler_seed()),
        ]);

        sampler.accept_many(all_tokens.iter().copied());

        let max_gen = MAX_GENERATION_TOKENS.min(
            (INFERENCE_CONTEXT_TOKENS as usize).saturating_sub(all_tokens.len()),
        );
        let max_loop = max_gen + SENTENCE_BOUNDARY_GRACE;

        let mut n_cur = state.kv_cursor;
        let mut generated = String::with_capacity(max_gen * 4);
        let mut gen_batch = LlamaBatch::new(1, 1);
        let mut hit_limit = false;
        let mut grace_tokens = 0;

        for i in 0..max_loop {
            if cancel.is_cancelled() {
                break;
            }
            if i >= max_gen {
                hit_limit = true;
            }
            if n_cur >= INFERENCE_CONTEXT_TOKENS as i32 {
                break;
            }

            let token = sampler.sample(&state.ctx, -1);

            if state.model.is_eog_token(token) {
                break;
            }

            let piece = state
                .model
                .token_to_str(token, Special::Tokenize)
                .unwrap_or_default();

            on_token(&piece);
            generated.push_str(&piece);

            state.cached_tokens.push(token);

            gen_batch.clear();
            gen_batch.add(token, n_cur, &[0], true)?;
            n_cur += 1;
            state.ctx.decode(&mut gen_batch)?;

            if hit_limit {
                let last_char = piece.chars().last().unwrap_or(' ');
                if last_char == '.' || last_char == '?' || last_char == '!' || piece.contains('\n')
                {
                    break;
                }
                grace_tokens += 1;
                if grace_tokens >= SENTENCE_BOUNDARY_GRACE {
                    on_token("...");
                    generated.push_str("...");
                    break;
                }
            }
        }

        state.kv_cursor = n_cur;

        Ok(generated.trim().to_string())
    }

    /// Generate a streaming response. Each token is sent via `callback`.
    pub async fn generate_stream<F>(
        &self,
        prompt: &str,
        cancel: CancellationToken,
        callback: F,
    ) -> Result<()>
    where
        F: Fn(String) + Send + 'static,
    {
        let state_arc = self.state.clone();
        let prompt = prompt.to_string();

        tokio::task::spawn_blocking(move || {
            let rt = tokio::runtime::Handle::current();
            let mut guard = rt.block_on(state_arc.lock());
            let state = guard
                .as_mut()
                .ok_or_else(|| anyhow::anyhow!("LLM not initialized — call initialize() first"))?;

            Self::maybe_evict_history(state);

            state.history.push(ChatMessage {
                role: "user".to_string(),
                content: prompt,
            });

            let result = Self::run_inference(state, &cancel, &|piece| {
                callback(piece.to_string());
            });

            match &result {
                Ok(answer) if !answer.is_empty() => {
                    state.history.push(ChatMessage {
                        role: "assistant".to_string(),
                        content: answer.clone(),
                    });
                }
                Err(_) => {
                    state.history.pop();
                }
                _ => {}
            }

            result.map(|_| ())
        })
        .await?
    }

    pub async fn is_initialized(&self) -> bool {
        self.state.lock().await.is_some()
    }

    /// Clear conversation history and KV cache.
    pub async fn reset(&self) {
        if let Some(state) = self.state.lock().await.as_mut() {
            state.history.clear();
            state.ctx.clear_kv_cache();
            state.kv_cursor = 0;
            state.cached_tokens.clear();
        }
    }
}

impl Default for LocalGgufProvider {
    fn default() -> Self {
        Self::new()
    }
}
