# Integration Examples

> Practical examples for integrating DX Onboard into your Rust project

## Table of Contents

1. [Basic Integration](#basic-integration)
2. [First-Run Detection](#first-run-detection)
3. [CLI Integration](#cli-integration)
4. [Configuration Management](#configuration-management)
5. [Custom Workflows](#custom-workflows)
6. [Error Handling](#error-handling)
7. [Async Integration](#async-integration)

---

## Basic Integration

### Minimal Example

```rust
// In your main.rs
use onboard::prompts::*;

fn main() -> anyhow::Result<()> {
    let name = input::input("Your name?")
        .placeholder("Developer")
        .interact()?;
    
    let theme = select("Choose theme")
        .item("dark", "Dark", "Night mode")
        .item("light", "Light", "Day mode")
        .interact()?;
    
    println!("Hello, {}! Using {} theme.", name, theme);
    Ok(())
}
```

### Using Complete Onboarding

```rust
use onboard::run_onboarding;

fn main() -> anyhow::Result<()> {
    // Run all 24 prompts
    let config = run_onboarding()?;
    
    // Access collected data
    println!("User: {}", config.name);
    println!("Email: {}", config.email);
    println!("Theme: {}", config.preferences.theme);
    println!("Editor: {}", config.preferences.editor);
    
    // Configuration is saved to dx.json automatically
    
    Ok(())
}
```

---

## First-Run Detection

### Check for Existing Configuration

```rust
use std::path::Path;
use onboard::run_onboarding;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Config {
    name: String,
    email: String,
}

fn main() -> anyhow::Result<()> {
    let config_path = "dx.json";
    
    if !Path::new(config_path).exists() {
        println!("First run detected! Let's set things up...");
        run_onboarding()?;
    } else {
        println!("Welcome back!");
        let json = fs::read_to_string(config_path)?;
        let config: Config = serde_json::from_str(&json)?;
        println!("Hello, {}!", config.name);
    }
    
    Ok(())
}
```

### Force Re-run Setup

```rust
use std::path::Path;
use onboard::run_onboarding;

fn main() -> anyhow::Result<()> {
    let config_path = "dx.json";
    
    if Path::new(config_path).exists() {
        let reconfigure = onboard::prompts::confirm("Configuration exists. Reconfigure?")
            .initial_value(false)
            .interact()?;
        
        if !reconfigure {
            println!("Using existing configuration");
            return Ok(());
        }
    }
    
    run_onboarding()?;
    Ok(())
}
```

---

## CLI Integration

### With Clap

```rust
use clap::{Parser, Subcommand};
use onboard::run_onboarding;

#[derive(Parser)]
#[command(name = "myapp")]
#[command(about = "My awesome application")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the interactive setup wizard
    Setup,
    /// Start the application
    Run,
    /// Show current configuration
    Config,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::Setup) => {
            run_onboarding()?;
        }
        Some(Commands::Run) => {
            // Check if configured
            if !std::path::Path::new("dx.json").exists() {
                eprintln!("Not configured. Run 'myapp setup' first.");
                std::process::exit(1);
            }
            // Run main app
            println!("Running application...");
        }
        Some(Commands::Config) => {
            let json = std::fs::read_to_string("dx.json")?;
            println!("{}", json);
        }
        None => {
            // Default: run app or setup if not configured
            if !std::path::Path::new("dx.json").exists() {
                println!("First run! Starting setup...");
                run_onboarding()?;
            } else {
                println!("Running application...");
            }
        }
    }
    
    Ok(())
}
```

### With Environment Variables

```rust
use std::env;
use onboard::run_onboarding;

fn main() -> anyhow::Result<()> {
    // Skip onboarding in CI
    if env::var("CI").is_ok() {
        println!("CI detected, using default configuration");
        return Ok(());
    }
    
    // Force onboarding if FORCE_SETUP is set
    if env::var("FORCE_SETUP").is_ok() {
        run_onboarding()?;
        return Ok(());
    }
    
    // Normal flow
    if !std::path::Path::new("dx.json").exists() {
        run_onboarding()?;
    }
    
    Ok(())
}
```

---

## Configuration Management

### Load and Use Configuration

```rust
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct AppConfig {
    name: String,
    email: String,
    preferences: Preferences,
}

#[derive(Deserialize)]
struct Preferences {
    theme: String,
    editor: String,
    notifications: bool,
}

fn load_config() -> anyhow::Result<AppConfig> {
    let json = fs::read_to_string("dx.json")?;
    let config: AppConfig = serde_json::from_str(&json)?;
    Ok(config)
}

fn main() -> anyhow::Result<()> {
    let config = load_config()?;
    
    // Apply theme
    match config.preferences.theme.as_str() {
        "dark" => apply_dark_theme(),
        "light" => apply_light_theme(),
        _ => apply_default_theme(),
    }
    
    // Configure editor integration
    setup_editor(&config.preferences.editor)?;
    
    // Enable/disable notifications
    if config.preferences.notifications {
        enable_notifications();
    }
    
    println!("Welcome, {}!", config.name);
    Ok(())
}

fn apply_dark_theme() { /* ... */ }
fn apply_light_theme() { /* ... */ }
fn apply_default_theme() { /* ... */ }
fn setup_editor(_editor: &str) -> anyhow::Result<()> { Ok(()) }
fn enable_notifications() { /* ... */ }
```

### Update Configuration

```rust
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Serialize)]
struct Config {
    name: String,
    email: String,
    theme: String,
}

fn update_config(key: &str, value: &str) -> anyhow::Result<()> {
    let json = fs::read_to_string("dx.json")?;
    let mut config: Config = serde_json::from_str(&json)?;
    
    match key {
        "name" => config.name = value.to_string(),
        "email" => config.email = value.to_string(),
        "theme" => config.theme = value.to_string(),
        _ => return Err(anyhow::anyhow!("Unknown key: {}", key)),
    }
    
    let json = serde_json::to_string_pretty(&config)?;
    fs::write("dx.json", json)?;
    
    Ok(())
}

fn main() -> anyhow::Result<()> {
    // Update theme
    update_config("theme", "dark")?;
    println!("Theme updated to dark");
    Ok(())
}
```

---

## Custom Workflows

### Multi-Step Setup

```rust
use onboard::prompts::*;

fn setup_user_info() -> anyhow::Result<(String, String)> {
    section_with_width("Step 1: User Information", 80, |lines| {
        lines.push("Let's start with your basic information".to_string());
    })?;
    
    let name = input::input("Your name?")
        .placeholder("John Doe")
        .interact()?;
    
    let email = email::email("Your email?")
        .interact()?;
    
    Ok((name, email))
}

fn setup_preferences() -> anyhow::Result<(String, bool)> {
    section_with_width("Step 2: Preferences", 80, |lines| {
        lines.push("Configure your preferences".to_string());
    })?;
    
    let theme = select("Choose theme")
        .item("dark", "Dark", "Night mode")
        .item("light", "Light", "Day mode")
        .interact()?;
    
    let notifications = toggle::toggle("Enable notifications?")
        .initial_value(true)
        .interact()?;
    
    Ok((theme, notifications))
}

fn confirm_setup(name: &str, email: &str, theme: &str, notifications: bool) -> anyhow::Result<bool> {
    section_with_width("Step 3: Review", 80, |lines| {
        lines.push(format!("Name: {}", name));
        lines.push(format!("Email: {}", email));
        lines.push(format!("Theme: {}", theme));
        lines.push(format!("Notifications: {}", if notifications { "ON" } else { "OFF" }));
    })?;
    
    confirm("Proceed with these settings?")
        .initial_value(true)
        .interact()
}

fn main() -> anyhow::Result<()> {
    intro("Welcome to My App Setup!")?;
    
    let (name, email) = setup_user_info()?;
    let (theme, notifications) = setup_preferences()?;
    
    if confirm_setup(&name, &email, &theme, notifications)? {
        log::success("Setup complete!".to_string())?;
        outro("Thanks for setting up My App!")?;
    } else {
        log::warning("Setup cancelled".to_string())?;
    }
    
    Ok(())
}
```

### Conditional Prompts

```rust
use onboard::prompts::*;

fn main() -> anyhow::Result<()> {
    let project_type = select("Project type?")
        .item("web", "Web Application", "")
        .item("cli", "CLI Tool", "")
        .item("library", "Library", "")
        .interact()?;
    
    match project_type.as_str() {
        "web" => {
            let framework = select("Web framework?")
                .item("react", "React", "")
                .item("vue", "Vue", "")
                .item("svelte", "Svelte", "")
                .interact()?;
            
            let use_typescript = confirm("Use TypeScript?")
                .initial_value(true)
                .interact()?;
            
            println!("Creating {} web app with TypeScript: {}", framework, use_typescript);
        }
        "cli" => {
            let use_clap = confirm("Use clap for CLI parsing?")
                .initial_value(true)
                .interact()?;
            
            println!("Creating CLI tool with clap: {}", use_clap);
        }
        "library" => {
            let visibility = select("Library visibility?")
                .item("public", "Public (crates.io)", "")
                .item("private", "Private", "")
                .interact()?;
            
            println!("Creating {} library", visibility);
        }
        _ => {}
    }
    
    Ok(())
}
```

---

## Error Handling

### Graceful Cancellation

```rust
use onboard::run_onboarding;

fn main() -> anyhow::Result<()> {
    match run_onboarding() {
        Ok(config) => {
            println!("Setup complete!");
            println!("Welcome, {}!", config.name);
        }
        Err(e) if e.to_string().contains("cancelled") => {
            println!("Setup cancelled. You can run setup again later.");
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("Setup error: {}", e);
            std::process::exit(1);
        }
    }
    
    Ok(())
}
```

### Retry on Error

```rust
use onboard::prompts::*;

fn get_valid_email() -> anyhow::Result<String> {
    loop {
        let email = email::email("Your email?").interact()?;
        
        // Additional validation
        if email.ends_with("@company.com") {
            return Ok(email);
        }
        
        log::warning("Please use your company email (@company.com)".to_string())?;
        
        let retry = confirm("Try again?")
            .initial_value(true)
            .interact()?;
        
        if !retry {
            return Err(anyhow::anyhow!("Email validation failed"));
        }
    }
}

fn main() -> anyhow::Result<()> {
    let email = get_valid_email()?;
    println!("Email: {}", email);
    Ok(())
}
```

### Fallback to Defaults

```rust
use onboard::run_onboarding;

fn create_default_config() -> Config {
    Config {
        name: "User".to_string(),
        email: "user@example.com".to_string(),
        theme: "dark".to_string(),
    }
}

struct Config {
    name: String,
    email: String,
    theme: String,
}

fn main() -> anyhow::Result<()> {
    let config = match run_onboarding() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Onboarding failed: {}", e);
            eprintln!("Using default configuration");
            return Ok(()); // Continue with defaults
        }
    };
    
    println!("User: {}", config.name);
    Ok(())
}
```

---

## Async Integration

### With Tokio

```rust
use tokio;
use onboard::run_onboarding;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Run onboarding in blocking task
    let config = tokio::task::spawn_blocking(|| {
        run_onboarding()
    }).await??;
    
    // Continue with async code
    println!("User: {}", config.name);
    
    // Start async services
    start_services(config).await?;
    
    Ok(())
}

async fn start_services(config: onboard::OnboardingResult) -> anyhow::Result<()> {
    println!("Starting services for {}...", config.name);
    // Your async code here
    Ok(())
}
```

### Background Service

```rust
use tokio;
use onboard::run_onboarding;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Run onboarding first
    let config = tokio::task::spawn_blocking(|| {
        run_onboarding()
    }).await??;
    
    // Start background service
    let service_handle = tokio::spawn(async move {
        loop {
            // Service logic
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });
    
    // Wait for service
    service_handle.await?;
    
    Ok(())
}
```

---

## Advanced Patterns

### Progress with Async Tasks

```rust
use onboard::prompts::progress::ProgressBar;
use tokio;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut progress = ProgressBar::new("Installing", 100);
    progress.start()?;
    
    for i in 0..=100 {
        progress.set(i)?;
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    }
    
    progress.finish("Complete!")?;
    Ok(())
}
```

### Dynamic Prompt Generation

```rust
use onboard::prompts::*;

fn main() -> anyhow::Result<()> {
    let features = vec!["auth", "database", "api", "ui"];
    
    let mut multiselect_prompt = multiselect("Select features");
    for feature in features {
        multiselect_prompt = multiselect_prompt.item(
            feature.to_string(),
            feature.to_uppercase(),
            format!("Enable {} feature", feature).as_str(),
        );
    }
    
    let selected = multiselect_prompt.interact()?;
    println!("Selected features: {:?}", selected);
    
    Ok(())
}
```

### Nested Workflows

```rust
use onboard::prompts::*;

fn configure_database() -> anyhow::Result<()> {
    let db_type = select("Database type?")
        .item("postgres", "PostgreSQL", "")
        .item("mysql", "MySQL", "")
        .item("sqlite", "SQLite", "")
        .interact()?;
    
    let host = input::input("Database host?")
        .placeholder("localhost")
        .interact()?;
    
    println!("Configured {} at {}", db_type, host);
    Ok(())
}

fn configure_auth() -> anyhow::Result<()> {
    let auth_type = select("Authentication?")
        .item("jwt", "JWT", "")
        .item("session", "Session", "")
        .interact()?;
    
    println!("Configured {} auth", auth_type);
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let features = multiselect("Select features")
        .item("database".to_string(), "Database".to_string(), "")
        .item("auth".to_string(), "Authentication".to_string(), "")
        .interact()?;
    
    for feature in features {
        match feature.as_str() {
            "database" => configure_database()?,
            "auth" => configure_auth()?,
            _ => {}
        }
    }
    
    Ok(())
}
```

---

## Testing

### Mock Configuration for Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    fn mock_config() -> onboard::OnboardingResult {
        onboard::OnboardingResult {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            website: "https://test.com".to_string(),
            phone: "+1234567890".to_string(),
            bio: "Test bio".to_string(),
            experience_years: 5,
            satisfaction_rating: 4,
            productivity_level: 75,
            work_hours: (9, 17),
            programming_languages: vec!["rust".to_string()],
            favorite_language: "rust".to_string(),
            framework: "react".to_string(),
            project_type: "web".to_string(),
            selected_skills: vec!["frontend".to_string()],
            runtime_environment: "real_os".to_string(),
            selected_components: vec!["tui".to_string()],
            selected_providers: vec!["openai".to_string()],
            preferences: onboard::OnboardingPreferences {
                theme: "dark".to_string(),
                editor: "vscode".to_string(),
                shell: "zsh".to_string(),
                notifications: true,
                auto_updates: false,
                telemetry: false,
            },
            wizard_completed_steps: 3,
            timestamp: "2026-03-19T10:00:00Z".to_string(),
        }
    }
    
    #[test]
    fn test_config_processing() {
        let config = mock_config();
        assert_eq!(config.name, "Test User");
        assert_eq!(config.preferences.theme, "dark");
    }
}
```

---

## Next Steps

- Explore [PROMPTS.md](PROMPTS.md) for detailed prompt documentation
- Check [THEMING.md](THEMING.md) for customization options
- Browse `src/prompts/` for implementation details
