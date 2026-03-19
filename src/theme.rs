//! Unified theme system for zeroclaw CLI/TUI
//!
//! This module provides a centralized theme system that loads from theme.toml
//! and provides consistent styling across all CLI and TUI components.

use console::Style;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::RwLock;

// ─────────────────────────────────────────────────────────────────────────────
// Theme Configuration
// ─────────────────────────────────────────────────────────────────────────────

/// Complete theme configuration loaded from TOML
#[derive(Debug, Clone, Deserialize, Serialize)]
#[derive(Default)]
pub struct ThemeConfig {
    #[serde(default)]
    pub colors: ColorConfig,
    #[serde(default)]
    pub symbols: SymbolConfig,
    #[serde(default)]
    pub rainbow: RainbowConfig,
}


/// Color configuration from TOML
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ColorConfig {
    #[serde(default = "default_primary")]
    pub primary: String,
    #[serde(default = "default_success")]
    pub success: String,
    #[serde(default = "default_warning")]
    pub warning: String,
    #[serde(default = "default_error")]
    pub error: String,
    #[serde(default = "default_dim")]
    pub dim: String,
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            primary: default_primary(),
            success: default_success(),
            warning: default_warning(),
            error: default_error(),
            dim: default_dim(),
        }
    }
}

fn default_primary() -> String {
    "#FFFFFF".to_string()
}
fn default_success() -> String {
    "#00FF00".to_string()
}
fn default_warning() -> String {
    "#FFFF00".to_string()
}
fn default_error() -> String {
    "#FF0000".to_string()
}
fn default_dim() -> String {
    "#808080".to_string()
}

/// Symbol configuration from TOML
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SymbolConfig {
    #[serde(default = "default_checkmark")]
    pub checkmark: String,
    #[serde(default = "default_info")]
    pub info: String,
    #[serde(default = "default_arrow_right")]
    pub arrow_right: String,
    #[serde(default = "default_step_active")]
    pub step_active: String,
    #[serde(default = "default_step_cancel")]
    pub step_cancel: String,
    #[serde(default = "default_step_error")]
    pub step_error: String,
}

impl Default for SymbolConfig {
    fn default() -> Self {
        Self {
            checkmark: default_checkmark(),
            info: default_info(),
            arrow_right: default_arrow_right(),
            step_active: default_step_active(),
            step_cancel: default_step_cancel(),
            step_error: default_step_error(),
        }
    }
}

fn default_checkmark() -> String {
    "√".to_string()
}
fn default_info() -> String {
    "i".to_string()
}
fn default_arrow_right() -> String {
    ">".to_string()
}
fn default_step_active() -> String {
    ">".to_string()
}
fn default_step_cancel() -> String {
    "x".to_string()
}
fn default_step_error() -> String {
    "!".to_string()
}

/// Rainbow effect configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RainbowConfig {
    #[serde(default = "default_rainbow_enabled")]
    pub enabled: bool,
    #[serde(default = "default_rainbow_speed")]
    pub speed: f32,
}

impl Default for RainbowConfig {
    fn default() -> Self {
        Self {
            enabled: default_rainbow_enabled(),
            speed: default_rainbow_speed(),
        }
    }
}

fn default_rainbow_enabled() -> bool {
    true
}
fn default_rainbow_speed() -> f32 {
    1.0
}

// ─────────────────────────────────────────────────────────────────────────────
// Theme Loading
// ─────────────────────────────────────────────────────────────────────────────

/// Find theme.toml in current directory or parent directories
fn find_theme_file() -> Option<PathBuf> {
    let cwd = std::env::current_dir().ok()?;

    // Check current directory first
    let theme_path = cwd.join("theme.toml");
    if theme_path.exists() {
        return Some(theme_path);
    }

    // Check onboard subdirectory
    let onboard_theme = cwd.join("onboard").join("theme.toml");
    if onboard_theme.exists() {
        return Some(onboard_theme);
    }

    // Walk up parent directories
    for ancestor in cwd.ancestors().skip(1) {
        let theme_path = ancestor.join("theme.toml");
        if theme_path.exists() {
            return Some(theme_path);
        }

        let onboard_theme = ancestor.join("onboard").join("theme.toml");
        if onboard_theme.exists() {
            return Some(onboard_theme);
        }
    }

    None
}

/// Load theme configuration from TOML file
pub fn load_theme_config() -> ThemeConfig {
    if let Some(theme_path) = find_theme_file()
        && let Ok(content) = fs::read_to_string(&theme_path)
            && let Ok(config) = toml::from_str::<ThemeConfig>(&content) {
                return config;
            }

    ThemeConfig::default()
}

// ─────────────────────────────────────────────────────────────────────────────
// Global Theme Instance
// ─────────────────────────────────────────────────────────────────────────────

/// Global theme configuration
pub static THEME: std::sync::LazyLock<RwLock<ThemeConfig>> = std::sync::LazyLock::new(|| RwLock::new(load_theme_config()));

/// Get the current theme configuration
pub fn theme() -> ThemeConfig {
    THEME.read().unwrap().clone()
}

/// Reload theme from disk
pub fn reload_theme() {
    if let Ok(mut theme) = THEME.write() {
        *theme = load_theme_config();
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Styled Output Helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Print a success message with themed styling
pub fn print_success(msg: impl AsRef<str>) {
    let theme = theme();
    let style = Style::new().green();
    println!(
        "  {} {}",
        style.apply_to(&theme.symbols.checkmark),
        msg.as_ref()
    );
}

/// Print an info message with themed styling
pub fn print_info(msg: impl AsRef<str>) {
    let theme = theme();
    let style = Style::new().cyan();
    println!("  {} {}", style.apply_to(&theme.symbols.info), msg.as_ref());
}

/// Print a warning message with themed styling
pub fn print_warning(msg: impl AsRef<str>) {
    let theme = theme();
    let style = Style::new().yellow();
    println!(
        "  {} {}",
        style.apply_to(&theme.symbols.step_error),
        msg.as_ref()
    );
}

/// Print an error message with themed styling
pub fn print_error(msg: impl AsRef<str>) {
    let theme = theme();
    let style = Style::new().red();
    eprintln!(
        "  {} {}",
        style.apply_to(&theme.symbols.step_cancel),
        msg.as_ref()
    );
}

/// Print a step message with themed styling
pub fn print_step(msg: impl AsRef<str>) {
    let theme = theme();
    let style = Style::new().white();
    println!(
        "  {} {}",
        style.apply_to(&theme.symbols.arrow_right),
        msg.as_ref()
    );
}
