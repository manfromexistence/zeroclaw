# UI Audit Report - ZeroClaw Project

## Executive Summary

The ZeroClaw project has successfully integrated the onboard UI framework (`src/ui/`) but still has several areas using inconsistent UI patterns. This report identifies all locations where UI improvements are needed.

---

## ✅ GOOD: Already Using Onboard UI Framework

These modules are correctly using the onboard UI framework:

1. **`src/onboard/wizard.rs`** - Uses `crate::ui::prompts` extensively
2. **`src/onboard/provider_setup.rs`** - Uses `crate::ui::prompts`
3. **`src/onboard/channel_setup.rs`** - Uses `crate::ui::prompts`
4. **`src/onboard/models.rs`** - Uses `crate::ui::prompts`
5. **`src/memory/cli.rs`** - Partially uses `crate::ui::prompts::PromptInteraction`

---

## ❌ BAD: Inconsistent UI Patterns Found

### 1. **Direct `println!` / `eprintln!` Usage**

These files use raw print statements instead of themed output functions:

#### **`src/skills/mod.rs`** (Lines 970-1096)
- **Issue**: Uses `println!` and `console::style` directly
- **Location**: `handle_command()` function
- **Examples**:
  ```rust
  println!("No skills installed.");
  println!("Installed skills ({}):", skills.len());
  console::style(&skill.name).white().bold()
  console::style("✓").green().bold()
  ```
- **Should use**: 
  - `crate::theme::print_success()` for success messages
  - `crate::theme::print_info()` for info messages
  - `crate::theme::print_error()` for errors
  - `crate::ui::prompts::log::*` functions

#### **`src/sop/mod.rs`** (Lines 355-500)
- **Issue**: Uses `println!` and `console::style` directly
- **Location**: SOP command handlers
- **Examples**:
  ```rust
  println!("No SOPs found.");
  console::style(&sop.name).white().bold()
  console::style("✓").green().bold()
  console::style("!").yellow().bold()
  ```
- **Should use**: Themed output functions

#### **`src/integrations/mod.rs`** (Lines 96-120)
- **Issue**: Uses `println!` and `console::style` directly
- **Location**: Integration info display
- **Examples**:
  ```rust
  println!("  {} {} — {}", icon, console::style(entry.name).white().bold(), entry.description);
  ```
- **Should use**: Themed output functions

#### **`src/memory/cli.rs`** (Lines 90-100)
- **Issue**: Uses `println!` directly
- **Examples**:
  ```rust
  println!("No memory entries found.");
  println!("No entries at offset {offset} (total: {total}).");
  ```
- **Should use**: `crate::theme::print_info()` or `crate::ui::prompts::log::info()`

---

### 2. **Direct `console::style` Usage**

These files import and use `console::style` directly instead of using the theme system:

1. **`src/onboard/wizard.rs`** - Line 15: `use console::style;`
2. **`src/memory/cli.rs`** - Line 11: `use console::style;`
3. **`src/skills/mod.rs`** - Uses `console::style` throughout
4. **`src/sop/mod.rs`** - Uses `console::style` throughout
5. **`src/integrations/mod.rs`** - Uses `console::style` throughout

**Problem**: These should use the theme system colors from `crate::theme::theme()` instead.

---

### 3. **Theme System Not Fully Utilized**

The theme system exists in `src/theme.rs` with helper functions:
- `print_success()`
- `print_info()`
- `print_warning()`
- `print_error()`
- `print_step()`

But these are only used in `src/theme.rs` itself and not consistently across the codebase.

---

## 📋 Recommended Changes

### Priority 1: Replace Direct Print Statements

**Files to update:**
1. `src/skills/mod.rs` - Replace all `println!` with themed functions
2. `src/sop/mod.rs` - Replace all `println!` with themed functions
3. `src/integrations/mod.rs` - Replace all `println!` with themed functions
4. `src/memory/cli.rs` - Replace all `println!` with themed functions

**Example transformation:**
```rust
// BEFORE (BAD)
println!("  {} Skill installed", console::style("✓").green().bold());

// AFTER (GOOD)
crate::theme::print_success("Skill installed");
// OR
crate::ui::prompts::log::success("Skill installed")?;
```

### Priority 2: Remove Direct `console::style` Imports

**Files to update:**
1. `src/onboard/wizard.rs` - Remove `use console::style;`
2. `src/memory/cli.rs` - Remove `use console::style;`
3. All other files using `console::style`

**Replace with:**
```rust
use crate::theme::{print_success, print_info, print_warning, print_error, print_step};
// OR
use crate::ui::prompts::log;
```

### Priority 3: Enhance Theme System

**Add to `src/theme.rs`:**
```rust
// List formatting helpers
pub fn print_list_header(title: &str, count: usize) {
    let theme = theme();
    let style = Style::new().white().bold();
    println!("{} ({}):", style.apply_to(title), count);
}

pub fn print_list_item(name: &str, version: &str, description: &str) {
    let theme = theme();
    println!("  {} {} — {}", 
        Style::new().white().bold().apply_to(name),
        Style::new().dim().apply_to(format!("v{}", version)),
        description
    );
}
```

---

## 🎨 Theme System Integration

### Current Theme System (`src/theme.rs`)

The theme system loads from `theme.toml` and provides:
- **Colors**: primary, success, warning, error, dim
- **Symbols**: checkmark, info, arrow, step_error, step_cancel, arrow_right
- **Helper functions**: `print_success()`, `print_info()`, `print_warning()`, `print_error()`, `print_step()`

### Onboard UI Framework (`src/ui/`)

The onboard UI framework provides:
- **Prompts**: `confirm()`, `select()`, `input()`, `password()`, etc.
- **Effects**: `RainbowEffect`, splash screens, animations
- **Logging**: `prompts::log::success()`, `prompts::log::info()`, `prompts::log::warning()`, `prompts::log::error()`

### Integration Strategy

Both systems should work together:
1. Use **onboard UI prompts** for interactive input
2. Use **theme system** for consistent output styling
3. Use **onboard UI logging** for structured messages

---

## 📊 Statistics

- **Total files with UI issues**: 5
- **Files using `console::style` directly**: 5
- **Files using raw `println!`**: 4
- **Files correctly using onboard UI**: 4
- **Completion percentage**: ~45% (onboard UI integrated in wizard flows)

---

## 🎯 Action Items

### Immediate (High Priority)
1. ✅ Replace `dialoguer` usage - **DONE** (already completed)
2. ⚠️ Replace direct `println!` in `src/skills/mod.rs`
3. ⚠️ Replace direct `println!` in `src/sop/mod.rs`
4. ⚠️ Replace direct `println!` in `src/integrations/mod.rs`
5. ⚠️ Replace direct `println!` in `src/memory/cli.rs`

### Secondary (Medium Priority)
6. Remove `console::style` imports and use theme system
7. Add list formatting helpers to theme system
8. Create consistent output patterns across all CLI commands

### Future (Low Priority)
9. Add theme customization documentation
10. Create theme presets (dark, light, minimal, etc.)
11. Add color scheme validation

---

## 📝 Dependencies

### Current UI Dependencies in `Cargo.toml`:
```toml
dialoguer = { version = "0.12", features = ["fuzzy-select"] }  # ⚠️ Can be removed after full migration
console = "0.16"                                                # ✅ Keep (used by theme system)
owo-colors = { version = "4.0", features = ["supports-colors"] } # ✅ Keep (onboard UI)
terminal_size = "0.4.3"                                         # ✅ Keep (onboard UI)
textwrap = "0.16"                                               # ✅ Keep (onboard UI)
zeroize = { version = "1.8", features = ["derive"] }           # ✅ Keep (security)
```

**Recommendation**: `dialoguer` can be removed once all direct usage is confirmed eliminated.

---

## ✨ Benefits of Full Migration

1. **Consistency**: All UI elements use the same styling
2. **Themeable**: Users can customize colors via `theme.toml`
3. **Maintainability**: Single source of truth for UI patterns
4. **Better UX**: Consistent symbols, colors, and formatting
5. **Smaller binary**: Remove `dialoguer` dependency

---

## 🔍 Search Patterns Used

To find UI issues, these patterns were searched:
- `use dialoguer` - No matches (good!)
- `dialoguer::` - No matches (good!)
- `println!|print!|eprintln!` - Found in 5 files
- `console::style` - Found in 5 files
- `use inquire|inquire::` - No matches (good!)

---

## 📅 Generated

Date: 2026-03-20
Tool: Kiro AI Assistant
Project: ZeroClaw v0.5.0
