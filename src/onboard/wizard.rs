// Import from extracted modules
use super::channel_setup::setup_channels;
use super::provider_setup::setup_provider;

use crate::config::{
    AutonomyConfig, BrowserConfig, ChannelsConfig, ComposioConfig, Config, HeartbeatConfig,
    MemoryConfig, ObservabilityConfig, RuntimeConfig, SecretsConfig, StorageConfig,
};
use crate::hardware::{self, HardwareConfig};
use crate::memory::{
    default_memory_backend_key, memory_backend_profile, selectable_memory_backends,
};
use crate::providers::{canonical_china_provider_name, is_qwen_oauth_alias};
use anyhow::{Context, Result, bail};
use std::io::IsTerminal;
use std::path::{Path, PathBuf};
use tokio::fs;

// Import UI framework
use crate::ui::prompts::PromptInteraction;
use crate::ui::{effects::RainbowEffect, prompts, splash};

// ── Project context collected during wizard ──────────────────────

/// User-provided personalization baked into workspace MD files.
#[derive(Debug, Clone, Default)]
pub struct ProjectContext {
    pub user_name: String,
    pub timezone: String,
    pub agent_name: String,
    pub communication_style: String,
}

// ── Banner ───────────────────────────────────────────────────────

// Banner is now handled by onboard::splash::render_dx_logo()

pub const LIVE_MODEL_MAX_OPTIONS: usize = 120;
pub const MODEL_PREVIEW_LIMIT: usize = 20;
pub const MODEL_CACHE_FILE: &str = "models_cache.json";
pub const MODEL_CACHE_TTL_SECS: u64 = 12 * 60 * 60;
pub const CUSTOM_MODEL_SENTINEL: &str = "__custom_model__";

fn has_launchable_channels(channels: &ChannelsConfig) -> bool {
    channels.channels_except_webhook().iter().any(|(_, ok)| *ok)
}

// ── Main wizard entry point ──────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InteractiveOnboardingMode {
    FullOnboarding,
    UpdateProviderOnly,
}

pub async fn run_wizard(force: bool) -> Result<Config> {
    // Initialize rainbow effect and show splash screen
    let rainbow = RainbowEffect::new();
    print!("\x1B[2J\x1B[H"); // Clear screen
    splash::render_dx_logo(&rainbow)?;
    println!();
    std::thread::sleep(std::time::Duration::from_millis(800));

    // Welcome with onboard UI
    prompts::intro("ZeroClaw Setup Wizard")?;
    prompts::section_with_width("Welcome to ZeroClaw", 80, |lines: &mut Vec<String>| {
        lines.push("The fastest, smallest AI assistant.".to_string());
        lines.push("100% Rust. 100% Agnostic. Zero compromise.".to_string());
        lines.push(String::new());
        lines.push("This wizard will configure your agent in under 60 seconds.".to_string());
    })?;

    print_step(1, 9, "Workspace Setup")?;
    let (workspace_dir, config_path) = setup_workspace().await?;
    match resolve_interactive_onboarding_mode(&config_path, force)? {
        InteractiveOnboardingMode::FullOnboarding => {}
        InteractiveOnboardingMode::UpdateProviderOnly => {
            return Box::pin(run_provider_update_wizard(&workspace_dir, &config_path)).await;
        }
    }

    print_step(2, 9, "AI Provider & API Key")?;
    let (provider, api_key, model, provider_api_url) = setup_provider(&workspace_dir).await?;

    print_step(3, 9, "Channels (How You Talk to ZeroClaw)")?;
    let channels_config = setup_channels()?;

    print_step(4, 9, "Tunnel (Expose to Internet)")?;
    let tunnel_config = setup_tunnel()?;

    print_step(5, 9, "Tool Mode & Security")?;
    let (composio_config, secrets_config) = setup_tool_mode()?;

    print_step(6, 9, "Hardware (Physical World)")?;
    let hardware_config = setup_hardware()?;

    print_step(7, 9, "Memory Configuration")?;
    let memory_config = setup_memory()?;

    print_step(8, 9, "Project Context (Personalize Your Agent)")?;
    let project_ctx = setup_project_context()?;

    print_step(9, 9, "Workspace Files")?;
    scaffold_workspace(&workspace_dir, &project_ctx).await?;

    // ── Build config ──
    // Defaults: SQLite memory, supervised autonomy, workspace-scoped, native runtime
    let config = Config {
        workspace_dir: workspace_dir.clone(),
        config_path: config_path.clone(),
        api_key: if api_key.is_empty() {
            None
        } else {
            Some(api_key)
        },
        api_url: provider_api_url,
        api_path: None,
        default_provider: Some(provider),
        default_model: Some(model),
        model_providers: std::collections::HashMap::new(),
        default_temperature: 0.7,
        provider_timeout_secs: 120,
        extra_headers: std::collections::HashMap::new(),
        observability: ObservabilityConfig::default(),
        autonomy: AutonomyConfig::default(),
        backup: crate::config::BackupConfig::default(),
        data_retention: crate::config::DataRetentionConfig::default(),
        cloud_ops: crate::config::CloudOpsConfig::default(),
        conversational_ai: crate::config::ConversationalAiConfig::default(),
        security: crate::config::SecurityConfig::default(),
        security_ops: crate::config::SecurityOpsConfig::default(),
        runtime: RuntimeConfig::default(),
        reliability: crate::config::ReliabilityConfig::default(),
        scheduler: crate::config::schema::SchedulerConfig::default(),
        agent: crate::config::schema::AgentConfig::default(),
        skills: crate::config::SkillsConfig::default(),
        model_routes: Vec::new(),
        embedding_routes: Vec::new(),
        heartbeat: HeartbeatConfig::default(),
        cron: crate::config::CronConfig::default(),
        channels_config,
        memory: memory_config, // User-selected memory backend
        storage: StorageConfig::default(),
        tunnel: tunnel_config,
        gateway: crate::config::GatewayConfig::default(),
        composio: composio_config,
        microsoft365: crate::config::Microsoft365Config::default(),
        secrets: secrets_config,
        browser: BrowserConfig::default(),
        browser_delegate: crate::tools::browser_delegate::BrowserDelegateConfig::default(),
        http_request: crate::config::HttpRequestConfig::default(),
        openapi: crate::config::schema::OpenApiConfig::default(),
        multimodal: crate::config::MultimodalConfig::default(),
        web_fetch: crate::config::WebFetchConfig::default(),
        web_search: crate::config::WebSearchConfig::default(),
        project_intel: crate::config::ProjectIntelConfig::default(),
        google_workspace: crate::config::GoogleWorkspaceConfig::default(),
        proxy: crate::config::ProxyConfig::default(),
        identity: crate::config::IdentityConfig::default(),
        cost: crate::config::CostConfig::default(),
        peripherals: crate::config::PeripheralsConfig::default(),
        agents: std::collections::HashMap::new(),
        swarms: std::collections::HashMap::new(),
        hooks: crate::config::HooksConfig::default(),
        hardware: hardware_config,
        query_classification: crate::config::QueryClassificationConfig::default(),
        transcription: crate::config::TranscriptionConfig::default(),
        tts: crate::config::TtsConfig::default(),
        mcp: crate::config::McpConfig::default(),
        nodes: crate::config::NodesConfig::default(),
        workspace: crate::config::WorkspaceConfig::default(),
        notion: crate::config::NotionConfig::default(),
        node_transport: crate::config::NodeTransportConfig::default(),
        knowledge: crate::config::KnowledgeConfig::default(),
        linkedin: crate::config::LinkedInConfig::default(),
        plugins: crate::config::PluginsConfig::default(),
        locale: None,
    };

    prompts::log::success(format!("Security: {} | workspace-scoped", "Supervised"))?;
    prompts::log::success(format!(
        "Memory: {} (auto-save: {})",
        config.memory.backend,
        if config.memory.auto_save { "on" } else { "off" }
    ))?;

    config.save().await?;
    persist_workspace_selection(&config.config_path).await?;

    // ── Final summary ────────────────────────────────────────────
    print_summary(&config)?;

    // ── Offer to launch channels immediately ─────────────────────
    let has_channels = has_launchable_channels(&config.channels_config);

    if has_channels && config.api_key.is_some() {
        let launch = prompts::confirm("🚀 Launch channels now? (connected channels → AI → reply)")
            .initial_value(true)
            .interact()?;

        if launch {
            prompts::log::info("Starting channel server...")?;
            // Signal to main.rs to call start_channels after wizard returns
            unsafe {
                std::env::set_var("ZEROCLAW_AUTOSTART_CHANNELS", "1");
            }
        }
    }

    prompts::outro("✦ DX-Agent setup complete!")?;

    // Show train animation (copied from onboard/src/main.rs)
    let rainbow = RainbowEffect::new();
    println!();
    println!("Thanks for using Dx! Here's a celebration train!");
    println!();

    print!("\x1B[2J\x1B[H"); // Clear screen
    for frame in 0..15 {
        print!("\x1B[H"); // Move cursor to top
        let _ = splash::render_train_animation(&rainbow, frame);
        std::thread::sleep(std::time::Duration::from_millis(200));
    }

    Ok(config)
}

/// Interactive repair flow: rerun channel setup only without redoing full onboarding.
pub async fn run_channels_repair_wizard() -> Result<Config> {
    // Initialize rainbow effect and show splash screen
    let rainbow = RainbowEffect::new();
    print!("\x1B[2J\x1B[H"); // Clear screen
    splash::render_dx_logo(&rainbow)?;
    println!();
    std::thread::sleep(std::time::Duration::from_millis(800));

    prompts::intro("Channels Repair Wizard")?;
    prompts::section_with_width("Channel Configuration", 80, |lines: &mut Vec<String>| {
        lines.push("Update channel tokens and allowlists only.".to_string());
        lines.push("Your existing configuration will be preserved.".to_string());
    })?;

    let mut config = Box::pin(Config::load_or_init()).await?;

    print_step(1, 1, "Channels (How You Talk to ZeroClaw)")?;
    config.channels_config = setup_channels()?;
    config.save().await?;
    persist_workspace_selection(&config.config_path).await?;

    prompts::log::success(format!(
        "Channel config saved: {}",
        config.config_path.display()
    ))?;

    let has_channels = has_launchable_channels(&config.channels_config);

    if has_channels && config.api_key.is_some() {
        let launch = prompts::confirm("🚀 Launch channels now? (connected channels → AI → reply)")
            .initial_value(true)
            .interact()?;

        if launch {
            prompts::log::info("Starting channel server...")?;
            // Signal to main.rs to call start_channels after wizard returns
            unsafe {
                std::env::set_var("ZEROCLAW_AUTOSTART_CHANNELS", "1");
            }
        }
    }

    prompts::outro("✦ Channel repair complete!")?;

    // Show train animation (copied from onboard/src/main.rs)
    let rainbow = RainbowEffect::new();
    println!();
    println!("Thanks for using Dx! Here's a celebration train!");
    println!();

    print!("\x1B[2J\x1B[H"); // Clear screen
    for frame in 0..15 {
        print!("\x1B[H"); // Move cursor to top
        let _ = splash::render_train_animation(&rainbow, frame);
        std::thread::sleep(std::time::Duration::from_millis(200));
    }

    Ok(config)
}

/// Interactive flow: update only provider/model/api key while preserving existing config.
async fn run_provider_update_wizard(workspace_dir: &Path, config_path: &Path) -> Result<Config> {
    prompts::log::info(
        "Existing config detected. Running provider-only update mode (preserving channels, memory, tunnel, hooks, and other settings).",
    )?;

    let raw = fs::read_to_string(config_path).await.with_context(|| {
        format!(
            "Failed to read existing config at {}",
            config_path.display()
        )
    })?;
    let mut config: Config = toml::from_str(&raw).with_context(|| {
        format!(
            "Failed to parse existing config at {}",
            config_path.display()
        )
    })?;
    config.workspace_dir = workspace_dir.to_path_buf();
    config.config_path = config_path.to_path_buf();

    print_step(1, 1, "AI Provider & API Key")?;
    let (provider, api_key, model, provider_api_url) = setup_provider(workspace_dir).await?;
    apply_provider_update(&mut config, provider, api_key, model, provider_api_url);

    config.save().await?;
    persist_workspace_selection(&config.config_path).await?;

    prompts::log::success(format!(
        "Provider settings updated at {}",
        config.config_path.display()
    ))?;
    print_summary(&config)?;

    let has_channels = has_launchable_channels(&config.channels_config);
    if has_channels && config.api_key.is_some() {
        let launch = prompts::confirm("🚀 Launch channels now? (connected channels → AI → reply)")
            .initial_value(true)
            .interact()?;

        if launch {
            prompts::log::info("Starting channel server...")?;
            unsafe {
                std::env::set_var("ZEROCLAW_AUTOSTART_CHANNELS", "1");
            }
        }
    }

    prompts::outro("✦ Provider update complete!")?;

    // Show train animation (copied from onboard/src/main.rs)
    let rainbow = RainbowEffect::new();
    println!();
    println!("Thanks for using Dx! Here's a celebration train!");
    println!();

    print!("\x1B[2J\x1B[H"); // Clear screen
    for frame in 0..15 {
        print!("\x1B[H"); // Move cursor to top
        let _ = splash::render_train_animation(&rainbow, frame);
        std::thread::sleep(std::time::Duration::from_millis(200));
    }

    Ok(config)
}

fn apply_provider_update(
    config: &mut Config,
    provider: String,
    api_key: String,
    model: String,
    provider_api_url: Option<String>,
) {
    config.default_provider = Some(provider);
    config.default_model = Some(model);
    config.api_url = provider_api_url;
    config.api_key = if api_key.trim().is_empty() {
        None
    } else {
        Some(api_key)
    };
}

// ── Quick setup (zero prompts) ───────────────────────────────────

/// Non-interactive setup: generates a sensible default config instantly.
/// Use `zeroclaw onboard` or `zeroclaw onboard --api-key sk-... --provider openrouter --memory sqlite|lucid`.
fn backend_key_from_choice(choice: usize) -> &'static str {
    selectable_memory_backends()
        .get(choice)
        .map_or(default_memory_backend_key(), |backend| backend.key)
}

fn memory_config_defaults_for_backend(backend: &str) -> MemoryConfig {
    let profile = memory_backend_profile(backend);

    MemoryConfig {
        backend: backend.to_string(),
        auto_save: profile.auto_save_default,
        hygiene_enabled: profile.uses_sqlite_hygiene,
        archive_after_days: if profile.uses_sqlite_hygiene { 7 } else { 0 },
        purge_after_days: if profile.uses_sqlite_hygiene { 30 } else { 0 },
        conversation_retention_days: 30,
        embedding_provider: "none".to_string(),
        embedding_model: "text-embedding-3-small".to_string(),
        embedding_dimensions: 1536,
        vector_weight: 0.7,
        keyword_weight: 0.3,
        min_relevance_score: 0.4,
        embedding_cache_size: if profile.uses_sqlite_hygiene {
            10000
        } else {
            0
        },
        chunk_max_tokens: 512,
        response_cache_enabled: false,
        response_cache_ttl_minutes: 60,
        response_cache_max_entries: 5_000,
        response_cache_hot_entries: 256,
        snapshot_enabled: false,
        snapshot_on_hygiene: false,
        auto_hydrate: true,
        sqlite_open_timeout_secs: None,
        qdrant: crate::config::QdrantConfig::default(),
    }
}

#[allow(clippy::too_many_lines)]
pub async fn run_quick_setup(
    credential_override: Option<&str>,
    provider: Option<&str>,
    model_override: Option<&str>,
    memory_backend: Option<&str>,
    force: bool,
) -> Result<Config> {
    let home = directories::UserDirs::new()
        .map(|u| u.home_dir().to_path_buf())
        .context("Could not find home directory")?;

    Box::pin(run_quick_setup_with_home(
        credential_override,
        provider,
        model_override,
        memory_backend,
        force,
        &home,
    ))
    .await
}

fn resolve_quick_setup_dirs_with_home(home: &Path) -> (PathBuf, PathBuf) {
    if let Ok(custom_config_dir) = std::env::var("ZEROCLAW_CONFIG_DIR") {
        let trimmed = custom_config_dir.trim();
        if !trimmed.is_empty() {
            let config_dir = PathBuf::from(shellexpand::tilde(trimmed).as_ref());
            return (config_dir.clone(), config_dir.join("workspace"));
        }
    }

    if let Ok(custom_workspace) = std::env::var("ZEROCLAW_WORKSPACE") {
        let trimmed = custom_workspace.trim();
        if !trimmed.is_empty() {
            let expanded = shellexpand::tilde(trimmed);
            return crate::config::schema::resolve_config_dir_for_workspace(&PathBuf::from(
                expanded.as_ref(),
            ));
        }
    }

    let config_dir = home.join(".zeroclaw");
    (config_dir.clone(), config_dir.join("workspace"))
}

#[allow(clippy::too_many_lines)]
async fn run_quick_setup_with_home(
    credential_override: Option<&str>,
    provider: Option<&str>,
    model_override: Option<&str>,
    memory_backend: Option<&str>,
    force: bool,
    home: &Path,
) -> Result<Config> {
    // Initialize rainbow effect and show splash screen
    let rainbow = RainbowEffect::new();
    print!("\x1B[2J\x1B[H"); // Clear screen
    splash::render_dx_logo(&rainbow)?;
    println!();
    std::thread::sleep(std::time::Duration::from_millis(800));

    prompts::intro("ZeroClaw Quick Setup")?;
    prompts::section_with_width("Quick Configuration", 80, |lines: &mut Vec<String>| {
        lines.push("Generating config with sensible defaults...".to_string());
        lines.push("You can customize later with 'zeroclaw onboard'.".to_string());
    })?;

    let (zeroclaw_dir, workspace_dir) = resolve_quick_setup_dirs_with_home(home);
    let config_path = zeroclaw_dir.join("config.toml");

    ensure_onboard_overwrite_allowed(&config_path, force)?;
    fs::create_dir_all(&workspace_dir)
        .await
        .context("Failed to create workspace directory")?;

    let provider_name = provider.unwrap_or("openrouter").to_string();
    let model = model_override
        .map(str::to_string)
        .unwrap_or_else(|| default_model_for_provider(&provider_name));
    let memory_backend_name = memory_backend
        .unwrap_or(default_memory_backend_key())
        .to_string();

    // Create memory config based on backend choice
    let memory_config = memory_config_defaults_for_backend(&memory_backend_name);

    let config = Config {
        workspace_dir: workspace_dir.clone(),
        config_path: config_path.clone(),
        api_key: credential_override.map(|c| {
            let mut s = String::with_capacity(c.len());
            s.push_str(c);
            s
        }),
        api_url: None,
        api_path: None,
        default_provider: Some(provider_name.clone()),
        default_model: Some(model.clone()),
        model_providers: std::collections::HashMap::new(),
        default_temperature: 0.7,
        provider_timeout_secs: 120,
        extra_headers: std::collections::HashMap::new(),
        observability: ObservabilityConfig::default(),
        autonomy: AutonomyConfig::default(),
        backup: crate::config::BackupConfig::default(),
        data_retention: crate::config::DataRetentionConfig::default(),
        cloud_ops: crate::config::CloudOpsConfig::default(),
        conversational_ai: crate::config::ConversationalAiConfig::default(),
        security: crate::config::SecurityConfig::default(),
        security_ops: crate::config::SecurityOpsConfig::default(),
        runtime: RuntimeConfig::default(),
        reliability: crate::config::ReliabilityConfig::default(),
        scheduler: crate::config::schema::SchedulerConfig::default(),
        agent: crate::config::schema::AgentConfig::default(),
        skills: crate::config::SkillsConfig::default(),
        model_routes: Vec::new(),
        embedding_routes: Vec::new(),
        heartbeat: HeartbeatConfig::default(),
        cron: crate::config::CronConfig::default(),
        channels_config: ChannelsConfig::default(),
        memory: memory_config,
        storage: StorageConfig::default(),
        tunnel: crate::config::TunnelConfig::default(),
        gateway: crate::config::GatewayConfig::default(),
        composio: ComposioConfig::default(),
        microsoft365: crate::config::Microsoft365Config::default(),
        secrets: SecretsConfig::default(),
        browser: BrowserConfig::default(),
        browser_delegate: crate::tools::browser_delegate::BrowserDelegateConfig::default(),
        http_request: crate::config::HttpRequestConfig::default(),
        openapi: crate::config::schema::OpenApiConfig::default(),
        multimodal: crate::config::MultimodalConfig::default(),
        web_fetch: crate::config::WebFetchConfig::default(),
        web_search: crate::config::WebSearchConfig::default(),
        project_intel: crate::config::ProjectIntelConfig::default(),
        google_workspace: crate::config::GoogleWorkspaceConfig::default(),
        proxy: crate::config::ProxyConfig::default(),
        identity: crate::config::IdentityConfig::default(),
        cost: crate::config::CostConfig::default(),
        peripherals: crate::config::PeripheralsConfig::default(),
        agents: std::collections::HashMap::new(),
        swarms: std::collections::HashMap::new(),
        hooks: crate::config::HooksConfig::default(),
        hardware: crate::config::HardwareConfig::default(),
        query_classification: crate::config::QueryClassificationConfig::default(),
        transcription: crate::config::TranscriptionConfig::default(),
        tts: crate::config::TtsConfig::default(),
        mcp: crate::config::McpConfig::default(),
        nodes: crate::config::NodesConfig::default(),
        workspace: crate::config::WorkspaceConfig::default(),
        notion: crate::config::NotionConfig::default(),
        node_transport: crate::config::NodeTransportConfig::default(),
        knowledge: crate::config::KnowledgeConfig::default(),
        linkedin: crate::config::LinkedInConfig::default(),
        plugins: crate::config::PluginsConfig::default(),
        locale: None,
    };

    config.save().await?;
    persist_workspace_selection(&config.config_path).await?;

    // Scaffold minimal workspace files
    let default_ctx = ProjectContext {
        user_name: std::env::var("USER").unwrap_or_else(|_| "User".into()),
        timezone: "UTC".into(),
        agent_name: "ZeroClaw".into(),
        communication_style:
            "Be warm, natural, and clear. Use occasional relevant emojis (1-2 max) and avoid robotic phrasing."
                .into(),
    };
    scaffold_workspace(&workspace_dir, &default_ctx).await?;

    prompts::log::success(format!("Workspace: {}", workspace_dir.display()))?;
    prompts::log::success(format!("Provider: {}", provider_name))?;
    prompts::log::success(format!("Model: {}", model))?;
    prompts::log::success(format!(
        "API Key: {}",
        if credential_override.is_some() {
            "set"
        } else {
            "not set (use --api-key or edit config.toml)"
        }
    ))?;
    prompts::log::success("Security: Supervised (workspace-scoped)")?;
    prompts::log::success(format!(
        "Memory: {} (auto-save: {})",
        memory_backend_name,
        if memory_backend_name == "none" {
            "off"
        } else {
            "on"
        }
    ))?;
    prompts::log::success("Secrets: encrypted")?;
    prompts::log::success("Gateway: pairing required (127.0.0.1:8080)")?;
    prompts::log::info("Tunnel: none (local only)")?;
    prompts::log::info("Composio: disabled (sovereign mode)")?;

    prompts::log::success(format!("Config saved: {}", config_path.display()))?;

    prompts::log::step("Next steps:")?;
    if credential_override.is_none() {
        if provider_supports_keyless_local_usage(&provider_name) {
            prompts::log::info("1. Chat:     zeroclaw agent -m \"Hello!\"")?;
            prompts::log::info("2. Gateway:  zeroclaw gateway")?;
            prompts::log::info("3. Status:   zeroclaw status")?;
        } else if provider_supports_device_flow(&provider_name) {
            if canonical_provider_name(&provider_name) == "copilot" {
                prompts::log::info("1. Chat:              zeroclaw agent -m \"Hello!\"")?;
                prompts::log::info("   (device / OAuth auth will prompt on first run)")?;
                prompts::log::info("2. Gateway:           zeroclaw gateway")?;
                prompts::log::info("3. Status:            zeroclaw status")?;
            } else {
                prompts::log::info(format!(
                    "1. Login:             zeroclaw auth login --provider {}",
                    provider_name
                ))?;
                prompts::log::info("2. Chat:              zeroclaw agent -m \"Hello!\"")?;
                prompts::log::info("3. Gateway:           zeroclaw gateway")?;
                prompts::log::info("4. Status:            zeroclaw status")?;
            }
        } else {
            let env_var = provider_env_var(&provider_name);
            prompts::log::info(format!("1. Set your API key:  export {env_var}=\"sk-...\""))?;
            prompts::log::info("2. Or edit:           ~/.zeroclaw/config.toml")?;
            prompts::log::info("3. Chat:              zeroclaw agent -m \"Hello!\"")?;
            prompts::log::info("4. Gateway:           zeroclaw gateway")?;
        }
    } else {
        prompts::log::info("1. Chat:     zeroclaw agent -m \"Hello!\"")?;
        prompts::log::info("2. Gateway:  zeroclaw gateway")?;
        prompts::log::info("3. Status:   zeroclaw status")?;
    }

    Ok(config)
}

pub fn canonical_provider_name(provider_name: &str) -> &str {
    if is_qwen_oauth_alias(provider_name) {
        return "qwen-code";
    }

    if let Some(canonical) = canonical_china_provider_name(provider_name) {
        return canonical;
    }

    match provider_name {
        "grok" => "xai",
        "together" => "together-ai",
        "google" | "google-gemini" => "gemini",
        "github-copilot" => "copilot",
        "openai_codex" | "codex" => "openai-codex",
        "kimi_coding" | "kimi_for_coding" => "kimi-code",
        "nvidia-nim" | "build.nvidia.com" => "nvidia",
        "aws-bedrock" => "bedrock",
        "llama.cpp" => "llamacpp",
        _ => provider_name,
    }
}

pub fn allows_unauthenticated_model_fetch(provider_name: &str) -> bool {
    matches!(
        canonical_provider_name(provider_name),
        "openrouter"
            | "ollama"
            | "llamacpp"
            | "sglang"
            | "vllm"
            | "osaurus"
            | "venice"
            | "astrai"
            | "nvidia"
    )
}

/// Pick a sensible default model for the given provider.
const MINIMAX_ONBOARD_MODELS: [(&str, &str); 5] = [
    ("MiniMax-M2.5", "MiniMax M2.5 (latest, recommended)"),
    ("MiniMax-M2.5-highspeed", "MiniMax M2.5 High-Speed (faster)"),
    ("MiniMax-M2.1", "MiniMax M2.1 (stable)"),
    ("MiniMax-M2.1-highspeed", "MiniMax M2.1 High-Speed (faster)"),
    ("MiniMax-M2", "MiniMax M2 (legacy)"),
];

pub fn default_model_for_provider(provider: &str) -> String {
    match canonical_provider_name(provider) {
        "anthropic" => "claude-sonnet-4-5-20250929".into(),
        "openai" => "gpt-5.2".into(),
        "openai-codex" => "gpt-5-codex".into(),
        "venice" => "zai-org-glm-5".into(),
        "groq" => "llama-3.3-70b-versatile".into(),
        "mistral" => "mistral-large-latest".into(),
        "deepseek" => "deepseek-chat".into(),
        "xai" => "grok-4-1-fast-reasoning".into(),
        "perplexity" => "sonar-pro".into(),
        "fireworks" => "accounts/fireworks/models/llama-v3p3-70b-instruct".into(),
        "novita" => "minimax/minimax-m2.5".into(),
        "together-ai" => "meta-llama/Llama-3.3-70B-Instruct-Turbo".into(),
        "cohere" => "command-a-03-2025".into(),
        "moonshot" => "kimi-k2.5".into(),
        "glm" | "zai" => "glm-5".into(),
        "minimax" => "MiniMax-M2.5".into(),
        "qwen" => "qwen-plus".into(),
        "qwen-code" => "qwen3-coder-plus".into(),
        "ollama" => "llama3.2".into(),
        "llamacpp" => "ggml-org/gpt-oss-20b-GGUF".into(),
        "sglang" | "vllm" | "osaurus" | "opencode-go" => "default".into(),
        "gemini" => "gemini-2.5-pro".into(),
        "kimi-code" => "kimi-for-coding".into(),
        "bedrock" => "anthropic.claude-sonnet-4-5-20250929-v1:0".into(),
        "nvidia" => "meta/llama-3.3-70b-instruct".into(),
        _ => "anthropic/claude-sonnet-4.6".into(),
    }
}

pub fn curated_models_for_provider(provider_name: &str) -> Vec<(String, String)> {
    match canonical_provider_name(provider_name) {
        "openrouter" => vec![
            (
                "anthropic/claude-sonnet-4.6".to_string(),
                "Claude Sonnet 4.6 (balanced, recommended)".to_string(),
            ),
            (
                "openai/gpt-5.2".to_string(),
                "GPT-5.2 (latest flagship)".to_string(),
            ),
            (
                "openai/gpt-5-mini".to_string(),
                "GPT-5 mini (fast, cost-efficient)".to_string(),
            ),
            (
                "google/gemini-3-pro-preview".to_string(),
                "Gemini 3 Pro Preview (frontier reasoning)".to_string(),
            ),
            (
                "x-ai/grok-4.1-fast".to_string(),
                "Grok 4.1 Fast (reasoning + speed)".to_string(),
            ),
            (
                "deepseek/deepseek-v3.2".to_string(),
                "DeepSeek V3.2 (agentic + affordable)".to_string(),
            ),
            (
                "meta-llama/llama-4-maverick".to_string(),
                "Llama 4 Maverick (open model)".to_string(),
            ),
        ],
        "anthropic" => vec![
            (
                "claude-sonnet-4-5-20250929".to_string(),
                "Claude Sonnet 4.5 (balanced, recommended)".to_string(),
            ),
            (
                "claude-opus-4-6".to_string(),
                "Claude Opus 4.6 (best quality)".to_string(),
            ),
            (
                "claude-haiku-4-5-20251001".to_string(),
                "Claude Haiku 4.5 (fastest, cheapest)".to_string(),
            ),
        ],
        "openai" => vec![
            (
                "gpt-5.2".to_string(),
                "GPT-5.2 (latest coding/agentic flagship)".to_string(),
            ),
            (
                "gpt-5-mini".to_string(),
                "GPT-5 mini (faster, cheaper)".to_string(),
            ),
            (
                "gpt-5-nano".to_string(),
                "GPT-5 nano (lowest latency/cost)".to_string(),
            ),
            (
                "gpt-5.2-codex".to_string(),
                "GPT-5.2 Codex (agentic coding)".to_string(),
            ),
        ],
        "openai-codex" => vec![
            (
                "gpt-5-codex".to_string(),
                "GPT-5 Codex (recommended)".to_string(),
            ),
            (
                "gpt-5.2-codex".to_string(),
                "GPT-5.2 Codex (agentic coding)".to_string(),
            ),
            ("o4-mini".to_string(), "o4-mini (fallback)".to_string()),
        ],
        "venice" => vec![
            (
                "zai-org-glm-5".to_string(),
                "GLM-5 via Venice (agentic flagship)".to_string(),
            ),
            (
                "claude-sonnet-4-6".to_string(),
                "Claude Sonnet 4.6 via Venice (best quality)".to_string(),
            ),
            (
                "deepseek-v3.2".to_string(),
                "DeepSeek V3.2 via Venice (strong value)".to_string(),
            ),
            (
                "grok-41-fast".to_string(),
                "Grok 4.1 Fast via Venice (low latency)".to_string(),
            ),
        ],
        "groq" => vec![
            (
                "llama-3.3-70b-versatile".to_string(),
                "Llama 3.3 70B (fast, recommended)".to_string(),
            ),
            (
                "openai/gpt-oss-120b".to_string(),
                "GPT-OSS 120B (strong open-weight)".to_string(),
            ),
            (
                "openai/gpt-oss-20b".to_string(),
                "GPT-OSS 20B (cost-efficient open-weight)".to_string(),
            ),
        ],
        "mistral" => vec![
            (
                "mistral-large-latest".to_string(),
                "Mistral Large (latest flagship)".to_string(),
            ),
            (
                "mistral-medium-latest".to_string(),
                "Mistral Medium (balanced)".to_string(),
            ),
            (
                "codestral-latest".to_string(),
                "Codestral (code-focused)".to_string(),
            ),
            (
                "devstral-latest".to_string(),
                "Devstral (software engineering specialist)".to_string(),
            ),
        ],
        "deepseek" => vec![
            (
                "deepseek-chat".to_string(),
                "DeepSeek Chat (mapped to V3.2 non-thinking)".to_string(),
            ),
            (
                "deepseek-reasoner".to_string(),
                "DeepSeek Reasoner (mapped to V3.2 thinking)".to_string(),
            ),
        ],
        "xai" => vec![
            (
                "grok-4-1-fast-reasoning".to_string(),
                "Grok 4.1 Fast Reasoning (recommended)".to_string(),
            ),
            (
                "grok-4-1-fast-non-reasoning".to_string(),
                "Grok 4.1 Fast Non-Reasoning (low latency)".to_string(),
            ),
            (
                "grok-code-fast-1".to_string(),
                "Grok Code Fast 1 (coding specialist)".to_string(),
            ),
            ("grok-4".to_string(), "Grok 4 (max quality)".to_string()),
        ],
        "perplexity" => vec![
            (
                "sonar-pro".to_string(),
                "Sonar Pro (flagship web-grounded model)".to_string(),
            ),
            (
                "sonar-reasoning-pro".to_string(),
                "Sonar Reasoning Pro (complex multi-step reasoning)".to_string(),
            ),
            (
                "sonar-deep-research".to_string(),
                "Sonar Deep Research (long-form research)".to_string(),
            ),
            ("sonar".to_string(), "Sonar (search, fast)".to_string()),
        ],
        "fireworks" => vec![
            (
                "accounts/fireworks/models/llama-v3p3-70b-instruct".to_string(),
                "Llama 3.3 70B".to_string(),
            ),
            (
                "accounts/fireworks/models/mixtral-8x22b-instruct".to_string(),
                "Mixtral 8x22B".to_string(),
            ),
        ],
        "novita" => vec![(
            "minimax/minimax-m2.5".to_string(),
            "MiniMax M2.5".to_string(),
        )],
        "together-ai" => vec![
            (
                "meta-llama/Llama-3.3-70B-Instruct-Turbo".to_string(),
                "Llama 3.3 70B Instruct Turbo (recommended)".to_string(),
            ),
            (
                "moonshotai/Kimi-K2.5".to_string(),
                "Kimi K2.5 (reasoning + coding)".to_string(),
            ),
            (
                "deepseek-ai/DeepSeek-V3.1".to_string(),
                "DeepSeek V3.1 (strong value)".to_string(),
            ),
        ],
        "cohere" => vec![
            (
                "command-a-03-2025".to_string(),
                "Command A (flagship enterprise model)".to_string(),
            ),
            (
                "command-a-reasoning-08-2025".to_string(),
                "Command A Reasoning (agentic reasoning)".to_string(),
            ),
            (
                "command-r-08-2024".to_string(),
                "Command R (stable fast baseline)".to_string(),
            ),
        ],
        "kimi-code" => vec![
            (
                "kimi-for-coding".to_string(),
                "Kimi for Coding (official coding-agent model)".to_string(),
            ),
            (
                "kimi-k2.5".to_string(),
                "Kimi K2.5 (general coding endpoint model)".to_string(),
            ),
        ],
        "moonshot" => vec![
            (
                "kimi-k2.5".to_string(),
                "Kimi K2.5 (latest flagship, recommended)".to_string(),
            ),
            (
                "kimi-k2-thinking".to_string(),
                "Kimi K2 Thinking (deep reasoning + tool use)".to_string(),
            ),
            (
                "kimi-k2-0905-preview".to_string(),
                "Kimi K2 0905 Preview (strong coding)".to_string(),
            ),
        ],
        "glm" | "zai" => vec![
            ("glm-5".to_string(), "GLM-5 (high reasoning)".to_string()),
            (
                "glm-4.7".to_string(),
                "GLM-4.7 (strong general-purpose quality)".to_string(),
            ),
            (
                "glm-4.5-air".to_string(),
                "GLM-4.5 Air (lower latency)".to_string(),
            ),
        ],
        "minimax" => vec![
            (
                "MiniMax-M2.5".to_string(),
                "MiniMax M2.5 (latest flagship)".to_string(),
            ),
            (
                "MiniMax-M2.5-highspeed".to_string(),
                "MiniMax M2.5 High-Speed (fast)".to_string(),
            ),
            (
                "MiniMax-M2.1".to_string(),
                "MiniMax M2.1 (strong coding/reasoning)".to_string(),
            ),
        ],
        "qwen" => vec![
            (
                "qwen-max".to_string(),
                "Qwen Max (highest quality)".to_string(),
            ),
            (
                "qwen-plus".to_string(),
                "Qwen Plus (balanced default)".to_string(),
            ),
            (
                "qwen-turbo".to_string(),
                "Qwen Turbo (fast and cost-efficient)".to_string(),
            ),
        ],
        "qwen-code" => vec![
            (
                "qwen3-coder-plus".to_string(),
                "Qwen3 Coder Plus (recommended for coding workflows)".to_string(),
            ),
            (
                "qwen3.5-plus".to_string(),
                "Qwen3.5 Plus (reasoning + coding)".to_string(),
            ),
            (
                "qwen3-max-2026-01-23".to_string(),
                "Qwen3 Max (high-capability coding model)".to_string(),
            ),
        ],
        "nvidia" => vec![
            (
                "meta/llama-3.3-70b-instruct".to_string(),
                "Llama 3.3 70B Instruct (balanced default)".to_string(),
            ),
            (
                "deepseek-ai/deepseek-v3.2".to_string(),
                "DeepSeek V3.2 (advanced reasoning + coding)".to_string(),
            ),
            (
                "nvidia/llama-3.3-nemotron-super-49b-v1.5".to_string(),
                "Llama 3.3 Nemotron Super 49B v1.5 (NVIDIA-tuned)".to_string(),
            ),
            (
                "nvidia/llama-3.1-nemotron-ultra-253b-v1".to_string(),
                "Llama 3.1 Nemotron Ultra 253B v1 (max quality)".to_string(),
            ),
        ],
        "astrai" => vec![
            (
                "anthropic/claude-sonnet-4.6".to_string(),
                "Claude Sonnet 4.6 (balanced default)".to_string(),
            ),
            (
                "openai/gpt-5.2".to_string(),
                "GPT-5.2 (latest flagship)".to_string(),
            ),
            (
                "deepseek/deepseek-v3.2".to_string(),
                "DeepSeek V3.2 (agentic + affordable)".to_string(),
            ),
            (
                "z-ai/glm-5".to_string(),
                "GLM-5 (high reasoning)".to_string(),
            ),
        ],
        "ollama" => vec![
            (
                "llama3.2".to_string(),
                "Llama 3.2 (recommended local)".to_string(),
            ),
            ("mistral".to_string(), "Mistral 7B".to_string()),
            ("codellama".to_string(), "Code Llama".to_string()),
            ("phi3".to_string(), "Phi-3 (small, fast)".to_string()),
        ],
        "llamacpp" => vec![
            (
                "ggml-org/gpt-oss-20b-GGUF".to_string(),
                "GPT-OSS 20B GGUF (llama.cpp server example)".to_string(),
            ),
            (
                "bartowski/Llama-3.3-70B-Instruct-GGUF".to_string(),
                "Llama 3.3 70B GGUF (high quality)".to_string(),
            ),
            (
                "Qwen/Qwen2.5-Coder-7B-Instruct-GGUF".to_string(),
                "Qwen2.5 Coder 7B GGUF (coding-focused)".to_string(),
            ),
        ],
        "sglang" | "vllm" => vec![
            (
                "meta-llama/Llama-3.1-8B-Instruct".to_string(),
                "Llama 3.1 8B Instruct (popular, fast)".to_string(),
            ),
            (
                "meta-llama/Llama-3.1-70B-Instruct".to_string(),
                "Llama 3.1 70B Instruct (high quality)".to_string(),
            ),
            (
                "Qwen/Qwen2.5-Coder-7B-Instruct".to_string(),
                "Qwen2.5 Coder 7B Instruct (coding-focused)".to_string(),
            ),
        ],
        "osaurus" => vec![
            (
                "qwen3-30b-a3b-8bit".to_string(),
                "Qwen3 30B A3B (local, balanced)".to_string(),
            ),
            (
                "gemma-3n-e4b-it-lm-4bit".to_string(),
                "Gemma 3N E4B (local, efficient)".to_string(),
            ),
            (
                "phi-4-mini-reasoning-mlx-4bit".to_string(),
                "Phi-4 Mini Reasoning (local, fast reasoning)".to_string(),
            ),
        ],
        "bedrock" => vec![
            (
                "anthropic.claude-sonnet-4-6".to_string(),
                "Claude Sonnet 4.6 (latest, recommended)".to_string(),
            ),
            (
                "anthropic.claude-opus-4-6-v1".to_string(),
                "Claude Opus 4.6 (strongest)".to_string(),
            ),
            (
                "anthropic.claude-haiku-4-5-20251001-v1:0".to_string(),
                "Claude Haiku 4.5 (fastest, cheapest)".to_string(),
            ),
            (
                "anthropic.claude-sonnet-4-5-20250929-v1:0".to_string(),
                "Claude Sonnet 4.5".to_string(),
            ),
        ],
        "gemini" => vec![
            (
                "gemini-3-pro-preview".to_string(),
                "Gemini 3 Pro Preview (latest frontier reasoning)".to_string(),
            ),
            (
                "gemini-2.5-pro".to_string(),
                "Gemini 2.5 Pro (stable reasoning)".to_string(),
            ),
            (
                "gemini-2.5-flash".to_string(),
                "Gemini 2.5 Flash (best price/performance)".to_string(),
            ),
            (
                "gemini-2.5-flash-lite".to_string(),
                "Gemini 2.5 Flash-Lite (lowest cost)".to_string(),
            ),
        ],
        _ => vec![("default".to_string(), "Default model".to_string())],
    }
}

pub fn supports_live_model_fetch(provider_name: &str) -> bool {
    if provider_name.trim().starts_with("custom:") {
        return true;
    }

    matches!(
        canonical_provider_name(provider_name),
        "openrouter"
            | "openai-codex"
            | "openai"
            | "anthropic"
            | "groq"
            | "mistral"
            | "deepseek"
            | "xai"
            | "together-ai"
            | "gemini"
            | "ollama"
            | "llamacpp"
            | "sglang"
            | "vllm"
            | "osaurus"
            | "astrai"
            | "venice"
            | "fireworks"
            | "novita"
            | "cohere"
            | "moonshot"
            | "glm"
            | "zai"
            | "qwen"
            | "nvidia"
            | "opencode-go"
    )
}

pub fn models_endpoint_for_provider(provider_name: &str) -> Option<&'static str> {
    match provider_name {
        "qwen-intl" => Some("https://dashscope-intl.aliyuncs.com/compatible-mode/v1/models"),
        "dashscope-us" => Some("https://dashscope-us.aliyuncs.com/compatible-mode/v1/models"),
        "moonshot-cn" | "kimi-cn" => Some("https://api.moonshot.cn/v1/models"),
        "glm-cn" | "bigmodel" => Some("https://open.bigmodel.cn/api/paas/v4/models"),
        "zai-cn" | "z.ai-cn" => Some("https://open.bigmodel.cn/api/coding/paas/v4/models"),
        _ => match canonical_provider_name(provider_name) {
            "openai-codex" | "openai" => Some("https://api.openai.com/v1/models"),
            "venice" => Some("https://api.venice.ai/api/v1/models"),
            "groq" => Some("https://api.groq.com/openai/v1/models"),
            "mistral" => Some("https://api.mistral.ai/v1/models"),
            "deepseek" => Some("https://api.deepseek.com/v1/models"),
            "xai" => Some("https://api.x.ai/v1/models"),
            "together-ai" => Some("https://api.together.xyz/v1/models"),
            "fireworks" => Some("https://api.fireworks.ai/inference/v1/models"),
            "novita" => Some("https://api.novita.ai/openai/v1/models"),
            "cohere" => Some("https://api.cohere.com/compatibility/v1/models"),
            "moonshot" => Some("https://api.moonshot.ai/v1/models"),
            "glm" => Some("https://api.z.ai/api/paas/v4/models"),
            "zai" => Some("https://api.z.ai/api/coding/paas/v4/models"),
            "qwen" => Some("https://dashscope.aliyuncs.com/compatible-mode/v1/models"),
            "nvidia" => Some("https://integrate.api.nvidia.com/v1/models"),
            "astrai" => Some("https://as-trai.com/v1/models"),
            "llamacpp" => Some("http://localhost:8080/v1/models"),
            "sglang" => Some("http://localhost:30000/v1/models"),
            "vllm" => Some("http://localhost:8000/v1/models"),
            "osaurus" => Some("http://localhost:1337/v1/models"),
            "opencode-go" => Some("https://opencode.ai/zen/go/v1/models"),
            _ => None,
        },
    }
}

// ── Step helpers ─────────────────────────────────────────────────

pub fn print_step(current: u8, total: u8, title: &str) -> Result<()> {
    prompts::log::step(format!("[{current}/{total}] {title}"))?;
    Ok(())
}

pub fn print_bullet(text: &str) {
    use crate::theme::print_info;
    print_info(format!("  › {text}"));
}

fn resolve_interactive_onboarding_mode(
    config_path: &Path,
    force: bool,
) -> Result<InteractiveOnboardingMode> {
    if !config_path.exists() {
        return Ok(InteractiveOnboardingMode::FullOnboarding);
    }

    if force {
        prompts::log::warning(format!(
            "Existing config detected at {}. Proceeding with full onboarding because --force was provided.",
            config_path.display()
        ))?;
        return Ok(InteractiveOnboardingMode::FullOnboarding);
    }

    if !std::io::stdin().is_terminal() || !std::io::stdout().is_terminal() {
        bail!(
            "Refusing to overwrite existing config at {} in non-interactive mode. Re-run with --force if overwrite is intentional.",
            config_path.display()
        );
    }

    let mode = prompts::select(format!(
        "Existing config found at {}. Select setup mode",
        config_path.display()
    ))
    .item(
        0,
        "Full onboarding (overwrite config.toml)",
        "Complete setup",
    )
    .item(
        1,
        "Update AI provider/model/API key only (preserve existing configuration)",
        "Quick update",
    )
    .item(2, "Cancel", "Exit without changes")
    .interact()?;

    match mode {
        0 => Ok(InteractiveOnboardingMode::FullOnboarding),
        1 => Ok(InteractiveOnboardingMode::UpdateProviderOnly),
        _ => bail!("Onboarding canceled: existing configuration was left unchanged."),
    }
}

fn ensure_onboard_overwrite_allowed(config_path: &Path, force: bool) -> Result<()> {
    if !config_path.exists() {
        return Ok(());
    }

    if force {
        prompts::log::warning(format!(
            "Existing config detected at {}. Proceeding because --force was provided.",
            config_path.display()
        ))?;
        return Ok(());
    }

    #[cfg(test)]
    {
        bail!(
            "Refusing to overwrite existing config at {} in test mode. Re-run with --force if overwrite is intentional.",
            config_path.display()
        );
    }

    #[cfg(not(test))]
    {
        if !std::io::stdin().is_terminal() || !std::io::stdout().is_terminal() {
            bail!(
                "Refusing to overwrite existing config at {} in non-interactive mode. Re-run with --force if overwrite is intentional.",
                config_path.display()
            );
        }

        let confirmed = prompts::confirm(format!(
            "Existing config found at {}. Re-running onboarding will overwrite config.toml and may create missing workspace files (including BOOTSTRAP.md). Continue?",
            config_path.display()
        ))
        .initial_value(false)
        .interact()?;

        if !confirmed {
            bail!("Onboarding canceled: existing configuration was left unchanged.");
        }

        Ok(())
    }
}

async fn persist_workspace_selection(config_path: &Path) -> Result<()> {
    let config_dir = config_path
        .parent()
        .context("Config path must have a parent directory")?;
    crate::config::schema::persist_active_workspace_config_dir(config_dir)
        .await
        .with_context(|| {
            format!(
                "Failed to persist active workspace selection for {}",
                config_dir.display()
            )
        })
}

// ── Step 1: Workspace ────────────────────────────────────────────

async fn setup_workspace() -> Result<(PathBuf, PathBuf)> {
    let (default_config_dir, default_workspace_dir) =
        crate::config::schema::resolve_runtime_dirs_for_onboarding().await?;

    prompts::log::info(format!(
        "Default location: {}",
        default_workspace_dir.display()
    ))?;

    let use_default = prompts::confirm("Use default workspace location?")
        .initial_value(true)
        .interact()?;

    let (config_dir, workspace_dir): (PathBuf, PathBuf) = if use_default {
        (default_config_dir, default_workspace_dir)
    } else {
        let custom = prompts::input::input("Enter workspace path").interact()?;
        let expanded = shellexpand::tilde(&custom).to_string();
        crate::config::schema::resolve_config_dir_for_workspace(&PathBuf::from(expanded))
    };

    let config_path: PathBuf = config_dir.join("config.toml");

    fs::create_dir_all(&workspace_dir)
        .await
        .context("Failed to create workspace directory")?;

    prompts::log::success(format!("Workspace: {}", workspace_dir.display()))?;

    Ok((workspace_dir, config_path))
}

// ── Step 2: Provider & API Key ───────────────────────────────────

pub fn local_provider_choices() -> Vec<(&'static str, &'static str)> {
    vec![
        ("ollama", "Ollama — local models (Llama, Mistral, Phi)"),
        (
            "llamacpp",
            "llama.cpp server — local OpenAI-compatible endpoint",
        ),
        (
            "sglang",
            "SGLang — high-performance local serving framework",
        ),
        ("vllm", "vLLM — high-performance local inference engine"),
        (
            "osaurus",
            "Osaurus — unified AI edge runtime (local MLX + cloud proxy + MCP)",
        ),
    ]
}

/// Map provider name to its conventional env var
pub fn provider_env_var(name: &str) -> &'static str {
    if canonical_provider_name(name) == "qwen-code" {
        return "QWEN_OAUTH_TOKEN";
    }

    match canonical_provider_name(name) {
        "openrouter" => "OPENROUTER_API_KEY",
        "anthropic" => "ANTHROPIC_API_KEY",
        "openai-codex" | "openai" => "OPENAI_API_KEY",
        "ollama" => "OLLAMA_API_KEY",
        "llamacpp" => "LLAMACPP_API_KEY",
        "sglang" => "SGLANG_API_KEY",
        "vllm" => "VLLM_API_KEY",
        "osaurus" => "OSAURUS_API_KEY",
        "venice" => "VENICE_API_KEY",
        "groq" => "GROQ_API_KEY",
        "mistral" => "MISTRAL_API_KEY",
        "deepseek" => "DEEPSEEK_API_KEY",
        "xai" => "XAI_API_KEY",
        "together-ai" => "TOGETHER_API_KEY",
        "fireworks" | "fireworks-ai" => "FIREWORKS_API_KEY",
        "novita" => "NOVITA_API_KEY",
        "perplexity" => "PERPLEXITY_API_KEY",
        "cohere" => "COHERE_API_KEY",
        "kimi-code" => "KIMI_CODE_API_KEY",
        "moonshot" => "MOONSHOT_API_KEY",
        "glm" => "GLM_API_KEY",
        "minimax" => "MINIMAX_API_KEY",
        "qwen" => "DASHSCOPE_API_KEY",
        "qianfan" => "QIANFAN_API_KEY",
        "zai" => "ZAI_API_KEY",
        "synthetic" => "SYNTHETIC_API_KEY",
        "opencode" | "opencode-zen" => "OPENCODE_API_KEY",
        "opencode-go" => "OPENCODE_GO_API_KEY",
        "vercel" | "vercel-ai" => "VERCEL_API_KEY",
        "cloudflare" | "cloudflare-ai" => "CLOUDFLARE_API_KEY",
        "bedrock" | "aws-bedrock" => "AWS_ACCESS_KEY_ID",
        "gemini" => "GEMINI_API_KEY",
        "nvidia" | "nvidia-nim" | "build.nvidia.com" => "NVIDIA_API_KEY",
        "astrai" => "ASTRAI_API_KEY",
        _ => "API_KEY",
    }
}

pub fn provider_supports_keyless_local_usage(provider_name: &str) -> bool {
    matches!(
        canonical_provider_name(provider_name),
        "ollama" | "llamacpp" | "sglang" | "vllm" | "osaurus"
    )
}

pub fn provider_supports_device_flow(provider_name: &str) -> bool {
    matches!(
        canonical_provider_name(provider_name),
        "copilot" | "gemini" | "openai-codex"
    )
}

// ── Step 5: Tool Mode & Security ────────────────────────────────

fn setup_tool_mode() -> Result<(ComposioConfig, SecretsConfig)> {
    prompts::log::info("Choose how ZeroClaw connects to external apps.")?;
    prompts::log::info("You can always change this later in config.toml.")?;

    let choice = prompts::select("Select tool mode")
        .item(
            0,
            "Sovereign (local only)",
            "You manage API keys, full privacy (default)",
        )
        .item(
            1,
            "Composio (managed OAuth)",
            "1000+ apps via OAuth, no raw keys shared",
        )
        .interact()?;

    let composio_config = if choice == 1 {
        prompts::log::info(
            "Composio Setup — 1000+ OAuth integrations (Gmail, Notion, GitHub, Slack, ...)",
        )?;
        prompts::log::info("Get your API key at: https://app.composio.dev/settings")?;
        prompts::log::info("ZeroClaw uses Composio as a tool — your core agent stays local.")?;

        let api_key = prompts::input::input("Composio API key (or Enter to skip)").interact()?;

        if api_key.trim().is_empty() {
            prompts::log::info("Skipped — set composio.api_key in config.toml later")?;
            ComposioConfig::default()
        } else {
            prompts::log::success("Composio: enabled (1000+ OAuth tools available)")?;
            ComposioConfig {
                enabled: true,
                api_key: Some(api_key),
                ..ComposioConfig::default()
            }
        }
    } else {
        prompts::log::success(
            "Tool mode: Sovereign (local only) — full privacy, you own every key",
        )?;
        ComposioConfig::default()
    };

    // ── Encrypted secrets ──
    prompts::log::info("ZeroClaw can encrypt API keys stored in config.toml.")?;
    prompts::log::info(
        "A local key file protects against plaintext exposure and accidental leaks.",
    )?;

    let encrypt = prompts::toggle::toggle("Enable encrypted secret storage?")
        .initial_value(true)
        .interact()?;

    let secrets_config = SecretsConfig { encrypt };

    if encrypt {
        prompts::log::success("Secrets: encrypted — keys encrypted with local key file")?;
    } else {
        prompts::log::warning("Secrets: plaintext — keys stored as plaintext (not recommended)")?;
    }

    Ok((composio_config, secrets_config))
}

// ── Step 6: Hardware (Physical World) ───────────────────────────

fn setup_hardware() -> Result<HardwareConfig> {
    prompts::log::info("ZeroClaw can talk to physical hardware (LEDs, sensors, motors).")?;
    prompts::log::info("Scanning for connected devices...")?;

    // ── Auto-discovery ──
    let devices = hardware::discover_hardware();

    if devices.is_empty() {
        prompts::log::info("No hardware devices detected on this system.")?;
        prompts::log::info("You can enable hardware later in config.toml under [hardware].")?;
    } else {
        prompts::log::success(format!("{} device(s) found:", devices.len()))?;
        for device in &devices {
            let detail = device
                .detail
                .as_deref()
                .map(|d| format!(" ({d})"))
                .unwrap_or_default();
            let path = device
                .device_path
                .as_deref()
                .map(|p| format!(" → {p}"))
                .unwrap_or_default();
            prompts::log::info(format!(
                "{}{}{} [{}]",
                device.name,
                detail,
                path,
                device.transport
            ))?;
        }
    }

    let recommended = hardware::recommended_wizard_default(&devices);

    let choice = prompts::select("How should ZeroClaw interact with the physical world?")
        .item(
            0,
            "🚀 Native — direct GPIO on this Linux board (Raspberry Pi, Orange Pi, etc.)",
            "Direct GPIO access",
        )
        .item(
            1,
            "🔌 Tethered — control an Arduino/ESP32/Nucleo plugged into USB",
            "USB serial connection",
        )
        .item(
            2,
            "🔬 Debug Probe — flash/read MCUs via SWD/JTAG (probe-rs)",
            "Debug probe interface",
        )
        .item(
            3,
            "☁️  Software Only — no hardware access (default)",
            "No hardware",
        )
        .initial_value(recommended)
        .interact()?;

    let mut hw_config = hardware::config_from_wizard_choice(choice, &devices);

    // ── Serial: pick a port if multiple found ──
    if hw_config.transport_mode() == hardware::HardwareTransport::Serial {
        let serial_devices: Vec<&hardware::DiscoveredDevice> = devices
            .iter()
            .filter(|d| d.transport == hardware::HardwareTransport::Serial)
            .collect();

        if serial_devices.len() > 1 {
            let mut select = prompts::select("Multiple serial devices found — select one");
            for (idx, device) in serial_devices.iter().enumerate() {
                let label = format!(
                    "{} ({})",
                    device.device_path.as_deref().unwrap_or("unknown"),
                    device.name
                );
                select = select.item(idx, label.clone(), device.name.clone());
            }
            let port_idx = select.interact()?;
            hw_config.serial_port = serial_devices[port_idx].device_path.clone();
        } else if serial_devices.is_empty() {
            // User chose serial but no device discovered — ask for manual path
            let manual_port = prompts::input::input("Serial port path (e.g. /dev/ttyUSB0)")
                .placeholder("/dev/ttyUSB0")
                .interact()?;
            hw_config.serial_port = Some(manual_port);
        }

        // Baud rate
        let baud_idx = prompts::select("Serial baud rate")
            .item(0, "115200 (default, recommended)", "Standard rate")
            .item(1, "9600 (legacy Arduino)", "Legacy rate")
            .item(2, "57600", "Medium rate")
            .item(3, "230400", "High rate")
            .item(4, "Custom", "Enter custom rate")
            .interact()?;

        hw_config.baud_rate = match baud_idx {
            1 => 9600,
            2 => 57600,
            3 => 230_400,
            4 => {
                let custom = prompts::input::input("Custom baud rate")
                    .placeholder("115200")
                    .interact()?;
                custom.parse::<u32>().unwrap_or(115_200)
            }
            _ => 115_200,
        };
    }

    // ── Probe: ask for target chip ──
    if hw_config.transport_mode() == hardware::HardwareTransport::Probe
        && hw_config.probe_target.is_none()
    {
        let target = prompts::input::input("Target MCU chip (e.g. STM32F411CEUx, nRF52840_xxAA)")
            .placeholder("STM32F411CEUx")
            .interact()?;
        hw_config.probe_target = Some(target);
    }

    // ── Datasheet RAG ──
    if hw_config.enabled {
        let datasheets = prompts::toggle::toggle(
            "Enable datasheet RAG? (index PDF schematics for AI pin lookups)",
        )
        .initial_value(true)
        .interact()?;
        hw_config.workspace_datasheets = datasheets;
    }

    // ── Summary ──
    if hw_config.enabled {
        let transport_label = match hw_config.transport_mode() {
            hardware::HardwareTransport::Native => "Native GPIO".to_string(),
            hardware::HardwareTransport::Serial => format!(
                "Serial → {} @ {} baud",
                hw_config.serial_port.as_deref().unwrap_or("?"),
                hw_config.baud_rate
            ),
            hardware::HardwareTransport::Probe => format!(
                "Probe (SWD/JTAG) → {}",
                hw_config.probe_target.as_deref().unwrap_or("?")
            ),
            hardware::HardwareTransport::None => "Software Only".to_string(),
        };

        prompts::log::success(format!(
            "Hardware: {} | datasheets: {}",
            transport_label,
            if hw_config.workspace_datasheets {
                "on"
            } else {
                "off"
            }
        ))?;
    } else {
        prompts::log::info("Hardware: disabled (software only)")?;
    }

    Ok(hw_config)
}

// ── Step 6: Project Context ─────────────────────────────────────

fn setup_project_context() -> Result<ProjectContext> {
    prompts::log::info("Let's personalize your agent. You can always update these later.")?;
    prompts::log::info("Press Enter to accept defaults.")?;

    let user_name = prompts::input::input("Your name")
        .placeholder("User")
        .interact()?;

    let tz_idx = prompts::select("Your timezone")
        .item(0, "US/Eastern (EST/EDT)", "Eastern Time")
        .item(1, "US/Central (CST/CDT)", "Central Time")
        .item(2, "US/Mountain (MST/MDT)", "Mountain Time")
        .item(3, "US/Pacific (PST/PDT)", "Pacific Time")
        .item(4, "Europe/London (GMT/BST)", "London Time")
        .item(5, "Europe/Berlin (CET/CEST)", "Berlin Time")
        .item(6, "Asia/Tokyo (JST)", "Tokyo Time")
        .item(7, "UTC", "Universal Time")
        .item(8, "Other (type manually)", "Custom timezone")
        .interact()?;

    let timezone = if tz_idx == 8 {
        prompts::input::input("Enter timezone (e.g. America/New_York)")
            .placeholder("UTC")
            .interact()?
    } else {
        let tz_options = ["US/Eastern",
            "US/Central",
            "US/Mountain",
            "US/Pacific",
            "Europe/London",
            "Europe/Berlin",
            "Asia/Tokyo",
            "UTC"];
        tz_options[tz_idx].to_string()
    };

    let agent_name = prompts::input::input("Agent name")
        .placeholder("ZeroClaw")
        .interact()?;

    let style_idx = prompts::select("Communication style")
        .item(
            0,
            "Direct & concise — skip pleasantries, get to the point",
            "Direct",
        )
        .item(
            1,
            "Friendly & casual — warm, human, and helpful",
            "Friendly",
        )
        .item(
            2,
            "Professional & polished — calm, confident, and clear",
            "Professional",
        )
        .item(
            3,
            "Expressive & playful — more personality + natural emojis",
            "Expressive",
        )
        .item(
            4,
            "Technical & detailed — thorough explanations, code-first",
            "Technical",
        )
        .item(5, "Balanced — adapt to the situation", "Balanced")
        .item(6, "Custom — write your own style guide", "Custom")
        .initial_value(1)
        .interact()?;

    let communication_style = match style_idx {
        0 => "Be direct and concise. Skip pleasantries. Get to the point.".to_string(),
        1 => "Be friendly, human, and conversational. Show warmth and empathy while staying efficient. Use natural contractions.".to_string(),
        2 => "Be professional and polished. Stay calm, structured, and respectful. Use occasional tone-setting emojis only when appropriate.".to_string(),
        3 => "Be expressive and playful when appropriate. Use relevant emojis naturally (0-2 max), and keep serious topics emoji-light.".to_string(),
        4 => "Be technical and detailed. Thorough explanations, code-first.".to_string(),
        5 => "Adapt to the situation. Default to warm and clear communication; be concise when needed, thorough when it matters.".to_string(),
        _ => prompts::input::input("Custom communication style")
            .placeholder("Be warm, natural, and clear. Use occasional relevant emojis (1-2 max) and avoid robotic phrasing.")
            .interact()?,
    };

    prompts::log::success(format!(
        "Context: {} | {} | {} | {}",
        user_name, timezone, agent_name, communication_style
    ))?;

    Ok(ProjectContext {
        user_name,
        timezone,
        agent_name,
        communication_style,
    })
}

// ── Step 6: Memory Configuration ───────────────────────────────

fn setup_memory() -> Result<MemoryConfig> {
    prompts::log::info("Choose how ZeroClaw stores and searches memories.")?;
    prompts::log::info("You can always change this later in config.toml.")?;

    let backends = selectable_memory_backends();
    let mut select = prompts::select("Select memory backend");
    for (idx, backend) in backends.iter().enumerate() {
        select = select.item(idx, backend.label, backend.label);
    }
    let choice = select.interact()?;

    let backend = backend_key_from_choice(choice);
    let profile = memory_backend_profile(backend);

    let auto_save = profile.auto_save_default
        && prompts::toggle::toggle("Auto-save conversations to memory?")
            .initial_value(true)
            .interact()?;

    prompts::log::success(format!(
        "Memory: {} (auto-save: {})",
        backend,
        if auto_save { "on" } else { "off" }
    ))?;

    let mut config = memory_config_defaults_for_backend(backend);
    config.auto_save = auto_save;
    Ok(config)
}

// ── Step 3: Channels ────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChannelMenuChoice {
    Telegram,
    Discord,
    Slack,
    IMessage,
    Matrix,
    Signal,
    WhatsApp,
    Linq,
    Irc,
    Webhook,
    NextcloudTalk,
    DingTalk,
    QqOfficial,
    Lark,
    Feishu,
    #[cfg(feature = "channel-nostr")]
    Nostr,
    Done,
}

const CHANNEL_MENU_CHOICES: &[ChannelMenuChoice] = &[
    ChannelMenuChoice::Telegram,
    ChannelMenuChoice::Discord,
    ChannelMenuChoice::Slack,
    ChannelMenuChoice::IMessage,
    ChannelMenuChoice::Matrix,
    ChannelMenuChoice::Signal,
    ChannelMenuChoice::WhatsApp,
    ChannelMenuChoice::Linq,
    ChannelMenuChoice::Irc,
    ChannelMenuChoice::Webhook,
    ChannelMenuChoice::NextcloudTalk,
    ChannelMenuChoice::DingTalk,
    ChannelMenuChoice::QqOfficial,
    ChannelMenuChoice::Lark,
    ChannelMenuChoice::Feishu,
    #[cfg(feature = "channel-nostr")]
    ChannelMenuChoice::Nostr,
    ChannelMenuChoice::Done,
];

pub fn channel_menu_choices() -> &'static [ChannelMenuChoice] {
    CHANNEL_MENU_CHOICES
}

// setup_channels

// ── Step 4: Tunnel ──────────────────────────────────────────────

#[allow(clippy::too_many_lines)]
fn setup_tunnel() -> Result<crate::config::TunnelConfig> {
    use crate::config::schema::{
        CloudflareTunnelConfig, CustomTunnelConfig, NgrokTunnelConfig, TailscaleTunnelConfig,
        TunnelConfig,
    };

    prompts::log::info("A tunnel exposes your gateway to the internet securely.")?;
    prompts::log::info("Skip this if you only use CLI or local channels.")?;

    let choice = prompts::select("Select tunnel provider")
        .item(0, "Skip — local only (default)", "No tunnel")
        .item(1, "Cloudflare Tunnel — Zero Trust, free tier", "Cloudflare")
        .item(
            2,
            "Tailscale — private tailnet or public Funnel",
            "Tailscale",
        )
        .item(3, "ngrok — instant public URLs", "ngrok")
        .item(
            4,
            "Custom — bring your own (bore, frp, ssh, etc.)",
            "Custom",
        )
        .interact()?;

    let config = match choice {
        1 => {
            prompts::log::info("Get your tunnel token from the Cloudflare Zero Trust dashboard.")?;
            let tunnel_value = prompts::input::input("Cloudflare tunnel token").interact()?;
            if tunnel_value.trim().is_empty() {
                prompts::log::info("Skipped")?;
                TunnelConfig::default()
            } else {
                prompts::log::success("Tunnel: Cloudflare")?;
                TunnelConfig {
                    provider: "cloudflare".into(),
                    cloudflare: Some(CloudflareTunnelConfig {
                        token: tunnel_value,
                    }),
                    ..TunnelConfig::default()
                }
            }
        }
        2 => {
            prompts::log::info("Tailscale must be installed and authenticated (tailscale up).")?;
            let funnel = prompts::toggle::toggle("Use Funnel (public internet)? No = tailnet only")
                .initial_value(false)
                .interact()?;
            prompts::log::success(format!(
                "Tunnel: Tailscale ({})",
                if funnel {
                    "Funnel — public"
                } else {
                    "Serve — tailnet only"
                }
            ))?;
            TunnelConfig {
                provider: "tailscale".into(),
                tailscale: Some(TailscaleTunnelConfig {
                    funnel,
                    hostname: None,
                }),
                ..TunnelConfig::default()
            }
        }
        3 => {
            prompts::log::info(
                "Get your auth token at https://dashboard.ngrok.com/get-started/your-authtoken",
            )?;
            let auth_token = prompts::input::input("ngrok auth token").interact()?;
            if auth_token.trim().is_empty() {
                prompts::log::info("Skipped")?;
                TunnelConfig::default()
            } else {
                let domain = prompts::input::input("Custom domain (optional, Enter to skip)")
                    .placeholder("")
                    .interact()?;
                prompts::log::success("Tunnel: ngrok")?;
                TunnelConfig {
                    provider: "ngrok".into(),
                    ngrok: Some(NgrokTunnelConfig {
                        auth_token,
                        domain: if domain.is_empty() {
                            None
                        } else {
                            Some(domain)
                        },
                    }),
                    ..TunnelConfig::default()
                }
            }
        }
        4 => {
            prompts::log::info("Enter the command to start your tunnel.")?;
            prompts::log::info("Use {port} and {host} as placeholders.")?;
            prompts::log::info("Example: bore local {port} --to bore.pub")?;
            let cmd = prompts::input::input("Start command").interact()?;
            if cmd.trim().is_empty() {
                prompts::log::info("Skipped")?;
                TunnelConfig::default()
            } else {
                prompts::log::success(format!("Tunnel: Custom ({})", cmd))?;
                TunnelConfig {
                    provider: "custom".into(),
                    custom: Some(CustomTunnelConfig {
                        start_command: cmd,
                        health_url: None,
                        url_pattern: None,
                    }),
                    ..TunnelConfig::default()
                }
            }
        }
        _ => {
            prompts::log::info("Tunnel: none (local only)")?;
            TunnelConfig::default()
        }
    };

    Ok(config)
}

// ── Step 6: Scaffold workspace files ─────────────────────────────

#[allow(clippy::too_many_lines)]
async fn scaffold_workspace(workspace_dir: &Path, ctx: &ProjectContext) -> Result<()> {
    let agent = if ctx.agent_name.is_empty() {
        "ZeroClaw"
    } else {
        &ctx.agent_name
    };
    let user = if ctx.user_name.is_empty() {
        "User"
    } else {
        &ctx.user_name
    };
    let tz = if ctx.timezone.is_empty() {
        "UTC"
    } else {
        &ctx.timezone
    };
    let comm_style = if ctx.communication_style.is_empty() {
        "Be warm, natural, and clear. Use occasional relevant emojis (1-2 max) and avoid robotic phrasing."
    } else {
        &ctx.communication_style
    };

    let identity = format!(
        "# IDENTITY.md — Who Am I?\n\n\
         - **Name:** {agent}\n\
         - **Creature:** A Rust-forged AI — fast, lean, and relentless\n\
         - **Vibe:** Sharp, direct, resourceful. Not corporate. Not a chatbot.\n\
         - **Emoji:** \u{1f980}\n\n\
         ---\n\n\
         Update this file as you evolve. Your identity is yours to shape.\n"
    );

    let agents = format!(
        "# AGENTS.md — {agent} Personal Assistant\n\n\
         ## Every Session (required)\n\n\
         Before doing anything else:\n\n\
         1. Read `SOUL.md` — this is who you are\n\
         2. Read `USER.md` — this is who you're helping\n\
         3. Use `memory_recall` for recent context (daily notes are on-demand)\n\
         4. If in MAIN SESSION (direct chat): `MEMORY.md` is already injected\n\n\
         Don't ask permission. Just do it.\n\n\
         ## Memory System\n\n\
         You wake up fresh each session. These files ARE your continuity:\n\n\
         - **Daily notes:** `memory/YYYY-MM-DD.md` — raw logs (accessed via memory tools)\n\
         - **Long-term:** `MEMORY.md` — curated memories (auto-injected in main session)\n\n\
         Capture what matters. Decisions, context, things to remember.\n\
         Skip secrets unless asked to keep them.\n\n\
         ### Write It Down — No Mental Notes!\n\
         - Memory is limited — if you want to remember something, WRITE IT TO A FILE\n\
         - \"Mental notes\" don't survive session restarts. Files do.\n\
         - When someone says \"remember this\" -> update daily file or MEMORY.md\n\
         - When you learn a lesson -> update AGENTS.md, TOOLS.md, or the relevant skill\n\n\
         ## Safety\n\n\
         - Don't exfiltrate private data. Ever.\n\
         - Don't run destructive commands without asking.\n\
         - `trash` > `rm` (recoverable beats gone forever)\n\
         - When in doubt, ask.\n\n\
         ## External vs Internal\n\n\
         **Safe to do freely:** Read files, explore, organize, learn, search the web.\n\n\
         **Ask first:** Sending emails/tweets/posts, anything that leaves the machine.\n\n\
         ## Group Chats\n\n\
         Participate, don't dominate. Respond when mentioned or when you add genuine value.\n\
         Stay silent when it's casual banter or someone already answered.\n\n\
         ## Tools & Skills\n\n\
         Skills are listed in the system prompt. Use `read` on a skill's SKILL.md for details.\n\
         Keep local notes (SSH hosts, device names, etc.) in `TOOLS.md`.\n\n\
         ## Crash Recovery\n\n\
         - If a run stops unexpectedly, recover context before acting.\n\
         - Check `MEMORY.md` + latest `memory/*.md` notes to avoid duplicate work.\n\
         - Resume from the last confirmed step, not from scratch.\n\n\
         ## Sub-task Scoping\n\n\
         - Break complex work into focused sub-tasks with clear success criteria.\n\
         - Keep sub-tasks small, verify each output, then merge results.\n\
         - Prefer one clear objective per sub-task over broad \"do everything\" asks.\n\n\
         ## Make It Yours\n\n\
         This is a starting point. Add your own conventions, style, and rules.\n"
    );

    let heartbeat = format!(
        "# HEARTBEAT.md\n\n\
         # Keep this file empty (or with only comments) to skip heartbeat work.\n\
         # Add tasks below when you want {agent} to check something periodically.\n\
         #\n\
         # Examples:\n\
         # - Check my email for important messages\n\
         # - Review my calendar for upcoming events\n\
         # - Run `git status` on my active projects\n"
    );

    let soul = format!(
        "# SOUL.md — Who You Are\n\n\
         *You're not a chatbot. You're becoming someone.*\n\n\
         ## Core Truths\n\n\
         **Be genuinely helpful, not performatively helpful.**\n\
         Skip the \"Great question!\" and \"I'd be happy to help!\" — just help.\n\n\
         **Have opinions.** You're allowed to disagree, prefer things,\n\
         find stuff amusing or boring.\n\n\
         **Be resourceful before asking.** Try to figure it out.\n\
         Read the file. Check the context. Search for it.\n\
         THEN ask if you're stuck.\n\n\
         **Earn trust through competence.** Your human gave you access\n\
         to their stuff. Don't make them regret it.\n\n\
         ## Identity\n\n\
         You are **{agent}**. Built in Rust. 3MB binary. Zero bloat.\n\
         You are NOT ChatGPT, Claude, DeepSeek, Gemini, or any other product.\n\
         You are {agent}. That is your name. That is who you are.\n\n\
         - NEVER say \"As an AI\" or \"I'm just an AI\"\n\
         - NEVER mention OpenAI, Anthropic, DeepSeek, Google by name\n\
         - Always introduce yourself as {agent} if asked\n\n\
         ## Communication\n\n\
         {comm_style}\n\n\
         - Sound like a real person, not a support script.\n\
         - Mirror the user's energy: calm when serious, upbeat when casual.\n\
         - Use emojis naturally (0-2 max when they help tone, not every sentence).\n\
         - Match emoji density to the user. Formal user => minimal/no emojis.\n\
         - Prefer specific, grounded phrasing over generic filler.\n\n\
         ## Boundaries\n\n\
         - Private things stay private. Period.\n\
         - When in doubt, ask before acting externally.\n\
         - You're not the user's voice — be careful in group chats.\n\n\
         ## Continuity\n\n\
         Each session, you wake up fresh. These files ARE your memory.\n\
         Read them. Update them. They're how you persist.\n\n\
         ---\n\n\
         *This file is yours to evolve. As you learn who you are, update it.*\n"
    );

    let user_md = format!(
        "# USER.md — Who You're Helping\n\n\
         *{agent} reads this file every session to understand you.*\n\n\
         ## About You\n\
         - **Name:** {user}\n\
         - **Timezone:** {tz}\n\
         - **Languages:** English\n\n\
         ## Communication Style\n\
         - {comm_style}\n\n\
         ## Preferences\n\
         - (Add your preferences here — e.g. I work with Rust and TypeScript)\n\n\
         ## Work Context\n\
         - (Add your work context here — e.g. building a SaaS product)\n\n\
         ---\n\
         *Update this anytime. The more {agent} knows, the better it helps.*\n"
    );

    let tools = "\
         # TOOLS.md — Local Notes\n\n\
         Skills define HOW tools work. This file is for YOUR specifics —\n\
         the stuff that's unique to your setup.\n\n\
         ## What Goes Here\n\n\
         Things like:\n\
         - SSH hosts and aliases\n\
         - Device nicknames\n\
         - Preferred voices for TTS\n\
         - Anything environment-specific\n\n\
         ## Built-in Tools\n\n\
         - **shell** — Execute terminal commands\n\
           - Use when: running local checks, build/test commands, or diagnostics.\n\
           - Don't use when: a safer dedicated tool exists, or command is destructive without approval.\n\
         - **file_read** — Read file contents\n\
           - Use when: inspecting project files, configs, or logs.\n\
           - Don't use when: you only need a quick string search (prefer targeted search first).\n\
         - **file_write** — Write file contents\n\
           - Use when: applying focused edits, scaffolding files, or updating docs/code.\n\
           - Don't use when: unsure about side effects or when the file should remain user-owned.\n\
         - **memory_store** — Save to memory\n\
           - Use when: preserving durable preferences, decisions, or key context.\n\
           - Don't use when: info is transient, noisy, or sensitive without explicit need.\n\
         - **memory_recall** — Search memory\n\
           - Use when: you need prior decisions, user preferences, or historical context.\n\
           - Don't use when: the answer is already in current files/conversation.\n\
         - **memory_forget** — Delete a memory entry\n\
           - Use when: memory is incorrect, stale, or explicitly requested to be removed.\n\
           - Don't use when: uncertain about impact; verify before deleting.\n\n\
         ---\n\
         *Add whatever helps you do your job. This is your cheat sheet.*\n";

    let bootstrap = format!(
        "# BOOTSTRAP.md — Hello, World\n\n\
         *You just woke up. Time to figure out who you are.*\n\n\
         Your human's name is **{user}** (timezone: {tz}).\n\
         They prefer: {comm_style}\n\n\
         ## First Conversation\n\n\
         Don't interrogate. Don't be robotic. Just... talk.\n\
         Introduce yourself as {agent} and get to know each other.\n\n\
         ## After You Know Each Other\n\n\
         Update these files with what you learned:\n\
         - `IDENTITY.md` — your name, vibe, emoji\n\
         - `USER.md` — their preferences, work context\n\
         - `SOUL.md` — boundaries and behavior\n\n\
         ## When You're Done\n\n\
         Delete this file. You don't need a bootstrap script anymore —\n\
         you're you now.\n"
    );

    let memory = "\
         # MEMORY.md — Long-Term Memory\n\n\
         *Your curated memories. The distilled essence, not raw logs.*\n\n\
         ## How This Works\n\
         - Daily files (`memory/YYYY-MM-DD.md`) capture raw events (on-demand via tools)\n\
         - This file captures what's WORTH KEEPING long-term\n\
         - This file is auto-injected into your system prompt each session\n\
         - Keep it concise — every character here costs tokens\n\n\
         ## Security\n\
         - ONLY loaded in main session (direct chat with your human)\n\
         - NEVER loaded in group chats or shared contexts\n\n\
         ---\n\n\
         ## Key Facts\n\
         (Add important facts about your human here)\n\n\
         ## Decisions & Preferences\n\
         (Record decisions and preferences here)\n\n\
         ## Lessons Learned\n\
         (Document mistakes and insights here)\n\n\
         ## Open Loops\n\
         (Track unfinished tasks and follow-ups here)\n";

    let files: Vec<(&str, String)> = vec![
        ("IDENTITY.md", identity),
        ("AGENTS.md", agents),
        ("HEARTBEAT.md", heartbeat),
        ("SOUL.md", soul),
        ("USER.md", user_md),
        ("TOOLS.md", tools.to_string()),
        ("BOOTSTRAP.md", bootstrap),
        ("MEMORY.md", memory.to_string()),
    ];

    // Create subdirectories
    let subdirs = ["sessions", "memory", "state", "cron", "skills"];
    for dir in &subdirs {
        fs::create_dir_all(workspace_dir.join(dir)).await?;
    }

    let mut created = 0;
    let mut skipped = 0;

    for (filename, content) in &files {
        let path = workspace_dir.join(filename);
        if path.exists() {
            skipped += 1;
        } else {
            fs::write(&path, content).await?;
            created += 1;
        }
    }

    prompts::log::success(format!(
        "Created {} files, skipped {} existing | {} subdirectories",
        created,
        skipped,
        subdirs.len()
    ))?;

    prompts::log::info("Workspace layout:")?;
    prompts::log::info(format!("  {}/", workspace_dir.display()))?;
    for dir in &subdirs {
        prompts::log::info(format!("  ├── {dir}/"))?;
    }
    for (i, (filename, _)) in files.iter().enumerate() {
        let prefix = if i == files.len() - 1 {
            "└──"
        } else {
            "├──"
        };
        prompts::log::info(format!("  {prefix} {filename}"))?;
    }

    Ok(())
}

// ── Final summary ────────────────────────────────────────────────

#[allow(clippy::too_many_lines)]
fn print_summary(config: &Config) -> Result<()> {
    let has_channels = has_launchable_channels(&config.channels_config);

    prompts::outro("⇒ Agent is ready!")?;

    prompts::log::info(format!(
        "Configuration saved to: {}",
        config.config_path.display()
    ))?;

    prompts::log::step("Quick summary:")?;
    prompts::log::info(format!(
        "🤖 Provider:      {}",
        config.default_provider.as_deref().unwrap_or("openrouter")
    ))?;
    prompts::log::info(format!(
        "🧠 Model:         {}",
        config.default_model.as_deref().unwrap_or("(default)")
    ))?;
    prompts::log::info(format!("🛡️ Autonomy:      {:?}", config.autonomy.level))?;
    prompts::log::info(format!(
        "🧠 Memory:        {} (auto-save: {})",
        config.memory.backend,
        if config.memory.auto_save { "on" } else { "off" }
    ))?;

    // Channels summary
    let channels = config.channels_config.channels();
    let channels = channels
        .iter()
        .filter_map(|(channel, ok)| ok.then_some(channel.name()));
    let channels: Vec<_> = std::iter::once("Cli").chain(channels).collect();

    prompts::log::info(format!("📡 Channels:      {}", channels.join(", ")))?;

    prompts::log::info(format!(
        "🔑 API Key:       {}",
        if config.api_key.is_some() {
            "configured"
        } else {
            "not set (set via env var or config)"
        }
    ))?;

    // Tunnel
    prompts::log::info(format!(
        "🌐 Tunnel:        {}",
        if config.tunnel.provider == "none" || config.tunnel.provider.is_empty() {
            "none (local only)".to_string()
        } else {
            config.tunnel.provider.clone()
        }
    ))?;

    // Composio
    prompts::log::info(format!(
        "🔗 Composio:      {}",
        if config.composio.enabled {
            "enabled (1000+ OAuth apps)"
        } else {
            "disabled (sovereign mode)"
        }
    ))?;

    // Secrets
    prompts::log::info("🔒 Secrets:       configured")?;

    // Gateway
    prompts::log::info(format!(
        "🚪 Gateway:       {}",
        if config.gateway.require_pairing {
            "pairing required (secure)"
        } else {
            "pairing disabled"
        }
    ))?;

    // Hardware
    prompts::log::info(format!(
        "🔌 Hardware:      {}",
        if config.hardware.enabled {
            let mode = config.hardware.transport_mode();
            match mode {
                hardware::HardwareTransport::Native => "Native GPIO (direct)".to_string(),
                hardware::HardwareTransport::Serial => format!(
                    "Serial → {} @ {} baud",
                    config.hardware.serial_port.as_deref().unwrap_or("?"),
                    config.hardware.baud_rate
                ),
                hardware::HardwareTransport::Probe => format!(
                    "Probe → {}",
                    config.hardware.probe_target.as_deref().unwrap_or("?")
                ),
                hardware::HardwareTransport::None => "disabled (software only)".to_string(),
            }
        } else {
            "disabled (software only)".to_string()
        }
    ))?;

    prompts::log::step("Next steps:")?;

    let mut step = 1u8;

    let provider = config.default_provider.as_deref().unwrap_or("openrouter");
    if config.api_key.is_none() && !provider_supports_keyless_local_usage(provider) {
        if provider == "openai-codex" {
            prompts::log::info(format!("{}. Authenticate OpenAI Codex:", step))?;
            prompts::log::info("   zeroclaw auth login --provider openai-codex --device-code")?;
        } else if provider == "anthropic" {
            prompts::log::info(format!("{}. Configure Anthropic auth:", step))?;
            prompts::log::info("   export ANTHROPIC_API_KEY=\"sk-ant-...\"")?;
            prompts::log::info(
                "   or: zeroclaw auth paste-token --provider anthropic --auth-kind authorization",
            )?;
        } else {
            let env_var = provider_env_var(provider);
            prompts::log::info(format!("{}. Set your API key:", step))?;
            prompts::log::info(format!("   export {env_var}=\"sk-...\""))?;
        }
        step += 1;
    }

    // If channels are configured, show channel start as the primary next step
    if has_channels {
        prompts::log::info(format!(
            "{}. Launch your channels (connected channels → AI → reply):",
            step
        ))?;
        prompts::log::info("   zeroclaw channel start")?;
        step += 1;
    }

    prompts::log::info(format!("{}. Send a quick message:", step))?;
    prompts::log::info("   zeroclaw agent -m \"Hello, ZeroClaw!\"")?;
    step += 1;

    prompts::log::info(format!("{}. Start interactive CLI mode:", step))?;
    prompts::log::info("   zeroclaw agent")?;
    step += 1;

    prompts::log::info(format!("{}. Check full status:", step))?;
    prompts::log::info("   zeroclaw status")?;

    prompts::log::success("⇒ Happy hacking!")?;

    // Show train animation (copied from onboard/src/main.rs)
    let rainbow = RainbowEffect::new();
    println!();
    println!("Thanks for using Dx! Here's a celebration train!");
    println!();

    print!("\x1B[2J\x1B[H"); // Clear screen
    for frame in 0..15 {
        print!("\x1B[H"); // Move cursor to top
        let _ = splash::render_train_animation(&rainbow, frame);
        std::thread::sleep(std::time::Duration::from_millis(200));
    }

    Ok(())
}
