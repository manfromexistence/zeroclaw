# UI Integration Plan: Onboard Framework → Main Project

## Overview

This document identifies all UI locations in the main ZeroClaw project that should adopt the onboard framework's theming system (rainbow colors, effects, and consistent styling).

---

## Onboard Framework Features to Integrate

### Core Features from `onboard/`:

1. **Rainbow Effects** (`onboard/src/effects.rs`)
   - `RainbowEffect` struct with HSL-to-RGB color cycling
   - Smooth color transitions across characters
   - Configurable speed (0.5 cycles/sec default)

2. **Theme System** (`onboard/src/prompts/theme.rs` + `onboard/theme.toml`)
   - Configurable colors: primary, success, warning, error, dim
   - Configurable symbols: checkmark, cross, info, warning, error, arrow, bullet
   - Box drawing characters for borders
   - Rainbow toggle

3. **Splash/Logo System** (`onboard/src/splash.rs`)
   - 10 hardcoded ASCII art logos (randomly selected)
   - Rainbow-colored rendering
   - Train animation with smoke effects
   - Terminal-width-aware rendering

4. **24 Interactive Prompts** (`onboard/src/prompts/*.rs`)
   - Consistent styling across all prompt types
   - Theme-aware colors and symbols
   - Professional box-drawing borders

---

## Main Project UI Locations (Needs Integration)

### 1. **CLI Entry Point** (`src/main.rs`)

**Current State:**
- Plain text help message
- No banner/splash screen
- Basic `println!` statements

**Lines to Update:**
- Line 52-60: `print_no_command_help()` function
- Add rainbow banner before help text
- Use themed colors for command categories

**Recommendation:**
```rust
// Add at start of print_no_command_help()
use onboard::effects::RainbowEffect;
use onboard::splash::render_dx_logo;

let rainbow = RainbowEffect::new();
render_dx_logo(&rainbow)?;
println!();
```

---

### 2. **Onboarding Wizard** (`src/onboard/wizard.rs` - 7,294 lines!)

**Current State:**
- Uses basic `dialoguer` prompts (Input, Select, Confirm, Password)
- Uses `console::style()` for colors (limited palette)
- Plain text step indicators

**Key Functions to Update:**

#### a) `print_step()` (Line 2009)
```rust
// Current:
fn print_step(current: u8, total: u8, title: &str) {
    println!("\n  [{}/{}] {}", current, total, title);
}

// Should use: onboard rainbow effects + themed symbols
```

#### b) `print_bullet()` (Line 2019)
```rust
// Current:
fn print_bullet(text: &str) {
    println!("  • {}", text);
}

// Should use: themed bullet symbol with rainbow color
```

#### c) `print_summary()` (Line 5584)
- Currently uses plain text
- Should use rainbow-colored ASCII art + themed checkmarks

#### d) All `dialoguer` prompts throughout the file
- Replace with `onboard` prompts:
  - `dialoguer::Input` → `onboard::prompts::input::Input`
  - `dialoguer::Select` → `onboard::prompts::select::Select`
  - `dialoguer::Confirm` → `onboard::prompts::confirm::Confirm`
  - `dialoguer::Password` → `onboard::prompts::password::Password`

**Specific Locations:**
- Line 23: Import statements
- Line 2507: Gemini CLI confirmation
- Throughout: ~50+ dialoguer prompt usages

---

### 3. **Memory CLI** (`src/memory/cli.rs`)

**Current State:**
- Uses `console::style()` for basic colors
- Uses `dialoguer::Confirm` for confirmations

**Lines to Update:**
- Line 10: Import `console::style`
- Line 227-230: Delete confirmation prompt
- Line 283-286: Target deletion confirmation

**Recommendation:**
- Replace `console::style()` with themed colors
- Replace `dialoguer::Confirm` with `onboard::prompts::confirm::Confirm`
- Add rainbow effects to success messages

---

### 4. **Skills Management** (`src/skills/mod.rs`)

**Current State:**
- Uses `console::style()` for colored output
- Checkmarks (✓) and crosses (✗) hardcoded

**Lines to Update:**
- Line 982-986: Skill listing display
- Line 1021-1024: Audit success message
- Line 1030-1033: Audit failure message
- Line 1050-1053: Installation success (workspace)
- Line 1059-1062: Installation success (user)
- Line 1093-1096: Removal success

**Recommendation:**
- Load theme symbols instead of hardcoded ✓/✗
- Apply rainbow effects to skill names
- Use themed colors for status indicators

---

### 5. **SOP (Standard Operating Procedures)** (`src/sop/mod.rs`)

**Current State:**
- Uses `console::style()` for formatting
- Hardcoded symbols (✓, !)

**Lines to Update:**
- Line 372-377: SOP listing
- Line 418-421: Validation success
- Line 425-428: Validation warnings
- Line 455-458: SOP display header
- Line 485-488: Step display

**Recommendation:**
- Use themed symbols and colors
- Add rainbow effects to SOP names
- Consistent border styling with onboard framework

---

### 6. **Integrations Browser** (`src/integrations/mod.rs`)

**Current State:**
- Uses `console::style()` for bold text
- Plain `println!` for setup instructions

**Lines to Update:**
- Line 93-171: Entire `show_integration()` function
- Line 97: Integration name display
- Line 100-101: Category and status
- Line 107-170: Setup instructions for each integration

**Recommendation:**
- Use rainbow effects for integration names
- Use themed colors for categories
- Box-drawing borders around setup instructions
- Themed bullet points for steps

---

### 7. **Doctor/Diagnostics** (`src/doctor/mod.rs`)

**Current State:**
- Hardcoded emoji icons (🩺, ✅, ⚠️, ❌)
- Plain `println!` statements

**Lines to Update:**
- Line 96: Header "🩺 ZeroClaw Doctor"
- Line 103-110: Category headers and status icons
- Line 126: Summary line

**Recommendation:**
- Replace emojis with themed symbols
- Use rainbow effects for header
- Themed colors for severity levels
- Box-drawing borders around report

---

### 8. **Channel Management** (`src/channels/mod.rs`)

**Status:** Need to search for UI code

**Expected Updates:**
- Channel listing display
- Status indicators
- Connection messages

---

### 9. **Cron Task Display** (`src/cron/mod.rs`)

**Status:** Need to search for UI code

**Expected Updates:**
- Task listing
- Schedule display
- Status indicators

---

### 10. **Hardware Discovery** (`src/hardware/mod.rs`)

**Status:** Need to search for UI code

**Expected Updates:**
- Device listing
- Introspection output
- Connection status

---

## Implementation Strategy

### Phase 1: Core Infrastructure (Do First)

1. **Add onboard as dependency** in main `Cargo.toml`:
```toml
[dependencies]
onboard = { path = "./onboard" }
```

2. **Create theme loader** in `src/ui/mod.rs`:
```rust
pub mod theme;
pub mod effects;
pub mod splash;

pub use onboard::effects::RainbowEffect;
pub use onboard::splash::{render_dx_logo, render_train_animation};
pub use onboard::prompts::theme::DxTheme;
```

3. **Copy theme.toml** to main project root or `.zeroclaw/` config dir

### Phase 2: Replace Dialoguer (High Impact)

1. Update `src/onboard/wizard.rs`:
   - Replace all `dialoguer::*` imports with `onboard::prompts::*`
   - Update ~50+ prompt call sites
   - Test each wizard step

2. Update `src/memory/cli.rs`:
   - Replace confirmation prompts
   - Add rainbow effects to success messages

### Phase 3: Themed Output (Visual Polish)

1. Update all `console::style()` usage:
   - `src/skills/mod.rs`
   - `src/sop/mod.rs`
   - `src/integrations/mod.rs`
   - `src/doctor/mod.rs`

2. Replace hardcoded symbols (✓, ✗, •, etc.) with theme symbols

3. Add rainbow effects to:
   - Skill names
   - Integration names
   - SOP titles
   - Success messages

### Phase 4: Splash Screens (Delight)

1. Add banner to `src/main.rs`:
   - Show on startup (optional flag to disable)
   - Show in `print_no_command_help()`

2. Add train animation:
   - Show during long operations (optional)
   - Show on successful onboarding completion

### Phase 5: Box Drawing (Professional Look)

1. Add borders to:
   - Integration setup instructions
   - Doctor report sections
   - Skill listings
   - SOP step displays

2. Use consistent box-drawing characters from theme

---

## Questions for You

Before I start implementing, please clarify:

### 1. **Splash Screen Placement**
Where should the DX logo appear?
- [ ] A. On every `zeroclaw` command (before help text)
- [ ] B. Only on `zeroclaw onboard`
- [ ] C. Only when no command is provided
- [ ] D. Optional flag `--splash` to enable

### 2. **Train Animation Usage**
When should the train animation play?
- [ ] A. During onboarding wizard (between steps)
- [ ] B. On successful onboarding completion
- [ ] C. During long-running operations (model downloads, etc.)
- [ ] D. Never (too distracting)
- [ ] E. Optional flag `--train` to enable

### 3. **Rainbow Effects Scope**
Which text should get rainbow colors?
- [ ] A. Only logos and banners
- [ ] B. All headings and titles
- [ ] C. Skill/integration/SOP names
- [ ] D. Success messages
- [ ] E. Everything (maximum rainbow!)
- [ ] F. Configurable in theme.toml

### 4. **Dialoguer Replacement Priority**
Which prompts should I replace first?
- [ ] A. Onboarding wizard (highest impact, most work)
- [ ] B. Memory CLI confirmations (quick win)
- [ ] C. All at once (big bang approach)
- [ ] D. Gradual (one module at a time)

### 5. **Theme Configuration Location**
Where should `theme.toml` live?
- [ ] A. Project root (alongside Cargo.toml)
- [ ] B. `~/.zeroclaw/theme.toml` (user config)
- [ ] C. Both (user overrides project default)
- [ ] D. Embedded in binary (no external file)

### 6. **Backward Compatibility**
Should we keep plain text fallback?
- [ ] A. Yes, add `--no-color` flag
- [ ] B. Yes, detect NO_COLOR env var
- [ ] C. Yes, detect non-TTY output
- [ ] D. No, always use themed output

### 7. **Box Drawing Characters**
Should we use Unicode box drawing?
- [ ] A. Yes, always (modern terminals)
- [ ] B. Yes, with ASCII fallback for old terminals
- [ ] C. No, keep it simple
- [ ] D. Configurable in theme.toml

---

## File Count Summary

**Files to Modify:**
- `src/main.rs` (1 file)
- `src/onboard/wizard.rs` (1 file, 7,294 lines!)
- `src/memory/cli.rs` (1 file)
- `src/skills/mod.rs` (1 file)
- `src/sop/mod.rs` (1 file)
- `src/integrations/mod.rs` (1 file)
- `src/doctor/mod.rs` (1 file)
- `src/channels/mod.rs` (1 file, need to inspect)
- `src/cron/mod.rs` (1 file, need to inspect)
- `src/hardware/mod.rs` (1 file, need to inspect)

**Files to Create:**
- `src/ui/mod.rs` (new module)
- `src/ui/theme.rs` (theme loader)
- `src/ui/effects.rs` (re-export onboard effects)
- `src/ui/splash.rs` (re-export onboard splash)

**Total:** ~10-15 files to modify, 4 new files

---

## Next Steps

1. **Answer the 7 questions above**
2. I'll create a detailed TODO.md with specific tasks
3. We'll implement phase by phase
4. Test each phase before moving to the next

Ready to make ZeroClaw beautiful! 🌈🚂
