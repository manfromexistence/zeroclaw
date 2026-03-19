# UI Replacement Task - Help for Better AI

## Task Overview

Replace ALL old dialoguer-based UI in the ZeroClaw wizard with the beautiful onboard UI framework located in `/onboard`.

## Current Status

### ✅ Completed
1. Split wizard.rs from 7,293 lines into maintainable modules:
   - wizard.rs: 2,621 lines
   - models.rs: 790 lines (NEW)
   - provider_setup.rs: 745 lines
   - channel_setup.rs: 1,742 lines
   - test.rs: 1,475 lines

2. Added onboard dependency to Cargo.toml:
   ```toml
   onboard = { path = "./onboard" }
   ```

3. Integrated splash screen and intro/outro in wizard.rs:
   - Added rainbow logo animation
   - Added styled section boxes
   - Replaced static banner

4. Partially replaced prompts in wizard.rs:
   - ✅ All `Confirm::new()` → `prompts::confirm()`
   - ✅ Some `Select::new()` → `prompts::select()`
   - ⚠️ Many `Input::new()` still remain
   - ⚠️ Many `Select::new()` still remain
   - ⚠️ Many plain `println!` still remain

5. Added required import:
   ```rust
   use onboard::prompts::PromptInteraction;
   ```

### ❌ Remaining Work

**Files that need UI replacement:**
1. `src/onboard/wizard.rs` - Main wizard (partially done)
2. `src/onboard/provider_setup.rs` - Provider selection (NOT STARTED)
3. `src/onboard/channel_setup.rs` - Channel configuration (NOT STARTED)

---

## Onboard UI Framework Reference

### Location
`/onboard` - Standalone UI framework with 24 prompt types

### Key Files
- `onboard/src/lib.rs` - Main API
- `onboard/src/prompts/mod.rs` - All prompt types
- `onboard/src/splash.rs` - ASCII art & animations
- `onboard/src/effects.rs` - Rainbow colors

### How to Use Onboard Prompts

**CRITICAL:** Must import the trait:
```rust
use onboard::prompts::PromptInteraction;
```

#### 1. Confirm (Yes/No)
```rust
// OLD (dialoguer)
let confirmed = Confirm::new()
    .with_prompt("  Continue?")
    .default(true)
    .interact()?;

// NEW (onboard)
let confirmed = prompts::confirm("Continue?")
    .initial_value(true)
    .interact()?;
```

#### 2. Toggle (Boolean Switch)
```rust
// OLD (dialoguer - using Confirm)
let enabled = Confirm::new()
    .with_prompt("  Enable feature?")
    .default(true)
    .interact()?;

// NEW (onboard - better for on/off switches)
let enabled = prompts::toggle::toggle("Enable feature?")
    .initial_value(true)
    .interact()?;
```

#### 3. Text Input
```rust
// OLD (dialoguer)
let name: String = Input::new()
    .with_prompt("  Your name")
    .default("User".into())
    .interact_text()?;

// NEW (onboard)
let name = prompts::input::input("Your name")
    .placeholder("User")
    .interact()?;
```

#### 4. Single Select
```rust
// OLD (dialoguer)
let choice = Select::new()
    .with_prompt("  Choose option")
    .items(&["Option 1", "Option 2"])
    .default(0)
    .interact()?;

// NEW (onboard)
let choice = prompts::select("Choose option")
    .item(0, "Option 1", "First option")
    .item(1, "Option 2", "Second option")
    .interact()?;
```

#### 5. Multi Select
```rust
// OLD (dialoguer - not shown in code)
// NEW (onboard)
let choices = prompts::multiselect("Choose multiple")
    .item("opt1".to_string(), "Option 1".to_string(), "First")
    .item("opt2".to_string(), "Option 2".to_string(), "Second")
    .interact()?;
```

#### 6. Status Messages
```rust
// OLD (plain println)
println!("  {} Success!", style("✓").green().bold());
println!("  {} Warning", style("⚠").yellow());
println!("  {} Info", style("ℹ").dim());

// NEW (onboard)
prompts::log::success("Success!")?;
prompts::log::warning("Warning")?;
prompts::log::info("Info")?;
prompts::log::step("Step item")?;
```

#### 7. Sections with Boxes
```rust
// OLD (plain println)
println!("Welcome to ZeroClaw");
println!("This is a description");

// NEW (onboard)
prompts::section_with_width("Welcome to ZeroClaw", 80, |lines| {
    lines.push("This is a description".to_string());
    lines.push("More information here".to_string());
})?;
```

#### 8. Intro/Outro
```rust
// OLD (plain println with banner)
println!("{}", style(BANNER).cyan().bold());

// NEW (onboard)
let rainbow = RainbowEffect::new();
print!("\x1B[2J\x1B[H"); // Clear screen
splash::render_dx_logo(&rainbow)?;
prompts::intro("Title")?;
// ... wizard content ...
prompts::outro("🎉 Complete!")?;
```

---

## Remaining Replacements Needed

### In `src/onboard/wizard.rs`

**Input::new() replacements (15+ occurrences):**
- Line 1623: Serial port path input
- Line 1649: Custom baud rate input
- Line 1663: Target MCU chip input
- Line 1722: User name input
- Line 1746: Timezone input
- Line 1760: Agent name input
- Line 1788: Custom communication style input
- Line 1933: Cloudflare tunnel token input
- Line 1982: ngrok auth token input
- Line 1989: ngrok custom domain input
- Line 2017: Custom tunnel command input

**Select::new() replacements (5+ occurrences):**
- Line 1596: Hardware interaction mode select
- Line 1623: Serial port selection (multiple devices)
- Line 1647: Baud rate selection
- Line 1748: Timezone selection
- Line 1784: Communication style selection
- Line 1834: Memory backend selection
- Line 1932: Tunnel provider selection

**Plain println! replacements (100+ occurrences):**
- Replace all status messages with `prompts::log::*`
- Replace all success messages with `prompts::log::success()`
- Replace all info messages with `prompts::log::info()`
- Replace all warnings with `prompts::log::warning()`

### In `src/onboard/provider_setup.rs`

**Needs complete UI replacement:**
- All `Confirm::new()` → `prompts::confirm()`
- All `Input::new()` → `prompts::input::input()`
- All `Select::new()` → `prompts::select()`
- All `println!` → `prompts::log::*`
- Add rainbow effects to model selection
- Add progress bars for API calls

### In `src/onboard/channel_setup.rs`

**Needs complete UI replacement:**
- All `Confirm::new()` → `prompts::confirm()` or `prompts::toggle::toggle()`
- All `Input::new()` → `prompts::input::input()`
- All `Select::new()` → `prompts::select()`
- Channel selection → `prompts::multiselect()`
- All `println!` → `prompts::log::*`
- Add connection test progress bars

---

## Code Examples from Current Implementation

### Example 1: Confirm Replacement (DONE)
```rust
// BEFORE
let launch: bool = Confirm::new()
    .with_prompt(format!(
        "  {} Launch channels now?",
        style("🚀").cyan()
    ))
    .default(true)
    .interact()?;

// AFTER
let launch = prompts::confirm("🚀 Launch channels now? (connected channels → AI → reply)")
    .initial_value(true)
    .interact()?;
```

### Example 2: Toggle Replacement (DONE)
```rust
// BEFORE
let encrypt = Confirm::new()
    .with_prompt("  Enable encrypted secret storage?")
    .default(true)
    .interact()?;

// AFTER
let encrypt = prompts::toggle::toggle("Enable encrypted secret storage?")
    .initial_value(true)
    .interact()?;
```

### Example 3: Select Replacement (DONE)
```rust
// BEFORE
let mode = Select::new()
    .with_prompt("  Select setup mode")
    .items(options)
    .default(1)
    .interact()?;

// AFTER
let mode = prompts::select("Select setup mode")
    .item(0, "Full onboarding", "Complete setup")
    .item(1, "Update provider only", "Quick update")
    .item(2, "Cancel", "Exit")
    .interact()?;
```

### Example 4: Status Messages (DONE)
```rust
// BEFORE
println!(
    "  {} Workspace: {}",
    style("✓").green().bold(),
    style(workspace_dir.display()).green()
);

// AFTER
prompts::log::success(format!("Workspace: {}", workspace_dir.display()))?;
```

---

## Testing

After making changes, test with:
```bash
cargo check --lib
cargo run -- onboard --help
cargo run -- onboard
```

---

## Important Notes

1. **DO NOT** remove the `dialoguer` import yet - it's still used in provider_setup.rs and channel_setup.rs
2. **DO NOT** remove the `console::style` import yet - still used in some places
3. **ALWAYS** import `use onboard::prompts::PromptInteraction;` when using onboard prompts
4. **PREFER** `prompts::toggle::toggle()` over `prompts::confirm()` for on/off switches
5. **USE** `prompts::log::*` instead of plain `println!` for all status messages
6. **ADD** rainbow effects and animations where appropriate
7. **KEEP** the code functional - don't break existing logic

---

## Next Steps for Better AI

1. **Phase 1:** Complete wizard.rs replacements
   - Replace all remaining `Input::new()` calls
   - Replace all remaining `Select::new()` calls
   - Replace all `println!` with `prompts::log::*`

2. **Phase 2:** Update provider_setup.rs
   - Replace all dialoguer prompts
   - Add rainbow effects to model selection
   - Add spinners for API calls

3. **Phase 3:** Update channel_setup.rs
   - Replace all dialoguer prompts
   - Use `prompts::multiselect()` for channel selection
   - Add progress bars for connection tests

4. **Phase 4:** Cleanup
   - Remove unused `dialoguer` imports
   - Remove unused `console::style` calls
   - Add train animation on completion

---

## Compilation Status

✅ Currently compiles successfully
✅ No errors
⚠️ 3 warnings (unused imports - can be fixed later)

---

## File Locations

- Main wizard: `src/onboard/wizard.rs`
- Provider setup: `src/onboard/provider_setup.rs`
- Channel setup: `src/onboard/channel_setup.rs`
- Models: `src/onboard/models.rs`
- Onboard framework: `onboard/src/`

---

## Summary

The task is to systematically replace every old UI element with the beautiful onboard framework. The pattern is clear from the examples above. A better AI should be able to complete this by following the patterns and replacing all occurrences in all three files.

**Estimated remaining work:** 150+ replacements across 3 files.
