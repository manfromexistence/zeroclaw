# ZeroClaw Complete Details & Capabilities

## Overview
**ZeroClaw** is a Rust-based autonomous AI agent runtime - a lightweight, secure, and highly efficient alternative to OpenClaw. It's designed to run on minimal hardware (even $10 boards) while providing enterprise-grade features.

## 🎯 Key Statistics

### Performance
- **Binary Size:** ~8.8 MB (99% smaller than OpenClaw's 1GB+)
- **RAM Usage:** < 5 MB (vs OpenClaw's 1GB+)
- **Startup Time:** < 10ms (vs OpenClaw's 500s+ on low-end hardware)
- **Language:** 100% Rust (memory-safe, no runtime overhead)
- **Cost:** Runs on $10 hardware vs $599 Mac Mini for OpenClaw

### Provider Support
- **Total Providers:** 60 providers
- **Comparison:** OpenCode has 75+ providers (ZeroClaw has 15 fewer)
- **All Major Providers Covered:** OpenAI, Anthropic, Google, OpenRouter, Groq, Mistral, DeepSeek, Cerebras, and 52 more

## 💰 Cost Structure

### ZeroClaw is 100% FREE and Open Source
- **License:** Dual MIT/Apache 2.0
- **No Subscription Fees:** Zero cost to use
- **No API Markup:** Direct provider pricing
- **No Hidden Costs:** Completely transparent

### What You Pay For
You only pay for:
1. **LLM Provider API costs** (your choice of provider)
2. **Optional hosting** (if you deploy to cloud)
3. **Optional hardware** (if running on dedicated device)

### Provider Costs (Examples from your .env)
- **Mistral:** 1 billion tokens/month FREE ✓ (currently working)
- **Groq:** 14,400 requests/day FREE
- **Cerebras:** 1M tokens/day FREE
- **DeepSeek:** Free credits on signup
- **OpenRouter:** 200 requests/day FREE (but your key is out of credits)
- **Gemini:** 1,500 requests/day FREE (but your key failed)

## 🛠️ Supported Tools (50+ Built-in)

### File Operations
- `file_read` - Read files with sandboxing
- `file_write` - Write files with security checks
- `file_edit` - Edit files with pattern matching
- `glob_search` - Search files by pattern
- `content_search` - Search file contents (ripgrep)
- `pdf_read` - Extract text from PDFs

### Shell & System
- `shell` - Execute shell commands (sandboxed)
- `cli_discovery` - Discover installed CLI tools
- `screenshot` - Capture screen images

### Memory & Data
- `memory_store` - Store persistent memories
- `memory_recall` - Search and retrieve memories
- `memory_forget` - Delete memory entries
- `backup_tool` - Create/restore workspace backups
- `data_management` - Lifecycle and retention management

### Scheduling & Automation
- `cron_add` - Schedule recurring tasks
- `cron_list` - List scheduled jobs
- `cron_remove` - Remove scheduled jobs
- `cron_update` - Update job configuration
- `cron_run` - Execute job immediately
- `cron_runs` - View execution history
- `schedule` - One-time scheduled tasks

### Web & Network
- `web_search_tool` - Search the web (DuckDuckGo/Brave)
- `web_fetch` - Fetch web page content
- `http_request` - Make HTTP API calls
- `browser` - Browser automation (Fantoccini)
- `browser_open` - Open URLs in system browser

### Git Operations
- `git_operations` - Structured git commands (status, diff, commit, push, etc.)

### Cloud & Infrastructure
- `cloud_ops` - Cloud operations (IaC review, cost analysis)
- `cloud_patterns` - Architecture pattern suggestions
- `proxy_config` - Proxy configuration management

### Hardware Integration
- `hardware_board_info` - Get board information
- `hardware_memory_map` - View memory layout
- `hardware_memory_read` - Read hardware memory (STM32, RPi)

### Agent Collaboration
- `delegate` - Delegate tasks to sub-agents
- `swarm` - Multi-agent coordination
- `node_tool` - Distributed node operations
- `workspace_tool` - Multi-workspace management

### Integrations
- `composio` - Composio platform integration (100+ SaaS tools)
- `microsoft365` - Microsoft 365 integration
- `notion_tool` - Notion workspace integration
- `pushover` - Push notifications
- `mcp_tool` - Model Context Protocol tools

### Security & Operations
- `security_ops` - Security playbooks and triage
- `project_intel` - Project analysis and risk assessment
- `sop_*` - Standard Operating Procedures (list, execute, approve, status)
- `tool_search` - Search available tools
- `image_info` - Image metadata extraction
- `model_routing_config` - Dynamic model routing

## 🔌 Supported Providers (60 Total)

### Major Providers
- **OpenRouter** - 75+ models aggregator
- **Anthropic** - Claude models (Sonnet, Opus, Haiku)
- **OpenAI** - GPT-4, GPT-4o, o1, etc.
- **Google Gemini** - Gemini Pro, Flash, etc.
- **Groq** - Ultra-fast LPU inference
- **Mistral** - Mistral Large, Small, etc.
- **DeepSeek** - DeepSeek Reasoner, Coder
- **Cerebras** - Fastest inference (1M tokens/day free)
- **xAI** - Grok models
- **Cohere** - Command models

### Cloud Providers
- **Azure OpenAI** - Enterprise OpenAI
- **Amazon Bedrock** - AWS managed AI
- **Google Vertex AI** - GCP AI platform
- **Cloudflare AI** - Edge AI
- **Vercel AI Gateway** - Unified gateway

### Specialized Providers
- **GitHub Copilot** - Use your Copilot subscription
- **Together AI** - Open model hosting
- **Fireworks AI** - Fast inference
- **Replicate** - Model marketplace
- **Hugging Face** - Open models
- **SambaNova** - RDU-powered inference
- **Hyperbolic** - High-performance inference
- **DeepInfra** - Scalable inference
- **Baseten** - Model deployment
- **Anyscale** - Ray-based serving

### Regional/Specialized
- **Moonshot (Kimi)** - Chinese market
- **GLM (Zhipu)** - Chinese models
- **Qwen (DashScope)** - Alibaba models
- **Doubao (Volcengine)** - ByteDance models
- **Baichuan** - Chinese LLMs
- **01.AI (Yi)** - Chinese models
- **Tencent Hunyuan** - Tencent models
- **Z.AI** - GLM coding plan
- **MiniMax** - Chinese multimodal

### Local/Self-Hosted
- **Ollama** - Local model runner
- **LM Studio** - Local GUI runner
- **llama.cpp** - C++ inference
- **vLLM** - Fast local serving
- **SGLang** - Structured generation
- **LiteLLM** - Proxy layer

### Enterprise/Platform
- **SAP AI Core** - 40+ models
- **STACKIT** - European sovereign AI
- **OVHcloud** - European cloud AI
- **Scaleway** - French cloud AI
- **Nebius** - AI studio
- **Helicone** - Observability gateway
- **OpenCode Zen** - Tested models
- **OpenCode Go** - Low-cost subscription

### Other Providers
- Venice, Telnyx, Perplexity, AI21 Labs, Reka, Friendli, Lepton, Stepfun, Nscale, NVIDIA NIM, SiliconFlow, AiHubMix, Novita, IO.NET, Firmware, Cortecs, 302.AI, and more

## 📡 Supported Channels (21 Messaging Platforms)

- **CLI** - Command-line interface (always available)
- **Telegram** - Bot API integration
- **Discord** - Bot integration
- **Slack** - Workspace integration
- **WhatsApp** - WhatsApp Web protocol
- **Signal** - Private messaging
- **Matrix** - Decentralized chat (E2EE support)
- **iMessage** - Apple Messages
- **Mattermost** - Self-hosted Slack alternative
- **Microsoft Teams** - Enterprise chat
- **IRC** - Classic chat protocol
- **Email** - SMTP/IMAP integration
- **Lark/Feishu** - ByteDance workspace
- **DingTalk** - Alibaba workspace
- **WeCom** - Tencent enterprise
- **QQ Official** - Tencent messaging
- **Nostr** - Decentralized protocol
- **NextCloud Talk** - Self-hosted chat
- **Linq** - Messaging platform
- **WATI** - WhatsApp Business API
- **ClawdTalk** - Custom protocol
- **Webhook** - HTTP webhook receiver

## 🔒 Security Features

### Built-in Security
- **Workspace Sandboxing** - All operations confined to workspace
- **Path Validation** - Prevents directory traversal
- **Command Allowlisting** - Only approved commands
- **Domain Allowlisting** - Restricted web access
- **Encrypted Secrets** - ChaCha20-Poly1305 encryption
- **Pairing Authentication** - Gateway access control
- **Rate Limiting** - Prevent abuse
- **Audit Logging** - Track all operations
- **OTP Support** - Two-factor authentication
- **E-Stop** - Emergency shutdown system

### Sandboxing Options
- **Native** - Built-in Rust sandboxing
- **Docker** - Container isolation
- **Landlock** - Linux kernel LSM
- **Bubblewrap** - User namespace sandboxing
- **Firejail** - Namespace/seccomp sandbox

### Security Policies
- Max actions per hour (default: 20)
- Max cost per day (default: $5)
- Forbidden paths (system directories)
- Allowed commands (git, npm, cargo, etc.)
- Tool approval requirements
- High-risk command blocking

## 🧠 Memory Backends

- **SQLite** - Local database (default, encrypted)
- **Markdown** - Human-readable files
- **PostgreSQL** - Enterprise database
- **Qdrant** - Vector database (embeddings)
- **None** - Stateless mode

### Memory Features
- Auto-save
- Hygiene (archival/purge)
- Embedding support
- Vector + keyword hybrid search
- Response caching
- Snapshot support
- Auto-hydration

## 🌐 Gateway & API

### HTTP Gateway
- **Default Port:** 42617
- **Host:** 127.0.0.1 (localhost)
- **Authentication:** Pairing code system
- **Rate Limits:** 60 webhook requests/min
- **Body Limit:** 64KB (1MB for config)
- **Timeout:** 30 seconds
- **Idempotency:** Optional header support

### API Endpoints
- `POST /webhook` - Send messages
- `GET /api/status` - System status
- `GET /api/config` - Get configuration
- `PUT /api/config` - Update configuration
- `GET /api/memory` - List memories
- `POST /api/memory` - Store memory
- `DELETE /api/memory/{key}` - Delete memory
- `GET /api/cron` - List cron jobs
- `POST /api/cron` - Add cron job
- `DELETE /api/cron/{id}` - Remove job
- `GET /api/tools` - List available tools
- `GET /api/cost` - Cost tracking
- `GET /api/events` - SSE event stream
- `POST /pair` - Pairing authentication
- `GET /health` - Health check
- `WS /ws/chat` - WebSocket streaming

## 🚀 Deployment Options

### Local Development
```bash
cargo build --release
zeroclaw onboard
zeroclaw agent -m "Hello!"
```

### System Service
```bash
zeroclaw service install
zeroclaw service start
zeroclaw service status
```

### Docker
```bash
docker run -v ~/.zeroclaw:/root/.zeroclaw zeroclaw/zeroclaw
```

### Edge Devices
- Raspberry Pi (all models)
- Orange Pi
- Rock Pi
- Any ARM/x86 board with 5MB+ RAM

### Cloud Platforms
- AWS (EC2, Lambda, ECS)
- Google Cloud (Compute, Cloud Run)
- Azure (VMs, Container Instances)
- DigitalOcean, Linode, Vultr
- Fly.io, Railway, Render

## 📊 Observability

### Metrics Backends
- **Prometheus** - Metrics scraping
- **OpenTelemetry** - OTLP traces + metrics
- **None** - Disabled (default)

### Monitoring
- LLM request tracking
- Tool call monitoring
- Cost tracking (per-model breakdown)
- Error rates
- Latency metrics
- Token usage

## 🔧 Configuration

### Config File Location
- **User:** `~/.zeroclaw/config.toml`
- **Workspace:** `.zeroclaw/config.toml`

### Key Configuration Sections
- `[provider]` - LLM provider settings
- `[autonomy]` - Security and permissions
- `[memory]` - Persistence backend
- `[gateway]` - HTTP server settings
- `[channels]` - Messaging platforms
- `[security]` - Sandbox and policies
- `[scheduler]` - Cron configuration
- `[observability]` - Metrics/tracing
- `[runtime]` - Execution environment
- `[tools]` - Tool configuration
- `[skills]` - Custom skills
- `[hooks]` - Event hooks
- `[mcp]` - Model Context Protocol
- `[cost]` - Budget limits

## 🎓 Use Cases

### Personal Assistant
- Chat via Telegram/Discord/Slack
- Schedule reminders
- Search the web
- Manage files
- Run commands

### Development Automation
- Code review
- Git operations
- CI/CD integration
- Documentation generation
- Test execution

### Edge Computing
- IoT device control
- Sensor data processing
- Local inference
- Offline operation
- Hardware integration

### Enterprise
- Multi-tenant workspaces
- Audit logging
- SSO integration
- Cost tracking
- Compliance controls

### Research & Education
- Experiment tracking
- Data analysis
- Report generation
- Literature search
- Collaborative workflows

## 📦 Installation Methods

### Homebrew (macOS/Linux)
```bash
brew install zeroclaw
```

### One-Click Installer
```bash
curl -fsSL https://raw.githubusercontent.com/zeroclaw-labs/zeroclaw/master/install.sh | bash
```

### Pre-built Binaries
Download from GitHub Releases for:
- Linux (x86_64, aarch64, armv7)
- macOS (x86_64, aarch64)
- Windows (x86_64)

### From Source
```bash
git clone https://github.com/zeroclaw-labs/zeroclaw.git
cd zeroclaw
cargo build --release --locked
cargo install --path . --force --locked
```

## 🆚 ZeroClaw vs OpenCode vs OpenClaw

| Feature | ZeroClaw | OpenCode | OpenClaw |
|---------|----------|----------|----------|
| **Language** | Rust | TypeScript | TypeScript |
| **Binary Size** | 8.8 MB | N/A (Node) | ~28 MB |
| **RAM Usage** | < 5 MB | ~100 MB | > 1 GB |
| **Startup Time** | < 10ms | ~1s | > 500s (low-end) |
| **Providers** | 60 | 75+ | 60+ |
| **Tools** | 50+ built-in | Extensible | 30+ built-in |
| **Channels** | 21 | CLI only | 21 |
| **Hardware** | $10 boards | Desktop | Mac Mini ($599) |
| **License** | MIT/Apache 2.0 | MIT | MIT |
| **Cost** | FREE | FREE | FREE |
| **Best For** | Edge/embedded | Coding | Full-featured |

## 🎯 When to Choose ZeroClaw

Choose ZeroClaw if you need:
- ✅ Minimal resource usage
- ✅ Edge/embedded deployment
- ✅ Fast startup times
- ✅ Memory safety (Rust)
- ✅ Multi-channel support
- ✅ Hardware integration
- ✅ Enterprise security
- ✅ Cost efficiency

Choose OpenCode if you need:
- ✅ Maximum provider count (75+)
- ✅ Coding-focused workflows
- ✅ Node.js ecosystem
- ✅ Rapid prototyping

Choose OpenClaw if you need:
- ✅ Maximum features
- ✅ Desktop environment
- ✅ Plenty of RAM/CPU
- ✅ Mature ecosystem

## 📚 Documentation

- **Main Docs:** `docs/README.md`
- **CLI Reference:** `docs/reference/cli/commands-reference.md`
- **Config Reference:** `docs/reference/api/config-reference.md`
- **Providers:** `docs/reference/api/providers-reference.md`
- **Channels:** `docs/reference/api/channels-reference.md`
- **Operations:** `docs/ops/operations-runbook.md`
- **Troubleshooting:** `docs/ops/troubleshooting.md`
- **Security:** `docs/security/README.md`
- **Hardware:** `docs/hardware/README.md`
- **Contributing:** `CONTRIBUTING.md`

## 🤝 Community & Support

- **GitHub:** https://github.com/zeroclaw-labs/zeroclaw
- **Website:** https://zeroclawlabs.ai
- **X/Twitter:** @zeroclawlabs
- **Facebook:** facebook.com/groups/zeroclaw
- **Reddit:** r/zeroclawlabs
- **Donate:** buymeacoffee.com/argenistherose

## 🎉 Summary

ZeroClaw is a **production-ready, enterprise-grade AI agent runtime** that:
- Costs **$0** to use (only pay for LLM API calls)
- Runs on **$10 hardware** (99% cheaper than alternatives)
- Uses **< 5 MB RAM** (99% less than OpenClaw)
- Supports **60 providers** (all major ones covered)
- Includes **50+ tools** (file, shell, web, git, cloud, hardware, etc.)
- Connects to **21 messaging platforms**
- Provides **enterprise security** (sandboxing, encryption, audit logs)
- Is **100% open source** (MIT/Apache 2.0)

**Your current setup:** Working perfectly with Mistral (1 billion tokens/month FREE) ✓
