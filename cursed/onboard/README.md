# DX Onboard

> Interactive TUI onboarding library for Rust applications - 24 beautiful prompt types with rainbow animations

## What is this?

DX Onboard is a standalone TUI (Terminal User Interface) library that provides interactive prompts for collecting user input. It's designed to be integrated into the larger DX platform but works completely standalone.

## Features

- 24 different interactive prompt types
- Rainbow color effects and ASCII art
- Train animation on exit
- TOML-based theming system
- Runtime environment detection
- Password hashing with Argon2
- Configuration export to JSON

## Project Structure

```
onboard/
├── src/
│   ├── lib.rs              # Public API and run_onboarding()
│   ├── main.rs             # Standalone binary entry point
│   ├── effects.rs          # Rainbow color effects
│   ├── splash.rs           # ASCII art and train animation
│   └── prompts/            # All 24 prompt types
│       ├── mod.rs          # Exports and shared traits
│       ├── theme.rs        # Theme system
│       ├── input.rs        # Text input
│       ├── email.rs        # Email validation
│       ├── password.rs     # Secure password
│       ├── url.rs          # URL validation
│       ├── phone_input.rs  # Phone number
│       ├── number.rs       # Numeric input
│       ├── rating.rs       # Star rating
│       ├── slider.rs       # Single slider
│       ├── range_slider.rs # Range slider
│       ├── toggle.rs       # Boolean toggle
│       ├── select.rs       # Single selection
│       ├── multiselect.rs  # Multiple selection
│       ├── tags.rs         # Tag input
│       ├── autocomplete.rs # Autocomplete
│       ├── search_filter.rs# Filtered search
│       ├── text.rs         # Multi-line text
│       ├── wizard.rs       # Multi-step wizard
│       ├── progress.rs     # Progress bar
│       ├── spinner.rs      # Loading spinner
│       ├── log.rs          # Logging utilities
│       ├── confirm.rs      # Yes/No confirmation
│       └── trash/          # Archived prompts
├── theme.toml              # Default theme configuration
├── Cargo.toml              # Dependencies
├── README.md               # This file
└── PROMPTS.md              # Complete prompt usage guide
```

## Integration into Your Project

### Step 1: Add as Dependency

In your main project's `Cargo.toml`:

```toml
[dependencies]
onboard = { path = "./onboard" }
```

Or if it's in a workspace:

```toml
[workspace]
members = ["onboard", "your-main-crate"]

# In your-main-crate/Cargo.toml
[dependencies]
onboard = { path = "../onboard" }
```

### Step 2: Use in Your Code

```rust
use onboard::prompts::*;

fn main() -> anyhow::Result<()> {
    // Use individual prompts
    let name = input::input("What's your name?")
        .placeholder("Developer")
        .interact()?;
    
    let theme = select("Choose theme")
        .item("dark", "Dark", "Night mode")
        .item("light", "Light", "Day mode")
        .interact()?;
    
    println!("Hello, {}! Theme: {}", name, theme);
    Ok(())
}
```

### Step 3: Or Run Complete Onboarding

```rust
use onboard::run_onboarding;

fn main() -> anyhow::Result<()> {
    // Runs all 24 prompts and saves to dx.json
    let config = run_onboarding()?;
    
    println!("User: {}", config.name);
    println!("Email: {}", config.email);
    println!("Theme: {}", config.preferences.theme);
    
    Ok(())
}
```

## Documentation

- **[PROMPTS.md](PROMPTS.md)** - Complete guide to all 24 prompt types with code examples
- **[THEMING.md](THEMING.md)** - How to customize colors and symbols
- **[EXAMPLES.md](EXAMPLES.md)** - Common integration patterns and recipes

## Quick Examples

### Text Input
```rust
let name = onboard::prompts::input::input("Your name?")
    .placeholder("John Doe")
    .interact()?;
```

### Email with Validation
```rust
let email = onboard::prompts::email::email("Your email?")
    .initial_value("user@example.com")
    .interact()?;
```

### Single Selection
```rust
let choice = onboard::prompts::select("Choose one")
    .item("opt1", "Option 1", "First option")
    .item("opt2", "Option 2", "Second option")
    .interact()?;
```

### Multiple Selection
```rust
let choices = onboard::prompts::multiselect("Choose multiple")
    .item("a".to_string(), "Option A".to_string(), "First")
    .item("b".to_string(), "Option B".to_string(), "Second")
    .interact()?;
```

### Boolean Toggle
```rust
let enabled = onboard::prompts::toggle::toggle("Enable feature?")
    .initial_value(true)
    .interact()?;
```

### Progress Bar
```rust
let mut progress = onboard::prompts::progress::ProgressBar::new("Loading", 100);
progress.start()?;
progress.set(50)?;
progress.finish("Done!")?;
```

## Dependencies

This crate requires:
- `owo-colors` - Terminal colors
- `terminal_size` - Terminal dimensions
- `serde` + `toml` - Theme configuration
- `argon2` - Password hashing
- `chrono` - Timestamps
- `anyhow` - Error handling
- `ctrlc` - Signal handling
- `rand` - Random selection

## Terminal Requirements

- 24-bit true color support
- Minimum 80 columns width
- UTF-8 encoding
- ANSI escape sequence support

## Platform Support

- Linux (all major terminals)
- macOS (Terminal.app, iTerm2, etc.)
- Windows (Windows Terminal, ConEmu, etc.)

## License

Same as parent DX project.

## Next Steps

1. Read [PROMPTS.md](PROMPTS.md) for detailed prompt usage
2. Check [THEMING.md](THEMING.md) for customization
3. Browse [EXAMPLES.md](EXAMPLES.md) for integration patterns
4. Explore `src/prompts/` for implementation details
