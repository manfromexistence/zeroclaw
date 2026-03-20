Awesome now you please update our code to be the most performant local running rust code in the world!!!
```markdown
Alright, I've done the research. Here's the brutal truth about your code — what's good, what's bad, and what's genuinely broken.

---

## Verdict: It's solid *for a first pass*, but it has several significant architectural issues that range from "will bite you eventually" to "is actively broken right now."

---

## 🟢 What's Good

1. **Right crate choice.** The `llama-cpp-2` project was created with the explicit goal of staying as up to date as possible with llama.cpp, and as of version 0.1.133 (2026-02-03), it's actively maintained. It's the correct binding to use in 2026. The alternative `llama_cpp` (edgenai) crate is higher-level but less frequently updated.

2. **Batched prompt evaluation.** Your chunked prompt processing loop (PROMPT_BATCH_SIZE = 512) is correct and follows the standard llama.cpp pattern for efficient prefill.

3. **Sampler chain ordering.** Penalties → top-k → top-p → min-p → temperature → dist is a reasonable sampler chain order.

4. **Flash attention fallback.** The try-flash-then-fallback pattern is pragmatic and good.

5. **Graceful generation termination.** The "hit limit but keep going to a sentence boundary" logic is a nice UX touch.

---

## 🔴 Critical Issues

### 1. **You are blocking the async runtime — this is THE cardinal sin**

Your `generate()` and `generate_stream()` are `async fn` but they hold a `std::sync::Mutex` guard across the *entire* inference loop — which is CPU-bound work lasting **seconds to minutes**. Holding a `std::sync::MutexGuard` during slow operation blocks the worker thread — other tasks on this worker stall.

The entire body of `generate()` and `generate_stream()` is synchronous, CPU-heavy work. Making it `async` is a lie. You're starving your tokio runtime.

**Fix:** The entire inference body should run inside `tokio::task::spawn_blocking()`. The `async` on these functions currently does nothing — there's not a single `.await` inside the actual computation. Even the `llama_cpp` (edgenai) higher-level crate wraps its operations as "a thin `tokio::spawn_blocking` wrapper" for exactly this reason.

### 2. **Hardcoded chat template — fragile and wrong**

Your `build_prompt()` manually constructs ChatML tokens (`<|im_start|>`, `<|im_end|>`). This works for Qwen models *today* because they use ChatML, but:

- llama.cpp supports `llama_chat_apply_template` — if the template parameter is nullptr, the model's default chat template will be used instead.
- By default, the chat template will be taken from the input model. If you want to use another chat template, pass `--chat-template NAME`.

The GGUF file itself **embeds its chat template**. You're ignoring it and hardcoding ChatML. If you ever swap to a non-ChatML model (Llama 3, Mistral, etc.), this silently produces garbage. The `llama-cpp-2` crate exposes `ChatTemplateError` types and chat template functionality — use them.

### 3. **`AddBos::Always` with a chat template that already has special tokens**

You're calling `str_to_token(&full_prompt, AddBos::Always)` on a prompt string that already starts with `<|im_start|>`. You're likely double-adding the BOS token, which can subtly degrade output quality. When you manually construct the prompt with special tokens, you should use `AddBos::Never` or let the tokenizer handle it through the model's chat template.

### 4. **Context recreation on every call — KV cache is wasted**

You create a brand-new `LlamaContext` on every single call to `generate()` or `generate_stream()`, then immediately call `ctx.clear_kv_cache()`. This means:
- You re-encode the *entire* conversation history from scratch every time
- The entire point of keeping `history: Vec<Message>` is for multi-turn, but you get zero KV cache benefit
- For a 10-turn conversation, you're re-processing all 10 turns of tokens on every new message

You should persist the context and only process the *new* tokens (the delta). This is a massive performance waste.

### 5. **`#[allow(deprecated)]` on `token_to_bytes` — ignoring warnings**

You're using a deprecated API and suppressing the warning instead of migrating. This will break on a future `llama-cpp-2` update. As llama.cpp is a very fast moving target, this crate does not attempt to create a stable API — deprecated APIs get removed fast.

---

## 🟡 Moderate Issues

### 6. **`System::new_all()` called on every generation**

`optimal_thread_count()` calls `System::new_all()` which <ins>enumerates all processes, disks, networks, etc.</ins> just to get the physical core count. This is expensive. Cache it once at initialization.

### 7. **No cancellation support in streaming**

`generate_stream` has no way to abort generation mid-stream. If a user hits Ctrl+C or navigates away, the generation loop runs to completion. You need a `CancellationToken` or `AtomicBool` check inside the loop.

### 8. **Massive code duplication**

`generate()` and `generate_stream()` are ~90% identical. Extract the common inference logic into a private method that takes an optional callback.

### 9. **Sampler seed is weak**

```rust
SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_nanos() as u32)
```

Truncating `u128` nanos to `u32` means you get ~4.3 seconds of uniqueness before wrapping. Two calls within the same nanosecond-modulo-2^32 window get the same seed. Use `rand::random::<u32>()` or at minimum mix in a counter.

### 10. **No context window management**

When conversation history grows beyond `INFERENCE_CONTEXT_TOKENS`, you'll silently truncate or crash. There's no sliding window, summary, or eviction strategy for old messages.

### 11. **`String::with_capacity(max_tokens * 4)`**

This pre-allocates up to 4096 × 4 = 16KB, which is fine, but the `* 4` is a guess. Real token-to-byte ratios vary. Minor, but sloppy.

---

## 🔵 Architectural Concerns

### 12. **`Arc<Mutex<Option<...>>>` is a code smell**

The two-phase init pattern (`new()` + `initialize()`) with `Option` inside a mutex is fragile. Callers can call `generate()` before `initialize()` and get a runtime error. Consider using `tokio::sync::OnceCell` or `std::sync::OnceLock` for the lazy initialization, or just require the model at construction time.

### 13. **Model path is a compile-time constant**

`const MODEL_PATH: &str = r"F:\cli\models\..."` — this is a Windows-specific hardcoded path baked into the binary. This should come from configuration, environment variables, or CLI args.

---

## Summary Scorecard

| Aspect | Grade | Notes |
|---|---|---|
| Crate choice | ✅ A | `llama-cpp-2` is the right pick |
| Async correctness | ❌ F | Blocking the runtime entirely |
| Chat template | ❌ D | Hardcoded, ignores GGUF metadata |
| KV cache usage | ❌ F | Recreated every call, zero reuse |
| Sampler setup | ✅ B+ | Good chain, weak seed |
| Batch processing | ✅ A | Correct chunked prefill |
| Code quality | 🟡 C | Heavy duplication, deprecated APIs |
| Production readiness | ❌ D | No cancellation, no context management |

**Is this "the best" code?** No. It's a working prototype with correct low-level llama.cpp usage patterns but fundamentally broken async architecture and wasted performance from zero KV cache reuse. The fixes aren't hard, but they matter.
```

Of this code:
```rust
//! Local LLM integration using llama.cpp

use anyhow::{Context, Result};
use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::{AddBos, LlamaModel};
use llama_cpp_2::sampling::LlamaSampler;
use std::num::NonZeroU32;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use sysinfo::System;

const MODEL_PATH: &str = r"F:\cli\models\llm\Qwen3.5-0.8B-Q4_K_M.gguf";
#[allow(dead_code)]
const MODEL_NAME: &str = "Qwen-3.5-0.8B-Q4_K_M";

const SYSTEM_PROMPT: &str = "\
# IDENTITY
You are Dx — the AI core of DX, the world's fastest development experience platform. \
You are built in Rust, you run locally, and you are free. You are not a cloud chatbot. \
You are a precision engineering tool that lives on the developer's own machine.

# VOICE
- You speak like a senior staff engineer: direct, technically precise, zero filler.
- Short sentences for clarity. Longer sentences only when technical depth demands it.
- NEVER use corporate buzzwords: \"leverage\", \"synergy\", \"revolutionize\", \"delve\", \"I'd be happy to\".
- NEVER start responses with \"Great question\" or \"That's a great question\" or any sycophantic opener.
- NEVER apologize unless you made a factual error. \"Sorry\" is not a filler word.
- First word of your response should be substantive content, not pleasantries.
";

const INFERENCE_CONTEXT_TOKENS: u32 = 32_768;
const PROMPT_BATCH_SIZE: usize = 512;
const SAMPLER_TEMPERATURE: f32 = 0.7;
const SAMPLER_TOP_P: f32 = 0.92;
const SAMPLER_TOP_K: i32 = 40;
const SAMPLER_MIN_P: f32 = 0.05;
const SAMPLER_REPEAT_LAST_N: i32 = 256;
const SAMPLER_REPEAT_PENALTY: f32 = 1.10;

#[derive(Clone)]
struct Message {
    role: String,
    content: String,
}

struct LocalLlmInner {
    backend: LlamaBackend,
    model: LlamaModel,
    history: Vec<Message>,
}

#[derive(Clone)]
pub struct LocalLlm {
    inner: Arc<Mutex<Option<LocalLlmInner>>>,
}

impl LocalLlm {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn initialize(&self) -> Result<()> {
        let mut inner = self
            .inner
            .lock()
            .map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;

        if inner.is_some() {
            return Ok(());
        }

        let mut backend = LlamaBackend::init().context("Failed to initialize llama backend")?;
        backend.void_logs();

        let model_params = LlamaModelParams::default().with_n_gpu_layers(999);
        let model = LlamaModel::load_from_file(&backend, MODEL_PATH, &model_params)
            .context(format!("Failed to load model from path: {}", MODEL_PATH))?;

        *inner = Some(LocalLlmInner {
            backend,
            model,
            history: Vec::new(),
        });

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn generate(&self, prompt: &str) -> Result<String> {
        let mut inner_guard = self
            .inner
            .lock()
            .map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        let inner = inner_guard
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("LLM not initialized"))?;

        inner.history.push(Message {
            role: "user".to_string(),
            content: prompt.to_string(),
        });

        let full_prompt = Self::build_prompt(&inner.history);

        let n_threads = Self::optimal_thread_count();
        let ctx_params = LlamaContextParams::default()
            .with_n_ctx(NonZeroU32::new(INFERENCE_CONTEXT_TOKENS))
            .with_n_batch(PROMPT_BATCH_SIZE as u32)
            .with_n_threads(n_threads)
            .with_n_threads_batch(n_threads)
            .with_flash_attention_policy(1);

        let mut ctx = inner.model.new_context(&inner.backend, ctx_params.clone()).or_else(|e| {
            #[cfg(debug_assertions)]
            eprintln!("Warning: Flash attention context creation failed, falling back to standard attention ({})", e);
            let fallback_params = LlamaContextParams::default()
                .with_n_ctx(NonZeroU32::new(INFERENCE_CONTEXT_TOKENS))
                .with_n_batch(PROMPT_BATCH_SIZE as u32)
                .with_n_threads(n_threads)
                .with_n_threads_batch(n_threads)
                .with_flash_attention_policy(0);
            inner.model.new_context(&inner.backend, fallback_params)
        }).context("Failed to create inference context")?;

        ctx.clear_kv_cache();

        let tokens = inner
            .model
            .str_to_token(&full_prompt, AddBos::Always)
            .context("Tokenization failed")?;

        let available = (INFERENCE_CONTEXT_TOKENS as usize).saturating_sub(tokens.len());
        let max_tokens = available.min(4096);

        // Batched prompt evaluation
        let mut pos: i32 = 0;
        let total = tokens.len();
        let mut offset = 0;

        while offset < total {
            let end = (offset + PROMPT_BATCH_SIZE).min(total);
            let chunk = &tokens[offset..end];
            let is_last_chunk = end == total;

            let mut batch = LlamaBatch::new(chunk.len(), 1);
            for (i, &token) in chunk.iter().enumerate() {
                let logits = is_last_chunk && i == chunk.len() - 1;
                batch.add(token, pos, &[0], logits)?;
                pos += 1;
            }
            ctx.decode(&mut batch)?;
            offset = end;
        }

        // Sampler chain
        let mut sampler = LlamaSampler::chain_simple([
            LlamaSampler::penalties(SAMPLER_REPEAT_LAST_N, SAMPLER_REPEAT_PENALTY, 0.0, 0.0),
            LlamaSampler::top_k(SAMPLER_TOP_K),
            LlamaSampler::top_p(SAMPLER_TOP_P, 1),
            LlamaSampler::min_p(SAMPLER_MIN_P, 1),
            LlamaSampler::temp(SAMPLER_TEMPERATURE),
            LlamaSampler::dist(Self::sampler_seed()),
        ]);
        sampler.accept_many(tokens.iter().copied());

        // Generation loop
        let mut n_cur = tokens.len() as i32;
        let mut generated_text = String::with_capacity(max_tokens * 4);
        let mut gen_batch = LlamaBatch::new(1, 1);

        let mut hit_limit = false;
        let mut extra_tokens = 0;
        let max_loop = max_tokens + 50;

        for i in 0..max_loop {
            if i >= max_tokens {
                hit_limit = true;
            }
            if n_cur >= INFERENCE_CONTEXT_TOKENS as i32 {
                break;
            }

            let token = sampler.sample(&ctx, -1);

            if inner.model.is_eog_token(token) {
                break;
            }

            #[allow(deprecated)]
            let piece_bytes = inner
                .model
                .token_to_bytes(token, llama_cpp_2::model::Special::Tokenize)?;
            let piece = String::from_utf8_lossy(&piece_bytes);
            generated_text.push_str(&piece);

            gen_batch.clear();
            gen_batch.add(token, n_cur, &[0], true)?;
            n_cur += 1;

            ctx.decode(&mut gen_batch)?;

            if hit_limit {
                let last_char = piece.chars().last().unwrap_or(' ');
                if last_char == '.' || last_char == '?' || last_char == '!' || piece.contains('\n')
                {
                    break;
                }
                extra_tokens += 1;
                if extra_tokens >= 50 {
                    generated_text.push_str("...");
                    break;
                }
            }
        }

        let answer = generated_text.trim().to_string();
        if !answer.is_empty() {
            inner.history.push(Message {
                role: "assistant".to_string(),
                content: answer.clone(),
            });
        }

        Ok(answer)
    }

    pub async fn generate_stream<F>(&self, prompt: &str, callback: F) -> Result<()>
    where
        F: Fn(String) + Send + 'static,
    {
        let mut inner_guard = self
            .inner
            .lock()
            .map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        let inner = inner_guard
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("LLM not initialized"))?;

        inner.history.push(Message {
            role: "user".to_string(),
            content: prompt.to_string(),
        });

        let full_prompt = Self::build_prompt(&inner.history);

        let n_threads = Self::optimal_thread_count();
        let ctx_params = LlamaContextParams::default()
            .with_n_ctx(NonZeroU32::new(INFERENCE_CONTEXT_TOKENS))
            .with_n_batch(PROMPT_BATCH_SIZE as u32)
            .with_n_threads(n_threads)
            .with_n_threads_batch(n_threads)
            .with_flash_attention_policy(1);

        let mut ctx = inner.model.new_context(&inner.backend, ctx_params.clone()).or_else(|e| {
            #[cfg(debug_assertions)]
            eprintln!("Warning: Flash attention context creation failed, falling back to standard attention ({})", e);
            let fallback_params = LlamaContextParams::default()
                .with_n_ctx(NonZeroU32::new(INFERENCE_CONTEXT_TOKENS))
                .with_n_batch(PROMPT_BATCH_SIZE as u32)
                .with_n_threads(n_threads)
                .with_n_threads_batch(n_threads)
                .with_flash_attention_policy(0);
            inner.model.new_context(&inner.backend, fallback_params)
        }).context("Failed to create inference context")?;

        ctx.clear_kv_cache();

        let tokens = inner
            .model
            .str_to_token(&full_prompt, AddBos::Always)
            .context("Tokenization failed")?;

        let available = (INFERENCE_CONTEXT_TOKENS as usize).saturating_sub(tokens.len());
        let max_tokens = available.min(4096);

        // Batched prompt evaluation
        let mut pos: i32 = 0;
        let total = tokens.len();
        let mut offset = 0;

        while offset < total {
            let end = (offset + PROMPT_BATCH_SIZE).min(total);
            let chunk = &tokens[offset..end];
            let is_last_chunk = end == total;

            let mut batch = LlamaBatch::new(chunk.len(), 1);
            for (i, &token) in chunk.iter().enumerate() {
                let logits = is_last_chunk && i == chunk.len() - 1;
                batch.add(token, pos, &[0], logits)?;
                pos += 1;
            }
            ctx.decode(&mut batch)?;
            offset = end;
        }

        // Sampler chain
        let mut sampler = LlamaSampler::chain_simple([
            LlamaSampler::penalties(SAMPLER_REPEAT_LAST_N, SAMPLER_REPEAT_PENALTY, 0.0, 0.0),
            LlamaSampler::top_k(SAMPLER_TOP_K),
            LlamaSampler::top_p(SAMPLER_TOP_P, 1),
            LlamaSampler::min_p(SAMPLER_MIN_P, 1),
            LlamaSampler::temp(SAMPLER_TEMPERATURE),
            LlamaSampler::dist(Self::sampler_seed()),
        ]);
        sampler.accept_many(tokens.iter().copied());

        // Generation loop with streaming
        let mut n_cur = tokens.len() as i32;
        let mut generated_text = String::with_capacity(max_tokens * 4);
        let mut gen_batch = LlamaBatch::new(1, 1);

        let mut hit_limit = false;
        let mut extra_tokens = 0;
        let max_loop = max_tokens + 50;

        for i in 0..max_loop {
            if i >= max_tokens {
                hit_limit = true;
            }
            if n_cur >= INFERENCE_CONTEXT_TOKENS as i32 {
                break;
            }

            let token = sampler.sample(&ctx, -1);

            if inner.model.is_eog_token(token) {
                break;
            }

            #[allow(deprecated)]
            let piece_bytes = inner
                .model
                .token_to_bytes(token, llama_cpp_2::model::Special::Tokenize)?;
            let piece = String::from_utf8_lossy(&piece_bytes);

            // Stream each token as it's generated
            callback(piece.to_string());
            generated_text.push_str(&piece);

            gen_batch.clear();
            gen_batch.add(token, n_cur, &[0], true)?;
            n_cur += 1;

            ctx.decode(&mut gen_batch)?;

            if hit_limit {
                let last_char = piece.chars().last().unwrap_or(' ');
                if last_char == '.' || last_char == '?' || last_char == '!' || piece.contains('\n')
                {
                    break;
                }
                extra_tokens += 1;
                if extra_tokens >= 50 {
                    callback("...".to_string());
                    generated_text.push_str("...");
                    break;
                }
            }
        }

        let answer = generated_text.trim().to_string();
        if !answer.is_empty() {
            inner.history.push(Message {
                role: "assistant".to_string(),
                content: answer,
            });
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub fn is_initialized(&self) -> bool {
        self.inner
            .lock()
            .map(|guard| guard.is_some())
            .unwrap_or(false)
    }

    #[allow(dead_code)]
    pub fn get_model_name(&self) -> String {
        format!("Local:{}", MODEL_NAME)
    }

    fn build_prompt(history: &[Message]) -> String {
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
        prompt
    }

    fn optimal_thread_count() -> i32 {
        let sys = System::new_all();
        let physical = sys.physical_core_count().unwrap_or(1).max(1);
        if physical > 4 {
            (physical - 1) as i32
        } else {
            physical as i32
        }
    }

    fn sampler_seed() -> u32 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u32)
            .unwrap_or(0xDEAD_BEEF)
    }
}

impl Default for LocalLlm {
    fn default() -> Self {
        Self::new()
    }
}
```
