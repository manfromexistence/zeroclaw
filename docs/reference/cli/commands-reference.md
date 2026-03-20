# Agent Commands Reference

This reference is derived from the current CLI surface (`agent --help`).

Last verified: **February 21, 2026**.

## Top-Level Commands

| Command | Purpose |
|---|---|
| `onboard` | Initialize workspace/config quickly or interactively |
| `agent` | Run interactive chat or single-message mode |
| `gateway` | Start webhook and WhatsApp HTTP gateway |
| `daemon` | Start supervised runtime (gateway + channels + optional heartbeat/scheduler) |
| `service` | Manage user-level OS service lifecycle |
| `doctor` | Run diagnostics and freshness checks |
| `status` | Print current configuration and system summary |
| `estop` | Engage/resume emergency stop levels and inspect estop state |
| `cron` | Manage scheduled tasks |
| `models` | Refresh provider model catalogs |
| `providers` | List provider IDs, aliases, and active provider |
| `channel` | Manage channels and channel health checks |
| `integrations` | Inspect integration details |
| `skills` | List/install/remove skills |
| `migrate` | Import from external runtimes (currently OpenClaw) |
| `config` | Export machine-readable config schema |
| `completions` | Generate shell completion scripts to stdout |
| `hardware` | Discover and introspect USB hardware |
| `peripheral` | Configure and flash peripherals |

## Command Groups

### `onboard`

- `agent onboard`
- `agent onboard --channels-only`
- `agent onboard --force`
- `agent onboard --reinit`
- `agent onboard --api-key <KEY> --provider <ID> --memory <sqlite|lucid|markdown|none>`
- `agent onboard --api-key <KEY> --provider <ID> --model <MODEL_ID> --memory <sqlite|lucid|markdown|none>`
- `agent onboard --api-key <KEY> --provider <ID> --model <MODEL_ID> --memory <sqlite|lucid|markdown|none> --force`

`onboard` safety behavior:

- If `config.toml` already exists, onboarding offers two modes:
  - Full onboarding (overwrite `config.toml`)
  - Provider-only update (update provider/model/API key while preserving existing channels, tunnel, memory, hooks, and other settings)
- In non-interactive environments, existing `config.toml` causes a safe refusal unless `--force` is passed.
- Use `agent onboard --channels-only` when you only need to rotate channel tokens/allowlists.
- Use `agent onboard --reinit` to start fresh. This backs up your existing config directory with a timestamp suffix and creates a new configuration from scratch.

### `agent`

- `agent agent`
- `agent agent -m "Hello"`
- `agent agent --provider <ID> --model <MODEL> --temperature <0.0-2.0>`
- `agent agent --peripheral <board:path>`

Tip:

- In interactive chat, you can ask for route changes in natural language (for example “conversation uses kimi, coding uses gpt-5.3-codex”); the assistant can persist this via tool `model_routing_config`.

### `gateway` / `daemon`

- `agent gateway [--host <HOST>] [--port <PORT>]`
- `agent daemon [--host <HOST>] [--port <PORT>]`

### `estop`

- `agent estop` (engage `kill-all`)
- `agent estop --level network-kill`
- `agent estop --level domain-block --domain "*.chase.com" [--domain "*.paypal.com"]`
- `agent estop --level tool-freeze --tool shell [--tool browser]`
- `agent estop status`
- `agent estop resume`
- `agent estop resume --network`
- `agent estop resume --domain "*.chase.com"`
- `agent estop resume --tool shell`
- `agent estop resume --otp <123456>`

Notes:

- `estop` commands require `[security.estop].enabled = true`.
- When `[security.estop].require_otp_to_resume = true`, `resume` requires OTP validation.
- OTP prompt appears automatically if `--otp` is omitted.

### `service`

- `agent service install`
- `agent service start`
- `agent service stop`
- `agent service restart`
- `agent service status`
- `agent service uninstall`

### `cron`

- `agent cron list`
- `agent cron add <expr> [--tz <IANA_TZ>] <command>`
- `agent cron add-at <rfc3339_timestamp> <command>`
- `agent cron add-every <every_ms> <command>`
- `agent cron once <delay> <command>`
- `agent cron remove <id>`
- `agent cron pause <id>`
- `agent cron resume <id>`

Notes:

- Mutating schedule/cron actions require `cron.enabled = true`.
- Shell command payloads for schedule creation (`create` / `add` / `once`) are validated by security command policy before job persistence.

### `models`

- `agent models refresh`
- `agent models refresh --provider <ID>`
- `agent models refresh --force`

`models refresh` currently supports live catalog refresh for provider IDs: `openrouter`, `openai`, `anthropic`, `groq`, `mistral`, `deepseek`, `xai`, `together-ai`, `gemini`, `ollama`, `llamacpp`, `sglang`, `vllm`, `astrai`, `venice`, `fireworks`, `cohere`, `moonshot`, `glm`, `zai`, `qwen`, and `nvidia`.

### `doctor`

- `agent doctor`
- `agent doctor models [--provider <ID>] [--use-cache]`
- `agent doctor traces [--limit <N>] [--event <TYPE>] [--contains <TEXT>]`
- `agent doctor traces --id <TRACE_ID>`

`doctor traces` reads runtime tool/model diagnostics from `observability.runtime_trace_path`.

### `channel`

- `agent channel list`
- `agent channel start`
- `agent channel doctor`
- `agent channel bind-telegram <IDENTITY>`
- `agent channel add <type> <json>`
- `agent channel remove <name>`

Runtime in-chat commands (Telegram/Discord while channel server is running):

- `/models`
- `/models <provider>`
- `/model`
- `/model <model-id>`
- `/new`

Channel runtime also watches `config.toml` and hot-applies updates to:
- `default_provider`
- `default_model`
- `default_temperature`
- `api_key` / `api_url` (for the default provider)
- `reliability.*` provider retry settings

`add/remove` currently route you back to managed setup/manual config paths (not full declarative mutators yet).

### `integrations`

- `agent integrations info <name>`

### `skills`

- `agent skills list`
- `agent skills audit <source_or_name>`
- `agent skills install <source>`
- `agent skills remove <name>`

`<source>` accepts git remotes (`https://...`, `http://...`, `ssh://...`, and `git@host:owner/repo.git`) or a local filesystem path.

`skills install` always runs a built-in static security audit before the skill is accepted. The audit blocks:
- symlinks inside the skill package
- script-like files (`.sh`, `.bash`, `.zsh`, `.ps1`, `.bat`, `.cmd`)
- high-risk command snippets (for example pipe-to-shell payloads)
- markdown links that escape the skill root, point to remote markdown, or target script files

Use `skills audit` to manually validate a candidate skill directory (or an installed skill by name) before sharing it.

Skill manifests (`SKILL.toml`) support `prompts` and `[[tools]]`; both are injected into the agent system prompt at runtime, so the model can follow skill instructions without manually reading skill files.

### `migrate`

- `agent migrate openclaw [--source <path>] [--dry-run]`

### `config`

- `agent config schema`

`config schema` prints a JSON Schema (draft 2020-12) for the full `config.toml` contract to stdout.

### `completions`

- `agent completions bash`
- `agent completions fish`
- `agent completions zsh`
- `agent completions powershell`
- `agent completions elvish`

`completions` is stdout-only by design so scripts can be sourced directly without log/warning contamination.

### `hardware`

- `agent hardware discover`
- `agent hardware introspect <path>`
- `agent hardware info [--chip <chip_name>]`

### `peripheral`

- `agent peripheral list`
- `agent peripheral add <board> <path>`
- `agent peripheral flash [--port <serial_port>]`
- `agent peripheral setup-uno-q [--host <ip_or_host>]`
- `agent peripheral flash-nucleo`

## Validation Tip

To verify docs against your current binary quickly:

```bash
agent --help
agent <command> --help
```
