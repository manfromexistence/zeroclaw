//! Model catalog management and caching
//!
//! This module handles fetching, caching, and managing AI model catalogs
//! from various providers.

use crate::ui::prompts;
use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tokio::fs;

use crate::config::Config;

// Import provider helpers from wizard
use super::wizard::{
    MODEL_CACHE_FILE, MODEL_CACHE_TTL_SECS, MODEL_PREVIEW_LIMIT,
    allows_unauthenticated_model_fetch, canonical_provider_name, models_endpoint_for_provider,
    provider_env_var, supports_live_model_fetch,
};

// ── Model Cache Structures ───────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ModelCacheEntry {
    provider: String,
    fetched_at_unix: u64,
    models: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct ModelCacheState {
    entries: Vec<ModelCacheEntry>,
}

#[derive(Debug, Clone)]
pub struct CachedModels {
    pub models: Vec<String>,
    pub age_secs: u64,
}

// ── Helper Functions ─────────────────────────────────────────────

fn model_cache_path(workspace_dir: &Path) -> PathBuf {
    workspace_dir.join("state").join(MODEL_CACHE_FILE)
}

fn now_unix_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |duration| duration.as_secs())
}

fn build_model_fetch_client() -> Result<reqwest::blocking::Client> {
    reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(8))
        .connect_timeout(Duration::from_secs(4))
        .build()
        .context("failed to build model-fetch HTTP client")
}

fn normalize_model_ids(ids: Vec<String>) -> Vec<String> {
    let mut unique = BTreeMap::new();
    for id in ids {
        let trimmed = id.trim();
        if !trimmed.is_empty() {
            unique
                .entry(trimmed.to_ascii_lowercase())
                .or_insert_with(|| trimmed.to_string());
        }
    }
    unique.into_values().collect()
}

pub fn humanize_age(age_secs: u64) -> String {
    if age_secs < 60 {
        format!("{age_secs}s")
    } else if age_secs < 60 * 60 {
        format!("{}m", age_secs / 60)
    } else {
        format!("{}h", age_secs / (60 * 60))
    }
}

pub fn build_model_options(model_ids: Vec<String>, source: &str) -> Vec<(String, String)> {
    model_ids
        .into_iter()
        .map(|model_id| {
            let label = format!("{model_id} ({source})");
            (model_id, label)
        })
        .collect()
}

fn print_model_preview(models: &[String]) {
    for model in models.iter().take(MODEL_PREVIEW_LIMIT) {
        let _ = prompts::log::step(model.as_str());
    }

    if models.len() > MODEL_PREVIEW_LIMIT {
        let _ = prompts::log::step(format!(
            "... and {} more",
            models.len() - MODEL_PREVIEW_LIMIT
        ));
    }
}

// ── Parse Functions ──────────────────────────────────────────────

fn parse_openai_compatible_model_ids(payload: &Value) -> Vec<String> {
    let mut models = Vec::new();

    if let Some(data) = payload.get("data").and_then(Value::as_array) {
        for model in data {
            if let Some(id) = model.get("id").and_then(Value::as_str) {
                models.push(id.to_string());
            }
        }
    } else if let Some(data) = payload.as_array() {
        for model in data {
            if let Some(id) = model.get("id").and_then(Value::as_str) {
                models.push(id.to_string());
            }
        }
    }

    normalize_model_ids(models)
}

fn parse_gemini_model_ids(payload: &Value) -> Vec<String> {
    let Some(models) = payload.get("models").and_then(Value::as_array) else {
        return Vec::new();
    };

    let mut ids = Vec::new();
    for model in models {
        let supports_generate_content = model
            .get("supportedGenerationMethods")
            .and_then(Value::as_array)
            .is_none_or(|methods| {
                methods
                    .iter()
                    .any(|method| method.as_str() == Some("generateContent"))
            });

        if !supports_generate_content {
            continue;
        }

        if let Some(name) = model.get("name").and_then(Value::as_str) {
            ids.push(name.trim_start_matches("models/").to_string());
        }
    }

    normalize_model_ids(ids)
}

fn parse_ollama_model_ids(payload: &Value) -> Vec<String> {
    let Some(models) = payload.get("models").and_then(Value::as_array) else {
        return Vec::new();
    };

    let mut ids = Vec::new();
    for model in models {
        if let Some(name) = model.get("name").and_then(Value::as_str) {
            ids.push(name.to_string());
        }
    }

    normalize_model_ids(ids)
}

// ── Fetch Functions ──────────────────────────────────────────────

fn fetch_openai_compatible_models(
    endpoint: &str,
    api_key: Option<&str>,
    allow_unauthenticated: bool,
) -> Result<Vec<String>> {
    let client = build_model_fetch_client()?;
    let mut request = client.get(endpoint);

    if let Some(api_key) = api_key {
        request = request.bearer_auth(api_key);
    } else if !allow_unauthenticated {
        bail!("model fetch requires API key for endpoint {endpoint}");
    }

    let payload: Value = request
        .send()
        .and_then(reqwest::blocking::Response::error_for_status)
        .with_context(|| format!("model fetch failed: GET {endpoint}"))?
        .json()
        .context("failed to parse model list response")?;

    Ok(parse_openai_compatible_model_ids(&payload))
}

fn fetch_openrouter_models(api_key: Option<&str>) -> Result<Vec<String>> {
    let client = build_model_fetch_client()?;
    let mut request = client.get("https://openrouter.ai/api/v1/models");
    if let Some(api_key) = api_key {
        request = request.bearer_auth(api_key);
    }

    let payload: Value = request
        .send()
        .and_then(reqwest::blocking::Response::error_for_status)
        .context("model fetch failed: GET https://openrouter.ai/api/v1/models")?
        .json()
        .context("failed to parse OpenRouter model list response")?;

    Ok(parse_openai_compatible_model_ids(&payload))
}

fn fetch_anthropic_models(api_key: Option<&str>) -> Result<Vec<String>> {
    let Some(api_key) = api_key else {
        bail!("Anthropic model fetch requires API key or OAuth token");
    };

    let client = build_model_fetch_client()?;
    let mut request = client
        .get("https://api.anthropic.com/v1/models")
        .header("anthropic-version", "2023-06-01");

    if api_key.starts_with("sk-ant-oat01-") {
        request = request
            .header("Authorization", format!("Bearer {api_key}"))
            .header("anthropic-beta", "oauth-2025-04-20");
    } else {
        request = request.header("x-api-key", api_key);
    }

    let response = request
        .send()
        .context("model fetch failed: GET https://api.anthropic.com/v1/models")?;

    let status = response.status();
    if !status.is_success() {
        let body = response.text().unwrap_or_default();
        bail!("Anthropic model list request failed (HTTP {status}): {body}");
    }

    let payload: Value = response
        .json()
        .context("failed to parse Anthropic model list response")?;

    Ok(parse_openai_compatible_model_ids(&payload))
}

fn fetch_gemini_models(api_key: Option<&str>) -> Result<Vec<String>> {
    let Some(api_key) = api_key else {
        bail!("Gemini model fetch requires API key");
    };

    let client = build_model_fetch_client()?;
    let payload: Value = client
        .get("https://generativelanguage.googleapis.com/v1beta/models")
        .query(&[("key", api_key), ("pageSize", "200")])
        .send()
        .and_then(reqwest::blocking::Response::error_for_status)
        .context("model fetch failed: GET Gemini models")?
        .json()
        .context("failed to parse Gemini model list response")?;

    Ok(parse_gemini_model_ids(&payload))
}

fn fetch_ollama_models() -> Result<Vec<String>> {
    let client = build_model_fetch_client()?;
    let payload: Value = client
        .get("http://localhost:11434/api/tags")
        .send()
        .and_then(reqwest::blocking::Response::error_for_status)
        .context("model fetch failed: GET http://localhost:11434/api/tags")?
        .json()
        .context("failed to parse Ollama model list response")?;

    Ok(parse_ollama_model_ids(&payload))
}

// ── Ollama Helpers ───────────────────────────────────────────────

pub fn normalize_ollama_endpoint_url(raw_url: &str) -> String {
    let trimmed = raw_url.trim().trim_end_matches('/');
    if trimmed.is_empty() {
        return String::new();
    }
    trimmed
        .strip_suffix("/api")
        .unwrap_or(trimmed)
        .trim_end_matches('/')
        .to_string()
}

fn ollama_endpoint_is_local(endpoint_url: &str) -> bool {
    reqwest::Url::parse(endpoint_url)
        .ok()
        .and_then(|url| url.host_str().map(|host| host.to_ascii_lowercase()))
        .is_some_and(|host| matches!(host.as_str(), "localhost" | "127.0.0.1" | "::1" | "0.0.0.0"))
}

pub fn ollama_uses_remote_endpoint(provider_api_url: Option<&str>) -> bool {
    let Some(endpoint) = provider_api_url else {
        return false;
    };

    let normalized = normalize_ollama_endpoint_url(endpoint);
    if normalized.is_empty() {
        return false;
    }

    !ollama_endpoint_is_local(&normalized)
}

fn resolve_live_models_endpoint(
    provider_name: &str,
    provider_api_url: Option<&str>,
) -> Option<String> {
    if let Some(raw_base) = provider_name.strip_prefix("custom:") {
        let normalized = raw_base.trim().trim_end_matches('/');
        if normalized.is_empty() {
            return None;
        }
        if normalized.ends_with("/models") {
            return Some(normalized.to_string());
        }
        return Some(format!("{normalized}/models"));
    }

    if matches!(
        canonical_provider_name(provider_name),
        "llamacpp" | "sglang" | "vllm" | "osaurus"
    )
        && let Some(url) = provider_api_url
            .map(str::trim)
            .filter(|url| !url.is_empty())
        {
            let normalized = url.trim_end_matches('/');
            if normalized.ends_with("/models") {
                return Some(normalized.to_string());
            }
            return Some(format!("{normalized}/models"));
        }

    if canonical_provider_name(provider_name) == "openai-codex"
        && let Some(url) = provider_api_url
            .map(str::trim)
            .filter(|url| !url.is_empty())
        {
            let normalized = url.trim_end_matches('/');
            if normalized.ends_with("/models") {
                return Some(normalized.to_string());
            }
            return Some(format!("{normalized}/models"));
        }

    models_endpoint_for_provider(provider_name).map(str::to_string)
}

pub fn fetch_live_models_for_provider(
    provider_name: &str,
    api_key: &str,
    provider_api_url: Option<&str>,
) -> Result<Vec<String>> {
    let requested_provider_name = provider_name;
    let provider_name = canonical_provider_name(provider_name);
    let ollama_remote = provider_name == "ollama" && ollama_uses_remote_endpoint(provider_api_url);
    let api_key = if api_key.trim().is_empty() {
        if provider_name == "ollama" && !ollama_remote {
            None
        } else {
            std::env::var(provider_env_var(provider_name))
                .ok()
                .or_else(|| {
                    // Anthropic also accepts OAuth setup-tokens via ANTHROPIC_OAUTH_TOKEN
                    if provider_name == "anthropic" {
                        std::env::var("ANTHROPIC_OAUTH_TOKEN").ok()
                    } else if provider_name == "minimax" {
                        std::env::var("MINIMAX_OAUTH_TOKEN").ok()
                    } else {
                        None
                    }
                })
                .map(|value| value.trim().to_string())
                .filter(|value| !value.is_empty())
        }
    } else {
        Some(api_key.trim().to_string())
    };

    let models = match provider_name {
        "openrouter" => fetch_openrouter_models(api_key.as_deref())?,
        "anthropic" => fetch_anthropic_models(api_key.as_deref())?,
        "gemini" => fetch_gemini_models(api_key.as_deref())?,
        "ollama" => {
            if ollama_remote {
                // Remote Ollama endpoints can serve cloud-routed models.
                vec![
                    "glm-5:cloud".to_string(),
                    "glm-4.7:cloud".to_string(),
                    "gpt-oss:20b:cloud".to_string(),
                    "gpt-oss:120b:cloud".to_string(),
                    "gemini-3-flash-preview:cloud".to_string(),
                    "qwen3-coder-next:cloud".to_string(),
                    "qwen3-coder:480b:cloud".to_string(),
                    "kimi-k2.5:cloud".to_string(),
                    "minimax-m2.5:cloud".to_string(),
                    "deepseek-v3.1:671b:cloud".to_string(),
                ]
            } else {
                fetch_ollama_models()?
                    .into_iter()
                    .filter(|model_id| !model_id.ends_with(":cloud"))
                    .collect()
            }
        }
        _ => {
            if let Some(endpoint) =
                resolve_live_models_endpoint(requested_provider_name, provider_api_url)
            {
                let allow_unauthenticated =
                    allows_unauthenticated_model_fetch(requested_provider_name);
                fetch_openai_compatible_models(
                    &endpoint,
                    api_key.as_deref(),
                    allow_unauthenticated,
                )?
            } else {
                Vec::new()
            }
        }
    };

    Ok(models)
}

// ── Model Cache Functions ────────────────────────────────────────

async fn load_model_cache_state(workspace_dir: &Path) -> Result<ModelCacheState> {
    let path = model_cache_path(workspace_dir);
    if !path.exists() {
        return Ok(ModelCacheState::default());
    }

    let raw = fs::read_to_string(&path)
        .await
        .with_context(|| format!("failed to read model cache at {}", path.display()))?;

    match serde_json::from_str::<ModelCacheState>(&raw) {
        Ok(state) => Ok(state),
        Err(_) => Ok(ModelCacheState::default()),
    }
}

async fn save_model_cache_state(workspace_dir: &Path, state: &ModelCacheState) -> Result<()> {
    let path = model_cache_path(workspace_dir);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).await.with_context(|| {
            format!(
                "failed to create model cache directory {}",
                parent.display()
            )
        })?;
    }

    let json = serde_json::to_vec_pretty(state).context("failed to serialize model cache")?;
    fs::write(&path, json)
        .await
        .with_context(|| format!("failed to write model cache at {}", path.display()))?;

    Ok(())
}

pub async fn cache_live_models_for_provider(
    workspace_dir: &Path,
    provider_name: &str,
    models: &[String],
) -> Result<()> {
    let normalized_models = normalize_model_ids(models.to_vec());
    if normalized_models.is_empty() {
        return Ok(());
    }

    let mut state = load_model_cache_state(workspace_dir).await?;
    let now = now_unix_secs();

    if let Some(entry) = state
        .entries
        .iter_mut()
        .find(|entry| entry.provider == provider_name)
    {
        entry.fetched_at_unix = now;
        entry.models = normalized_models;
    } else {
        state.entries.push(ModelCacheEntry {
            provider: provider_name.to_string(),
            fetched_at_unix: now,
            models: normalized_models,
        });
    }

    save_model_cache_state(workspace_dir, &state).await
}

async fn load_cached_models_for_provider_internal(
    workspace_dir: &Path,
    provider_name: &str,
    ttl_secs: Option<u64>,
) -> Result<Option<CachedModels>> {
    let state = load_model_cache_state(workspace_dir).await?;
    let now = now_unix_secs();

    let Some(entry) = state
        .entries
        .into_iter()
        .find(|entry| entry.provider == provider_name)
    else {
        return Ok(None);
    };

    if entry.models.is_empty() {
        return Ok(None);
    }

    let age_secs = now.saturating_sub(entry.fetched_at_unix);
    if ttl_secs.is_some_and(|ttl| age_secs > ttl) {
        return Ok(None);
    }

    Ok(Some(CachedModels {
        models: entry.models,
        age_secs,
    }))
}

pub async fn load_cached_models_for_provider(
    workspace_dir: &Path,
    provider_name: &str,
    ttl_secs: u64,
) -> Result<Option<CachedModels>> {
    load_cached_models_for_provider_internal(workspace_dir, provider_name, Some(ttl_secs)).await
}

pub async fn load_any_cached_models_for_provider(
    workspace_dir: &Path,
    provider_name: &str,
) -> Result<Option<CachedModels>> {
    load_cached_models_for_provider_internal(workspace_dir, provider_name, None).await
}

// ── Public API Functions ─────────────────────────────────────────

pub async fn run_models_refresh(
    config: &Config,
    provider_override: Option<&str>,
    force: bool,
) -> Result<()> {
    let provider_name = provider_override
        .or(config.default_provider.as_deref())
        .unwrap_or("openrouter")
        .trim()
        .to_string();

    if provider_name.is_empty() {
        anyhow::bail!("Provider name cannot be empty");
    }

    if !supports_live_model_fetch(&provider_name) {
        anyhow::bail!("Provider '{provider_name}' does not support live model discovery yet");
    }

    if !force
        && let Some(cached) = load_cached_models_for_provider(
            &config.workspace_dir,
            &provider_name,
            MODEL_CACHE_TTL_SECS,
        )
        .await?
        {
            prompts::log::info(format!(
                "Using cached model list for '{}' (updated {} ago):",
                provider_name,
                humanize_age(cached.age_secs)
            ))?;
            print_model_preview(&cached.models);
            prompts::log::info(format!(
                "Tip: run `zeroclaw models refresh --force --provider {}` to fetch latest now.",
                provider_name
            ))?;
            return Ok(());
        }

    let api_key = config.api_key.clone().unwrap_or_default();

    match fetch_live_models_for_provider(&provider_name, &api_key, config.api_url.as_deref()) {
        Ok(models) if !models.is_empty() => {
            cache_live_models_for_provider(&config.workspace_dir, &provider_name, &models).await?;
            prompts::log::success(format!(
                "Refreshed '{}' model cache with {} models.",
                provider_name,
                models.len()
            ))?;
            print_model_preview(&models);
            Ok(())
        }
        Ok(_) => {
            if let Some(stale_cache) =
                load_any_cached_models_for_provider(&config.workspace_dir, &provider_name).await?
            {
                prompts::log::warning(format!(
                    "Provider returned no models; using stale cache (updated {} ago):",
                    humanize_age(stale_cache.age_secs)
                ))?;
                print_model_preview(&stale_cache.models);
                return Ok(());
            }

            anyhow::bail!("Provider '{}' returned an empty model list", provider_name)
        }
        Err(error) => {
            if let Some(stale_cache) =
                load_any_cached_models_for_provider(&config.workspace_dir, &provider_name).await?
            {
                prompts::log::warning(format!(
                    "Live refresh failed ({}). Falling back to stale cache (updated {} ago):",
                    error,
                    humanize_age(stale_cache.age_secs)
                ))?;
                print_model_preview(&stale_cache.models);
                return Ok(());
            }

            Err(error)
                .with_context(|| format!("failed to refresh models for provider '{provider_name}'"))
        }
    }
}

pub async fn run_models_list(config: &Config, provider_override: Option<&str>) -> Result<()> {
    let provider_name = provider_override
        .or(config.default_provider.as_deref())
        .unwrap_or("openrouter");

    let cached = load_any_cached_models_for_provider(&config.workspace_dir, provider_name).await?;

    let Some(cached) = cached else {
        prompts::log::warning(format!(
            "No cached models for '{provider_name}'. Run: zeroclaw models refresh --provider {provider_name}"
        ))?;
        return Ok(());
    };

    prompts::log::info(format!(
        "{} models for '{}' (cached {} ago):",
        cached.models.len(),
        provider_name,
        humanize_age(cached.age_secs)
    ))?;

    for model in &cached.models {
        if config.default_model.as_deref() == Some(model.as_str()) {
            prompts::log::success(format!("* {model}"))?;
        } else {
            prompts::log::step(model.as_str())?;
        }
    }

    Ok(())
}

pub async fn run_models_set(config: &Config, model: &str) -> Result<()> {
    let model = model.trim();
    if model.is_empty() {
        anyhow::bail!("Model name cannot be empty");
    }

    let mut updated = config.clone();
    updated.default_model = Some(model.to_string());
    updated.save().await?;

    prompts::log::success(format!("Default model set to '{model}'."))?;

    Ok(())
}

pub async fn run_models_status(config: &Config) -> Result<()> {
    let provider = config.default_provider.as_deref().unwrap_or("openrouter");
    let model = config.default_model.as_deref().unwrap_or("(not set)");

    prompts::log::info(format!("Provider:  {provider}"))?;
    prompts::log::info(format!("Model:     {model}"))?;
    prompts::log::info(format!("Temp:      {:.1}", config.default_temperature))?;

    match load_any_cached_models_for_provider(&config.workspace_dir, provider).await? {
        Some(cached) => {
            prompts::log::info(format!(
                "Cache:     {} models (updated {} ago)",
                cached.models.len(),
                humanize_age(cached.age_secs)
            ))?;
            let fresh = cached.age_secs < MODEL_CACHE_TTL_SECS;
            if fresh {
                prompts::log::success("Freshness: fresh")?;
            } else {
                prompts::log::warning("Freshness: stale")?;
            }
        }
        None => {
            prompts::log::warning("Cache:     none")?;
        }
    }

    Ok(())
}

pub async fn cached_model_catalog_stats(
    config: &Config,
    provider_name: &str,
) -> Result<Option<(usize, u64)>> {
    let Some(cached) =
        load_any_cached_models_for_provider(&config.workspace_dir, provider_name).await?
    else {
        return Ok(None);
    };
    Ok(Some((cached.models.len(), cached.age_secs)))
}

pub async fn run_models_refresh_all(config: &Config, force: bool) -> Result<()> {
    let mut targets: Vec<String> = crate::providers::list_providers()
        .into_iter()
        .map(|provider| provider.name.to_string())
        .filter(|name| supports_live_model_fetch(name))
        .collect();

    targets.sort();
    targets.dedup();

    if targets.is_empty() {
        anyhow::bail!("No providers support live model discovery");
    }

    prompts::log::info(format!(
        "Refreshing model catalogs for {} providers (force: {})",
        targets.len(),
        if force { "yes" } else { "no" }
    ))?;

    let mut ok_count = 0usize;
    let mut fail_count = 0usize;

    for provider_name in &targets {
        prompts::log::step(format!("== {} ==", provider_name))?;
        match run_models_refresh(config, Some(provider_name), force).await {
            Ok(()) => {
                ok_count += 1;
            }
            Err(error) => {
                fail_count += 1;
                prompts::log::error(format!("failed: {error}"))?;
            }
        }
    }

    prompts::log::info(format!(
        "Summary: {} succeeded, {} failed",
        ok_count, fail_count
    ))?;

    if ok_count == 0 {
        anyhow::bail!("Model refresh failed for all providers")
    }
    Ok(())
}
