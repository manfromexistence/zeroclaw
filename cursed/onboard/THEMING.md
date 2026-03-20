# Theming Guide

> Customize colors and symbols in DX Onboard

## Overview

DX Onboard uses a TOML-based theming system that allows you to customize:
- Colors (primary, secondary, success, warning, error, dim)
- Symbols (checkmark, cross, info, warning, error, arrow, bullet)

## Default Theme

The default theme is defined in `theme.toml`:

```toml
[colors]
primary = "white"
secondary = "cyan"
success = "green"
warning = "yellow"
error = "red"
dim = "bright-black"

[symbols]
checkmark = "√"
cross = "×"
info = "i"
warning = "!"
error = "×"
arrow = "→"
bullet = "•"
```

## Color Options

### Standard Colors
- `black`
- `red`
- `green`
- `yellow`
- `blue`
- `magenta`
- `cyan`
- `white`

### Bright Colors
- `bright-black` (gray)
- `bright-red`
- `bright-green`
- `bright-yellow`
- `bright-blue`
- `bright-magenta`
- `bright-cyan`
- `bright-white`

## Color Usage

### Primary Color
Used for:
- Main headings
- Selected items
- Important text

### Secondary Color
Used for:
- Subheadings
- Hints
- Secondary information

### Success Color
Used for:
- Success messages
- Completed tasks
- Positive feedback

### Warning Color
Used for:
- Warning messages
- Caution indicators
- Non-critical issues

### Error Color
Used for:
- Error messages
- Failed operations
- Critical issues

### Dim Color
Used for:
- Info messages
- Placeholders
- Less important text

## Symbol Customization

### Checkmark
Used for:
- Completed items
- Success indicators
- Confirmed selections

Default: `√`
Alternatives: `✓`, `✔`, `[x]`, `*`

### Cross
Used for:
- Failed items
- Cancelled operations
- Deselected items

Default: `×`
Alternatives: `✗`, `✘`, `[ ]`, `-`

### Info
Used for:
- Information messages
- Help text
- Neutral indicators

Default: `i`
Alternatives: `ℹ`, `(i)`, `[i]`, `>`

### Warning
Used for:
- Warning messages
- Caution indicators
- Important notices

Default: `!`
Alternatives: `⚠`, `(!!)`, `[!]`, `*`

### Error
Used for:
- Error messages
- Failed operations
- Critical issues

Default: `×`
Alternatives: `✗`, `[X]`, `!!`, `ERR`

### Arrow
Used for:
- Navigation indicators
- Direction pointers
- Flow indicators

Default: `→`
Alternatives: `>`, `=>`, `->`, `▶`

### Bullet
Used for:
- List items
- Step indicators
- Unordered lists

Default: `•`
Alternatives: `*`, `-`, `·`, `○`, `►`

## Creating Custom Themes

### Example: Cyberpunk Theme

```toml
[colors]
primary = "magenta"
secondary = "cyan"
success = "bright-green"
warning = "yellow"
error = "bright-red"
dim = "bright-black"

[symbols]
checkmark = "✓"
cross = "✗"
info = "ℹ"
warning = "⚠"
error = "✗"
arrow = "▶"
bullet = "►"
```

### Example: Minimal Theme

```toml
[colors]
primary = "white"
secondary = "white"
success = "white"
warning = "white"
error = "white"
dim = "bright-black"

[symbols]
checkmark = "[x]"
cross = "[ ]"
info = "[i]"
warning = "[!]"
error = "[X]"
arrow = ">"
bullet = "-"
```

### Example: High Contrast Theme

```toml
[colors]
primary = "bright-white"
secondary = "bright-cyan"
success = "bright-green"
warning = "bright-yellow"
error = "bright-red"
dim = "white"

[symbols]
checkmark = "✓✓"
cross = "✗✗"
info = "INFO"
warning = "WARN"
error = "ERROR"
arrow = "==>"
bullet = "***"
```

## Loading Custom Themes

### Method 1: Replace theme.toml

Simply replace the `theme.toml` file in the onboard directory:

```bash
cp my-theme.toml onboard/theme.toml
```

### Method 2: Programmatic Loading

```rust
use onboard::prompts::theme::DxTheme;
use std::fs;

fn load_custom_theme() -> anyhow::Result<DxTheme> {
    let theme_toml = fs::read_to_string("custom_theme.toml")?;
    let theme: DxTheme = toml::from_str(&theme_toml)?;
    Ok(theme)
}
```

### Method 3: Create Theme in Code

```rust
use onboard::prompts::theme::{DxTheme, ColorScheme, Symbols};

fn create_dark_theme() -> DxTheme {
    DxTheme {
        colors: ColorScheme {
            primary: "cyan".to_string(),
            secondary: "blue".to_string(),
            success: "green".to_string(),
            warning: "yellow".to_string(),
            error: "red".to_string(),
            dim: "bright-black".to_string(),
        },
        symbols: Symbols {
            checkmark: "✓".to_string(),
            cross: "✗".to_string(),
            info: "ℹ".to_string(),
            warning: "⚠".to_string(),
            error: "✗".to_string(),
            arrow: "→".to_string(),
            bullet: "•".to_string(),
        },
    }
}
```

## Theme Structure

The theme is defined by two main structures:

### ColorScheme

```rust
pub struct ColorScheme {
    pub primary: String,
    pub secondary: String,
    pub success: String,
    pub warning: String,
    pub error: String,
    pub dim: String,
}
```

### Symbols

```rust
pub struct Symbols {
    pub checkmark: String,
    pub cross: String,
    pub info: String,
    pub warning: String,
    pub error: String,
    pub arrow: String,
    pub bullet: String,
}
```

## Applying Colors in Code

The theme system provides helper methods to apply colors:

```rust
use onboard::prompts::theme::DxTheme;
use owo_colors::OwoColorize;

let theme = DxTheme::load()?;

// Apply primary color
let text = "Hello".to_string();
let colored = theme.primary.apply_to(&text);
println!("{}", colored);

// Apply success color
let success_msg = "Done!".to_string();
let colored = theme.success.apply_to(&success_msg);
println!("{}", colored);
```

## Terminal Compatibility

### True Color Support

Most modern terminals support 24-bit true color. Check with:

```bash
echo $COLORTERM
# Should output: truecolor or 24bit
```

### Fallback Colors

If your terminal doesn't support true color, the theme will fall back to 16-color mode automatically.

### Testing Colors

Test your theme colors:

```rust
use onboard::prompts::theme::DxTheme;

fn test_theme() -> anyhow::Result<()> {
    let theme = DxTheme::load()?;
    
    println!("{}", theme.primary.apply_to(&"Primary".to_string()));
    println!("{}", theme.secondary.apply_to(&"Secondary".to_string()));
    println!("{}", theme.success.apply_to(&"Success".to_string()));
    println!("{}", theme.warning.apply_to(&"Warning".to_string()));
    println!("{}", theme.error.apply_to(&"Error".to_string()));
    println!("{}", theme.dim.apply_to(&"Dim".to_string()));
    
    Ok(())
}
```

## Best Practices

1. **Contrast** - Ensure sufficient contrast between text and background
2. **Consistency** - Use colors consistently across your application
3. **Accessibility** - Consider color-blind users (avoid red/green only)
4. **Testing** - Test themes in different terminals
5. **Fallbacks** - Provide ASCII alternatives for Unicode symbols

## Common Theme Presets

### Dark Mode
```toml
[colors]
primary = "cyan"
secondary = "blue"
success = "green"
warning = "yellow"
error = "red"
dim = "bright-black"
```

### Light Mode
```toml
[colors]
primary = "blue"
secondary = "magenta"
success = "green"
warning = "yellow"
error = "red"
dim = "black"
```

### Monochrome
```toml
[colors]
primary = "white"
secondary = "bright-white"
success = "white"
warning = "bright-white"
error = "bright-white"
dim = "bright-black"
```

### Solarized Dark
```toml
[colors]
primary = "cyan"
secondary = "blue"
success = "green"
warning = "yellow"
error = "red"
dim = "bright-black"
```

## Troubleshooting

### Colors Not Showing

1. Check terminal supports colors:
```bash
tput colors
# Should output: 256 or higher
```

2. Check COLORTERM variable:
```bash
echo $COLORTERM
```

3. Try a different terminal emulator

### Symbols Not Displaying

1. Ensure terminal uses UTF-8 encoding
2. Install a font with Unicode support (e.g., Nerd Fonts)
3. Use ASCII alternatives in theme.toml

### Theme Not Loading

1. Check theme.toml syntax:
```bash
cargo run --example validate_theme
```

2. Verify file location (should be in onboard/ directory)
3. Check file permissions

## Next Steps

- Try different color combinations
- Create theme presets for your team
- Test themes in different terminals
- Share your themes with the community
