# Complete Prompt Guide

> Detailed usage guide for all 24 prompt types in DX Onboard

## Table of Contents

1. [Text Input Prompts](#text-input-prompts)
2. [Numeric Prompts](#numeric-prompts)
3. [Selection Prompts](#selection-prompts)
4. [Boolean Prompts](#boolean-prompts)
5. [Progress Indicators](#progress-indicators)
6. [Workflow Prompts](#workflow-prompts)
7. [Logging & Layout](#logging--layout)

---

## Text Input Prompts

### 1. Basic Input

Simple text input with optional placeholder.

```rust
use onboard::prompts::input;

let name = input::input("What's your name?")
    .placeholder("John Doe")
    .interact()?;

println!("Name: {}", name);
```

**Methods:**
- `.placeholder(text: &str)` - Set placeholder text
- `.initial_value(value: &str)` - Set initial value
- `.interact() -> Result<String>` - Show prompt and get input

### 2. Email Input

Email input with automatic validation.

```rust
use onboard::prompts::email;

let email = email::email("What's your email?")
    .initial_value("user@example.com")
    .interact()?;

println!("Email: {}", email);
```

**Methods:**
- `.initial_value(email: &str)` - Set initial email
- `.interact() -> Result<String>` - Show prompt and get validated email

**Validation:** Checks for basic email format (contains @ and .)

### 3. Password Input

Secure password input (characters are masked).

```rust
use onboard::prompts::password;

let password = password::password("Enter password")
    .interact()?;

// Hash it with Argon2
use argon2::{Argon2, password_hash::{PasswordHasher, SaltString}};
use rand::thread_rng;

let salt = SaltString::generate(&mut thread_rng());
let hash = Argon2::default()
    .hash_password(password.as_bytes(), &salt)?
    .to_string();

println!("Password hashed successfully");
```

**Methods:**
- `.interact() -> Result<String>` - Show prompt and get password

**Security:** Input is masked with asterisks

### 4. URL Input

URL input with validation.

```rust
use onboard::prompts::url;

let website = url::url("Your website?")
    .interact()?;

println!("Website: {}", website);
```

**Methods:**
- `.interact() -> Result<String>` - Show prompt and get validated URL

**Validation:** Checks for http:// or https:// prefix

### 5. Phone Input

Phone number input.

```rust
use onboard::prompts::phone_input;

let phone = phone_input::phone_input("Phone number?")
    .interact()?;

println!("Phone: {}", phone);
```

**Methods:**
- `.interact() -> Result<String>` - Show prompt and get phone number

### 6. Multi-line Text

Multi-line text area for longer input.

```rust
use onboard::prompts::text;

let bio = text::text("Tell us about yourself")
    .placeholder("Write your bio...")
    .interact()?;

println!("Bio ({} chars): {}", bio.len(), bio);
```

**Methods:**
- `.placeholder(text: &str)` - Set placeholder
- `.interact() -> Result<String>` - Show prompt and get text

### 7. Tags Input

Comma-separated tag input.

```rust
use onboard::prompts::tags;

let languages = tags::tags("Programming languages?")
    .placeholder("rust, python, javascript")
    .interact()?;

println!("Languages: {:?}", languages);
// Output: Languages: ["rust", "python", "javascript"]
```

**Methods:**
- `.placeholder(text: &str)` - Set placeholder
- `.interact() -> Result<Vec<String>>` - Show prompt and get tags

**Format:** User types comma-separated values

---

## Numeric Prompts

### 8. Number Input

Integer input with min/max constraints.

```rust
use onboard::prompts::number;

let age = number::number("Your age?")
    .min(0)
    .max(120)
    .interact()?;

println!("Age: {}", age);
```

**Methods:**
- `.min(value: i64)` - Set minimum value
- `.max(value: i64)` - Set maximum value
- `.interact() -> Result<i64>` - Show prompt and get number

### 9. Rating

Star rating selector (1-5 stars by default).

```rust
use onboard::prompts::rating;

let satisfaction = rating::rating("Rate your experience")
    .max(5)
    .interact()?;

println!("Rating: {}/5 stars", satisfaction);
```

**Methods:**
- `.max(stars: usize)` - Set maximum stars
- `.interact() -> Result<usize>` - Show prompt and get rating

### 10. Slider

Single value slider.

```rust
use onboard::prompts::slider;

let volume = slider::slider("Volume level", 0, 100)
    .initial_value(75)
    .interact()?;

println!("Volume: {}%", volume);
```

**Methods:**
- `.initial_value(value: i64)` - Set initial position
- `.interact() -> Result<i64>` - Show prompt and get value

**Parameters:**
- `message: &str` - Prompt message
- `min: i64` - Minimum value
- `max: i64` - Maximum value

### 11. Range Slider

Select a range (start and end values).

```rust
use onboard::prompts::range_slider;

let (start, end) = range_slider::range_slider("Work hours", 0, 24)
    .initial_range(9, 17)
    .interact()?;

println!("Work hours: {}:00 - {}:00", start, end);
```

**Methods:**
- `.initial_range(start: i64, end: i64)` - Set initial range
- `.interact() -> Result<(i64, i64)>` - Show prompt and get range

**Parameters:**
- `message: &str` - Prompt message
- `min: i64` - Minimum value
- `max: i64` - Maximum value

---

## Selection Prompts

### 12. Single Select

Choose one option from a list.

```rust
use onboard::prompts::select;

let theme = select("Choose theme")
    .item("dark", "Dark Theme", "Perfect for night coding")
    .item("light", "Light Theme", "Easy on the eyes")
    .item("auto", "Auto Theme", "Follows system")
    .interact()?;

println!("Selected: {}", theme);
```

**Methods:**
- `.item(value: &str, label: &str, hint: &str)` - Add option
- `.interact() -> Result<String>` - Show prompt and get selection

**Navigation:** Use arrow keys to navigate, Enter to select

### 13. Multi Select

Choose multiple options from a list.

```rust
use onboard::prompts::multiselect;

let languages = multiselect("Select languages")
    .item("rust".to_string(), "Rust".to_string(), "Systems programming")
    .item("python".to_string(), "Python".to_string(), "Scripting")
    .item("javascript".to_string(), "JavaScript".to_string(), "Web development")
    .required(false)
    .interact()?;

println!("Selected: {:?}", languages);
```

**Methods:**
- `.item(value: String, label: String, hint: &str)` - Add option
- `.required(bool)` - Require at least one selection
- `.interact() -> Result<Vec<String>>` - Show prompt and get selections

**Navigation:** Use arrow keys, Space to toggle, Enter to confirm

### 14. Autocomplete

Searchable autocomplete prompt.

```rust
use onboard::prompts::autocomplete;

let language = autocomplete::autocomplete("Favorite language?")
    .item("rust", "Rust")
    .item("python", "Python")
    .item("javascript", "JavaScript")
    .item("typescript", "TypeScript")
    .item("go", "Go")
    .interact()?;

println!("Favorite: {}", language);
```

**Methods:**
- `.item(value: &str, label: &str)` - Add option
- `.interact() -> Result<String>` - Show prompt and get selection

**Usage:** Start typing to filter options

### 15. Search Filter

Filtered search with tags.

```rust
use onboard::prompts::search_filter;

let framework = search_filter::search_filter("Choose framework")
    .item("react", "React", vec!["frontend".to_string(), "javascript".to_string()])
    .item("vue", "Vue.js", vec!["frontend".to_string(), "javascript".to_string()])
    .item("angular", "Angular", vec!["frontend".to_string(), "typescript".to_string()])
    .item("express", "Express.js", vec!["backend".to_string(), "javascript".to_string()])
    .interact()?;

println!("Framework: {}", framework);
```

**Methods:**
- `.item(value: &str, label: &str, tags: Vec<String>)` - Add option with tags
- `.interact() -> Result<String>` - Show prompt and get selection

**Usage:** Type to search by name or tags

---

## Boolean Prompts

### 16. Toggle

On/off toggle switch.

```rust
use onboard::prompts::toggle;

let notifications = toggle::toggle("Enable notifications?")
    .initial_value(true)
    .interact()?;

println!("Notifications: {}", if notifications { "ON" } else { "OFF" });
```

**Methods:**
- `.initial_value(bool)` - Set initial state
- `.interact() -> Result<bool>` - Show prompt and get value

### 17. Confirm

Yes/No confirmation.

```rust
use onboard::prompts::confirm;

let proceed = confirm("Continue with setup?")
    .initial_value(true)
    .interact()?;

if proceed {
    println!("Proceeding...");
} else {
    println!("Cancelled");
}
```

**Methods:**
- `.initial_value(bool)` - Set initial value (default: false)
- `.interact() -> Result<bool>` - Show prompt and get confirmation

---

## Progress Indicators

### 18. Progress Bar

Visual progress indicator.

```rust
use onboard::prompts::progress::ProgressBar;
use std::thread;
use std::time::Duration;

let mut progress = ProgressBar::new("Installing dependencies", 100);
progress.start()?;

for i in 0..=100 {
    progress.set(i)?;
    if i == 25 {
        progress.set_message("Downloading packages...")?;
    } else if i == 50 {
        progress.set_message("Extracting files...")?;
    } else if i == 75 {
        progress.set_message("Finalizing...")?;
    }
    thread::sleep(Duration::from_millis(50));
}

progress.finish("Installation complete!")?;
```

**Methods:**
- `.start() -> Result<()>` - Start showing progress
- `.set(value: usize) -> Result<()>` - Update progress
- `.set_message(msg: &str) -> Result<()>` - Update message
- `.finish(msg: &str) -> Result<()>` - Complete progress

**Parameters:**
- `message: &str` - Initial message
- `total: usize` - Total steps

### 19. Spinner

Loading spinner animation.

```rust
use onboard::prompts::spinner;
use std::thread;
use std::time::Duration;

let mut spinner = spinner::spinner("Processing...");
spinner.start()?;

// Do work
thread::sleep(Duration::from_secs(2));

spinner.stop("Processing complete!")?;
```

**Methods:**
- `.start() -> Result<()>` - Start spinner
- `.stop(message: &str) -> Result<()>` - Stop spinner with message

---

## Workflow Prompts

### 20. Wizard

Multi-step wizard process.

```rust
use onboard::prompts::wizard;

let completed_steps = wizard::wizard("Project Setup")
    .step("Basic Info", "Enter project details")
    .step("Configuration", "Configure settings")
    .step("Review", "Review and confirm")
    .interact()?;

println!("Completed {} steps", completed_steps);
```

**Methods:**
- `.step(name: &str, description: &str)` - Add step
- `.interact() -> Result<usize>` - Show wizard and get completed steps

---

## Logging & Layout

### 21. Logging Functions

Display formatted messages.

```rust
use onboard::prompts::log;

log::info("Starting process...".to_string())?;
log::success("Task completed!".to_string())?;
log::warning("Using default config".to_string())?;
log::error("Connection failed".to_string())?;
log::step("Installing dependencies")?;
```

**Functions:**
- `log::info(message: String) -> Result<()>` - Info message (dim color)
- `log::success(message: String) -> Result<()>` - Success message (green)
- `log::warning(message: String) -> Result<()>` - Warning message (yellow)
- `log::error(message: String) -> Result<()>` - Error message (red)
- `log::step(message: &str) -> Result<()>` - Step message (with bullet)

### 22. Intro/Outro

Welcome and farewell messages.

```rust
use onboard::prompts::{intro, outro};

intro("Welcome to DX Setup!")?;

// ... do onboarding ...

outro("Setup complete! Thanks for using DX.")?;
```

**Functions:**
- `intro(message: &str) -> Result<()>` - Display intro message
- `outro(message: &str) -> Result<()>` - Display outro message

### 23. Section

Formatted text section with border.

```rust
use onboard::prompts::section_with_width;

section_with_width("Summary", 80, |lines| {
    lines.push("Name: John Doe".to_string());
    lines.push("Email: john@example.com".to_string());
    lines.push("Theme: Dark".to_string());
    lines.push("".to_string());
    lines.push("Setup complete!".to_string());
})?;
```

**Function:**
- `section_with_width(title: &str, width: usize, content: F) -> Result<()>`
  - `title` - Section title
  - `width` - Section width in characters
  - `content` - Closure that adds lines to the section

### 24. Train Animation

Animated ASCII train (shown on exit).

```rust
use onboard::splash::render_train_animation;
use onboard::effects::RainbowEffect;
use std::thread;
use std::time::Duration;

let rainbow = RainbowEffect::new();

print!("\x1B[2J\x1B[H"); // Clear screen

for frame in 0..15 {
    print!("\x1B[H"); // Move cursor to top
    render_train_animation(&rainbow, frame)?;
    thread::sleep(Duration::from_millis(200));
}
```

**Function:**
- `render_train_animation(rainbow: &RainbowEffect, frame: usize) -> io::Result<()>`
  - `rainbow` - Rainbow effect for colors
  - `frame` - Animation frame number

---

## Common Patterns

### Pattern 1: Conditional Prompts

```rust
let use_advanced = confirm("Use advanced settings?")
    .initial_value(false)
    .interact()?;

if use_advanced {
    let timeout = number::number("Timeout (seconds)?")
        .min(1)
        .max(300)
        .interact()?;
    
    let retry = toggle::toggle("Enable retry?")
        .initial_value(true)
        .interact()?;
}
```

### Pattern 2: Validation Loop

```rust
loop {
    let email = email::email("Your email?").interact()?;
    
    if email.ends_with("@company.com") {
        break;
    }
    
    log::warning("Please use your company email".to_string())?;
}
```

### Pattern 3: Progress with Steps

```rust
let steps = vec!["Download", "Extract", "Install", "Configure"];
let mut progress = ProgressBar::new("Setup", steps.len());
progress.start()?;

for (i, step) in steps.iter().enumerate() {
    progress.set_message(step)?;
    // Do work
    thread::sleep(Duration::from_secs(1));
    progress.set(i + 1)?;
}

progress.finish("Setup complete!")?;
```

### Pattern 4: Multi-step Workflow

```rust
intro("Welcome to DX Setup!")?;

// Step 1: Basic info
section_with_width("Step 1: Basic Information", 80, |lines| {
    lines.push("Let's start with some basic information".to_string());
})?;

let name = input::input("Your name?").interact()?;
let email = email::email("Your email?").interact()?;

// Step 2: Preferences
section_with_width("Step 2: Preferences", 80, |lines| {
    lines.push("Configure your preferences".to_string());
})?;

let theme = select("Theme?")
    .item("dark", "Dark", "")
    .item("light", "Light", "")
    .interact()?;

// Step 3: Confirmation
section_with_width("Step 3: Review", 80, |lines| {
    lines.push(format!("Name: {}", name));
    lines.push(format!("Email: {}", email));
    lines.push(format!("Theme: {}", theme));
})?;

let proceed = confirm("Proceed with setup?")
    .initial_value(true)
    .interact()?;

if proceed {
    outro("Setup complete!")?;
} else {
    log::warning("Setup cancelled".to_string())?;
}
```

---

## Error Handling

All prompts return `Result<T, anyhow::Error>`. Handle errors appropriately:

```rust
use anyhow::Result;

fn setup() -> Result<()> {
    let name = input::input("Name?").interact()?;
    
    let email = match email::email("Email?").interact() {
        Ok(e) => e,
        Err(e) if e.to_string().contains("cancelled") => {
            log::warning("Setup cancelled".to_string())?;
            return Ok(());
        }
        Err(e) => return Err(e),
    };
    
    Ok(())
}
```

## Theme Customization

All prompts use the theme system. See [THEMING.md](THEMING.md) for customization.

## Next Steps

- Check [THEMING.md](THEMING.md) for color customization
- Browse [EXAMPLES.md](EXAMPLES.md) for integration patterns
- Explore `src/prompts/` for implementation details
