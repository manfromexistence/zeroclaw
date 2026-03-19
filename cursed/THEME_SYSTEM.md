# Theme System Documentation

## Overview

The theme system has been extracted into a dedicated module (`theme.rs`) that provides unified theming across all prompts in the onboarding experience. Themes can be customized via a `theme.toml` file.

**Key Changes:**
- ✅ Removed all hardcoded blue colors - now uses theme.primary
- ✅ Removed all emojis - replaced with ASCII symbols
- ✅ All symbols are now configurable via TOML

## Features

- TOML-based configuration for easy customization
- Serde serialization/deserialization support
- Automatic fallback to default theme if config file is missing
- Hot-reloadable theme settings
- Support for custom colors and ASCII symbols (no emojis)
- Consistent theming across all 21 active prompts

## Theme Module (`src/prompts/theme.rs`)

The theme module contains:

### 1. ThemeConfig
Complete theme configuration structure with:
- `colors` - Color configuration (ColorConfig)
- `symbols` - Symbol configuration (SymbolConfig)
- `rainbow` - Rainbow effect configuration (RainbowConfig)

### 2. ColorConfig
Color scheme with hex color codes:
- `primary` - White (#FFFFFF) for main UI elements (changed from cyan)
- `success` - Green (#00FF00) for successful operations
- `warning` - Yellow (#FFFF00) for warnings
- `error` - Red (#FF0000) for errors
- `dim` - Gray (#808080) for borders and secondary elements

**Color Usage:**
- Info messages now use `dim` color instead of `primary` for better readability
- Primary color changed from cyan to white for better contrast
- All theme colors are fully configurable via TOML

### 3. SymbolConfig
ASCII symbols for consistent visual appearance (no emojis):
- Step indicators: `>`, `x`, `!`, `>`
- Selection indicators: `(*)`, `( )`, `[ ]`, `[x]`
- UI symbols: `√` (checkmark), `i` (info), `>` (arrow)
- Slider symbols: `=` (filled), `-` (empty), `O` (handle)
- Border elements: `│`, `╭`, `╮`, `╰`, `╯`, `├`, `─`

### 4. RainbowConfig
Rainbow effect settings:
- `enabled` - Enable/disable rainbow effects (default: true)
- `speed` - Animation speed multiplier (default: 1.0)

### 5. DxTheme
Runtime theme with console::Style instances for applying colors.

### 6. Symbols
Runtime symbol collection loaded from configuration.

### 7. Rainbow Effects
Animated rainbow coloring for special symbols using the `RainbowEffect` from effects module.

## Configuration File (`theme.toml`)

Place a `theme.toml` file in the project root to customize the theme:

```toml
[colors]
primary = "#FFFFFF"  # Changed from cyan to white
success = "#00FF00"
warning = "#FFFF00"
error = "#FF0000"
dim = "#808080"      # Used for info messages

[symbols]
# ASCII symbols only (no emojis)
step_active = ">"
checkmark = "√"
info = "i"           # Uses dim color, not primary
radio_active = "(*)"
radio_inactive = "( )"
# ... more symbols

[rainbow]
enabled = true
speed = 1.0
```

## Usage

### Loading Theme

The theme is automatically loaded when the application starts:

```rust
use onboard::prompts::theme::{load_theme_or_default, init_theme};

// Load theme from theme.toml or use defaults
let config = load_theme_or_default();

// Initialize the global theme
init_theme();
```

### Accessing Theme Elements

```rust
use onboard::prompts::theme::{THEME, SYMBOLS, rainbow_symbol};

// Access theme colors (replaces hardcoded blue)
let theme = THEME.read().unwrap();
let styled_text = theme.primary.apply_to("Hello");

// Access symbols (no emojis)
let symbols = &*SYMBOLS;
let checkmark = symbols.checkmark.as_str(); // "√" instead of "✓"

// Use rainbow effects
let rainbow_text = rainbow_symbol(&symbols.step_active, 0);
```

## Symbol Replacements Made

| Old (Emoji/Unicode) | New (ASCII) | Usage |
|---------------------|-------------|-------|
| ✓ | √ | Checkmark/success |
| ♦ | > | Step indicator |
| ● | (*) | Active radio |
| ○ | ( ) | Inactive radio |
| ◼ | [x] | Selected checkbox |
| ◻ | [ ] | Unselected checkbox |
| ★ | * | Rating stars |
| ☆ | - | Empty rating |
| ⚠ | ! | Warning |
| Blue colors | theme.primary | All blue text |

## Active Prompts (21 total)

All prompts now use theme colors and ASCII symbols:

1. `autocomplete` - Autocomplete with suggestions
2. `confirm` - Yes/no confirmation
3. `email` - Email input with validation
4. `input` - Basic text input
5. `matrix_select` - Skills rating matrix
6. `multiselect` - Multiple selection
7. `number` - Number input
8. `password` - Masked password input
9. `phone_input` - Phone number input
10. `progress` - Progress bar
11. `range_slider` - Range selection
12. `rating` - Star rating (using * instead of ★)
13. `search_filter` - Search with filtering
14. `select` - Single selection
15. `slider` - Value slider
16. `spinner` - Loading spinner
17. `tags` - Tag input
18. `text` - Multi-line text area
19. `toggle` - Toggle switches
20. `tree_select` - Tree selection
21. `url` - URL input
22. `wizard` - Multi-step wizard

## Dependencies

- `serde` (v1.0) with derive feature - Serialization framework
- `toml` (v1.0.7) - TOML parser and serializer
- `console` - Terminal styling
- `once_cell` - Lazy static initialization
- `owo-colors` - True color support

## Implementation Details

All prompts now import theme elements from the theme module:

```rust
use crate::prompts::theme::{THEME, SYMBOLS, rainbow_symbol};
```

The theme is globally accessible and thread-safe via `RwLock`. Symbol fields are accessed using `.as_str()` to convert from `String` to `&str` for compatibility with console styling functions.

**No more hardcoded blue colors or emojis anywhere in the codebase!**
