//! Theme system for all prompts
//! 
//! This module provides a unified theming system that all prompts use for consistent
//! visual appearance across the onboarding experience.
//! 
//! Themes can be customized via a `theme.toml` file in the project root.

use console::Style;
use once_cell::sync::Lazy;
use std::sync::RwLock;
use std::path::Path;
use std::fs;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use crate::effects::RainbowEffect;

// ─────────────────────────────────────────────────────────────────────────────
// TOML Configuration Structures
// ─────────────────────────────────────────────────────────────────────────────

/// Complete theme configuration loaded from TOML
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ThemeConfig {
    #[serde(default)]
    pub colors: ColorConfig,
    #[serde(default)]
    pub symbols: SymbolConfig,
    #[serde(default)]
    pub rainbow: RainbowConfig,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            colors: ColorConfig::default(),
            symbols: SymbolConfig::default(),
            rainbow: RainbowConfig::default(),
        }
    }
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

fn default_primary() -> String { "#FFFFFF".to_string() }
fn default_success() -> String { "#00FF00".to_string() }
fn default_warning() -> String { "#FFFF00".to_string() }
fn default_error() -> String { "#FF0000".to_string() }
fn default_dim() -> String { "#808080".to_string() }

/// Symbol configuration from TOML
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SymbolConfig {
    #[serde(default = "default_step_active")]
    pub step_active: String,
    #[serde(default = "default_step_cancel")]
    pub step_cancel: String,
    #[serde(default = "default_step_error")]
    pub step_error: String,
    #[serde(default = "default_step_submit")]
    pub step_submit: String,
    #[serde(default = "default_bar_start")]
    pub bar_start: String,
    #[serde(default = "default_bar")]
    pub bar: String,
    #[serde(default = "default_bar_end")]
    pub bar_end: String,
    #[serde(default = "default_radio_active")]
    pub radio_active: String,
    #[serde(default = "default_radio_inactive")]
    pub radio_inactive: String,
    #[serde(default = "default_checkbox_active")]
    pub checkbox_active: String,
    #[serde(default = "default_checkbox_selected")]
    pub checkbox_selected: String,
    #[serde(default = "default_checkbox_inactive")]
    pub checkbox_inactive: String,
    #[serde(default = "default_password_mask")]
    pub password_mask: String,
    #[serde(default = "default_bar_h")]
    pub bar_h: String,
    #[serde(default = "default_corner_top_right")]
    pub corner_top_right: String,
    #[serde(default = "default_connect_left")]
    pub connect_left: String,
    #[serde(default = "default_corner_bottom_right")]
    pub corner_bottom_right: String,
    #[serde(default = "default_box_top_left")]
    pub box_top_left: String,
    #[serde(default = "default_box_top_right")]
    pub box_top_right: String,
    #[serde(default = "default_box_bottom_left")]
    pub box_bottom_left: String,
    #[serde(default = "default_box_bottom_right")]
    pub box_bottom_right: String,
    #[serde(default = "default_box_horizontal")]
    pub box_horizontal: String,
    #[serde(default = "default_box_vertical")]
    pub box_vertical: String,
    #[serde(default = "default_box_left_t")]
    pub box_left_t: String,
    #[serde(default = "default_box_right_t")]
    pub box_right_t: String,
    #[serde(default = "default_checkmark")]
    pub checkmark: String,
    #[serde(default = "default_info")]
    pub info: String,
    #[serde(default = "default_arrow_right")]
    pub arrow_right: String,
    #[serde(default = "default_slider_filled")]
    pub slider_filled: String,
    #[serde(default = "default_slider_empty")]
    pub slider_empty: String,
    #[serde(default = "default_slider_handle")]
    pub slider_handle: String,
}

impl Default for SymbolConfig {
    fn default() -> Self {
        Self {
            step_active: default_step_active(),
            step_cancel: default_step_cancel(),
            step_error: default_step_error(),
            step_submit: default_step_submit(),
            bar_start: default_bar_start(),
            bar: default_bar(),
            bar_end: default_bar_end(),
            radio_active: default_radio_active(),
            radio_inactive: default_radio_inactive(),
            checkbox_active: default_checkbox_active(),
            checkbox_selected: default_checkbox_selected(),
            checkbox_inactive: default_checkbox_inactive(),
            password_mask: default_password_mask(),
            bar_h: default_bar_h(),
            corner_top_right: default_corner_top_right(),
            connect_left: default_connect_left(),
            corner_bottom_right: default_corner_bottom_right(),
            box_top_left: default_box_top_left(),
            box_top_right: default_box_top_right(),
            box_bottom_left: default_box_bottom_left(),
            box_bottom_right: default_box_bottom_right(),
            box_horizontal: default_box_horizontal(),
            box_vertical: default_box_vertical(),
            box_left_t: default_box_left_t(),
            box_right_t: default_box_right_t(),
            checkmark: default_checkmark(),
            info: default_info(),
            arrow_right: default_arrow_right(),
            slider_filled: default_slider_filled(),
            slider_empty: default_slider_empty(),
            slider_handle: default_slider_handle(),
        }
    }
}

fn default_step_active() -> String { ">".to_string() }
fn default_step_cancel() -> String { "x".to_string() }
fn default_step_error() -> String { "!".to_string() }
fn default_step_submit() -> String { ">".to_string() }
fn default_bar_start() -> String { "╭".to_string() }
fn default_bar() -> String { "│".to_string() }
fn default_bar_end() -> String { "╰".to_string() }
fn default_radio_active() -> String { "(*)".to_string() }
fn default_radio_inactive() -> String { "( )".to_string() }
fn default_checkbox_active() -> String { "[ ]".to_string() }
fn default_checkbox_selected() -> String { "[x]".to_string() }
fn default_checkbox_inactive() -> String { "[ ]".to_string() }
fn default_password_mask() -> String { "*".to_string() }
fn default_bar_h() -> String { "─".to_string() }
fn default_corner_top_right() -> String { "╮".to_string() }
fn default_connect_left() -> String { "├".to_string() }
fn default_corner_bottom_right() -> String { "╯".to_string() }
fn default_box_top_left() -> String { "╭".to_string() }
fn default_box_top_right() -> String { "╮".to_string() }
fn default_box_bottom_left() -> String { "╰".to_string() }
fn default_box_bottom_right() -> String { "╯".to_string() }
fn default_box_horizontal() -> String { "─".to_string() }
fn default_box_vertical() -> String { "│".to_string() }
fn default_box_left_t() -> String { "├".to_string() }
fn default_box_right_t() -> String { "╯".to_string() }
fn default_checkmark() -> String { "√".to_string() }
fn default_info() -> String { "i".to_string() }
fn default_arrow_right() -> String { ">".to_string() }
fn default_slider_filled() -> String { "=".to_string() }
fn default_slider_empty() -> String { "-".to_string() }
fn default_slider_handle() -> String { "O".to_string() }

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

fn default_rainbow_enabled() -> bool { true }
fn default_rainbow_speed() -> f32 { 1.0 }

// ─────────────────────────────────────────────────────────────────────────────
// Theme Loading
// ─────────────────────────────────────────────────────────────────────────────

/// Load theme configuration from TOML file
pub fn load_theme_config<P: AsRef<Path>>(path: P) -> Result<ThemeConfig, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: ThemeConfig = toml::from_str(&content)?;
    Ok(config)
}

/// Load theme from default location (theme.toml) or use defaults
pub fn load_theme_or_default() -> ThemeConfig {
    load_theme_config("theme.toml")
        .unwrap_or_else(|_| ThemeConfig::default())
}

// ─────────────────────────────────────────────────────────────────────────────
// Theme Configuration
// ─────────────────────────────────────────────────────────────────────────────

/// Core theme colors and styles used across all prompts
pub struct DxTheme {
    pub primary: Style,
    pub success: Style,
    pub warning: Style,
    pub error: Style,
    pub dim: Style,
}

impl Default for DxTheme {
    fn default() -> Self {
        Self {
            primary: Style::new().white(),
            success: Style::new().green(),
            warning: Style::new().yellow(),
            error: Style::new().red(),
            dim: Style::new().dim(),
        }
    }
}

/// Global theme instance
pub static THEME: Lazy<RwLock<DxTheme>> = Lazy::new(|| RwLock::new(DxTheme::default()));

// ─────────────────────────────────────────────────────────────────────────────
// Rainbow Animation
// ─────────────────────────────────────────────────────────────────────────────

/// Global rainbow effect for animated symbols
pub static RAINBOW: Lazy<RwLock<RainbowEffect>> = Lazy::new(|| RwLock::new(RainbowEffect::new()));

/// Get a rainbow-colored symbol at a specific index
pub fn rainbow_symbol(symbol: &str, index: usize) -> String {
    if let Ok(rainbow) = RAINBOW.read() {
        let color = rainbow.color_at(index);
        symbol.truecolor(color.r, color.g, color.b).to_string()
    } else {
        symbol.to_string()
    }
}

/// Get a rainbow-colored step_submit symbol (♦)
pub fn rainbow_step_submit() -> String {
    let symbols = &*SYMBOLS;
    rainbow_symbol(&symbols.step_submit, 0)
}

/// Get a rainbow-colored step_active symbol  
pub fn rainbow_step_active() -> String {
    let symbols = &*SYMBOLS;
    rainbow_symbol(&symbols.step_active, 1)
}

// ─────────────────────────────────────────────────────────────────────────────
// Symbols
// ─────────────────────────────────────────────────────────────────────────────

/// Unicode symbols used across all prompts for consistent visual appearance
#[derive(Clone)]
pub struct Symbols {
    pub step_active: String,
    pub step_cancel: String,
    pub step_error: String,
    pub step_submit: String,
    pub bar_start: String,
    pub bar: String,
    pub bar_end: String,
    pub radio_active: String,
    pub radio_inactive: String,
    pub checkbox_active: String,
    pub checkbox_selected: String,
    pub checkbox_inactive: String,
    pub password_mask: char,
    pub bar_h: String,
    pub corner_top_right: String,
    pub connect_left: String,
    pub corner_bottom_right: String,
    pub box_top_left: String,
    pub box_top_right: String,
    pub box_bottom_left: String,
    pub box_bottom_right: String,
    pub box_horizontal: String,
    pub box_vertical: String,
    pub box_left_t: String,
    pub box_right_t: String,
    pub checkmark: String,
    pub info: String,
    pub arrow_right: String,
    pub slider_filled: String,
    pub slider_empty: String,
    pub slider_handle: String,
}

impl Symbols {
    fn from_config(config: &SymbolConfig) -> Self {
        Self {
            step_active: config.step_active.clone(),
            step_cancel: config.step_cancel.clone(),
            step_error: config.step_error.clone(),
            step_submit: config.step_submit.clone(),
            bar_start: config.bar_start.clone(),
            bar: config.bar.clone(),
            bar_end: config.bar_end.clone(),
            radio_active: config.radio_active.clone(),
            radio_inactive: config.radio_inactive.clone(),
            checkbox_active: config.checkbox_active.clone(),
            checkbox_selected: config.checkbox_selected.clone(),
            checkbox_inactive: config.checkbox_inactive.clone(),
            password_mask: config.password_mask.chars().next().unwrap_or('*'),
            bar_h: config.bar_h.clone(),
            corner_top_right: config.corner_top_right.clone(),
            connect_left: config.connect_left.clone(),
            corner_bottom_right: config.corner_bottom_right.clone(),
            box_top_left: config.box_top_left.clone(),
            box_top_right: config.box_top_right.clone(),
            box_bottom_left: config.box_bottom_left.clone(),
            box_bottom_right: config.box_bottom_right.clone(),
            box_horizontal: config.box_horizontal.clone(),
            box_vertical: config.box_vertical.clone(),
            box_left_t: config.box_left_t.clone(),
            box_right_t: config.box_right_t.clone(),
            checkmark: config.checkmark.clone(),
            info: config.info.clone(),
            arrow_right: config.arrow_right.clone(),
            slider_filled: config.slider_filled.clone(),
            slider_empty: config.slider_empty.clone(),
            slider_handle: config.slider_handle.clone(),
        }
    }
    
    /// Get a reference to a symbol field as &str
    pub fn get(&self, field: &str) -> &str {
        match field {
            "bar" => &self.bar,
            "step_submit" => &self.step_submit,
            "step_active" => &self.step_active,
            "radio_active" => &self.radio_active,
            "radio_inactive" => &self.radio_inactive,
            "bar_start" => &self.bar_start,
            "connect_left" => &self.connect_left,
            "corner_top_right" => &self.corner_top_right,
            "box_vertical" => &self.box_vertical,
            "corner_bottom_right" => &self.corner_bottom_right,
            "checkmark" => &self.checkmark,
            "info" => &self.info,
            "arrow_right" => &self.arrow_right,
            _ => "",
        }
    }
}

impl Default for Symbols {
    fn default() -> Self {
        Self::from_config(&SymbolConfig::default())
    }
}

/// Global symbols instance
pub static SYMBOLS: Lazy<Symbols> = Lazy::new(|| {
    let config = load_theme_or_default();
    Symbols::from_config(&config.symbols)
});

/// Initialize theme system from configuration
pub fn init_theme() {
    let _config = load_theme_or_default();
    
    // Update global theme
    if let Ok(mut theme) = THEME.write() {
        *theme = DxTheme::default(); // Colors are applied via console::Style
    }
    
    // Symbols are loaded via SYMBOLS lazy static
    // Rainbow is loaded via RAINBOW lazy static
}
