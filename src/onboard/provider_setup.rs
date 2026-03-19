//! Provider setup and configuration
//!
//! This module handles interactive provider selection and API key configuration.

use crate::ui::prompts;
use crate::ui::prompts::PromptInteraction;
use anyhow::{Context, Result};
use std::path::Path;

// Import from wizard.rs - the provider helper functions
use super::wizard::{
    CUSTOM_MODEL_SENTINEL, LIVE_MODEL_MAX_OPTIONS, MODEL_CACHE_TTL_SECS,
    allows_unauthenticated_model_fetch, canonical_provider_name, curated_models_for_provider,
    default_model_for_provider, local_provider_choices, provider_env_var,
    supports_live_model_fetch,
};

// Import from models.rs - model fetching and caching functions
use super::models::{
    build_model_options, cache_live_models_for_provider, fetch_live_models_for_provider,
    humanize_age, load_any_cached_models_for_provider, load_cached_models_for_provider,
    normalize_ollama_endpoint_url, ollama_uses_remote_endpoint,
};

// Import provider alias functions
use crate::providers::{
    is_glm_alias, is_glm_cn_alias, is_minimax_alias, is_moonshot_alias, is_qianfan_alias,
    is_qwen_alias, is_zai_alias, is_zai_cn_alias,
};

#[allow(clippy::too_many_lines)]
pub async fn setup_provider(
    workspace_dir: &Path,
) -> Result<(String, String, String, Option<String>)> {
    // ── Tier selection ──
    let tier_idx = prompts::select("Select provider category")
        .item(0, "⭐ Recommended", "OpenRouter, Venice, Anthropic, OpenAI, Gemini")
        .item(1, "⚡ Fast inference", "Groq, Fireworks, Together AI, NVIDIA NIM")
        .item(2, "🌐 Gateway / proxy", "Vercel AI, Cloudflare AI, Amazon Bedrock")
        .item(3, "🔬 Specialized", "Moonshot/Kimi, GLM/Zhipu, MiniMax, Qwen/DashScope, Qianfan, Z.AI, Synthetic, OpenCode Zen, Cohere")
        .item(4, "🏠 Local / private", "Ollama, llama.cpp server, vLLM — no API key needed")
        .item(5, "🔧 Custom", "bring your own OpenAI-compatible API")
        .interact()?;

    let providers: Vec<(&str, &str)> = match tier_idx {
        0 => vec![
            (
                "openrouter",
                "OpenRouter — 200+ models, 1 API key (recommended)",
            ),
            ("venice", "Venice AI — privacy-first (Llama, Opus)"),
            ("anthropic", "Anthropic — Claude Sonnet & Opus (direct)"),
            ("openai", "OpenAI — GPT-4o, o1, GPT-5 (direct)"),
            (
                "openai-codex",
                "OpenAI Codex (ChatGPT subscription OAuth, no API key)",
            ),
            ("deepseek", "DeepSeek — V3 & R1 (affordable)"),
            ("mistral", "Mistral — Large & Codestral"),
            ("xai", "xAI — Grok 3 & 4"),
            ("perplexity", "Perplexity — search-augmented AI"),
            (
                "gemini",
                "Google Gemini — Gemini 2.0 Flash & Pro (supports CLI auth)",
            ),
        ],
        1 => vec![
            ("groq", "Groq — ultra-fast LPU inference"),
            ("fireworks", "Fireworks AI — fast open-source inference"),
            ("novita", "Novita AI — affordable open-source inference"),
            ("together-ai", "Together AI — open-source model hosting"),
            ("nvidia", "NVIDIA NIM — DeepSeek, Llama, & more"),
        ],
        2 => vec![
            ("vercel", "Vercel AI Gateway"),
            ("cloudflare", "Cloudflare AI Gateway"),
            (
                "astrai",
                "Astrai — compliant AI routing (PII stripping, cost optimization)",
            ),
            ("bedrock", "Amazon Bedrock — AWS managed models"),
        ],
        3 => vec![
            (
                "kimi-code",
                "Kimi Code — coding-optimized Kimi API (KimiCLI)",
            ),
            (
                "qwen-code",
                "Qwen Code — OAuth tokens reused from ~/.qwen/oauth_creds.json",
            ),
            ("moonshot", "Moonshot — Kimi API (China endpoint)"),
            (
                "moonshot-intl",
                "Moonshot — Kimi API (international endpoint)",
            ),
            ("glm", "GLM — ChatGLM / Zhipu (international endpoint)"),
            ("glm-cn", "GLM — ChatGLM / Zhipu (China endpoint)"),
            (
                "minimax",
                "MiniMax — international endpoint (api.minimax.io)",
            ),
            ("minimax-cn", "MiniMax — China endpoint (api.minimaxi.com)"),
            ("qwen", "Qwen — DashScope China endpoint"),
            ("qwen-intl", "Qwen — DashScope international endpoint"),
            ("qwen-us", "Qwen — DashScope US endpoint"),
            ("qianfan", "Qianfan — Baidu AI models (China endpoint)"),
            ("zai", "Z.AI — global coding endpoint"),
            ("zai-cn", "Z.AI — China coding endpoint (open.bigmodel.cn)"),
            ("synthetic", "Synthetic — Synthetic AI models"),
            ("opencode", "OpenCode Zen — code-focused AI"),
            ("opencode-go", "OpenCode Go — Subsidized code-focused AI"),
            ("cohere", "Cohere — Command R+ & embeddings"),
        ],
        4 => local_provider_choices(),
        _ => vec![], // Custom — handled below
    };

    // ── Custom / BYOP flow ──
    if providers.is_empty() {
        prompts::section_with_width(
            "Custom Provider Setup — any OpenAI-compatible API",
            70,
            |lines: &mut Vec<String>| {
                lines.push(
                    "ZeroClaw works with ANY API that speaks the OpenAI chat completions format."
                        .to_string(),
                );
                lines.push(
                    "Examples: LiteLLM, LocalAI, vLLM, text-generation-webui, LM Studio, etc."
                        .to_string(),
                );
            },
        )?;

        let base_url = prompts::input::input(
            "API base URL (e.g. http://localhost:1234 or https://my-api.com)",
        )
        .interact()?;

        let base_url = base_url.trim().trim_end_matches('/').to_string();
        if base_url.is_empty() {
            anyhow::bail!("Custom provider requires a base URL.");
        }

        let api_key = prompts::input::input("API key (or Enter to skip if not needed)")
            .placeholder("optional")
            .interact()?;

        let model = prompts::input::input("Model name (e.g. llama3, gpt-4o, mistral)")
            .placeholder("default")
            .interact()?;

        let provider_name = format!("custom:{base_url}");

        prompts::log::success(format!("Provider: {} | Model: {}", provider_name, model))?;

        return Ok((provider_name, api_key, model, None));
    }

    let mut select = prompts::select("Select your AI provider");
    for (idx, (_, label)) in providers.iter().enumerate() {
        select = select.item(idx, *label, "");
    }
    let provider_idx = select.interact()?;

    let provider_name = providers[provider_idx].0;

    // ── API key / endpoint ──
    let mut provider_api_url: Option<String> = None;
    let api_key = if provider_name == "ollama" {
        let use_remote_ollama =
            prompts::confirm("Use a remote Ollama endpoint (for example Ollama Cloud)?")
                .initial_value(false)
                .interact()?;

        if use_remote_ollama {
            let raw_url = prompts::input::input("Remote Ollama endpoint URL")
                .placeholder("https://ollama.com")
                .interact()?;

            let normalized_url = normalize_ollama_endpoint_url(&raw_url);
            if normalized_url.is_empty() {
                anyhow::bail!("Remote Ollama endpoint URL cannot be empty.");
            }
            let parsed = reqwest::Url::parse(&normalized_url)
                .context("Remote Ollama endpoint URL must be a valid URL")?;
            if !matches!(parsed.scheme(), "http" | "https") {
                anyhow::bail!("Remote Ollama endpoint URL must use http:// or https://");
            }

            provider_api_url = Some(normalized_url.clone());

            prompts::log::info(format!("Remote endpoint configured: {}", normalized_url))?;
            if raw_url.trim().trim_end_matches('/') != normalized_url {
                prompts::log::step("Normalized endpoint to base URL (removed trailing /api).")?;
            }
            prompts::log::step("If you use cloud-only models, append :cloud to the model ID.")?;

            let key =
                prompts::input::input("API key for remote Ollama endpoint (or Enter to skip)")
                    .placeholder("optional")
                    .interact()?;

            if key.trim().is_empty() {
                prompts::log::info(
                    "No API key provided. Set OLLAMA_API_KEY later if required by your endpoint.",
                )?;
            }

            key
        } else {
            prompts::log::info(
                "Using local Ollama at http://localhost:11434 (no API key needed).",
            )?;
            String::new()
        }
    } else if matches!(provider_name, "llamacpp" | "llama.cpp") {
        let raw_url = prompts::input::input("llama.cpp server endpoint URL")
            .placeholder("http://localhost:8080/v1")
            .interact()?;

        let normalized_url = raw_url.trim().trim_end_matches('/').to_string();
        if normalized_url.is_empty() {
            anyhow::bail!("llama.cpp endpoint URL cannot be empty.");
        }
        provider_api_url = Some(normalized_url.clone());

        prompts::log::info(format!(
            "Using llama.cpp server endpoint: {}",
            normalized_url
        ))?;
        prompts::log::step(
            "No API key needed unless your llama.cpp server is started with --api-key.",
        )?;

        let key = prompts::input::input("API key for llama.cpp server (or Enter to skip)")
            .placeholder("optional")
            .interact()?;

        if key.trim().is_empty() {
            prompts::log::info(
                "No API key provided. Set LLAMACPP_API_KEY later only if your server requires authentication.",
            )?;
        }

        key
    } else if provider_name == "sglang" {
        let raw_url = prompts::input::input("SGLang server endpoint URL")
            .placeholder("http://localhost:30000/v1")
            .interact()?;

        let normalized_url = raw_url.trim().trim_end_matches('/').to_string();
        if normalized_url.is_empty() {
            anyhow::bail!("SGLang endpoint URL cannot be empty.");
        }
        provider_api_url = Some(normalized_url.clone());

        prompts::log::info(format!("Using SGLang server endpoint: {}", normalized_url))?;
        prompts::log::step("No API key needed unless your SGLang server requires authentication.")?;

        let key = prompts::input::input("API key for SGLang server (or Enter to skip)")
            .placeholder("optional")
            .interact()?;

        if key.trim().is_empty() {
            prompts::log::info(
                "No API key provided. Set SGLANG_API_KEY later only if your server requires authentication.",
            )?;
        }

        key
    } else if provider_name == "vllm" {
        let raw_url = prompts::input::input("vLLM server endpoint URL")
            .placeholder("http://localhost:8000/v1")
            .interact()?;

        let normalized_url = raw_url.trim().trim_end_matches('/').to_string();
        if normalized_url.is_empty() {
            anyhow::bail!("vLLM endpoint URL cannot be empty.");
        }
        provider_api_url = Some(normalized_url.clone());

        prompts::log::info(format!("Using vLLM server endpoint: {}", normalized_url))?;
        prompts::log::step("No API key needed unless your vLLM server requires authentication.")?;

        let key = prompts::input::input("API key for vLLM server (or Enter to skip)")
            .placeholder("optional")
            .interact()?;

        if key.trim().is_empty() {
            prompts::log::info(
                "No API key provided. Set VLLM_API_KEY later only if your server requires authentication.",
            )?;
        }

        key
    } else if provider_name == "osaurus" {
        let raw_url = prompts::input::input("Osaurus server endpoint URL")
            .placeholder("http://localhost:1337/v1")
            .interact()?;

        let normalized_url = raw_url.trim().trim_end_matches('/').to_string();
        if normalized_url.is_empty() {
            anyhow::bail!("Osaurus endpoint URL cannot be empty.");
        }
        provider_api_url = Some(normalized_url.clone());

        prompts::log::info(format!("Using Osaurus server endpoint: {}", normalized_url))?;
        prompts::log::step(
            "No API key needed unless your Osaurus server requires authentication.",
        )?;

        let key = prompts::input::input("API key for Osaurus server (or Enter to skip)")
            .placeholder("optional")
            .interact()?;

        if key.trim().is_empty() {
            prompts::log::info(
                "No API key provided. Set OSAURUS_API_KEY later only if your server requires authentication.",
            )?;
        }

        key
    } else if canonical_provider_name(provider_name) == "gemini" {
        // Special handling for Gemini: check for CLI auth first
        if crate::providers::gemini::GeminiProvider::has_cli_credentials() {
            prompts::log::success("Gemini CLI credentials detected! You can skip the API key.")?;
            prompts::log::step("ZeroClaw will reuse your existing Gemini CLI authentication.")?;

            let use_cli = prompts::confirm("Use existing Gemini CLI authentication?")
                .initial_value(true)
                .interact()?;

            if use_cli {
                prompts::log::success("Using Gemini CLI OAuth tokens")?;
                String::new() // Empty key = will use CLI tokens
            } else {
                prompts::log::step("Get your API key at: https://aistudio.google.com/app/apikey")?;
                prompts::input::input("Paste your Gemini API key")
                    .placeholder("optional")
                    .interact()?
            }
        } else if std::env::var("GEMINI_API_KEY").is_ok() {
            prompts::log::success("GEMINI_API_KEY environment variable detected!")?;
            String::new()
        } else {
            prompts::log::step("Get your API key at: https://aistudio.google.com/app/apikey")?;
            prompts::log::step("Or run `gemini` CLI to authenticate (tokens will be reused).")?;

            prompts::input::input("Paste your Gemini API key (or press Enter to skip)")
                .placeholder("optional")
                .interact()?
        }
    } else if canonical_provider_name(provider_name) == "anthropic" {
        if std::env::var("ANTHROPIC_OAUTH_TOKEN").is_ok() {
            prompts::log::success("ANTHROPIC_OAUTH_TOKEN environment variable detected!")?;
            String::new()
        } else if std::env::var("ANTHROPIC_API_KEY").is_ok() {
            prompts::log::success("ANTHROPIC_API_KEY environment variable detected!")?;
            String::new()
        } else {
            prompts::log::step("Get your API key at: https://console.anthropic.com/settings/keys")?;
            prompts::log::step("Or run `claude setup-token` to get an OAuth setup-token.")?;

            let key =
                prompts::input::input("Paste your API key or setup-token (or press Enter to skip)")
                    .placeholder("optional")
                    .interact()?;

            if key.is_empty() {
                prompts::log::info(
                    "Skipped. Set ANTHROPIC_API_KEY or ANTHROPIC_OAUTH_TOKEN or edit config.toml later.",
                )?;
            }

            key
        }
    } else if canonical_provider_name(provider_name) == "qwen-code" {
        if std::env::var("QWEN_OAUTH_TOKEN").is_ok() {
            prompts::log::success("QWEN_OAUTH_TOKEN environment variable detected!")?;
            "qwen-oauth".to_string()
        } else {
            prompts::log::step(
                "Qwen Code OAuth credentials are usually stored in ~/.qwen/oauth_creds.json.",
            )?;
            prompts::log::step(
                "Run `qwen` once and complete OAuth login to populate cached credentials.",
            )?;
            prompts::log::step("You can also set QWEN_OAUTH_TOKEN directly.")?;

            let key = prompts::input::input(
                "Paste your Qwen OAuth token (or press Enter to auto-detect cached OAuth)",
            )
            .placeholder("optional")
            .interact()?;

            if key.trim().is_empty() {
                prompts::log::info(
                    "Using OAuth auto-detection. Set QWEN_OAUTH_TOKEN and optional QWEN_OAUTH_RESOURCE_URL if needed.",
                )?;
                "qwen-oauth".to_string()
            } else {
                key
            }
        }
    } else {
        let key_url = if is_moonshot_alias(provider_name)
            || canonical_provider_name(provider_name) == "kimi-code"
        {
            "https://platform.moonshot.cn/console/api-keys"
        } else if canonical_provider_name(provider_name) == "qwen-code" {
            "https://qwen.readthedocs.io/en/latest/getting_started/installation.html"
        } else if is_glm_cn_alias(provider_name) || is_zai_cn_alias(provider_name) {
            "https://open.bigmodel.cn/usercenter/proj-mgmt/apikeys"
        } else if is_glm_alias(provider_name) || is_zai_alias(provider_name) {
            "https://platform.z.ai/"
        } else if is_minimax_alias(provider_name) {
            "https://www.minimaxi.com/user-center/basic-information"
        } else if is_qwen_alias(provider_name) {
            "https://help.aliyun.com/zh/model-studio/developer-reference/get-api-key"
        } else if is_qianfan_alias(provider_name) {
            "https://cloud.baidu.com/doc/WENXINWORKSHOP/s/7lm0vxo78"
        } else {
            match provider_name {
                "openrouter" => "https://openrouter.ai/keys",
                "openai" => "https://platform.openai.com/api-keys",
                "venice" => "https://venice.ai/settings/api",
                "groq" => "https://console.groq.com/keys",
                "mistral" => "https://console.mistral.ai/api-keys",
                "deepseek" => "https://platform.deepseek.com/api_keys",
                "together-ai" => "https://api.together.xyz/settings/api-keys",
                "fireworks" => "https://fireworks.ai/account/api-keys",
                "novita" => "https://novita.ai/settings/key-management",
                "perplexity" => "https://www.perplexity.ai/settings/api",
                "xai" => "https://console.x.ai",
                "cohere" => "https://dashboard.cohere.com/api-keys",
                "vercel" => "https://vercel.com/account/tokens",
                "cloudflare" => "https://dash.cloudflare.com/profile/api-tokens",
                "nvidia" | "nvidia-nim" | "build.nvidia.com" => "https://build.nvidia.com/",
                "bedrock" => "https://console.aws.amazon.com/iam",
                "gemini" => "https://aistudio.google.com/app/apikey",
                "astrai" => "https://as-trai.com",
                _ => "",
            }
        };

        if matches!(provider_name, "bedrock" | "aws-bedrock") {
            // Bedrock uses AWS AKSK, not a single API key.
            prompts::log::step("Bedrock uses AWS credentials (not a single API key).")?;
            prompts::log::step(
                "Set AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY environment variables.",
            )?;
            prompts::log::step("Optionally set AWS_REGION for the region (default: us-east-1).")?;
            if !key_url.is_empty() {
                prompts::log::step(format!("Manage IAM credentials at: {}", key_url))?;
            }
            String::new()
        } else {
            if !key_url.is_empty() {
                prompts::log::step(format!("Get your API key at: {}", key_url))?;
            }
            prompts::log::step("You can also set it later via env var or config file.")?;

            let key = prompts::input::input("Paste your API key (or press Enter to skip)")
                .placeholder("optional")
                .interact()?;

            if key.is_empty() {
                let env_var = provider_env_var(provider_name);
                prompts::log::info(format!(
                    "Skipped. Set {} or edit config.toml later.",
                    env_var
                ))?;
            }

            key
        }
    };

    // ── Model selection ──
    let canonical_provider = canonical_provider_name(provider_name);
    let mut model_options: Vec<(String, String)> = curated_models_for_provider(canonical_provider);

    let mut live_options: Option<Vec<(String, String)>> = None;

    if supports_live_model_fetch(provider_name) {
        let ollama_remote = canonical_provider == "ollama"
            && ollama_uses_remote_endpoint(provider_api_url.as_deref());
        let can_fetch_without_key =
            allows_unauthenticated_model_fetch(provider_name) && !ollama_remote;
        let has_api_key = !api_key.trim().is_empty()
            || ((canonical_provider != "ollama" || ollama_remote)
                && std::env::var(provider_env_var(provider_name))
                    .ok()
                    .is_some_and(|value| !value.trim().is_empty()))
            || (provider_name == "minimax"
                && std::env::var("MINIMAX_OAUTH_TOKEN")
                    .ok()
                    .is_some_and(|value| !value.trim().is_empty()));

        if canonical_provider == "ollama" && ollama_remote && !has_api_key {
            prompts::log::info(
                "Remote Ollama live-model refresh needs an API key (OLLAMA_API_KEY); using curated models.",
            )?;
        }

        if can_fetch_without_key || has_api_key {
            if let Some(cached) =
                load_cached_models_for_provider(workspace_dir, provider_name, MODEL_CACHE_TTL_SECS)
                    .await?
            {
                let shown_count = cached.models.len().min(LIVE_MODEL_MAX_OPTIONS);
                prompts::log::step(format!(
                    "Found cached models ({}) updated {} ago.",
                    shown_count,
                    humanize_age(cached.age_secs)
                ))?;

                live_options = Some(build_model_options(
                    cached
                        .models
                        .into_iter()
                        .take(LIVE_MODEL_MAX_OPTIONS)
                        .collect(),
                    "cached",
                ));
            }

            let should_fetch_now = prompts::confirm(if live_options.is_some() {
                "Refresh models from provider now?"
            } else {
                "Fetch latest models from provider now?"
            })
            .initial_value(live_options.is_none())
            .interact()?;

            if should_fetch_now {
                match fetch_live_models_for_provider(
                    provider_name,
                    &api_key,
                    provider_api_url.as_deref(),
                ) {
                    Ok(live_model_ids) if !live_model_ids.is_empty() => {
                        cache_live_models_for_provider(
                            workspace_dir,
                            provider_name,
                            &live_model_ids,
                        )
                        .await?;

                        let fetched_count = live_model_ids.len();
                        let shown_count = fetched_count.min(LIVE_MODEL_MAX_OPTIONS);
                        let shown_models: Vec<String> = live_model_ids
                            .into_iter()
                            .take(LIVE_MODEL_MAX_OPTIONS)
                            .collect();

                        if shown_count < fetched_count {
                            prompts::log::step(format!(
                                "Fetched {} models. Showing first {}.",
                                fetched_count, shown_count
                            ))?;
                        } else {
                            prompts::log::step(format!("Fetched {} live models.", shown_count))?;
                        }

                        live_options = Some(build_model_options(shown_models, "live"));
                    }
                    Ok(_) => {
                        prompts::log::warning("Provider returned no models; using curated list.")?;
                    }
                    Err(error) => {
                        prompts::log::warning(format!(
                            "Live fetch failed ({}); using cached/curated list.",
                            error
                        ))?;

                        if live_options.is_none()
                            && let Some(stale) =
                                load_any_cached_models_for_provider(workspace_dir, provider_name)
                                    .await?
                            {
                                prompts::log::step(format!(
                                    "Loaded stale cache from {} ago.",
                                    humanize_age(stale.age_secs)
                                ))?;

                                live_options = Some(build_model_options(
                                    stale
                                        .models
                                        .into_iter()
                                        .take(LIVE_MODEL_MAX_OPTIONS)
                                        .collect(),
                                    "stale-cache",
                                ));
                            }
                    }
                }
            }
        } else {
            prompts::log::info("No API key detected, so using curated model list.")?;
            prompts::log::step("Tip: add an API key and rerun onboarding to fetch live models.")?;
        }
    }

    if let Some(live_model_options) = live_options {
        let source_idx = prompts::select("Model source")
            .item(
                0,
                format!("Provider model list ({})", live_model_options.len()),
                "",
            )
            .item(
                1,
                format!("Curated starter list ({})", model_options.len()),
                "",
            )
            .interact()?;

        if source_idx == 0 {
            model_options = live_model_options;
        }
    }

    if model_options.is_empty() {
        model_options.push((
            default_model_for_provider(provider_name),
            "Provider default model".to_string(),
        ));
    }

    model_options.push((
        CUSTOM_MODEL_SENTINEL.to_string(),
        "Custom model ID (type manually)".to_string(),
    ));

    let mut select = prompts::select("Select your default model");
    for (idx, (model_id, label)) in model_options.iter().enumerate() {
        select = select.item(idx, format!("{} — {}", label, model_id), "");
    }
    let model_idx = select.interact()?;

    let selected_model = model_options[model_idx].0.clone();

    let model = if selected_model == CUSTOM_MODEL_SENTINEL {
        prompts::input::input("Enter custom model ID")
            .placeholder("e.g. llama3, gpt-4o, mistral")
            .interact()?
    } else {
        selected_model
    };

    prompts::log::success(format!("Provider: {} | Model: {}", provider_name, model))?;

    Ok((provider_name.to_string(), api_key, model, provider_api_url))
}
