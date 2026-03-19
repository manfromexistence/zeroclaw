//! Channel setup and configuration
//!
//! This module handles interactive setup for all communication channels
//! (Telegram, Discord, Slack, Matrix, etc.)

use anyhow::Result;
use onboard::prompts;
use onboard::prompts::PromptInteraction;
use serde_json::Value;
use std::time::Duration;

use crate::config::{
    ChannelsConfig, DiscordConfig, IMessageConfig, LarkConfig, MatrixConfig, SlackConfig,
    TelegramConfig, WebhookConfig,
};

#[cfg(feature = "channel-nostr")]
use crate::config::schema::{default_nostr_relays, NostrConfig};

use crate::config::schema::{
    DingTalkConfig, IrcConfig, LarkReceiveMode, LinqConfig, NextcloudTalkConfig, QQConfig,
    SignalConfig, StreamMode, WhatsAppConfig,
};

// Import helper functions and types from wizard
use super::wizard::{ChannelMenuChoice, channel_menu_choices};

#[allow(clippy::too_many_lines)]
pub fn setup_channels() -> Result<ChannelsConfig> {
    prompts::log::info("Channels let you talk to ZeroClaw from anywhere.")?;
    prompts::log::info("CLI is always available. Connect more channels now.")?;

    let mut config = ChannelsConfig::default();
    let menu_choices = channel_menu_choices();

    loop {
        let options: Vec<String> = menu_choices
            .iter()
            .map(|choice| match choice {
                ChannelMenuChoice::Telegram => format!(
                    "Telegram   {}",
                    if config.telegram.is_some() {
                        "✅ connected"
                    } else {
                        "— connect your bot"
                    }
                ),
                ChannelMenuChoice::Discord => format!(
                    "Discord    {}",
                    if config.discord.is_some() {
                        "✅ connected"
                    } else {
                        "— connect your bot"
                    }
                ),
                ChannelMenuChoice::Slack => format!(
                    "Slack      {}",
                    if config.slack.is_some() {
                        "✅ connected"
                    } else {
                        "— connect your bot"
                    }
                ),
                ChannelMenuChoice::IMessage => format!(
                    "iMessage   {}",
                    if config.imessage.is_some() {
                        "✅ configured"
                    } else {
                        "— macOS only"
                    }
                ),
                ChannelMenuChoice::Matrix => format!(
                    "Matrix     {}",
                    if config.matrix.is_some() {
                        "✅ connected"
                    } else {
                        "— self-hosted chat"
                    }
                ),
                ChannelMenuChoice::Signal => format!(
                    "Signal     {}",
                    if config.signal.is_some() {
                        "✅ connected"
                    } else {
                        "— signal-cli daemon bridge"
                    }
                ),
                ChannelMenuChoice::WhatsApp => format!(
                    "WhatsApp   {}",
                    if config.whatsapp.is_some() {
                        "✅ connected"
                    } else {
                        "— Business Cloud API"
                    }
                ),
                ChannelMenuChoice::Linq => format!(
                    "Linq       {}",
                    if config.linq.is_some() {
                        "✅ connected"
                    } else {
                        "— iMessage/RCS/SMS via Linq API"
                    }
                ),
                ChannelMenuChoice::Irc => format!(
                    "IRC        {}",
                    if config.irc.is_some() {
                        "✅ configured"
                    } else {
                        "— IRC over TLS"
                    }
                ),
                ChannelMenuChoice::Webhook => format!(
                    "Webhook    {}",
                    if config.webhook.is_some() {
                        "✅ configured"
                    } else {
                        "— HTTP endpoint"
                    }
                ),
                ChannelMenuChoice::NextcloudTalk => format!(
                    "Nextcloud  {}",
                    if config.nextcloud_talk.is_some() {
                        "✅ connected"
                    } else {
                        "— Talk webhook + OCS API"
                    }
                ),
                ChannelMenuChoice::DingTalk => format!(
                    "DingTalk   {}",
                    if config.dingtalk.is_some() {
                        "✅ connected"
                    } else {
                        "— DingTalk Stream Mode"
                    }
                ),
                ChannelMenuChoice::QqOfficial => format!(
                    "QQ Official {}",
                    if config.qq.is_some() {
                        "✅ connected"
                    } else {
                        "— Tencent QQ Bot"
                    }
                ),
                ChannelMenuChoice::Lark => format!(
                    "Lark       {}",
                    if config.lark.as_ref().is_some_and(|cfg| !cfg.use_feishu) {
                        "✅ connected"
                    } else {
                        "— Lark Bot"
                    }
                ),
                ChannelMenuChoice::Feishu => format!(
                    "Feishu     {}",
                    if config.feishu.is_some()
                        || config.lark.as_ref().is_some_and(|cfg| cfg.use_feishu)
                    {
                        "✅ connected"
                    } else {
                        "— Feishu Bot"
                    }
                ),
                #[cfg(feature = "channel-nostr")]
                ChannelMenuChoice::Nostr => format!(
                    "Nostr {}",
                    if config.nostr.is_some() {
                        "✅ connected"
                    } else {
                        "     — Nostr DMs"
                    }
                ),
                ChannelMenuChoice::Done => "Done — finish setup".to_string(),
            })
            .collect();

        let mut select = prompts::select("Connect a channel (or Done to continue)");
        for (i, option) in options.iter().enumerate() {
            select = select.item(i, option, "");
        }
        let selection: usize = select.interact()?;

        let choice = menu_choices
            .get(selection)
            .copied()
            .unwrap_or(ChannelMenuChoice::Done);

        match choice {
            ChannelMenuChoice::Telegram => {
                // ── Telegram ──
                prompts::section_with_width("Telegram Setup — talk to ZeroClaw from Telegram", 70, |lines| {
                    lines.push("1. Open Telegram and message @BotFather".to_string());
                    lines.push("2. Send /newbot and follow the prompts".to_string());
                    lines.push("3. Copy the bot token and paste it below".to_string());
                })?;

                let token = prompts::input::input("Bot token (from @BotFather)")
                    .placeholder("paste your bot token here")
                    .interact()?;

                if token.trim().is_empty() {
                    prompts::log::info("Skipped")?;
                    continue;
                }

                // Test connection (run entirely in separate thread — reqwest::blocking Response
                // must be used and dropped there to avoid "Cannot drop a runtime" panic)
                prompts::log::step("Testing connection...")?;
                let token_clone = token.clone();
                let thread_result = std::thread::spawn(move || {
                    let client = reqwest::blocking::Client::new();
                    let url = format!("https://api.telegram.org/bot{token_clone}/getMe");
                    let resp = client.get(&url).send()?;
                    let ok = resp.status().is_success();
                    let data: serde_json::Value = resp.json().unwrap_or_default();
                    let bot_name = data
                        .get("result")
                        .and_then(|r| r.get("username"))
                        .and_then(serde_json::Value::as_str)
                        .unwrap_or("unknown")
                        .to_string();
                    Ok::<_, reqwest::Error>((ok, bot_name))
                })
                .join();
                match thread_result {
                    Ok(Ok((true, bot_name))) => {
                        prompts::log::success(format!("Connected as @{bot_name}"))?;
                    }
                    _ => {
                        prompts::log::error("Connection failed — check your token and try again")?;
                        continue;
                    }
                }

                prompts::log::info("Allowlist your own Telegram identity first (recommended for secure + fast setup).")?;
                prompts::log::info("Use your @username without '@' (example: argenis), or your numeric Telegram user ID.")?;
                prompts::log::info("Use '*' only for temporary open testing.")?;

                let users_str = prompts::input::input(
                    "Allowed Telegram identities (comma-separated: username without '@' and/or numeric user ID, '*' for all)",
                )
                .placeholder("username or user ID")
                .interact()?;

                let allowed_users = if users_str.trim() == "*" {
                    vec!["*".into()]
                } else {
                    users_str
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect()
                };

                if allowed_users.is_empty() {
                    prompts::log::warning("No users allowlisted — Telegram inbound messages will be denied until you add your username/user ID or '*'.")?;
                }

                config.telegram = Some(TelegramConfig {
                    bot_token: token,
                    allowed_users,
                    stream_mode: StreamMode::default(),
                    draft_update_interval_ms: 1000,
                    interrupt_on_new_message: false,
                    mention_only: false,
                    ack_reactions: None,
                });
            }
            ChannelMenuChoice::Discord => {
                // ── Discord ──
                prompts::section_with_width("Discord Setup — talk to ZeroClaw from Discord", 70, |lines| {
                    lines.push("1. Go to https://discord.com/developers/applications".to_string());
                    lines.push("2. Create a New Application → Bot → Copy token".to_string());
                    lines.push("3. Enable MESSAGE CONTENT intent under Bot settings".to_string());
                    lines.push("4. Invite bot to your server with messages permission".to_string());
                })?;

                let token = prompts::input::input("Bot token")
                    .placeholder("paste your bot token here")
                    .interact()?;

                if token.trim().is_empty() {
                    prompts::log::info("Skipped")?;
                    continue;
                }

                // Test connection (run entirely in separate thread — Response must be used/dropped there)
                prompts::log::step("Testing connection...")?;
                let token_clone = token.clone();
                let thread_result = std::thread::spawn(move || {
                    let client = reqwest::blocking::Client::new();
                    let resp = client
                        .get("https://discord.com/api/v10/users/@me")
                        .header("Authorization", format!("Bot {token_clone}"))
                        .send()?;
                    let ok = resp.status().is_success();
                    let data: serde_json::Value = resp.json().unwrap_or_default();
                    let bot_name = data
                        .get("username")
                        .and_then(serde_json::Value::as_str)
                        .unwrap_or("unknown")
                        .to_string();
                    Ok::<_, reqwest::Error>((ok, bot_name))
                })
                .join();
                match thread_result {
                    Ok(Ok((true, bot_name))) => {
                        prompts::log::success(format!("Connected as {bot_name}"))?;
                    }
                    _ => {
                        prompts::log::error("Connection failed — check your token and try again")?;
                        continue;
                    }
                }

                let guild = prompts::input::input("Server (guild) ID (optional, Enter to skip)")
                    .placeholder("guild ID")
                    .interact()?;

                prompts::log::info("Allowlist your own Discord user ID first (recommended).")?;
                prompts::log::info("Get it in Discord: Settings -> Advanced -> Developer Mode (ON), then right-click your profile -> Copy User ID.")?;
                prompts::log::info("Use '*' only for temporary open testing.")?;

                let allowed_users_str = prompts::input::input(
                    "Allowed Discord user IDs (comma-separated, recommended: your own ID, '*' for all)",
                )
                .placeholder("user IDs")
                .interact()?;

                let allowed_users = if allowed_users_str.trim().is_empty() {
                    vec![]
                } else {
                    allowed_users_str
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect()
                };

                if allowed_users.is_empty() {
                    prompts::log::warning("No users allowlisted — Discord inbound messages will be denied until you add IDs or '*'.")?;
                }

                config.discord = Some(DiscordConfig {
                    bot_token: token,
                    guild_id: if guild.is_empty() { None } else { Some(guild) },
                    allowed_users,
                    listen_to_bots: false,
                    mention_only: false,
                });
            }
            ChannelMenuChoice::Slack => {
                // ── Slack ──
                prompts::section_with_width("Slack Setup — talk to ZeroClaw from Slack", 70, |lines| {
                    lines.push("1. Go to https://api.slack.com/apps → Create New App".to_string());
                    lines.push("2. Add Bot Token Scopes: chat:write, channels:history".to_string());
                    lines.push("3. Install to workspace and copy the Bot Token".to_string());
                })?;

                let token = prompts::input::input("Bot token (xoxb-...)")
                    .placeholder("xoxb-...")
                    .interact()?;

                if token.trim().is_empty() {
                    prompts::log::info("Skipped")?;
                    continue;
                }

                // Test connection (run entirely in separate thread — Response must be used/dropped there)
                prompts::log::step("Testing connection...")?;
                let token_clone = token.clone();
                let thread_result = std::thread::spawn(move || {
                    let client = reqwest::blocking::Client::new();
                    let resp = client
                        .get("https://slack.com/api/auth.test")
                        .bearer_auth(&token_clone)
                        .send()?;
                    let ok = resp.status().is_success();
                    let data: serde_json::Value = resp.json().unwrap_or_default();
                    let api_ok = data
                        .get("ok")
                        .and_then(serde_json::Value::as_bool)
                        .unwrap_or(false);
                    let team = data
                        .get("team")
                        .and_then(serde_json::Value::as_str)
                        .unwrap_or("unknown")
                        .to_string();
                    let err = data
                        .get("error")
                        .and_then(serde_json::Value::as_str)
                        .unwrap_or("unknown error")
                        .to_string();
                    Ok::<_, reqwest::Error>((ok, api_ok, team, err))
                })
                .join();
                match thread_result {
                    Ok(Ok((true, true, team, _))) => {
                        prompts::log::success(format!("Connected to workspace: {team}"))?;
                    }
                    Ok(Ok((true, false, _, err))) => {
                        prompts::log::error(format!("Slack error: {err}"))?;
                        continue;
                    }
                    _ => {
                        prompts::log::error("Connection failed — check your token")?;
                        continue;
                    }
                }

                let app_token = prompts::input::input("App token (xapp-..., optional, Enter to skip)")
                    .placeholder("xapp-...")
                    .interact()?;

                let channel = prompts::input::input(
                    "Default channel ID (optional, Enter to skip for all accessible channels; '*' also means all)",
                )
                .placeholder("channel ID")
                .interact()?;

                prompts::log::info("Allowlist your own Slack member ID first (recommended).")?;
                prompts::log::info("Member IDs usually start with 'U' (open your Slack profile -> More -> Copy member ID).")?;
                prompts::log::info("Use '*' only for temporary open testing.")?;

                let allowed_users_str = prompts::input::input(
                    "Allowed Slack user IDs (comma-separated, recommended: your own member ID, '*' for all)",
                )
                .placeholder("member IDs")
                .interact()?;

                let allowed_users = if allowed_users_str.trim().is_empty() {
                    vec![]
                } else {
                    allowed_users_str
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect()
                };

                if allowed_users.is_empty() {
                    prompts::log::warning("No users allowlisted — Slack inbound messages will be denied until you add IDs or '*'.")?;
                }

                config.slack = Some(SlackConfig {
                    bot_token: token,
                    app_token: if app_token.is_empty() {
                        None
                    } else {
                        Some(app_token)
                    },
                    channel_id: if channel.is_empty() {
                        None
                    } else {
                        Some(channel)
                    },
                    allowed_users,
                    interrupt_on_new_message: false,
                    mention_only: false,
                });
            }
            ChannelMenuChoice::IMessage => {
                // ── iMessage ──
                prompts::section_with_width("iMessage Setup — macOS only, reads from Messages.app", 70, |lines| {
                    lines.push("ZeroClaw reads your iMessage database and replies via AppleScript.".to_string());
                    lines.push("You need to grant Full Disk Access to your terminal in System Settings.".to_string());
                })?;

                if !cfg!(target_os = "macos") {
                    prompts::log::warning("iMessage is only available on macOS.")?;
                    continue;
                }

                let contacts_str = prompts::input::input("Allowed contacts (comma-separated phone/email, or * for all)")
                    .placeholder("*")
                    .interact()?;

                let contacts_str = if contacts_str.trim().is_empty() {
                    "*".to_string()
                } else {
                    contacts_str
                };

                let allowed_contacts = if contacts_str.trim() == "*" {
                    vec!["*".into()]
                } else {
                    contacts_str
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect()
                };

                config.imessage = Some(IMessageConfig { allowed_contacts });
                prompts::log::success(format!("iMessage configured (contacts: {contacts_str})"))?;
            }
            ChannelMenuChoice::Matrix => {
                // ── Matrix ──
                prompts::section_with_width("Matrix Setup — self-hosted, federated chat", 70, |lines| {
                    lines.push("You need a Matrix account and an access token.".to_string());
                    lines.push("Get a token via Element → Settings → Help & About → Access Token.".to_string());
                })?;

                let homeserver = prompts::input::input("Homeserver URL (e.g. https://matrix.org)")
                    .placeholder("https://matrix.org")
                    .interact()?;

                if homeserver.trim().is_empty() {
                    prompts::log::info("Skipped")?;
                    continue;
                }

                let access_token = prompts::input::input("Access token")
                    .placeholder("paste your access token")
                    .interact()?;

                if access_token.trim().is_empty() {
                    prompts::log::info("Skipped — token required")?;
                    continue;
                }

                // Test connection (run entirely in separate thread — Response must be used/dropped there)
                let hs = homeserver.trim_end_matches('/');
                prompts::log::step("Testing connection...")?;
                let hs_owned = hs.to_string();
                let access_token_clone = access_token.clone();
                let thread_result = std::thread::spawn(move || {
                    let client = reqwest::blocking::Client::new();
                    let resp = client
                        .get(format!("{hs_owned}/_matrix/client/v3/account/whoami"))
                        .header("Authorization", format!("Bearer {access_token_clone}"))
                        .send()?;
                    let ok = resp.status().is_success();

                    if !ok {
                        return Ok::<_, reqwest::Error>((false, None, None));
                    }

                    let payload: Value = match resp.json() {
                        Ok(payload) => payload,
                        Err(_) => Value::Null,
                    };
                    let user_id = payload
                        .get("user_id")
                        .and_then(|value| value.as_str())
                        .map(|value| value.to_string());
                    let device_id = payload
                        .get("device_id")
                        .and_then(|value| value.as_str())
                        .map(|value| value.to_string());

                    Ok::<_, reqwest::Error>((true, user_id, device_id))
                })
                .join();

                let (detected_user_id, detected_device_id) = match thread_result {
                    Ok(Ok((true, user_id, device_id))) => {
                        prompts::log::success("Connection verified")?;

                        if device_id.is_none() {
                            prompts::log::warning("Homeserver did not return device_id from whoami. If E2EE decryption fails, set channels.matrix.device_id manually in config.toml.")?;
                        }

                        (user_id, device_id)
                    }
                    _ => {
                        prompts::log::error("Connection failed — check homeserver URL and token")?;
                        continue;
                    }
                };

                let room_id = prompts::input::input("Room ID (e.g. !abc123:matrix.org)")
                    .placeholder("!abc123:matrix.org")
                    .interact()?;

                let users_str = prompts::input::input("Allowed users (comma-separated @user:server, or * for all)")
                    .placeholder("*")
                    .interact()?;

                let users_str = if users_str.trim().is_empty() {
                    "*".to_string()
                } else {
                    users_str
                };

                let allowed_users = if users_str.trim() == "*" {
                    vec!["*".into()]
                } else {
                    users_str.split(',').map(|s| s.trim().to_string()).collect()
                };

                config.matrix = Some(MatrixConfig {
                    homeserver: homeserver.trim_end_matches('/').to_string(),
                    access_token,
                    user_id: detected_user_id,
                    device_id: detected_device_id,
                    room_id,
                    allowed_users,
                });
            }
            ChannelMenuChoice::Signal => {
                // ── Signal ──
                prompts::section_with_width("Signal Setup — signal-cli daemon bridge", 70, |lines| {
                    lines.push("1. Run signal-cli daemon with HTTP enabled (default port 8686).".to_string());
                    lines.push("2. Ensure your Signal account is registered in signal-cli.".to_string());
                    lines.push("3. Optionally scope to DMs only or to a specific group.".to_string());
                })?;

                let http_url = prompts::input::input("signal-cli HTTP URL")
                    .placeholder("http://127.0.0.1:8686")
                    .interact()?;

                let http_url = if http_url.trim().is_empty() {
                    "http://127.0.0.1:8686".to_string()
                } else {
                    http_url
                };

                let account = prompts::input::input("Account number (E.164, e.g. +1234567890)")
                    .placeholder("+1234567890")
                    .interact()?;

                if account.trim().is_empty() {
                    prompts::log::info("Skipped — account number required")?;
                    continue;
                }

                let scope_choice: usize = prompts::select("Message scope")
                    .item(0, "All messages (DMs + groups)", "Receive everything")
                    .item(1, "DM only", "Private messages only")
                    .item(2, "Specific group ID", "Single group")
                    .interact()?;

                let group_id = match scope_choice {
                    1 => Some("dm".to_string()),
                    2 => {
                        let group_input = prompts::input::input("Group ID")
                            .placeholder("group ID")
                            .interact()?;
                        let group_input = group_input.trim().to_string();
                        if group_input.is_empty() {
                            prompts::log::info("Skipped — group ID required")?;
                            continue;
                        }
                        Some(group_input)
                    }
                    _ => None,
                };

                let allowed_from_raw = prompts::input::input(
                    "Allowed sender numbers (comma-separated +1234567890, or * for all)",
                )
                .placeholder("*")
                .interact()?;

                let allowed_from_raw = if allowed_from_raw.trim().is_empty() {
                    "*".to_string()
                } else {
                    allowed_from_raw
                };

                let allowed_from = if allowed_from_raw.trim() == "*" {
                    vec!["*".into()]
                } else {
                    allowed_from_raw
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect()
                };

                let ignore_attachments = prompts::toggle::toggle("Ignore attachment-only messages?")
                    .initial_value(false)
                    .interact()?;

                let ignore_stories = prompts::toggle::toggle("Ignore incoming stories?")
                    .initial_value(true)
                    .interact()?;

                config.signal = Some(SignalConfig {
                    http_url: http_url.trim_end_matches('/').to_string(),
                    account: account.trim().to_string(),
                    group_id,
                    allowed_from,
                    ignore_attachments,
                    ignore_stories,
                });

                prompts::log::success("Signal configured")?;
            }
            ChannelMenuChoice::WhatsApp => {
                // ── WhatsApp ──
                prompts::section_with_width("WhatsApp Setup", 70, |_lines| {})?;

                let mode_idx: usize = prompts::select("Choose WhatsApp mode")
                    .item(0, "WhatsApp Web", "QR / pair-code, no Meta Business API")
                    .item(1, "WhatsApp Business Cloud API", "Webhook-based")
                    .interact()?;

                if mode_idx == 0 {
                    // Compile-time check: warn early if the feature is not enabled.
                    #[cfg(not(feature = "whatsapp-web"))]
                    {
                        prompts::log::warning("The 'whatsapp-web' feature is not compiled in. WhatsApp Web will not work at runtime.")?;
                        prompts::log::info("Rebuild with: cargo build --features whatsapp-web")?;
                    }

                    prompts::log::info("Mode: WhatsApp Web")?;
                    prompts::log::step("1. Build with --features whatsapp-web")?;
                    prompts::log::step("2. Start channel/daemon and scan QR in WhatsApp > Linked Devices")?;
                    prompts::log::step("3. Keep session_path persistent so relogin is not required")?;

                    let session_path = prompts::input::input("Session database path")
                        .placeholder("~/.zeroclaw/state/whatsapp-web/session.db")
                        .interact()?;

                    let session_path = if session_path.trim().is_empty() {
                        "~/.zeroclaw/state/whatsapp-web/session.db".to_string()
                    } else {
                        session_path
                    };

                    let pair_phone = prompts::input::input(
                        "Pair phone (optional, digits only; leave empty to use QR flow)",
                    )
                    .placeholder("digits only")
                    .interact()?;

                    let pair_code = if pair_phone.trim().is_empty() {
                        String::new()
                    } else {
                        prompts::input::input(
                            "Custom pair code (optional, leave empty for auto-generated)",
                        )
                        .placeholder("pair code")
                        .interact()?
                    };

                    let users_str = prompts::input::input(
                        "Allowed phone numbers (comma-separated +1234567890, or * for all)",
                    )
                    .placeholder("*")
                    .interact()?;

                    let users_str = if users_str.trim().is_empty() {
                        "*".to_string()
                    } else {
                        users_str
                    };

                    let allowed_numbers = if users_str.trim() == "*" {
                        vec!["*".into()]
                    } else {
                        users_str.split(',').map(|s| s.trim().to_string()).collect()
                    };

                    config.whatsapp = Some(WhatsAppConfig {
                        access_token: None,
                        phone_number_id: None,
                        verify_token: None,
                        app_secret: None,
                        session_path: Some(session_path.trim().to_string()),
                        pair_phone: (!pair_phone.trim().is_empty())
                            .then(|| pair_phone.trim().to_string()),
                        pair_code: (!pair_code.trim().is_empty())
                            .then(|| pair_code.trim().to_string()),
                        allowed_numbers,
                    });

                    prompts::log::success("WhatsApp Web configuration saved.")?;
                    continue;
                }

                prompts::log::info("Mode: Business Cloud API")?;
                prompts::log::step("1. Go to developers.facebook.com and create a WhatsApp app")?;
                prompts::log::step("2. Add the WhatsApp product and get your phone number ID")?;
                prompts::log::step("3. Generate a temporary access token (System User)")?;
                prompts::log::step("4. Configure webhook URL to: https://your-domain/whatsapp")?;

                let access_token = prompts::input::input("Access token (from Meta Developers)")
                    .placeholder("paste access token")
                    .interact()?;

                if access_token.trim().is_empty() {
                    prompts::log::info("Skipped")?;
                    continue;
                }

                let phone_number_id = prompts::input::input("Phone number ID (from WhatsApp app settings)")
                    .placeholder("phone number ID")
                    .interact()?;

                if phone_number_id.trim().is_empty() {
                    prompts::log::info("Skipped — phone number ID required")?;
                    continue;
                }

                let verify_token = prompts::input::input("Webhook verify token (create your own)")
                    .placeholder("zeroclaw-whatsapp-verify")
                    .interact()?;

                let verify_token = if verify_token.trim().is_empty() {
                    "zeroclaw-whatsapp-verify".to_string()
                } else {
                    verify_token
                };

                // Test connection (run entirely in separate thread — Response must be used/dropped there)
                prompts::log::step("Testing connection...")?;
                let phone_number_id_clone = phone_number_id.clone();
                let access_token_clone = access_token.clone();
                let thread_result = std::thread::spawn(move || {
                    let client = reqwest::blocking::Client::new();
                    let url = format!(
                        "https://graph.facebook.com/v18.0/{}",
                        phone_number_id_clone.trim()
                    );
                    let resp = client
                        .get(&url)
                        .header(
                            "Authorization",
                            format!("Bearer {}", access_token_clone.trim()),
                        )
                        .send()?;
                    Ok::<_, reqwest::Error>(resp.status().is_success())
                })
                .join();
                match thread_result {
                    Ok(Ok(true)) => {
                        prompts::log::success("Connected to WhatsApp API")?;
                    }
                    _ => {
                        prompts::log::error("Connection failed — check access token and phone number ID")?;
                        continue;
                    }
                }

                let users_str = prompts::input::input(
                    "Allowed phone numbers (comma-separated +1234567890, or * for all)",
                )
                .placeholder("*")
                .interact()?;

                let users_str = if users_str.trim().is_empty() {
                    "*".to_string()
                } else {
                    users_str
                };

                let allowed_numbers = if users_str.trim() == "*" {
                    vec!["*".into()]
                } else {
                    users_str.split(',').map(|s| s.trim().to_string()).collect()
                };

                config.whatsapp = Some(WhatsAppConfig {
                    access_token: Some(access_token.trim().to_string()),
                    phone_number_id: Some(phone_number_id.trim().to_string()),
                    verify_token: Some(verify_token.trim().to_string()),
                    app_secret: None, // Can be set via ZEROCLAW_WHATSAPP_APP_SECRET env var
                    session_path: None,
                    pair_phone: None,
                    pair_code: None,
                    allowed_numbers,
                });
            }
            ChannelMenuChoice::Linq => {
                // ── Linq ──
                prompts::section_with_width("Linq Setup — iMessage/RCS/SMS via Linq API", 70, |lines| {
                    lines.push("1. Sign up at linqapp.com and get your Partner API token".to_string());
                    lines.push("2. Note your Linq phone number (E.164 format)".to_string());
                    lines.push("3. Configure webhook URL to: https://your-domain/linq".to_string());
                })?;

                let api_token = prompts::input::input("API token (Linq Partner API token)")
                    .placeholder("paste API token")
                    .interact()?;

                if api_token.trim().is_empty() {
                    prompts::log::info("Skipped")?;
                    continue;
                }

                let from_phone = prompts::input::input("From phone number (E.164 format, e.g. +12223334444)")
                    .placeholder("+12223334444")
                    .interact()?;

                if from_phone.trim().is_empty() {
                    prompts::log::info("Skipped — phone number required")?;
                    continue;
                }

                // Test connection
                prompts::log::step("Testing connection...")?;
                let api_token_clone = api_token.clone();
                let thread_result = std::thread::spawn(move || {
                    let client = reqwest::blocking::Client::new();
                    let url = "https://api.linqapp.com/api/partner/v3/phonenumbers";
                    let resp = client
                        .get(url)
                        .header(
                            "Authorization",
                            format!("Bearer {}", api_token_clone.trim()),
                        )
                        .send()?;
                    Ok::<_, reqwest::Error>(resp.status().is_success())
                })
                .join();
                match thread_result {
                    Ok(Ok(true)) => {
                        prompts::log::success("Connected to Linq API")?;
                    }
                    _ => {
                        prompts::log::error("Connection failed — check API token")?;
                        continue;
                    }
                }

                let users_str = prompts::input::input(
                    "Allowed sender numbers (comma-separated +1234567890, or * for all)",
                )
                .placeholder("*")
                .interact()?;

                let users_str = if users_str.trim().is_empty() {
                    "*".to_string()
                } else {
                    users_str
                };

                let allowed_senders = if users_str.trim() == "*" {
                    vec!["*".into()]
                } else {
                    users_str.split(',').map(|s| s.trim().to_string()).collect()
                };

                let signing_secret = prompts::input::input("Webhook signing secret (optional, press Enter to skip)")
                    .placeholder("signing secret")
                    .interact()?;

                config.linq = Some(LinqConfig {
                    api_token: api_token.trim().to_string(),
                    from_phone: from_phone.trim().to_string(),
                    signing_secret: if signing_secret.trim().is_empty() {
                        None
                    } else {
                        Some(signing_secret.trim().to_string())
                    },
                    allowed_senders,
                });
            }
            ChannelMenuChoice::Irc => {
                // ── IRC ──
                prompts::section_with_width("IRC Setup — IRC over TLS", 70, |lines| {
                    lines.push("IRC connects over TLS to any IRC server".to_string());
                    lines.push("Supports SASL PLAIN and NickServ authentication".to_string());
                })?;

                let server = prompts::input::input("IRC server (hostname)")
                    .placeholder("irc.libera.chat")
                    .interact()?;

                if server.trim().is_empty() {
                    prompts::log::info("Skipped")?;
                    continue;
                }

                let port_str = prompts::input::input("Port")
                    .placeholder("6697")
                    .interact()?;

                let port_str = if port_str.trim().is_empty() {
                    "6697".to_string()
                } else {
                    port_str
                };

                let port: u16 = match port_str.trim().parse() {
                    Ok(p) => p,
                    Err(_) => {
                        prompts::log::info("Invalid port, using 6697")?;
                        6697
                    }
                };

                let nickname = prompts::input::input("Bot nickname")
                    .placeholder("zeroclaw")
                    .interact()?;

                if nickname.trim().is_empty() {
                    prompts::log::info("Skipped — nickname required")?;
                    continue;
                }

                let channels_str = prompts::input::input("Channels to join (comma-separated: #channel1,#channel2)")
                    .placeholder("#channel1,#channel2")
                    .interact()?;

                let channels = if channels_str.trim().is_empty() {
                    vec![]
                } else {
                    channels_str
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect()
                };

                prompts::log::info("Allowlist nicknames that can interact with the bot (case-insensitive).")?;
                prompts::log::info("Use '*' to allow anyone (not recommended for production).")?;

                let users_str = prompts::input::input("Allowed nicknames (comma-separated, or * for all)")
                    .placeholder("*")
                    .interact()?;

                let allowed_users = if users_str.trim() == "*" {
                    vec!["*".into()]
                } else {
                    users_str
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect()
                };

                if allowed_users.is_empty() {
                    prompts::log::warning("Empty allowlist — only you can interact. Add nicknames above.")?;
                }

                prompts::log::info("Optional authentication (leave empty to skip each):")?;

                let server_password = prompts::input::input("Server password (for bouncers like ZNC, leave empty if none)")
                    .placeholder("optional")
                    .interact()?;

                let nickserv_password = prompts::input::input("NickServ password (leave empty if none)")
                    .placeholder("optional")
                    .interact()?;

                let sasl_password = prompts::input::input("SASL PLAIN password (leave empty if none)")
                    .placeholder("optional")
                    .interact()?;

                let verify_tls = prompts::toggle::toggle("Verify TLS certificate?")
                    .initial_value(true)
                    .interact()?;

                prompts::log::success(format!(
                    "IRC configured as {}@{}:{}",
                    nickname.trim(),
                    server.trim(),
                    port
                ))?;

                config.irc = Some(IrcConfig {
                    server: server.trim().to_string(),
                    port,
                    nickname: nickname.trim().to_string(),
                    username: None,
                    channels,
                    allowed_users,
                    server_password: if server_password.trim().is_empty() {
                        None
                    } else {
                        Some(server_password.trim().to_string())
                    },
                    nickserv_password: if nickserv_password.trim().is_empty() {
                        None
                    } else {
                        Some(nickserv_password.trim().to_string())
                    },
                    sasl_password: if sasl_password.trim().is_empty() {
                        None
                    } else {
                        Some(sasl_password.trim().to_string())
                    },
                    verify_tls: Some(verify_tls),
                });
            }
            ChannelMenuChoice::Webhook => {
                // ── Webhook ──
                prompts::section_with_width("Webhook Setup — HTTP endpoint for custom integrations", 70, |_lines| {})?;

                let port = prompts::input::input("Port")
                    .placeholder("8080")
                    .interact()?;

                let port = if port.trim().is_empty() {
                    "8080".to_string()
                } else {
                    port
                };

                let secret = prompts::input::input("Secret (optional, Enter to skip)")
                    .placeholder("optional")
                    .interact()?;

                config.webhook = Some(WebhookConfig {
                    port: port.parse().unwrap_or(8080),
                    listen_path: None,
                    send_url: None,
                    send_method: None,
                    auth_header: None,
                    secret: if secret.is_empty() {
                        None
                    } else {
                        Some(secret)
                    },
                });
                prompts::log::success(format!("Webhook on port {port}"))?;
            }
            ChannelMenuChoice::NextcloudTalk => {
                // ── Nextcloud Talk ──
                prompts::section_with_width("Nextcloud Talk Setup — Talk webhook receive + OCS API send", 70, |lines| {
                    lines.push("1. Configure your Nextcloud Talk bot app and app token.".to_string());
                    lines.push("2. Set webhook URL to: https://<your-public-url>/nextcloud-talk".to_string());
                    lines.push("3. Keep webhook_secret aligned with Nextcloud signature headers if enabled.".to_string());
                })?;

                let base_url = prompts::input::input("Nextcloud base URL (e.g. https://cloud.example.com)")
                    .placeholder("https://cloud.example.com")
                    .interact()?;

                let base_url = base_url.trim().trim_end_matches('/').to_string();
                if base_url.is_empty() {
                    prompts::log::info("Skipped — base URL required")?;
                    continue;
                }

                let app_token = prompts::input::input("App token (Talk bot token)")
                    .placeholder("paste app token")
                    .interact()?;

                if app_token.trim().is_empty() {
                    prompts::log::info("Skipped — app token required")?;
                    continue;
                }

                let webhook_secret = prompts::input::input("Webhook secret (optional, Enter to skip)")
                    .placeholder("optional")
                    .interact()?;

                let allowed_users_raw = prompts::input::input("Allowed Nextcloud actor IDs (comma-separated, or * for all)")
                    .placeholder("*")
                    .interact()?;

                let allowed_users_raw = if allowed_users_raw.trim().is_empty() {
                    "*".to_string()
                } else {
                    allowed_users_raw
                };

                let allowed_users = if allowed_users_raw.trim() == "*" {
                    vec!["*".into()]
                } else {
                    allowed_users_raw
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect()
                };

                config.nextcloud_talk = Some(NextcloudTalkConfig {
                    base_url,
                    app_token: app_token.trim().to_string(),
                    webhook_secret: if webhook_secret.trim().is_empty() {
                        None
                    } else {
                        Some(webhook_secret.trim().to_string())
                    },
                    allowed_users,
                });

                prompts::log::success("Nextcloud Talk configured")?;
            }
            ChannelMenuChoice::DingTalk => {
                // ── DingTalk ──
                prompts::section_with_width("DingTalk Setup — DingTalk Stream Mode", 70, |lines| {
                    lines.push("1. Go to DingTalk developer console (open.dingtalk.com)".to_string());
                    lines.push("2. Create an app and enable the Stream Mode bot".to_string());
                    lines.push("3. Copy the Client ID (AppKey) and Client Secret (AppSecret)".to_string());
                })?;

                let client_id = prompts::input::input("Client ID (AppKey)")
                    .placeholder("AppKey")
                    .interact()?;

                if client_id.trim().is_empty() {
                    prompts::log::info("Skipped")?;
                    continue;
                }

                let client_secret = prompts::input::input("Client Secret (AppSecret)")
                    .placeholder("AppSecret")
                    .interact()?;

                // Test connection
                prompts::log::step("Testing connection...")?;
                let client = reqwest::blocking::Client::new();
                let body = serde_json::json!({
                    "clientId": client_id,
                    "clientSecret": client_secret,
                });
                match client
                    .post("https://api.dingtalk.com/v1.0/gateway/connections/open")
                    .json(&body)
                    .send()
                {
                    Ok(resp) if resp.status().is_success() => {
                        prompts::log::success("DingTalk credentials verified")?;
                    }
                    _ => {
                        prompts::log::error("Connection failed — check your credentials")?;
                        continue;
                    }
                }

                let users_str = prompts::input::input("Allowed staff IDs (comma-separated, '*' for all)")
                    .placeholder("*")
                    .interact()?;

                let allowed_users: Vec<String> = users_str
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();

                config.dingtalk = Some(DingTalkConfig {
                    client_id,
                    client_secret,
                    allowed_users,
                });
            }
            ChannelMenuChoice::QqOfficial => {
                // ── QQ Official ──
                prompts::section_with_width("QQ Official Setup — Tencent QQ Bot SDK", 70, |lines| {
                    lines.push("1. Go to QQ Bot developer console (q.qq.com)".to_string());
                    lines.push("2. Create a bot application".to_string());
                    lines.push("3. Copy the App ID and App Secret".to_string());
                })?;

                let app_id = prompts::input::input("App ID")
                    .placeholder("App ID")
                    .interact()?;

                if app_id.trim().is_empty() {
                    prompts::log::info("Skipped")?;
                    continue;
                }

                let app_secret = prompts::input::input("App Secret")
                    .placeholder("App Secret")
                    .interact()?;

                // Test connection
                prompts::log::step("Testing connection...")?;
                let client = reqwest::blocking::Client::new();
                let body = serde_json::json!({
                    "appId": app_id,
                    "clientSecret": app_secret,
                });
                match client
                    .post("https://bots.qq.com/app/getAppAccessToken")
                    .json(&body)
                    .send()
                {
                    Ok(resp) if resp.status().is_success() => {
                        let data: serde_json::Value = resp.json().unwrap_or_default();
                        if data.get("access_token").is_some() {
                            prompts::log::success("QQ Bot credentials verified")?;
                        } else {
                            prompts::log::error("Auth error — check your credentials")?;
                            continue;
                        }
                    }
                    _ => {
                        prompts::log::error("Connection failed — check your credentials")?;
                        continue;
                    }
                }

                let users_str = prompts::input::input("Allowed user IDs (comma-separated, '*' for all)")
                    .placeholder("*")
                    .interact()?;

                let allowed_users: Vec<String> = users_str
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();

                config.qq = Some(QQConfig {
                    app_id,
                    app_secret,
                    allowed_users,
                });
            }
            ChannelMenuChoice::Lark | ChannelMenuChoice::Feishu => {
                let is_feishu = matches!(choice, ChannelMenuChoice::Feishu);
                let provider_label = if is_feishu { "Feishu" } else { "Lark" };
                let provider_host = if is_feishu {
                    "open.feishu.cn"
                } else {
                    "open.larksuite.com"
                };
                let base_url = if is_feishu {
                    "https://open.feishu.cn/open-apis"
                } else {
                    "https://open.larksuite.com/open-apis"
                };

                // ── Lark / Feishu ──
                prompts::section_with_width(
                    &format!("{provider_label} Setup — talk to ZeroClaw from {provider_label}"),
                    70,
                    |lines| {
                        lines.push(format!(
                            "1. Go to {provider_label} Open Platform ({provider_host})"
                        ));
                        lines.push("2. Create an app and enable 'Bot' capability".to_string());
                        lines.push("3. Copy the App ID and App Secret".to_string());
                    },
                )?;

                let app_id = prompts::input::input("App ID")
                    .placeholder("App ID")
                    .interact()?;
                let app_id = app_id.trim().to_string();

                if app_id.trim().is_empty() {
                    prompts::log::info("Skipped")?;
                    continue;
                }

                let app_secret = prompts::input::input("App Secret")
                    .placeholder("App Secret")
                    .interact()?;
                let app_secret = app_secret.trim().to_string();

                if app_secret.is_empty() {
                    prompts::log::error("App Secret is required")?;
                    continue;
                }

                // Test connection (run entirely in separate thread — Response must be used/dropped there)
                prompts::log::step("Testing connection...")?;
                let app_id_clone = app_id.clone();
                let app_secret_clone = app_secret.clone();
                let endpoint = format!("{base_url}/auth/v3/tenant_access_token/internal");

                let thread_result = std::thread::spawn(move || {
                    let client = reqwest::blocking::Client::builder()
                        .timeout(Duration::from_secs(8))
                        .connect_timeout(Duration::from_secs(4))
                        .build()
                        .map_err(|err| format!("failed to build HTTP client: {err}"))?;
                    let body = serde_json::json!({
                        "app_id": app_id_clone,
                        "app_secret": app_secret_clone,
                    });

                    let response = client
                        .post(endpoint)
                        .json(&body)
                        .send()
                        .map_err(|err| format!("request error: {err}"))?;

                    let status = response.status();
                    let payload: Value = response.json().unwrap_or_default();
                    let has_token = payload
                        .get("tenant_access_token")
                        .and_then(Value::as_str)
                        .is_some_and(|token| !token.trim().is_empty());

                    if status.is_success() && has_token {
                        return Ok::<(), String>(());
                    }

                    let detail = payload
                        .get("msg")
                        .or_else(|| payload.get("message"))
                        .and_then(Value::as_str)
                        .unwrap_or("unknown error");

                    Err(format!("auth rejected ({status}): {detail}"))
                })
                .join();

                match thread_result {
                    Ok(Ok(())) => {
                        prompts::log::success(format!("{provider_label} credentials verified"))?;
                    }
                    Ok(Err(reason)) => {
                        prompts::log::error("Connection failed — check your credentials")?;
                        prompts::log::info(reason)?;
                        continue;
                    }
                    Err(_) => {
                        prompts::log::error("Connection failed — check your credentials")?;
                        continue;
                    }
                }

                let receive_mode_choice: usize = prompts::select("Receive Mode")
                    .item(0, "WebSocket", "Recommended, no public IP needed")
                    .item(1, "Webhook", "Requires public HTTPS endpoint")
                    .interact()?;

                let receive_mode = if receive_mode_choice == 0 {
                    LarkReceiveMode::Websocket
                } else {
                    LarkReceiveMode::Webhook
                };

                let verification_token = if receive_mode == LarkReceiveMode::Webhook {
                    let token = prompts::input::input("Verification Token (optional, for Webhook mode)")
                        .placeholder("optional")
                        .interact()?;
                    if token.is_empty() {
                        None
                    } else {
                        Some(token)
                    }
                } else {
                    None
                };

                if receive_mode == LarkReceiveMode::Webhook && verification_token.is_none() {
                    prompts::log::warning("Verification Token is empty — webhook authenticity checks are reduced.")?;
                }

                let port = if receive_mode == LarkReceiveMode::Webhook {
                    let p = prompts::input::input("Webhook Port")
                        .placeholder("8080")
                        .interact()?;
                    let p = if p.trim().is_empty() {
                        "8080".to_string()
                    } else {
                        p
                    };
                    Some(p.parse().unwrap_or(8080))
                } else {
                    None
                };

                let users_str = prompts::input::input("Allowed user Open IDs (comma-separated, '*' for all)")
                    .placeholder("*")
                    .interact()?;

                let allowed_users: Vec<String> = users_str
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();

                if allowed_users.is_empty() {
                    prompts::log::warning(format!(
                        "No users allowlisted — {provider_label} inbound messages will be denied until you add Open IDs or '*'."
                    ))?;
                }

                config.lark = Some(LarkConfig {
                    app_id,
                    app_secret,
                    verification_token,
                    encrypt_key: None,
                    allowed_users,
                    mention_only: false,
                    use_feishu: is_feishu,
                    receive_mode,
                    port,
                });
            }
            #[cfg(feature = "channel-nostr")]
            ChannelMenuChoice::Nostr => {
                // ── Nostr ──
                prompts::section_with_width("Nostr Setup — private messages via NIP-04 & NIP-17", 70, |lines| {
                    lines.push("ZeroClaw will listen for encrypted DMs on Nostr relays.".to_string());
                    lines.push("You need a Nostr private key (hex or nsec) and at least one relay.".to_string());
                })?;

                let private_key = prompts::input::input("Private key (hex or nsec1...)")
                    .placeholder("hex or nsec1...")
                    .interact()?;

                if private_key.trim().is_empty() {
                    prompts::log::info("Skipped")?;
                    continue;
                }

                // Validate the key immediately
                match nostr_sdk::Keys::parse(private_key.trim()) {
                    Ok(keys) => {
                        prompts::log::success(format!(
                            "Key valid — public key: {}",
                            keys.public_key().to_hex()
                        ))?;
                    }
                    Err(_) => {
                        prompts::log::error("Invalid private key — check format and try again")?;
                        continue;
                    }
                }

                let default_relays = default_nostr_relays().join(",");
                let relays_str = prompts::input::input("Relay URLs (comma-separated, Enter for defaults)")
                    .placeholder(&default_relays)
                    .interact()?;

                let relays_str = if relays_str.trim().is_empty() {
                    default_relays
                } else {
                    relays_str
                };

                let relays: Vec<String> = relays_str
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();

                prompts::log::info("Allowlist pubkeys that can message the bot (hex or npub).")?;
                prompts::log::info("Use '*' to allow anyone (not recommended for production).")?;

                let pubkeys_str = prompts::input::input("Allowed pubkeys (comma-separated, or * for all)")
                    .placeholder("*")
                    .interact()?;

                let allowed_pubkeys: Vec<String> = if pubkeys_str.trim() == "*" {
                    vec!["*".into()]
                } else {
                    pubkeys_str
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect()
                };

                if allowed_pubkeys.is_empty() {
                    prompts::log::warning("No pubkeys allowlisted — inbound messages will be denied until you add pubkeys or '*'.")?;
                }

                config.nostr = Some(NostrConfig {
                    private_key: private_key.trim().to_string(),
                    relays: relays.clone(),
                    allowed_pubkeys,
                });

                prompts::log::success(format!(
                    "Nostr configured with {} relay(s)",
                    relays.len()
                ))?;
            }
            ChannelMenuChoice::Done => break,
        }
    }

    // Summary line
    let channels = config.channels();
    let channels = channels
        .iter()
        .filter_map(|(channel, ok)| ok.then_some(channel.name()));
    let channels: Vec<_> = std::iter::once("Cli").chain(channels).collect();
    let active = channels.join(", ");

    prompts::log::success(format!("Channels: {active}"))?;

    Ok(config)
}