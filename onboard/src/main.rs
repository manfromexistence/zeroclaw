//! DX Onboarding - Interactive Setup Experience
#![allow(dead_code)]

mod effects;
mod prompts;
mod splash;

use anyhow::Result;
use argon2::Argon2;
use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::Local;
use rand::thread_rng;
use serde::Serialize;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

use effects::RainbowEffect;
use prompts::PromptInteraction;
use splash::{render_dx_logo, render_train_animation};

#[derive(Debug, Clone, Copy)]
enum RuntimeEnvironment {
    RealOs,
    Vps,
    Container,
    Restricted,
}

impl RuntimeEnvironment {
    fn as_str(self) -> &'static str {
        match self {
            RuntimeEnvironment::RealOs => "real_os",
            RuntimeEnvironment::Vps => "vps",
            RuntimeEnvironment::Container => "container",
            RuntimeEnvironment::Restricted => "restricted",
        }
    }

    fn label(self) -> &'static str {
        match self {
            RuntimeEnvironment::RealOs => "Real OS workstation",
            RuntimeEnvironment::Vps => "VPS / Cloud VM",
            RuntimeEnvironment::Container => "Docker / Container",
            RuntimeEnvironment::Restricted => "Restricted / CI runner",
        }
    }

    fn hint(self) -> &'static str {
        match self {
            RuntimeEnvironment::RealOs => "Best for desktop app + extension onboarding",
            RuntimeEnvironment::Vps => "Best for remote gateway + channel bridge",
            RuntimeEnvironment::Container => "Best for ephemeral test/deploy environments",
            RuntimeEnvironment::Restricted => "Best for non-interactive automation",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
struct OnboardingResult {
    // Basic Info
    name: String,
    email: String,
    website: String,
    phone: String,
    bio: String,

    // Experience & Skills
    experience_years: i64,
    satisfaction_rating: usize,
    productivity_level: i64,
    work_hours: (i64, i64),
    programming_languages: Vec<String>,
    favorite_language: String,
    framework: String,
    project_type: String,
    selected_skills: Vec<String>,

    // System & Environment
    runtime_environment: String,
    selected_components: Vec<String>,
    selected_providers: Vec<String>,

    // Preferences
    preferences: OnboardingPreferences,

    // Workflow Data
    wizard_completed_steps: usize,

    // Metadata
    timestamp: String,
}

#[derive(Debug, Clone, Serialize)]
struct OnboardingPreferences {
    theme: String,
    editor: String,
    shell: String,
    notifications: bool,
    auto_updates: bool,
    telemetry: bool,
}

fn detect_runtime_environment() -> RuntimeEnvironment {
    let ci = env::var("CI")
        .map(|value| {
            let normalized = value.to_ascii_lowercase();
            normalized == "1" || normalized == "true"
        })
        .unwrap_or(false);
    if ci {
        return RuntimeEnvironment::Restricted;
    }

    let container_detected = Path::new("/.dockerenv").exists()
        || env::var("KUBERNETES_SERVICE_HOST").is_ok()
        || env::var("DOCKER_CONTAINER").is_ok()
        || fs::read_to_string("/proc/1/cgroup")
            .map(|content| {
                let lowered = content.to_ascii_lowercase();
                lowered.contains("docker")
                    || lowered.contains("containerd")
                    || lowered.contains("kubepods")
                    || lowered.contains("podman")
            })
            .unwrap_or(false);
    if container_detected {
        return RuntimeEnvironment::Container;
    }

    let cloud_hint = env::var("VERCEL")
        .or_else(|_| env::var("RAILWAY_ENVIRONMENT"))
        .or_else(|_| env::var("FLY_APP_NAME"))
        .or_else(|_| env::var("HEROKU_APP_NAME"))
        .or_else(|_| env::var("DIGITALOCEAN_APP_ID"))
        .or_else(|_| env::var("AWS_EXECUTION_ENV"))
        .or_else(|_| env::var("GCP_PROJECT"))
        .or_else(|_| env::var("AZURE_HTTP_USER_AGENT"))
        .is_ok();
    let virtualization_hint =
        Path::new("/proc/vz").exists() || Path::new("/proc/user_beancounters").exists();

    if cloud_hint || virtualization_hint {
        return RuntimeEnvironment::Vps;
    }

    RuntimeEnvironment::RealOs
}

fn find_workspace_root() -> PathBuf {
    let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    for ancestor in cwd.ancestors() {
        let cargo_toml = ancestor.join("Cargo.toml");
        if cargo_toml.exists()
            && let Ok(content) = fs::read_to_string(&cargo_toml)
            && content.contains("[workspace]")
        {
            return ancestor.to_path_buf();
        }
    }
    cwd
}

fn build_component_targets(runtime: RuntimeEnvironment) -> Vec<String> {
    match runtime {
        RuntimeEnvironment::RealOs => vec![
            "desktop_app".to_string(),
            "tui".to_string(),
            "ide_extension".to_string(),
            "browser_extension".to_string(),
            "local_website".to_string(),
        ],
        RuntimeEnvironment::Vps
        | RuntimeEnvironment::Container
        | RuntimeEnvironment::Restricted => {
            vec!["tui".to_string(), "local_website".to_string()]
        }
    }
}

fn hash_password(password: &str) -> Result<String> {
    let salt = argon2::password_hash::SaltString::generate(&mut thread_rng());
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|err| anyhow::anyhow!("password hashing failed: {}", err))?;
    Ok(hash.to_string())
}

fn verify_password(password: &str, password_hash: &str) -> bool {
    let parsed = match PasswordHash::new(password_hash) {
        Ok(value) => value,
        Err(_) => return false,
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok()
}

fn run_onboarding() -> Result<OnboardingResult> {
    // Initialize rainbow effect
    let rainbow = RainbowEffect::new();

    // Clear screen and show DX logo with random font
    print!("\x1B[2J\x1B[H"); // Clear screen and move cursor to top
    render_dx_logo(&rainbow)?;
    println!();
    thread::sleep(Duration::from_millis(1000));

    // Welcome
    prompts::intro("🚀 DX Onboarding - Complete Prompts Showcase")?;
    prompts::section_with_width("Welcome to DX", 80, |lines| {
        lines.push("This onboarding showcases ALL available prompt types!".to_string());
        lines.push(format!(
            "Detected runtime: {}",
            detect_runtime_environment().label()
        ));
        lines.push(format!(
            "Runtime hint: {}",
            detect_runtime_environment().hint()
        ));
        lines.push("".to_string());
        lines.push("Let's explore every single prompt component available.".to_string());
    })?;

    let runtime_env = detect_runtime_environment();
    prompts::log::info(format!("Runtime Environment: {}", runtime_env.label()))?;
    prompts::log::info(format!(
        "Workspace root: {}",
        find_workspace_root().display()
    ))?;

    // 1. Basic Input
    let name = prompts::input::input("What's your name?")
        .placeholder("Developer")
        .interact()?;

    // 2. Email Input with validation
    let email = prompts::email::email("What's your email?")
        .initial_value("dev@example.com")
        .interact()?;

    prompts::log::success(format!("Hello, {}! ({})", name, email))?;

    // 3. Password Input
    let use_password = prompts::confirm("Would you like to set up a password?")
        .initial_value(true)
        .interact()?;

    if use_password {
        let password = prompts::password::password("Enter a password").interact()?;

        match hash_password(&password) {
            Ok(hash) => {
                prompts::log::success("Password hashed successfully")?;
                if verify_password(&password, &hash) {
                    prompts::log::success("Password verification working")?;
                } else {
                    prompts::log::warning("Password verification failed")?;
                }
            }
            Err(e) => prompts::log::warning(format!("Password hashing failed: {}", e))?,
        }
    }

    // 4. URL Input
    let website = prompts::url::url("What's your website or portfolio URL?").interact()?;
    prompts::log::info(format!("Website: {}", website))?;

    // 5. Phone Input
    let phone = prompts::phone_input::phone_input("What's your phone number?").interact()?;
    prompts::log::info(format!("Phone: {}", phone))?;

    // 6. Number Input
    let experience_years = prompts::number::number("How many years of coding experience?")
        .min(0)
        .max(50)
        .interact()?;
    prompts::log::info(format!("Experience: {} years", experience_years))?;

    // 7. Rating
    let satisfaction = prompts::rating::rating("Rate your current dev setup satisfaction")
        .max(5)
        .interact()?;
    prompts::log::info(format!(
        "Current setup satisfaction: {}/5 stars",
        satisfaction
    ))?;

    // 8. Slider
    let productivity = prompts::slider::slider("Rate your productivity level (0-100)", 0, 100)
        .initial_value(75)
        .interact()?;
    prompts::log::info(format!("Productivity level: {}%", productivity))?;

    // 9. Range Slider
    let work_hours = prompts::range_slider::range_slider("Select your preferred work hours", 0, 24)
        .initial_range(9, 17)
        .interact()?;
    prompts::log::info(format!(
        "Work hours: {}:00 - {}:00",
        work_hours.0, work_hours.1
    ))?;

    // 10. Toggle switches
    let notifications = prompts::toggle::toggle("Enable desktop notifications")
        .initial_value(true)
        .interact()?;

    let auto_updates = prompts::toggle::toggle("Enable automatic updates")
        .initial_value(false)
        .interact()?;

    let telemetry = prompts::toggle::toggle("Share anonymous usage data")
        .initial_value(false)
        .interact()?;

    // 11. Single Select
    let theme = prompts::select("Choose your preferred theme")
        .item("dark", "Dark Theme", "Perfect for late-night coding")
        .item("light", "Light Theme", "Easy on the eyes during the day")
        .item("auto", "Auto Theme", "Follows system preference")
        .item("cyberpunk", "Cyberpunk", "Neon colors and futuristic vibes")
        .interact()?;
    prompts::log::info(format!("Selected theme: {}", theme))?;

    // 12. Editor preference
    let editor = prompts::select("What's your preferred code editor?")
        .item("vscode", "Visual Studio Code", "Most popular choice")
        .item("neovim", "Neovim", "Modal editing powerhouse")
        .item("vim", "Vim", "The classic")
        .item("emacs", "Emacs", "Extensible and customizable")
        .item("sublime", "Sublime Text", "Fast and lightweight")
        .item("atom", "Atom", "Hackable text editor")
        .interact()?;

    // 13. Shell preference
    let shell = prompts::select("What's your preferred shell?")
        .item("bash", "Bash", "The standard shell")
        .item("zsh", "Zsh", "Feature-rich with great plugins")
        .item("fish", "Fish", "User-friendly with smart defaults")
        .item("powershell", "PowerShell", "Cross-platform automation")
        .item("cmd", "Command Prompt", "Windows classic")
        .interact()?;

    // 14. Multi-select for components
    let components = build_component_targets(runtime_env);
    let mut component_multiselect =
        prompts::multiselect("Which components would you like to install?");
    for component in &components {
        component_multiselect =
            component_multiselect.item(component.clone(), component.clone(), "Available component");
    }
    let selected_components = component_multiselect.interact()?;

    if !selected_components.is_empty() {
        prompts::log::info("Selected components:")?;
        for component in &selected_components {
            prompts::log::step(component)?;
        }
    }

    // 15. Multi-select for providers
    let providers = vec![
        ("openai", "OpenAI"),
        ("anthropic", "Anthropic"),
        ("google", "Google Gemini"),
        ("github_copilot", "GitHub Copilot"),
        ("ollama", "Ollama (Local)"),
        ("huggingface", "Hugging Face"),
    ];

    let mut provider_multiselect =
        prompts::multiselect("Which AI providers would you like to configure?").required(false);
    for (id, name) in &providers {
        provider_multiselect =
            provider_multiselect.item(id.to_string(), name.to_string(), "AI Provider");
    }
    let selected_providers = provider_multiselect.interact()?;

    if !selected_providers.is_empty() {
        prompts::log::info("Selected providers:")?;
        for provider in &selected_providers {
            prompts::log::step(provider)?;
        }
    }

    // 16. Tags input for programming languages
    let languages = prompts::tags::tags("What programming languages do you use?")
        .placeholder("Type languages and press Enter")
        .interact()?;
    prompts::log::info(format!("Programming languages: {}", languages.join(", ")))?;

    // 17. Autocomplete
    let favorite_language =
        prompts::autocomplete::autocomplete("What's your favorite programming language?")
            .item("rust", "Rust")
            .item("javascript", "JavaScript")
            .item("typescript", "TypeScript")
            .item("python", "Python")
            .item("go", "Go")
            .item("java", "Java")
            .item("cpp", "C++")
            .item("csharp", "C#")
            .interact()?;
    prompts::log::info(format!("Favorite language: {}", favorite_language))?;

    // 18. Search Filter
    let framework = prompts::search_filter::search_filter("Choose your preferred web framework")
        .item(
            "React",
            "React",
            vec!["frontend".to_string(), "javascript".to_string()],
        )
        .item(
            "Vue.js",
            "Vue.js",
            vec!["frontend".to_string(), "javascript".to_string()],
        )
        .item(
            "Angular",
            "Angular",
            vec!["frontend".to_string(), "typescript".to_string()],
        )
        .item(
            "Svelte",
            "Svelte",
            vec!["frontend".to_string(), "javascript".to_string()],
        )
        .item(
            "Next.js",
            "Next.js",
            vec!["fullstack".to_string(), "react".to_string()],
        )
        .item(
            "Express.js",
            "Express.js",
            vec!["backend".to_string(), "javascript".to_string()],
        )
        .interact()?;
    prompts::log::info(format!("Preferred framework: {}", framework))?;

    // 19. Tree Select - simplified for now
    let project_type = prompts::select("What type of project are you working on?")
        .item(
            "web_frontend",
            "Web Frontend",
            "React, Vue, Angular applications",
        )
        .item("web_backend", "Web Backend", "APIs and server applications")
        .item(
            "mobile",
            "Mobile Development",
            "iOS, Android, Cross-platform",
        )
        .item("desktop", "Desktop Applications", "Native or Electron apps")
        .item("systems", "Systems Programming", "OS, embedded, low-level")
        .interact()?;
    prompts::log::info(format!("Project type: {}", project_type))?;

    // 20. Matrix Select - simplified for now
    let skills = prompts::multiselect("Rate your skills in different areas")
        .item("frontend", "Frontend Development", "HTML, CSS, JavaScript")
        .item("backend", "Backend Development", "APIs, databases, servers")
        .item("devops", "DevOps", "CI/CD, containers, cloud")
        .item("mobile", "Mobile Development", "iOS, Android apps")
        .item("aiml", "AI/ML", "Machine learning, data science")
        .interact()?;
    prompts::log::info(format!("Skills selected: {} areas", skills.len()))?;

    // 21. Progress Bar Demo
    prompts::log::info("Simulating setup progress...")?;
    let mut progress = prompts::progress::ProgressBar::new("Setting up environment", 100);
    progress.start()?;
    std::thread::sleep(std::time::Duration::from_millis(500));
    progress.set(25)?;
    progress.set_message("Installing dependencies...")?;
    std::thread::sleep(std::time::Duration::from_millis(500));
    progress.set(50)?;
    progress.set_message("Configuring settings...")?;
    std::thread::sleep(std::time::Duration::from_millis(500));
    progress.set(75)?;
    progress.set_message("Finalizing setup...")?;
    std::thread::sleep(std::time::Duration::from_millis(500));
    progress.finish("Setup complete!")?;

    // 22. Spinner Demo
    let mut spinner = prompts::spinner::spinner("Processing your configuration...");
    spinner.start()?;
    std::thread::sleep(std::time::Duration::from_millis(2000));
    spinner.stop("Configuration processed successfully!")?;

    // 23. Text Area
    let bio = prompts::text::text("Tell us about yourself")
        .placeholder("Write a short bio...")
        .interact()?;
    prompts::log::info(format!("Bio length: {} characters", bio.len()))?;

    // 24. Wizard (multi-step process)
    let wizard_result = prompts::wizard::wizard("Complete Project Setup")
        .step("Project Basics", "Set up basic project information")
        .step("Advanced Settings", "Configure advanced options")
        .step("Review & Confirm", "Review your settings")
        .interact()?;
    prompts::log::info(format!("Wizard completed: {}", wizard_result))?;

    // Final confirmation
    let proceed = prompts::confirm("Ready to complete the setup with all these amazing prompts?")
        .initial_value(true)
        .interact()?;

    if !proceed {
        prompts::log::warning("Setup cancelled by user")?;
        return Err(anyhow::anyhow!("Setup cancelled"));
    }

    // Create result with ALL collected data
    let preferences = OnboardingPreferences {
        theme: theme.to_string(),
        editor: editor.to_string(),
        shell: shell.to_string(),
        notifications,
        auto_updates,
        telemetry,
    };

    let result = OnboardingResult {
        // Basic Info
        name,
        email,
        website: website.clone(),
        phone: phone.clone(),
        bio,

        // Experience & Skills
        experience_years,
        satisfaction_rating: satisfaction,
        productivity_level: productivity,
        work_hours,
        programming_languages: languages.clone(),
        favorite_language: favorite_language.to_string(),
        framework: framework.to_string(),
        project_type: project_type.to_string(),
        selected_skills: skills.iter().map(|s| s.to_string()).collect(),

        // System & Environment
        runtime_environment: runtime_env.as_str().to_string(),
        selected_components,
        selected_providers,

        // Preferences
        preferences,

        // Workflow Data
        wizard_completed_steps: wizard_result,

        // Metadata
        timestamp: Local::now().to_rfc3339(),
    };

    // Final summary
    prompts::section_with_width("🎉 Complete Setup Summary", 80, |lines| {
        lines.push(format!("Name: {}", result.name));
        lines.push(format!("Email: {}", result.email));
        lines.push(format!("Website: {}", website));
        lines.push(format!("Phone: {}", phone));
        lines.push(format!("Experience: {} years", experience_years));
        lines.push(format!("Satisfaction: {}/5 stars", satisfaction));
        lines.push(format!("Productivity: {}%", productivity));
        lines.push(format!(
            "Work Hours: {}:00-{}:00",
            work_hours.0, work_hours.1
        ));
        lines.push(format!("Runtime: {}", runtime_env.label()));
        lines.push(format!("Theme: {}", result.preferences.theme));
        lines.push(format!("Editor: {}", result.preferences.editor));
        lines.push(format!("Shell: {}", result.preferences.shell));
        lines.push(format!("Favorite Language: {}", favorite_language));
        lines.push(format!("Framework: {}", framework));
        lines.push(format!("Project Type: {}", project_type));
        lines.push(format!("Components: {}", result.selected_components.len()));
        lines.push(format!("Providers: {}", result.selected_providers.len()));
        lines.push(format!("Languages: {}", languages.join(", ")));
        lines.push("".to_string());
        lines.push("🚀 24 ESSENTIAL PROMPT TYPES! 🚀".to_string());
        lines.push("Your DX environment is fully configured!".to_string());
    })?;

    // Save configuration
    let config_json = serde_json::to_string_pretty(&result)?;
    let config_path = find_workspace_root().join("dx.json");
    fs::write(&config_path, config_json)?;
    prompts::log::success(format!("Configuration saved to: {}", config_path.display()))?;

    prompts::outro("🎉 Complete onboarding with ALL prompts finished!")?;

    Ok(result)
}

fn async_main() -> Result<()> {
    // Set up Ctrl+C handler to show train animation on exit
    ctrlc::set_handler(|| {
        let rainbow = RainbowEffect::new();
        println!();
        println!("🚂 Exiting DX... Here's a farewell train!");
        println!();

        print!("\x1B[2J\x1B[H"); // Clear screen
        for frame in 0..15 {
            print!("\x1B[H"); // Move cursor to top
            let _ = render_train_animation(&rainbow, frame);
            thread::sleep(Duration::from_millis(200));
        }

        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    match run_onboarding() {
        Ok(result) => {
            println!("\n✨ Setup completed successfully!");
            println!("Welcome to DX, {}! 🚀", result.name);

            // Show outro train animation
            let rainbow = RainbowEffect::new();
            println!();
            println!("🚂 Thanks for using DX! Here's a celebration train!");
            println!();

            print!("\x1B[2J\x1B[H"); // Clear screen
            for frame in 0..15 {
                print!("\x1B[H"); // Move cursor to top
                let _ = render_train_animation(&rainbow, frame);
                thread::sleep(Duration::from_millis(200));
            }
        }
        Err(e) => {
            eprintln!("❌ Setup failed: {}", e);

            // Show train animation on error exit too
            let rainbow = RainbowEffect::new();
            println!();
            println!("🚂 Exiting DX...");
            println!();

            print!("\x1B[2J\x1B[H"); // Clear screen
            for frame in 0..15 {
                print!("\x1B[H"); // Move cursor to top
                let _ = render_train_animation(&rainbow, frame);
                thread::sleep(Duration::from_millis(200));
            }

            std::process::exit(1);
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    async_main()
}
