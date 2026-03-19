I'll explore the project structure to identify all UI-related code, focusing on onboarding and command list components.

Now let me explore the key UI-related directories:

Now let me check the web UI components:

Perfect! Now I have a comprehensive understanding. Let me create a detailed research document:

# UI Onboarding and Command List Code Location Research

## Executive Summary

Your Rust project has **two distinct UI systems**:

1. **Terminal UI (TUI)** - Built with custom prompts library in the `onboard/` crate
2. **Web UI** - React-based dashboard in the `web/` directory

---

## 1. Terminal UI (TUI) Onboarding System

### Primary Location: `onboard/` Crate

This is a **standalone Rust crate** providing 24 interactive terminal prompt types for onboarding.

#### Core Files:

**Main Library:**
- `onboard/src/lib.rs` - Main onboarding flow orchestration, 24 prompt types demo
- `onboard/src/main.rs` - Binary entry point for standalone onboard tool
- `onboard/Cargo.toml` - Dependencies (console, owo-colors, terminal_size, textwrap)

**Prompt Components (24 types):**
- `onboard/src/prompts/mod.rs` - Prompt module exports
- `onboard/src/prompts/input.rs` - Text input
- `onboard/src/prompts/email.rs` - Email validation
- `onboard/src/prompts/password.rs` - Password input
- `onboard/src/prompts/url.rs` - URL validation
- `onboard/src/prompts/phone_input.rs` - Phone number input
- `onboard/src/prompts/number.rs` - Numeric input
- `onboard/src/prompts/rating.rs` - Star rating
- `onboard/src/prompts/slider.rs` - Single value slider
- `onboard/src/prompts/range_slider.rs` - Range selection
- `onboard/src/prompts/toggle.rs` - Boolean toggle switches
- `onboard/src/prompts/select.rs` - Single selection menu
- `onboard/src/prompts/multiselect.rs` - Multiple selection
- `onboard/src/prompts/autocomplete.rs` - Autocomplete search
- `onboard/src/prompts/search_filter.rs` - Searchable list
- `onboard/src/prompts/tags.rs` - Tag input
- `onboard/src/prompts/tree_select.rs` - Hierarchical selection
- `onboard/src/prompts/matrix_select.rs` - Grid selection
- `onboard/src/prompts/progress.rs` - Progress bars
- `onboard/src/prompts/spinner.rs` - Loading spinners
- `onboard/src/prompts/text.rs` - Multi-line text area
- `onboard/src/prompts/wizard.rs` - Multi-step wizard
- `onboard/src/prompts/confirm.rs` - Yes/No confirmation
- `onboard/src/prompts/cursor.rs` - Cursor utilities
- `onboard/src/prompts/interaction.rs` - Interaction handling
- `onboard/src/prompts/theme.rs` - Theme configuration

**Visual Effects:**
- `onboard/src/effects.rs` - Rainbow and color effects
- `onboard/src/splash.rs` - ASCII art and logo rendering

**Documentation:**
- `onboard/README.md` - Library documentation
- `onboard/EXAMPLES.md` - Usage examples
- `onboard/PROMPTS.md` - Prompt type reference
- `onboard/THEMING.md` - Theme customization guide
- `onboard/theme.toml` - Theme configuration

### Integration in Main Application:

**Wizard Implementation:**
- `src/onboard/mod.rs` - Module exports
- `src/onboard/wizard.rs` - **7,294 lines** - Main interactive wizard with:
  - Provider setup (OpenAI, Anthropic, OpenRouter, etc.)
  - Channel configuration (Telegram, Discord, Slack, Matrix, etc.)
  - Memory backend selection (SQLite, Lucid, Markdown, PostgreSQL)
  - Hardware setup
  - Tunnel configuration
  - Security settings
  - Project context personalization

**Key Functions in `src/onboard/wizard.rs`:**
- `run_wizard()` - Full interactive onboarding (9 steps)
- `run_quick_setup()` - Non-interactive scriptable setup
- `run_channels_repair_wizard()` - Channel-only repair flow
- `run_provider_update_wizard()` - Provider-only update
- `setup_provider()` - AI provider configuration
- `setup_channels()` - Communication channels setup
- `setup_memory()` - Memory backend selection
- `setup_hardware()` - Hardware configuration
- `setup_project_context()` - User personalization

**CLI Entry Point:**
- `src/main.rs` (lines 1-935+ visible, 2,517 total) - Command-line interface with:
  - `Commands::Onboard` - Onboard command handler
  - Banner display (ASCII art)
  - Interactive vs quick mode detection
  - Auto-start channels option

---

## 2. Terminal Animations & Visual Effects

### Location: `cursed/` Directory

Custom terminal animations using `crossterm` library:

**Animation Modules:**
- `cursed/animations/mod.rs` - Animation initialization and utilities
- `cursed/animations/ascii_art.rs` - ASCII art rendering
- `cursed/animations/confetti.rs` - Celebration effects
- `cursed/animations/gameoflife.rs` - Conway's Game of Life
- `cursed/animations/images.rs` - Image rendering
- `cursed/animations/matrix.rs` - Matrix rain effect
- `cursed/animations/particles.rs` - Particle systems (rain, snow, stars)
- `cursed/animations/sounds.rs` - Audio visualization
- `cursed/animations/train.rs` - Train animation
- `cursed/animations/transitions.rs` - Fade, slide, typing effects
- `cursed/animations/video.rs` - Video playback
- `cursed/animations/visualizer.rs` - Audio visualizer

**Documentation:**
- `cursed/README.md` - Animation system overview
- `cursed/THEME_SYSTEM.md` - Theme system documentation

**Deprecated Components:**
- `cursed/trash/` - Old UI components (calendar, color picker, file browser, etc.)

---

## 3. Web UI Dashboard

### Location: `web/` Directory

React + TypeScript web dashboard with Vite build system.

#### Core Structure:

**Main Application:**
- `web/src/App.tsx` - Main app component with routing, auth, pairing dialog
- `web/src/main.tsx` - React entry point
- `web/index.html` - HTML template
- `web/vite.config.ts` - Vite configuration

**Layout Components:**
- `web/src/components/layout/Layout.tsx` - Main layout wrapper
- `web/src/components/layout/Header.tsx` - Top navigation bar
- `web/src/components/layout/Sidebar.tsx` - Side navigation menu

**Page Components:**
- `web/src/pages/Dashboard.tsx` - Main dashboard
- `web/src/pages/AgentChat.tsx` - Chat interface
- `web/src/pages/Tools.tsx` - Tool management
- `web/src/pages/Cron.tsx` - Scheduled tasks
- `web/src/pages/Integrations.tsx` - Integration management
- `web/src/pages/Memory.tsx` - Memory browser
- `web/src/pages/Config.tsx` - Configuration editor
- `web/src/pages/Cost.tsx` - Cost tracking
- `web/src/pages/Logs.tsx` - Log viewer
- `web/src/pages/Doctor.tsx` - Diagnostics
- `web/src/pages/Pairing.tsx` - Device pairing

**Hooks & Utilities:**
- `web/src/hooks/useApi.ts` - API client hook
- `web/src/hooks/useAuth.ts` - Authentication hook
- `web/src/hooks/useDevices.ts` - Device management
- `web/src/hooks/useDraft.ts` - Draft state management
- `web/src/hooks/useSSE.ts` - Server-sent events
- `web/src/hooks/useWebSocket.ts` - WebSocket connection

**Libraries:**
- `web/src/lib/api.ts` - API client
- `web/src/lib/auth.ts` - Auth utilities
- `web/src/lib/i18n.ts` - Internationalization
- `web/src/lib/sse.ts` - SSE client
- `web/src/lib/ws.ts` - WebSocket client
- `web/src/lib/uuid.ts` - UUID generation

**Types:**
- `web/src/types/api.ts` - TypeScript API types

**Build Configuration:**
- `web/package.json` - NPM dependencies
- `web/tsconfig.json` - TypeScript config
- `web/.gitignore` - Git ignore rules

**Embedded Assets:**
- `web/dist/` - Built static assets (embedded in Rust binary via `rust-embed`)

---

## 4. Command List & Help System

### CLI Command Structure:

**Main Command Handler:**
- `src/main.rs` - Clap-based CLI with subcommands:
  - `onboard` - Interactive setup wizard
  - `agent` - Start AI agent loop
  - `gateway` - Manage gateway server
  - `daemon` - Start autonomous runtime
  - `service` - OS service management
  - `doctor` - Run diagnostics
  - `status` - System status
  - `estop` - Emergency stop
  - `cron` - Scheduled tasks
  - `models` - Model catalog management
  - `providers` - List AI providers
  - `channel` - Channel management
  - `integrations` - Integration browser
  - `skills` - Skill management
  - `migrate` - Data migration
  - `auth` - Authentication profiles
  - `hardware` - Hardware discovery
  - `peripheral` - Peripheral management
  - `memory` - Memory management
  - `config` - Configuration management
  - `update` - Self-update
  - `self-test` - Diagnostic tests
  - `completions` - Shell completions
  - `plugin` - WASM plugin management (optional)

**Command Implementations:**
- `src/commands/mod.rs` - Command module
- `src/commands/self_test.rs` - Self-test command
- `src/commands/update.rs` - Update command

**Subcommand Handlers:**
- `src/channels/mod.rs` - Channel commands (`list`, `add`, `remove`, `send`, `doctor`)
- `src/cron/mod.rs` - Cron commands (`list`, `add`, `pause`, `update`)
- `src/memory/cli.rs` - Memory commands (`list`, `get`, `stats`, `clear`)
- `src/skills/mod.rs` - Skill commands (`list`, `create`, `remove`)
- `src/peripherals/mod.rs` - Peripheral commands (`list`, `add`, `flash`)
- `src/hardware/mod.rs` - Hardware commands (`discover`, `introspect`, `info`)

**Help Text Generation:**
- Clap automatically generates help text from command definitions
- `print_no_command_help()` in `src/main.rs` - Custom help when no command provided
- Shell completion generation via `completions` subcommand

---

## 5. Dependencies & UI Libraries

### Terminal UI Stack:

**From `onboard/Cargo.toml`:**
- `console = "0.15"` - Terminal manipulation
- `owo-colors = "4.0"` - Color output
- `terminal_size = "0.4.3"` - Terminal size detection
- `textwrap = "0.16"` - Text wrapping

**From main `Cargo.toml`:**
- `dialoguer = "0.12"` - Interactive prompts (fuzzy-select)
- `console = "0.16"` - Terminal utilities
- `clap = "4.5"` - CLI argument parsing
- `clap_complete = "4.5"` - Shell completion generation

**Animation Stack (cursed/):**
- `crossterm` - Cross-platform terminal manipulation (used in animations)

### Web UI Stack:

**From `web/package.json`:**
- React 18+ - UI framework
- TypeScript - Type safety
- Vite - Build tool
- React Router - Routing
- Tailwind CSS (likely) - Styling

### Backend Web Server:

**From main `Cargo.toml`:**
- `axum = "0.8"` - HTTP server framework
- `tower = "0.5"` - Middleware
- `tower-http = "0.6"` - HTTP middleware
- `rust-embed = "8"` - Embed web assets in binary

---

## 6. Key UI-Related Configuration Files

**Onboarding:**
- `onboard/theme.toml` - Theme configuration for TUI
- `.env.example` - Environment variable template

**Web Dashboard:**
- `web/index.html` - HTML entry point
- `web/src/index.css` - Global styles

**Build System:**
- `build.rs` - Build script (likely embeds web assets)

---

## 7. Internationalization (i18n)

**Tool Descriptions:**
- `tool_descriptions/en.toml` - English tool descriptions
- `tool_descriptions/zh-CN.toml` - Chinese tool descriptions

**Web i18n:**
- `web/src/lib/i18n.ts` - i18n utilities
- `src/i18n.rs` - Backend i18n support

---

## Summary: Files to Update for UI Changes

### For Terminal Onboarding UI:
1. `onboard/src/lib.rs` - Main onboarding flow
2. `onboard/src/prompts/*.rs` - Individual prompt components
3. `src/onboard/wizard.rs` - ZeroClaw-specific wizard logic
4. `onboard/theme.toml` - Visual theme

### For Command List/Help:
1. `src/main.rs` - CLI command definitions (Clap structs)
2. `src/commands/*.rs` - Command implementations
3. Individual module `mod.rs` files for subcommands

### For Web Dashboard UI:
1. `web/src/App.tsx` - Main app and routing
2. `web/src/components/layout/*.tsx` - Layout components
3. `web/src/pages/*.tsx` - Page components
4. `web/src/index.css` - Global styles

### For Terminal Animations:
1. `cursed/animations/*.rs` - Animation implementations
2. `cursed/animations/mod.rs` - Animation utilities

All UI code is well-organized and separated by concern. The terminal UI uses a custom prompt library, the web UI is a standard React app, and command help is generated by Clap.

The on-board Rust project is a new Rust crate that I recently added here so please don't run it and run the whole other Rust CLI. Please run it in a low-end device mode where it may take more time but we'll run it as we don't have enough RAM. Don't do parallel or more than one job; just do one job and use `cargo run` instead of `cargo build` as we don't have much space. We have to use the incremental job one run so that it may take a longer time but will even work on our low-end device. 

$env:CARGO_BUILD_JOBS="1"; cargo run -j 1
$env:CARGO_BUILD_JOBS="1"; cargo run -j 1 -- --help

PS F:\zeroclaw> cargo run --release
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   F:\zeroclaw\onboard\Cargo.toml
workspace: F:\zeroclaw\Cargo.toml
warning: F:\zeroclaw\Cargo.toml: unused manifest key: lib.include
   Compiling zeroclawlabs v0.5.0 (F:\zeroclaw)
    Finished `release` profile [optimized] target(s) in 6m 25s
     Running `target\release\zeroclaw.exe`
No command provided.
Try `zeroclaw onboard` to initialize your workspace.

The fastest, smallest AI assistant.

Usage: zeroclaw [OPTIONS] <COMMAND>

Commands:
  onboard       Initialize your workspace and configuration
  agent         Start the AI agent loop
  gateway       Start/manage the gateway server (webhooks, websockets)
  daemon        Start long-running autonomous runtime (gateway + channels + heartbeat + scheduler)
  service       Manage OS service lifecycle (launchd/systemd user service)
  doctor        Run diagnostics for daemon/scheduler/channel freshness
  status        Show system status (full details)
  estop         Engage, inspect, and resume emergency-stop states
  cron          Configure and manage scheduled tasks
  models        Manage provider model catalogs
  providers     List supported AI providers
  channel       Manage channels (telegram, discord, slack)
  integrations  Browse 50+ integrations
  skills        Manage skills (user-defined capabilities)
  migrate       Migrate data from other agent runtimes
  auth          Manage provider subscription authentication profiles
  hardware      Discover and introspect USB hardware
  peripheral    Manage hardware peripherals (STM32, RPi GPIO, etc.)
  memory        Manage agent memory (list, get, stats, clear)
  config        Manage configuration
  update        Check for and apply updates
  self-test     Run diagnostic self-tests
  completions   Generate shell completion script to stdout
  help          Print this message or the help of the given subcommand(s)

Options:
      --config-dir <CONFIG_DIR>
  -h, --help                     Print help
  -V, --version                  Print version

Please look at our root onboard folder. It's a separate UI framework that I created to use on this big project.
Now about our root src folder, big project, please look for all of the places that use a UI and use our onboarding theming logic, rainbow colors, and effects. Ask me where to put what so that for clarification questions so that we can make our whole project beautiful like the onboard project, tui UI.

Now here's the thing: as you can see our one go to root folder UI framework is quiet professional, production-ready code so you don't have to recreate it on our main project. You just have to copy them and reconfigure them to be used on our main project. Please start working on it and ask me a clarification question about where to work first!!!
