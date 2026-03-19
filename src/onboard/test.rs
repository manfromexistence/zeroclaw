#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::sync::OnceLock;
    use tempfile::TempDir;
    use tokio::sync::Mutex;

    fn env_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    struct EnvVarGuard {
        key: &'static str,
        previous: Option<String>,
    }

    impl EnvVarGuard {
        fn set(key: &'static str, value: &str) -> Self {
            let previous = std::env::var(key).ok();
            unsafe { std::env::set_var(key, value) };
            Self { key, previous }
        }

        fn unset(key: &'static str) -> Self {
            let previous = std::env::var(key).ok();
            unsafe { std::env::remove_var(key) };
            Self { key, previous }
        }
    }

    impl Drop for EnvVarGuard {
        fn drop(&mut self) {
            if let Some(previous) = &self.previous {
                unsafe { std::env::set_var(self.key, previous) };
            } else {
                unsafe { std::env::remove_var(self.key) };
            }
        }
    }

    // ── ProjectContext defaults ──────────────────────────────────

    #[test]
    fn project_context_default_is_empty() {
        let ctx = ProjectContext::default();
        assert!(ctx.user_name.is_empty());
        assert!(ctx.timezone.is_empty());
        assert!(ctx.agent_name.is_empty());
        assert!(ctx.communication_style.is_empty());
    }

    #[test]
    fn apply_provider_update_preserves_non_provider_settings() {
        let mut config = Config::default();
        config.default_temperature = 1.23;
        config.memory.backend = "markdown".to_string();
        config.skills.open_skills_enabled = true;
        config.channels_config.cli = false;

        apply_provider_update(
            &mut config,
            "openrouter".to_string(),
            "sk-updated".to_string(),
            "openai/gpt-5.2".to_string(),
            Some("https://openrouter.ai/api/v1".to_string()),
        );

        assert_eq!(config.default_provider.as_deref(), Some("openrouter"));
        assert_eq!(config.default_model.as_deref(), Some("openai/gpt-5.2"));
        assert_eq!(config.api_key.as_deref(), Some("sk-updated"));
        assert_eq!(
            config.api_url.as_deref(),
            Some("https://openrouter.ai/api/v1")
        );
        assert_eq!(config.default_temperature, 1.23);
        assert_eq!(config.memory.backend, "markdown");
        assert!(config.skills.open_skills_enabled);
        assert!(!config.channels_config.cli);
    }

    #[test]
    fn apply_provider_update_clears_api_key_when_empty() {
        let mut config = Config::default();
        config.api_key = Some("sk-old".to_string());

        apply_provider_update(
            &mut config,
            "anthropic".to_string(),
            String::new(),
            "claude-sonnet-4-5-20250929".to_string(),
            None,
        );

        assert_eq!(config.default_provider.as_deref(), Some("anthropic"));
        assert_eq!(
            config.default_model.as_deref(),
            Some("claude-sonnet-4-5-20250929")
        );
        assert!(config.api_key.is_none());
        assert!(config.api_url.is_none());
    }

    #[tokio::test]
    async fn quick_setup_model_override_persists_to_config_toml() {
        let _env_guard = env_lock().lock().await;
        let _workspace_env = EnvVarGuard::unset("ZEROCLAW_WORKSPACE");
        let _config_env = EnvVarGuard::unset("ZEROCLAW_CONFIG_DIR");
        let tmp = TempDir::new().unwrap();

        let config = Box::pin(run_quick_setup_with_home(
            Some("sk-issue946"),
            Some("openrouter"),
            Some("custom-model-946"),
            Some("sqlite"),
            false,
            tmp.path(),
        ))
        .await
        .unwrap();

        assert_eq!(config.default_provider.as_deref(), Some("openrouter"));
        assert_eq!(config.default_model.as_deref(), Some("custom-model-946"));
        assert_eq!(config.api_key.as_deref(), Some("sk-issue946"));

        let config_raw = tokio::fs::read_to_string(config.config_path).await.unwrap();
        assert!(config_raw.contains("default_provider = \"openrouter\""));
        assert!(config_raw.contains("default_model = \"custom-model-946\""));
    }

    #[tokio::test]
    async fn quick_setup_without_model_uses_provider_default_model() {
        let _env_guard = env_lock().lock().await;
        let _workspace_env = EnvVarGuard::unset("ZEROCLAW_WORKSPACE");
        let _config_env = EnvVarGuard::unset("ZEROCLAW_CONFIG_DIR");
        let tmp = TempDir::new().unwrap();

        let config = Box::pin(run_quick_setup_with_home(
            Some("sk-issue946"),
            Some("anthropic"),
            None,
            Some("sqlite"),
            false,
            tmp.path(),
        ))
        .await
        .unwrap();

        let expected = default_model_for_provider("anthropic");
        assert_eq!(config.default_provider.as_deref(), Some("anthropic"));
        assert_eq!(config.default_model.as_deref(), Some(expected.as_str()));
    }

    #[tokio::test]
    async fn quick_setup_existing_config_requires_force_when_non_interactive() {
        let _env_guard = env_lock().lock().await;
        let _workspace_env = EnvVarGuard::unset("ZEROCLAW_WORKSPACE");
        let _config_env = EnvVarGuard::unset("ZEROCLAW_CONFIG_DIR");
        let tmp = TempDir::new().unwrap();
        let zeroclaw_dir = tmp.path().join(".zeroclaw");
        let config_path = zeroclaw_dir.join("config.toml");

        tokio::fs::create_dir_all(&zeroclaw_dir).await.unwrap();
        tokio::fs::write(&config_path, "default_provider = \"openrouter\"\n")
            .await
            .unwrap();

        let err = Box::pin(run_quick_setup_with_home(
            Some("sk-existing"),
            Some("openrouter"),
            Some("custom-model"),
            Some("sqlite"),
            false,
            tmp.path(),
        ))
        .await
        .expect_err("quick setup should refuse overwrite without --force");

        let err_text = err.to_string();
        assert!(err_text.contains("Refusing to overwrite existing config"));
        assert!(err_text.contains("--force"));
    }

    #[tokio::test]
    async fn quick_setup_existing_config_overwrites_with_force() {
        let _env_guard = env_lock().lock().await;
        let _workspace_env = EnvVarGuard::unset("ZEROCLAW_WORKSPACE");
        let _config_env = EnvVarGuard::unset("ZEROCLAW_CONFIG_DIR");
        let tmp = TempDir::new().unwrap();
        let zeroclaw_dir = tmp.path().join(".zeroclaw");
        let config_path = zeroclaw_dir.join("config.toml");

        tokio::fs::create_dir_all(&zeroclaw_dir).await.unwrap();
        tokio::fs::write(
            &config_path,
            "default_provider = \"anthropic\"\ndefault_model = \"stale-model\"\n",
        )
        .await
        .unwrap();

        let config = Box::pin(run_quick_setup_with_home(
            Some("sk-force"),
            Some("openrouter"),
            Some("custom-model-fresh"),
            Some("sqlite"),
            true,
            tmp.path(),
        ))
        .await
        .expect("quick setup should overwrite existing config with --force");

        assert_eq!(config.default_provider.as_deref(), Some("openrouter"));
        assert_eq!(config.default_model.as_deref(), Some("custom-model-fresh"));
        assert_eq!(config.api_key.as_deref(), Some("sk-force"));

        let config_raw = tokio::fs::read_to_string(config.config_path).await.unwrap();
        assert!(config_raw.contains("default_provider = \"openrouter\""));
        assert!(config_raw.contains("default_model = \"custom-model-fresh\""));
    }

    #[tokio::test]
    async fn quick_setup_respects_zero_claw_workspace_env_layout() {
        let _env_guard = env_lock().lock().await;
        let tmp = TempDir::new().unwrap();
        let workspace_root = tmp.path().join("zeroclaw-data");
        let workspace_dir = workspace_root.join("workspace");
        let expected_config_path = workspace_root.join(".zeroclaw").join("config.toml");

        let _workspace_env = EnvVarGuard::set(
            "ZEROCLAW_WORKSPACE",
            workspace_dir.to_string_lossy().as_ref(),
        );
        let _config_env = EnvVarGuard::unset("ZEROCLAW_CONFIG_DIR");

        let config = Box::pin(run_quick_setup_with_home(
            Some("sk-env"),
            Some("openrouter"),
            Some("model-env"),
            Some("sqlite"),
            false,
            tmp.path(),
        ))
        .await
        .expect("quick setup should honor ZEROCLAW_WORKSPACE");

        assert_eq!(config.workspace_dir, workspace_dir);
        assert_eq!(config.config_path, expected_config_path);
    }

    // ── scaffold_workspace: basic file creation ─────────────────

    #[tokio::test]
    async fn scaffold_creates_all_md_files() {
        let tmp = TempDir::new().unwrap();
        let ctx = ProjectContext::default();
        scaffold_workspace(tmp.path(), &ctx).await.unwrap();

        let expected = [
            "IDENTITY.md",
            "AGENTS.md",
            "HEARTBEAT.md",
            "SOUL.md",
            "USER.md",
            "TOOLS.md",
            "BOOTSTRAP.md",
            "MEMORY.md",
        ];
        for f in &expected {
            assert!(tmp.path().join(f).exists(), "missing file: {f}");
        }
    }

    #[tokio::test]
    async fn scaffold_creates_all_subdirectories() {
        let tmp = TempDir::new().unwrap();
        let ctx = ProjectContext::default();
        scaffold_workspace(tmp.path(), &ctx).await.unwrap();

        for dir in &["sessions", "memory", "state", "cron", "skills"] {
            assert!(tmp.path().join(dir).is_dir(), "missing subdirectory: {dir}");
        }
    }

    // ── scaffold_workspace: personalization ─────────────────────

    #[tokio::test]
    async fn scaffold_bakes_user_name_into_files() {
        let tmp = TempDir::new().unwrap();
        let ctx = ProjectContext {
            user_name: "Alice".into(),
            ..Default::default()
        };
        scaffold_workspace(tmp.path(), &ctx).await.unwrap();

        let user_md = tokio::fs::read_to_string(tmp.path().join("USER.md"))
            .await
            .unwrap();
        assert!(
            user_md.contains("**Name:** Alice"),
            "USER.md should contain user name"
        );

        let bootstrap = tokio::fs::read_to_string(tmp.path().join("BOOTSTRAP.md"))
            .await
            .unwrap();
        assert!(
            bootstrap.contains("**Alice**"),
            "BOOTSTRAP.md should contain user name"
        );
    }

    #[tokio::test]
    async fn scaffold_bakes_timezone_into_files() {
        let tmp = TempDir::new().unwrap();
        let ctx = ProjectContext {
            timezone: "US/Pacific".into(),
            ..Default::default()
        };
        scaffold_workspace(tmp.path(), &ctx).await.unwrap();

        let user_md = tokio::fs::read_to_string(tmp.path().join("USER.md"))
            .await
            .unwrap();
        assert!(
            user_md.contains("**Timezone:** US/Pacific"),
            "USER.md should contain timezone"
        );

        let bootstrap = tokio::fs::read_to_string(tmp.path().join("BOOTSTRAP.md"))
            .await
            .unwrap();
        assert!(
            bootstrap.contains("US/Pacific"),
            "BOOTSTRAP.md should contain timezone"
        );
    }

    #[tokio::test]
    async fn scaffold_bakes_agent_name_into_files() {
        let tmp = TempDir::new().unwrap();
        let ctx = ProjectContext {
            agent_name: "Crabby".into(),
            ..Default::default()
        };
        scaffold_workspace(tmp.path(), &ctx).await.unwrap();

        let identity = tokio::fs::read_to_string(tmp.path().join("IDENTITY.md"))
            .await
            .unwrap();
        assert!(
            identity.contains("**Name:** Crabby"),
            "IDENTITY.md should contain agent name"
        );

        let soul = tokio::fs::read_to_string(tmp.path().join("SOUL.md"))
            .await
            .unwrap();
        assert!(
            soul.contains("You are **Crabby**"),
            "SOUL.md should contain agent name"
        );

        let agents = tokio::fs::read_to_string(tmp.path().join("AGENTS.md"))
            .await
            .unwrap();
        assert!(
            agents.contains("Crabby Personal Assistant"),
            "AGENTS.md should contain agent name"
        );

        let heartbeat = tokio::fs::read_to_string(tmp.path().join("HEARTBEAT.md"))
            .await
            .unwrap();
        assert!(
            heartbeat.contains("Crabby"),
            "HEARTBEAT.md should contain agent name"
        );

        let bootstrap = tokio::fs::read_to_string(tmp.path().join("BOOTSTRAP.md"))
            .await
            .unwrap();
        assert!(
            bootstrap.contains("Introduce yourself as Crabby"),
            "BOOTSTRAP.md should contain agent name"
        );
    }

    #[tokio::test]
    async fn scaffold_bakes_communication_style() {
        let tmp = TempDir::new().unwrap();
        let ctx = ProjectContext {
            communication_style: "Be technical and detailed.".into(),
            ..Default::default()
        };
        scaffold_workspace(tmp.path(), &ctx).await.unwrap();

        let soul = tokio::fs::read_to_string(tmp.path().join("SOUL.md"))
            .await
            .unwrap();
        assert!(
            soul.contains("Be technical and detailed."),
            "SOUL.md should contain communication style"
        );

        let user_md = tokio::fs::read_to_string(tmp.path().join("USER.md"))
            .await
            .unwrap();
        assert!(
            user_md.contains("Be technical and detailed."),
            "USER.md should contain communication style"
        );

        let bootstrap = tokio::fs::read_to_string(tmp.path().join("BOOTSTRAP.md"))
            .await
            .unwrap();
        assert!(
            bootstrap.contains("Be technical and detailed."),
            "BOOTSTRAP.md should contain communication style"
        );
    }

    // ── scaffold_workspace: defaults when context is empty ──────

    #[tokio::test]
    async fn scaffold_uses_defaults_for_empty_context() {
        let tmp = TempDir::new().unwrap();
        let ctx = ProjectContext::default(); // all empty
        scaffold_workspace(tmp.path(), &ctx).await.unwrap();

        let identity = tokio::fs::read_to_string(tmp.path().join("IDENTITY.md"))
            .await
            .unwrap();
        assert!(
            identity.contains("**Name:** ZeroClaw"),
            "should default agent name to ZeroClaw"
        );

        let user_md = tokio::fs::read_to_string(tmp.path().join("USER.md"))
            .await
            .unwrap();
        assert!(
            user_md.contains("**Name:** User"),
            "should default user name to User"
        );
        assert!(
            user_md.contains("**Timezone:** UTC"),
            "should default timezone to UTC"
        );

        let soul = tokio::fs::read_to_string(tmp.path().join("SOUL.md"))
            .await
            .unwrap();
        assert!(
            soul.contains("Be warm, natural, and clear."),
            "should default communication style"
        );
    }

    // ── scaffold_workspace: skip existing files ─────────────────

    #[tokio::test]
    async fn scaffold_does_not_overwrite_existing_files() {
        let tmp = TempDir::new().unwrap();
        let ctx = ProjectContext {
            user_name: "Bob".into(),
            ..Default::default()
        };

        // Pre-create SOUL.md with custom content
        let soul_path = tmp.path().join("SOUL.md");
        fs::write(&soul_path, "# My Custom Soul\nDo not overwrite me.")
            .await
            .unwrap();

        scaffold_workspace(tmp.path(), &ctx).await.unwrap();

        // SOUL.md should be untouched
        let soul = tokio::fs::read_to_string(&soul_path).await.unwrap();
        assert!(
            soul.contains("Do not overwrite me"),
            "existing files should not be overwritten"
        );
        assert!(
            !soul.contains("You're not a chatbot"),
            "should not contain scaffold content"
        );

        // But USER.md should be created fresh
        let user_md = tokio::fs::read_to_string(tmp.path().join("USER.md"))
            .await
            .unwrap();
        assert!(user_md.contains("**Name:** Bob"));
    }

    // ── scaffold_workspace: idempotent ──────────────────────────

    #[tokio::test]
    async fn scaffold_is_idempotent() {
        let tmp = TempDir::new().unwrap();
        let ctx = ProjectContext {
            user_name: "Eve".into(),
            agent_name: "Claw".into(),
            ..Default::default()
        };

        scaffold_workspace(tmp.path(), &ctx).await.unwrap();
        let soul_v1 = tokio::fs::read_to_string(tmp.path().join("SOUL.md"))
            .await
            .unwrap();

        // Run again — should not change anything
        scaffold_workspace(tmp.path(), &ctx).await.unwrap();
        let soul_v2 = tokio::fs::read_to_string(tmp.path().join("SOUL.md"))
            .await
            .unwrap();

        assert_eq!(soul_v1, soul_v2, "scaffold should be idempotent");
    }

    // ── scaffold_workspace: all files are non-empty ─────────────

    #[tokio::test]
    async fn scaffold_files_are_non_empty() {
        let tmp = TempDir::new().unwrap();
        let ctx = ProjectContext::default();
        scaffold_workspace(tmp.path(), &ctx).await.unwrap();

        for f in &[
            "IDENTITY.md",
            "AGENTS.md",
            "HEARTBEAT.md",
            "SOUL.md",
            "USER.md",
            "TOOLS.md",
            "BOOTSTRAP.md",
            "MEMORY.md",
        ] {
            let content = tokio::fs::read_to_string(tmp.path().join(f)).await.unwrap();
            assert!(!content.trim().is_empty(), "{f} should not be empty");
        }
    }

    // ── scaffold_workspace: AGENTS.md references on-demand memory

    #[tokio::test]
    async fn agents_md_references_on_demand_memory() {
        let tmp = TempDir::new().unwrap();
        let ctx = ProjectContext::default();
        scaffold_workspace(tmp.path(), &ctx).await.unwrap();

        let agents = tokio::fs::read_to_string(tmp.path().join("AGENTS.md"))
            .await
            .unwrap();
        assert!(
            agents.contains("memory_recall"),
            "AGENTS.md should reference memory_recall for on-demand access"
        );
        assert!(
            agents.contains("on-demand"),
            "AGENTS.md should mention daily notes are on-demand"
        );
    }

    // ── scaffold_workspace: MEMORY.md warns about token cost ────

    #[tokio::test]
    async fn memory_md_warns_about_token_cost() {
        let tmp = TempDir::new().unwrap();
        let ctx = ProjectContext::default();
        scaffold_workspace(tmp.path(), &ctx).await.unwrap();

        let memory = tokio::fs::read_to_string(tmp.path().join("MEMORY.md"))
            .await
            .unwrap();
        assert!(
            memory.contains("costs tokens"),
            "MEMORY.md should warn about token cost"
        );
        assert!(
            memory.contains("auto-injected"),
            "MEMORY.md should mention it's auto-injected"
        );
    }

    // ── scaffold_workspace: TOOLS.md lists memory_forget ────────

    #[tokio::test]
    async fn tools_md_lists_all_builtin_tools() {
        let tmp = TempDir::new().unwrap();
        let ctx = ProjectContext::default();
        scaffold_workspace(tmp.path(), &ctx).await.unwrap();

        let tools = tokio::fs::read_to_string(tmp.path().join("TOOLS.md"))
            .await
            .unwrap();
        for tool in &[
            "shell",
            "file_read",
            "file_write",
            "memory_store",
            "memory_recall",
            "memory_forget",
        ] {
            assert!(
                tools.contains(tool),
                "TOOLS.md should list built-in tool: {tool}"
            );
        }
        assert!(
            tools.contains("Use when:"),
            "TOOLS.md should include 'Use when' guidance"
        );
        assert!(
            tools.contains("Don't use when:"),
            "TOOLS.md should include 'Don't use when' guidance"
        );
    }

    #[tokio::test]
    async fn soul_md_includes_emoji_awareness_guidance() {
        let tmp = TempDir::new().unwrap();
        let ctx = ProjectContext::default();
        scaffold_workspace(tmp.path(), &ctx).await.unwrap();

        let soul = tokio::fs::read_to_string(tmp.path().join("SOUL.md"))
            .await
            .unwrap();
        assert!(
            soul.contains("Use emojis naturally (0-2 max"),
            "SOUL.md should include emoji usage guidance"
        );
        assert!(
            soul.contains("Match emoji density to the user"),
            "SOUL.md should include emoji-awareness guidance"
        );
    }

    // ── scaffold_workspace: special characters in names ─────────

    #[tokio::test]
    async fn scaffold_handles_special_characters_in_names() {
        let tmp = TempDir::new().unwrap();
        let ctx = ProjectContext {
            user_name: "José María".into(),
            agent_name: "ZeroClaw-v2".into(),
            timezone: "Europe/Madrid".into(),
            communication_style: "Be direct.".into(),
        };
        scaffold_workspace(tmp.path(), &ctx).await.unwrap();

        let user_md = tokio::fs::read_to_string(tmp.path().join("USER.md"))
            .await
            .unwrap();
        assert!(user_md.contains("José María"));

        let soul = tokio::fs::read_to_string(tmp.path().join("SOUL.md"))
            .await
            .unwrap();
        assert!(soul.contains("ZeroClaw-v2"));
    }

    // ── scaffold_workspace: full personalization round-trip ─────

    #[tokio::test]
    async fn scaffold_full_personalization() {
        let tmp = TempDir::new().unwrap();
        let ctx = ProjectContext {
            user_name: "Argenis".into(),
            timezone: "US/Eastern".into(),
            agent_name: "Claw".into(),
            communication_style:
                "Be friendly, human, and conversational. Show warmth and empathy while staying efficient. Use natural contractions."
                    .into(),
        };
        scaffold_workspace(tmp.path(), &ctx).await.unwrap();

        // Verify every file got personalized
        let identity = tokio::fs::read_to_string(tmp.path().join("IDENTITY.md"))
            .await
            .unwrap();
        assert!(identity.contains("**Name:** Claw"));

        let soul = tokio::fs::read_to_string(tmp.path().join("SOUL.md"))
            .await
            .unwrap();
        assert!(soul.contains("You are **Claw**"));
        assert!(soul.contains("Be friendly, human, and conversational"));

        let user_md = tokio::fs::read_to_string(tmp.path().join("USER.md"))
            .await
            .unwrap();
        assert!(user_md.contains("**Name:** Argenis"));
        assert!(user_md.contains("**Timezone:** US/Eastern"));
        assert!(user_md.contains("Be friendly, human, and conversational"));

        let agents = tokio::fs::read_to_string(tmp.path().join("AGENTS.md"))
            .await
            .unwrap();
        assert!(agents.contains("Claw Personal Assistant"));

        let bootstrap = tokio::fs::read_to_string(tmp.path().join("BOOTSTRAP.md"))
            .await
            .unwrap();
        assert!(bootstrap.contains("**Argenis**"));
        assert!(bootstrap.contains("US/Eastern"));
        assert!(bootstrap.contains("Introduce yourself as Claw"));

        let heartbeat = tokio::fs::read_to_string(tmp.path().join("HEARTBEAT.md"))
            .await
            .unwrap();
        assert!(heartbeat.contains("Claw"));
    }

    // ── model helper coverage ───────────────────────────────────

    #[test]
    fn default_model_for_provider_uses_latest_defaults() {
        assert_eq!(
            default_model_for_provider("openrouter"),
            "anthropic/claude-sonnet-4.6"
        );
        assert_eq!(default_model_for_provider("openai"), "gpt-5.2");
        assert_eq!(default_model_for_provider("openai-codex"), "gpt-5-codex");
        assert_eq!(
            default_model_for_provider("anthropic"),
            "claude-sonnet-4-5-20250929"
        );
        assert_eq!(default_model_for_provider("qwen"), "qwen-plus");
        assert_eq!(default_model_for_provider("qwen-intl"), "qwen-plus");
        assert_eq!(default_model_for_provider("qwen-code"), "qwen3-coder-plus");
        assert_eq!(default_model_for_provider("glm-cn"), "glm-5");
        assert_eq!(default_model_for_provider("minimax-cn"), "MiniMax-M2.5");
        assert_eq!(default_model_for_provider("zai-cn"), "glm-5");
        assert_eq!(default_model_for_provider("gemini"), "gemini-2.5-pro");
        assert_eq!(default_model_for_provider("google"), "gemini-2.5-pro");
        assert_eq!(default_model_for_provider("kimi-code"), "kimi-for-coding");
        assert_eq!(
            default_model_for_provider("bedrock"),
            "anthropic.claude-sonnet-4-5-20250929-v1:0"
        );
        assert_eq!(
            default_model_for_provider("google-gemini"),
            "gemini-2.5-pro"
        );
        assert_eq!(default_model_for_provider("venice"), "zai-org-glm-5");
        assert_eq!(default_model_for_provider("moonshot"), "kimi-k2.5");
        assert_eq!(
            default_model_for_provider("nvidia"),
            "meta/llama-3.3-70b-instruct"
        );
        assert_eq!(
            default_model_for_provider("nvidia-nim"),
            "meta/llama-3.3-70b-instruct"
        );
        assert_eq!(
            default_model_for_provider("llamacpp"),
            "ggml-org/gpt-oss-20b-GGUF"
        );
        assert_eq!(default_model_for_provider("sglang"), "default");
        assert_eq!(default_model_for_provider("vllm"), "default");
        assert_eq!(
            default_model_for_provider("astrai"),
            "anthropic/claude-sonnet-4.6"
        );
    }

    #[test]
    fn canonical_provider_name_normalizes_regional_aliases() {
        assert_eq!(canonical_provider_name("qwen-intl"), "qwen");
        assert_eq!(canonical_provider_name("dashscope-us"), "qwen");
        assert_eq!(canonical_provider_name("qwen-code"), "qwen-code");
        assert_eq!(canonical_provider_name("qwen-oauth"), "qwen-code");
        assert_eq!(canonical_provider_name("codex"), "openai-codex");
        assert_eq!(canonical_provider_name("openai_codex"), "openai-codex");
        assert_eq!(canonical_provider_name("moonshot-intl"), "moonshot");
        assert_eq!(canonical_provider_name("kimi-cn"), "moonshot");
        assert_eq!(canonical_provider_name("kimi_coding"), "kimi-code");
        assert_eq!(canonical_provider_name("kimi_for_coding"), "kimi-code");
        assert_eq!(canonical_provider_name("glm-cn"), "glm");
        assert_eq!(canonical_provider_name("bigmodel"), "glm");
        assert_eq!(canonical_provider_name("minimax-cn"), "minimax");
        assert_eq!(canonical_provider_name("zai-cn"), "zai");
        assert_eq!(canonical_provider_name("z.ai-global"), "zai");
        assert_eq!(canonical_provider_name("nvidia-nim"), "nvidia");
        assert_eq!(canonical_provider_name("aws-bedrock"), "bedrock");
        assert_eq!(canonical_provider_name("build.nvidia.com"), "nvidia");
        assert_eq!(canonical_provider_name("llama.cpp"), "llamacpp");
    }

    #[test]
    fn curated_models_for_openai_include_latest_choices() {
        let ids: Vec<String> = curated_models_for_provider("openai")
            .into_iter()
            .map(|(id, _)| id)
            .collect();

        assert!(ids.contains(&"gpt-5.2".to_string()));
        assert!(ids.contains(&"gpt-5-mini".to_string()));
    }

    #[test]
    fn curated_models_for_glm_removes_deprecated_flash_plus_aliases() {
        let ids: Vec<String> = curated_models_for_provider("glm")
            .into_iter()
            .map(|(id, _)| id)
            .collect();

        assert!(ids.contains(&"glm-5".to_string()));
        assert!(ids.contains(&"glm-4.7".to_string()));
        assert!(ids.contains(&"glm-4.5-air".to_string()));
        assert!(!ids.contains(&"glm-4-plus".to_string()));
        assert!(!ids.contains(&"glm-4-flash".to_string()));
    }

    #[test]
    fn curated_models_for_openai_codex_include_codex_family() {
        let ids: Vec<String> = curated_models_for_provider("openai-codex")
            .into_iter()
            .map(|(id, _)| id)
            .collect();

        assert!(ids.contains(&"gpt-5-codex".to_string()));
        assert!(ids.contains(&"gpt-5.2-codex".to_string()));
    }

    #[test]
    fn curated_models_for_openrouter_use_valid_anthropic_id() {
        let ids: Vec<String> = curated_models_for_provider("openrouter")
            .into_iter()
            .map(|(id, _)| id)
            .collect();

        assert!(ids.contains(&"anthropic/claude-sonnet-4.6".to_string()));
    }

    #[test]
    fn curated_models_for_bedrock_include_verified_model_ids() {
        let ids: Vec<String> = curated_models_for_provider("bedrock")
            .into_iter()
            .map(|(id, _)| id)
            .collect();

        assert!(ids.contains(&"anthropic.claude-sonnet-4-6".to_string()));
        assert!(ids.contains(&"anthropic.claude-opus-4-6-v1".to_string()));
        assert!(ids.contains(&"anthropic.claude-haiku-4-5-20251001-v1:0".to_string()));
        assert!(ids.contains(&"anthropic.claude-sonnet-4-5-20250929-v1:0".to_string()));
    }

    #[test]
    fn curated_models_for_moonshot_drop_deprecated_aliases() {
        let ids: Vec<String> = curated_models_for_provider("moonshot")
            .into_iter()
            .map(|(id, _)| id)
            .collect();

        assert!(ids.contains(&"kimi-k2.5".to_string()));
        assert!(ids.contains(&"kimi-k2-thinking".to_string()));
        assert!(!ids.contains(&"kimi-latest".to_string()));
        assert!(!ids.contains(&"kimi-thinking-preview".to_string()));
    }

    #[test]
    fn allows_unauthenticated_model_fetch_for_public_catalogs() {
        assert!(allows_unauthenticated_model_fetch("openrouter"));
        assert!(allows_unauthenticated_model_fetch("venice"));
        assert!(allows_unauthenticated_model_fetch("nvidia"));
        assert!(allows_unauthenticated_model_fetch("nvidia-nim"));
        assert!(allows_unauthenticated_model_fetch("build.nvidia.com"));
        assert!(allows_unauthenticated_model_fetch("astrai"));
        assert!(allows_unauthenticated_model_fetch("ollama"));
        assert!(allows_unauthenticated_model_fetch("llamacpp"));
        assert!(allows_unauthenticated_model_fetch("llama.cpp"));
        assert!(allows_unauthenticated_model_fetch("sglang"));
        assert!(allows_unauthenticated_model_fetch("vllm"));
        assert!(!allows_unauthenticated_model_fetch("openai"));
        assert!(!allows_unauthenticated_model_fetch("deepseek"));
    }

    #[test]
    fn curated_models_for_kimi_code_include_official_agent_model() {
        let ids: Vec<String> = curated_models_for_provider("kimi-code")
            .into_iter()
            .map(|(id, _)| id)
            .collect();

        assert!(ids.contains(&"kimi-for-coding".to_string()));
        assert!(ids.contains(&"kimi-k2.5".to_string()));
    }

    #[test]
    fn curated_models_for_qwen_code_include_coding_plan_models() {
        let ids: Vec<String> = curated_models_for_provider("qwen-code")
            .into_iter()
            .map(|(id, _)| id)
            .collect();

        assert!(ids.contains(&"qwen3-coder-plus".to_string()));
        assert!(ids.contains(&"qwen3.5-plus".to_string()));
        assert!(ids.contains(&"qwen3-max-2026-01-23".to_string()));
    }

    #[test]
    fn supports_live_model_fetch_for_supported_and_unsupported_providers() {
        assert!(supports_live_model_fetch("openai"));
        assert!(supports_live_model_fetch("anthropic"));
        assert!(supports_live_model_fetch("gemini"));
        assert!(supports_live_model_fetch("google"));
        assert!(supports_live_model_fetch("grok"));
        assert!(supports_live_model_fetch("together"));
        assert!(supports_live_model_fetch("nvidia"));
        assert!(supports_live_model_fetch("nvidia-nim"));
        assert!(supports_live_model_fetch("build.nvidia.com"));
        assert!(supports_live_model_fetch("ollama"));
        assert!(supports_live_model_fetch("llamacpp"));
        assert!(supports_live_model_fetch("llama.cpp"));
        assert!(supports_live_model_fetch("sglang"));
        assert!(supports_live_model_fetch("vllm"));
        assert!(supports_live_model_fetch("astrai"));
        assert!(supports_live_model_fetch("venice"));
        assert!(supports_live_model_fetch("glm-cn"));
        assert!(supports_live_model_fetch("qwen-intl"));
        assert!(!supports_live_model_fetch("minimax-cn"));
        assert!(!supports_live_model_fetch("unknown-provider"));
    }

    #[test]
    fn curated_models_provider_aliases_share_same_catalog() {
        assert_eq!(
            curated_models_for_provider("xai"),
            curated_models_for_provider("grok")
        );
        assert_eq!(
            curated_models_for_provider("together-ai"),
            curated_models_for_provider("together")
        );
        assert_eq!(
            curated_models_for_provider("gemini"),
            curated_models_for_provider("google")
        );
        assert_eq!(
            curated_models_for_provider("gemini"),
            curated_models_for_provider("google-gemini")
        );
        assert_eq!(
            curated_models_for_provider("qwen"),
            curated_models_for_provider("qwen-intl")
        );
        assert_eq!(
            curated_models_for_provider("qwen"),
            curated_models_for_provider("dashscope-us")
        );
        assert_eq!(
            curated_models_for_provider("minimax"),
            curated_models_for_provider("minimax-cn")
        );
        assert_eq!(
            curated_models_for_provider("zai"),
            curated_models_for_provider("zai-cn")
        );
        assert_eq!(
            curated_models_for_provider("nvidia"),
            curated_models_for_provider("nvidia-nim")
        );
        assert_eq!(
            curated_models_for_provider("nvidia"),
            curated_models_for_provider("build.nvidia.com")
        );
        assert_eq!(
            curated_models_for_provider("llamacpp"),
            curated_models_for_provider("llama.cpp")
        );
        assert_eq!(
            curated_models_for_provider("bedrock"),
            curated_models_for_provider("aws-bedrock")
        );
    }

    #[test]
    fn curated_models_for_nvidia_include_nim_catalog_entries() {
        let ids: Vec<String> = curated_models_for_provider("nvidia")
            .into_iter()
            .map(|(id, _)| id)
            .collect();

        assert!(ids.contains(&"meta/llama-3.3-70b-instruct".to_string()));
        assert!(ids.contains(&"deepseek-ai/deepseek-v3.2".to_string()));
        assert!(ids.contains(&"nvidia/llama-3.3-nemotron-super-49b-v1.5".to_string()));
    }

    #[test]
    fn models_endpoint_for_provider_handles_region_aliases() {
        assert_eq!(
            models_endpoint_for_provider("glm-cn"),
            Some("https://open.bigmodel.cn/api/paas/v4/models")
        );
        assert_eq!(
            models_endpoint_for_provider("zai-cn"),
            Some("https://open.bigmodel.cn/api/coding/paas/v4/models")
        );
        assert_eq!(
            models_endpoint_for_provider("qwen-intl"),
            Some("https://dashscope-intl.aliyuncs.com/compatible-mode/v1/models")
        );
    }

    #[test]
    fn models_endpoint_for_provider_supports_additional_openai_compatible_providers() {
        assert_eq!(
            models_endpoint_for_provider("openai-codex"),
            Some("https://api.openai.com/v1/models")
        );
        assert_eq!(
            models_endpoint_for_provider("venice"),
            Some("https://api.venice.ai/api/v1/models")
        );
        assert_eq!(
            models_endpoint_for_provider("cohere"),
            Some("https://api.cohere.com/compatibility/v1/models")
        );
        assert_eq!(
            models_endpoint_for_provider("moonshot"),
            Some("https://api.moonshot.ai/v1/models")
        );
        assert_eq!(
            models_endpoint_for_provider("llamacpp"),
            Some("http://localhost:8080/v1/models")
        );
        assert_eq!(
            models_endpoint_for_provider("llama.cpp"),
            Some("http://localhost:8080/v1/models")
        );
        assert_eq!(
            models_endpoint_for_provider("sglang"),
            Some("http://localhost:30000/v1/models")
        );
        assert_eq!(
            models_endpoint_for_provider("vllm"),
            Some("http://localhost:8000/v1/models")
        );
        assert_eq!(models_endpoint_for_provider("perplexity"), None);
        assert_eq!(models_endpoint_for_provider("unknown-provider"), None);
    }

    #[test]
    fn resolve_live_models_endpoint_prefers_llamacpp_custom_url() {
        assert_eq!(
            resolve_live_models_endpoint("llamacpp", Some("http://127.0.0.1:8033/v1")),
            Some("http://127.0.0.1:8033/v1/models".to_string())
        );
        assert_eq!(
            resolve_live_models_endpoint("llama.cpp", Some("http://127.0.0.1:8033/v1/")),
            Some("http://127.0.0.1:8033/v1/models".to_string())
        );
        assert_eq!(
            resolve_live_models_endpoint("llamacpp", Some("http://127.0.0.1:8033/v1/models")),
            Some("http://127.0.0.1:8033/v1/models".to_string())
        );
    }

    #[test]
    fn resolve_live_models_endpoint_falls_back_to_provider_defaults() {
        assert_eq!(
            resolve_live_models_endpoint("llamacpp", None),
            Some("http://localhost:8080/v1/models".to_string())
        );
        assert_eq!(
            resolve_live_models_endpoint("sglang", None),
            Some("http://localhost:30000/v1/models".to_string())
        );
        assert_eq!(
            resolve_live_models_endpoint("vllm", None),
            Some("http://localhost:8000/v1/models".to_string())
        );
        assert_eq!(
            resolve_live_models_endpoint("venice", Some("http://localhost:9999/v1")),
            Some("https://api.venice.ai/api/v1/models".to_string())
        );
        assert_eq!(resolve_live_models_endpoint("unknown-provider", None), None);
    }

    #[test]
    fn resolve_live_models_endpoint_supports_custom_provider_urls() {
        assert_eq!(
            resolve_live_models_endpoint("custom:https://proxy.example.com/v1", None),
            Some("https://proxy.example.com/v1/models".to_string())
        );
        assert_eq!(
            resolve_live_models_endpoint("custom:https://proxy.example.com/v1/models", None),
            Some("https://proxy.example.com/v1/models".to_string())
        );
    }

    #[test]
    fn normalize_ollama_endpoint_url_strips_api_suffix_and_trailing_slash() {
        assert_eq!(
            normalize_ollama_endpoint_url(" https://ollama.com/api/ "),
            "https://ollama.com".to_string()
        );
        assert_eq!(
            normalize_ollama_endpoint_url("https://ollama.com/"),
            "https://ollama.com".to_string()
        );
        assert_eq!(normalize_ollama_endpoint_url(""), "");
    }

    #[test]
    fn ollama_uses_remote_endpoint_distinguishes_local_and_remote_urls() {
        assert!(!ollama_uses_remote_endpoint(None));
        assert!(!ollama_uses_remote_endpoint(Some("http://localhost:11434")));
        assert!(!ollama_uses_remote_endpoint(Some(
            "http://127.0.0.1:11434/api"
        )));
        assert!(ollama_uses_remote_endpoint(Some("https://ollama.com")));
        assert!(ollama_uses_remote_endpoint(Some("https://ollama.com/api")));
    }

    #[test]
    fn resolve_live_models_endpoint_prefers_vllm_custom_url() {
        assert_eq!(
            resolve_live_models_endpoint("vllm", Some("http://127.0.0.1:9000/v1")),
            Some("http://127.0.0.1:9000/v1/models".to_string())
        );
        assert_eq!(
            resolve_live_models_endpoint("vllm", Some("http://127.0.0.1:9000/v1/models")),
            Some("http://127.0.0.1:9000/v1/models".to_string())
        );
    }

    #[test]
    fn parse_openai_model_ids_supports_data_array_payload() {
        let payload = json!({
            "data": [
                {"id": "  gpt-5.1  "},
                {"id": "gpt-5-mini"},
                {"id": "gpt-5.1"},
                {"id": ""}
            ]
        });

        let ids = parse_openai_compatible_model_ids(&payload);
        assert_eq!(ids, vec!["gpt-5-mini".to_string(), "gpt-5.1".to_string()]);
    }

    #[test]
    fn parse_openai_model_ids_supports_root_array_payload() {
        let payload = json!([
            {"id": "alpha"},
            {"id": "beta"},
            {"id": "alpha"}
        ]);

        let ids = parse_openai_compatible_model_ids(&payload);
        assert_eq!(ids, vec!["alpha".to_string(), "beta".to_string()]);
    }

    #[test]
    fn normalize_model_ids_deduplicates_case_insensitively() {
        let ids = normalize_model_ids(vec![
            "GPT-5".to_string(),
            "gpt-5".to_string(),
            "gpt-5-mini".to_string(),
            " GPT-5-MINI ".to_string(),
        ]);
        assert_eq!(ids, vec!["GPT-5".to_string(), "gpt-5-mini".to_string()]);
    }

    #[test]
    fn parse_gemini_model_ids_filters_for_generate_content() {
        let payload = json!({
            "models": [
                {
                    "name": "models/gemini-2.5-pro",
                    "supportedGenerationMethods": ["generateContent", "countTokens"]
                },
                {
                    "name": "models/text-embedding-004",
                    "supportedGenerationMethods": ["embedContent"]
                },
                {
                    "name": "models/gemini-2.5-flash",
                    "supportedGenerationMethods": ["generateContent"]
                }
            ]
        });

        let ids = parse_gemini_model_ids(&payload);
        assert_eq!(
            ids,
            vec!["gemini-2.5-flash".to_string(), "gemini-2.5-pro".to_string()]
        );
    }

    #[test]
    fn parse_ollama_model_ids_extracts_and_deduplicates_names() {
        let payload = json!({
            "models": [
                {"name": "llama3.2:latest"},
                {"name": "mistral:latest"},
                {"name": "llama3.2:latest"}
            ]
        });

        let ids = parse_ollama_model_ids(&payload);
        assert_eq!(
            ids,
            vec!["llama3.2:latest".to_string(), "mistral:latest".to_string()]
        );
    }

    #[tokio::test]
    async fn model_cache_round_trip_returns_fresh_entry() {
        let tmp = TempDir::new().unwrap();
        let models = vec!["gpt-5.1".to_string(), "gpt-5-mini".to_string()];

        cache_live_models_for_provider(tmp.path(), "openai", &models)
            .await
            .unwrap();

        let cached = load_cached_models_for_provider(tmp.path(), "openai", MODEL_CACHE_TTL_SECS)
            .await
            .unwrap();
        let cached = cached.expect("expected fresh cached models");

        assert_eq!(cached.models.len(), 2);
        assert!(cached.models.contains(&"gpt-5.1".to_string()));
        assert!(cached.models.contains(&"gpt-5-mini".to_string()));
    }

    #[tokio::test]
    async fn model_cache_ttl_filters_stale_entries() {
        let tmp = TempDir::new().unwrap();
        let stale = ModelCacheState {
            entries: vec![ModelCacheEntry {
                provider: "openai".to_string(),
                fetched_at_unix: now_unix_secs().saturating_sub(MODEL_CACHE_TTL_SECS + 120),
                models: vec!["gpt-5.1".to_string()],
            }],
        };

        save_model_cache_state(tmp.path(), &stale).await.unwrap();

        let fresh = load_cached_models_for_provider(tmp.path(), "openai", MODEL_CACHE_TTL_SECS)
            .await
            .unwrap();
        assert!(fresh.is_none());

        let stale_any = load_any_cached_models_for_provider(tmp.path(), "openai")
            .await
            .unwrap();
        assert!(stale_any.is_some());
    }

    #[tokio::test]
    async fn run_models_refresh_uses_fresh_cache_without_network() {
        let tmp = TempDir::new().unwrap();

        cache_live_models_for_provider(tmp.path(), "openai", &["gpt-5.1".to_string()])
            .await
            .unwrap();

        let config = Config {
            workspace_dir: tmp.path().to_path_buf(),
            default_provider: Some("openai".to_string()),
            ..Config::default()
        };

        run_models_refresh(&config, None, false).await.unwrap();
    }

    #[tokio::test]
    async fn run_models_refresh_rejects_unsupported_provider() {
        let tmp = TempDir::new().unwrap();

        let config = Config {
            workspace_dir: tmp.path().to_path_buf(),
            // Use a non-provider channel key to keep this test deterministic and offline.
            default_provider: Some("imessage".to_string()),
            ..Config::default()
        };

        let err = run_models_refresh(&config, None, true).await.unwrap_err();
        assert!(err
            .to_string()
            .contains("does not support live model discovery"));
    }

    // ── provider_env_var ────────────────────────────────────────

    #[test]
    fn provider_env_var_known_providers() {
        assert_eq!(provider_env_var("openrouter"), "OPENROUTER_API_KEY");
        assert_eq!(provider_env_var("anthropic"), "ANTHROPIC_API_KEY");
        assert_eq!(provider_env_var("openai-codex"), "OPENAI_API_KEY");
        assert_eq!(provider_env_var("openai"), "OPENAI_API_KEY");
        assert_eq!(provider_env_var("ollama"), "OLLAMA_API_KEY");
        assert_eq!(provider_env_var("llamacpp"), "LLAMACPP_API_KEY");
        assert_eq!(provider_env_var("llama.cpp"), "LLAMACPP_API_KEY");
        assert_eq!(provider_env_var("sglang"), "SGLANG_API_KEY");
        assert_eq!(provider_env_var("vllm"), "VLLM_API_KEY");
        assert_eq!(provider_env_var("xai"), "XAI_API_KEY");
        assert_eq!(provider_env_var("grok"), "XAI_API_KEY"); // alias
        assert_eq!(provider_env_var("together"), "TOGETHER_API_KEY"); // alias
        assert_eq!(provider_env_var("together-ai"), "TOGETHER_API_KEY");
        assert_eq!(provider_env_var("google"), "GEMINI_API_KEY"); // alias
        assert_eq!(provider_env_var("google-gemini"), "GEMINI_API_KEY"); // alias
        assert_eq!(provider_env_var("gemini"), "GEMINI_API_KEY");
        assert_eq!(provider_env_var("qwen"), "DASHSCOPE_API_KEY");
        assert_eq!(provider_env_var("qwen-intl"), "DASHSCOPE_API_KEY");
        assert_eq!(provider_env_var("dashscope-us"), "DASHSCOPE_API_KEY");
        assert_eq!(provider_env_var("qwen-code"), "QWEN_OAUTH_TOKEN");
        assert_eq!(provider_env_var("qwen-oauth"), "QWEN_OAUTH_TOKEN");
        assert_eq!(provider_env_var("glm-cn"), "GLM_API_KEY");
        assert_eq!(provider_env_var("minimax-cn"), "MINIMAX_API_KEY");
        assert_eq!(provider_env_var("kimi-code"), "KIMI_CODE_API_KEY");
        assert_eq!(provider_env_var("kimi_coding"), "KIMI_CODE_API_KEY");
        assert_eq!(provider_env_var("kimi_for_coding"), "KIMI_CODE_API_KEY");
        assert_eq!(provider_env_var("minimax-oauth"), "MINIMAX_API_KEY");
        assert_eq!(provider_env_var("minimax-oauth-cn"), "MINIMAX_API_KEY");
        assert_eq!(provider_env_var("moonshot-intl"), "MOONSHOT_API_KEY");
        assert_eq!(provider_env_var("zai-cn"), "ZAI_API_KEY");
        assert_eq!(provider_env_var("nvidia"), "NVIDIA_API_KEY");
        assert_eq!(provider_env_var("nvidia-nim"), "NVIDIA_API_KEY"); // alias
        assert_eq!(provider_env_var("build.nvidia.com"), "NVIDIA_API_KEY"); // alias
        assert_eq!(provider_env_var("astrai"), "ASTRAI_API_KEY");
        assert_eq!(provider_env_var("opencode-go"), "OPENCODE_GO_API_KEY");
    }

    #[test]
    fn provider_supports_keyless_local_usage_for_local_providers() {
        assert!(provider_supports_keyless_local_usage("ollama"));
        assert!(provider_supports_keyless_local_usage("llamacpp"));
        assert!(provider_supports_keyless_local_usage("llama.cpp"));
        assert!(provider_supports_keyless_local_usage("sglang"));
        assert!(provider_supports_keyless_local_usage("vllm"));
        assert!(!provider_supports_keyless_local_usage("openai"));
    }

    #[test]
    fn provider_supports_device_flow_copilot() {
        assert!(provider_supports_device_flow("copilot"));
        assert!(provider_supports_device_flow("github-copilot"));
        assert!(provider_supports_device_flow("gemini"));
        assert!(provider_supports_device_flow("openai-codex"));
        assert!(!provider_supports_device_flow("openai"));
        assert!(!provider_supports_device_flow("openrouter"));
    }

    #[test]
    fn local_provider_choices_include_sglang() {
        let choices = local_provider_choices();
        assert!(choices.iter().any(|(provider, _)| *provider == "sglang"));
    }

    #[test]
    fn provider_env_var_unknown_falls_back() {
        assert_eq!(provider_env_var("some-new-provider"), "API_KEY");
    }

    #[test]
    fn backend_key_from_choice_maps_supported_backends() {
        assert_eq!(backend_key_from_choice(0), "sqlite");
        assert_eq!(backend_key_from_choice(1), "lucid");
        assert_eq!(backend_key_from_choice(2), "markdown");
        assert_eq!(backend_key_from_choice(3), "none");
        assert_eq!(backend_key_from_choice(999), "sqlite");
    }

    #[test]
    fn memory_backend_profile_marks_lucid_as_optional_sqlite_backed() {
        let lucid = memory_backend_profile("lucid");
        assert!(lucid.auto_save_default);
        assert!(lucid.uses_sqlite_hygiene);
        assert!(lucid.sqlite_based);
        assert!(lucid.optional_dependency);

        let markdown = memory_backend_profile("markdown");
        assert!(markdown.auto_save_default);
        assert!(!markdown.uses_sqlite_hygiene);

        let none = memory_backend_profile("none");
        assert!(!none.auto_save_default);
        assert!(!none.uses_sqlite_hygiene);

        let custom = memory_backend_profile("custom-memory");
        assert!(custom.auto_save_default);
        assert!(!custom.uses_sqlite_hygiene);
    }

    #[test]
    fn memory_config_defaults_for_lucid_enable_sqlite_hygiene() {
        let config = memory_config_defaults_for_backend("lucid");
        assert_eq!(config.backend, "lucid");
        assert!(config.auto_save);
        assert!(config.hygiene_enabled);
        assert_eq!(config.archive_after_days, 7);
        assert_eq!(config.purge_after_days, 30);
        assert_eq!(config.embedding_cache_size, 10000);
    }

    #[test]
    fn memory_config_defaults_for_none_disable_sqlite_hygiene() {
        let config = memory_config_defaults_for_backend("none");
        assert_eq!(config.backend, "none");
        assert!(!config.auto_save);
        assert!(!config.hygiene_enabled);
        assert_eq!(config.archive_after_days, 0);
        assert_eq!(config.purge_after_days, 0);
        assert_eq!(config.embedding_cache_size, 0);
    }

    #[test]
    fn channel_menu_choices_include_signal_nextcloud_lark_and_feishu() {
        assert!(channel_menu_choices().contains(&ChannelMenuChoice::Signal));
        assert!(channel_menu_choices().contains(&ChannelMenuChoice::NextcloudTalk));
        assert!(channel_menu_choices().contains(&ChannelMenuChoice::Lark));
        assert!(channel_menu_choices().contains(&ChannelMenuChoice::Feishu));
    }

    #[test]
    fn launchable_channels_include_signal_mattermost_qq_nextcloud_and_feishu() {
        let mut channels = ChannelsConfig::default();
        assert!(!has_launchable_channels(&channels));

        channels.signal = Some(crate::config::schema::SignalConfig {
            http_url: "http://127.0.0.1:8686".into(),
            account: "+1234567890".into(),
            group_id: None,
            allowed_from: vec!["*".into()],
            ignore_attachments: false,
            ignore_stories: true,
        });
        assert!(has_launchable_channels(&channels));

        channels.signal = None;
        channels.mattermost = Some(crate::config::schema::MattermostConfig {
            url: "https://mattermost.example.com".into(),
            bot_token: "token".into(),
            channel_id: Some("channel".into()),
            allowed_users: vec!["*".into()],
            thread_replies: Some(true),
            mention_only: Some(false),
        });
        assert!(has_launchable_channels(&channels));

        channels.mattermost = None;
        channels.qq = Some(crate::config::schema::QQConfig {
            app_id: "app-id".into(),
            app_secret: "app-secret".into(),
            allowed_users: vec!["*".into()],
        });
        assert!(has_launchable_channels(&channels));

        channels.qq = None;
        channels.nextcloud_talk = Some(crate::config::schema::NextcloudTalkConfig {
            base_url: "https://cloud.example.com".into(),
            app_token: "token".into(),
            webhook_secret: Some("secret".into()),
            allowed_users: vec!["*".into()],
        });
        assert!(has_launchable_channels(&channels));

        channels.nextcloud_talk = None;
        channels.feishu = Some(crate::config::schema::FeishuConfig {
            app_id: "cli_123".into(),
            app_secret: "secret".into(),
            encrypt_key: None,
            verification_token: None,
            allowed_users: vec!["*".into()],
            receive_mode: crate::config::schema::LarkReceiveMode::Websocket,
            port: None,
        });
        assert!(has_launchable_channels(&channels));
    }
}
