//! Complete prompt system with essential components
#![allow(unused_imports)]

pub mod cursor;
pub mod interaction;
pub mod theme;

// Essential prompts for onboarding
pub mod autocomplete;
pub mod confirm;
pub mod email;
pub mod input;
pub mod matrix_select;
pub mod multiselect;
pub mod number;
pub mod password;
pub mod phone_input;
pub mod progress;
pub mod range_slider;
pub mod rating;
pub mod search_filter;
pub mod select;
pub mod slider;
pub mod spinner;
pub mod tags;
pub mod text;
pub mod toggle;
pub mod tree_select;
pub mod url;
pub mod wizard;

use console::Term;
use owo_colors::OwoColorize;
use std::fmt::Display;
use std::io;

// Re-export theme system
pub use theme::{
    DxTheme, RAINBOW, SYMBOLS, THEME, rainbow_step_active, rainbow_step_submit, rainbow_symbol,
};

// Re-export essential prompts
pub use autocomplete::{Autocomplete, AutocompleteItem, autocomplete};
pub use confirm::Confirm;
pub use email::{EmailInput, email};
pub use input::Input;
pub use interaction::{PromptInteraction, State, Validate};
pub use matrix_select::{MatrixSelect, matrix_select};
pub use multiselect::{MultiSelect, MultiSelectItem};
pub use number::{Number, number};
pub use password::Password;
pub use phone_input::{PhoneInput, phone_input};
pub use progress::ProgressBar;
pub use range_slider::{RangeSlider, range_slider};
pub use rating::{Rating, rating};
pub use search_filter::{SearchFilter, search_filter};
pub use select::{Select, SelectItem};
pub use slider::{Slider, slider};
pub use spinner::Spinner;
pub use tags::{Tags, tags};
pub use text::{Text, text};
pub use toggle::{Toggle, toggle};
pub use tree_select::{TreeNode, TreeSelect, tree_select};
pub use url::{UrlInput, url};
pub use wizard::{Wizard, WizardStep, wizard};

// ─────────────────────────────────────────────────────────────────────────────
// Public API Functions
// ─────────────────────────────────────────────────────────────────────────────

fn term_write(line: impl Display) -> io::Result<()> {
    Term::stderr().write_str(line.to_string().as_str())
}

pub fn intro(title: impl Display) -> io::Result<()> {
    let theme = THEME.read().unwrap();
    let symbols = &*SYMBOLS;
    term_write(format!(
        "{}{}{}",
        theme.dim.apply_to(symbols.bar_start.as_str()),
        theme.dim.apply_to("─"),
        format!(" {}", title)
    ))?;
    term_write("\n")?;
    term_write(format!("{}\n", theme.dim.apply_to(symbols.bar.as_str())))
}

pub fn outro(message: impl Display) -> io::Result<()> {
    let theme = THEME.read().unwrap();
    let symbols = &*SYMBOLS;
    let rainbow_step_submit = rainbow_symbol(&symbols.step_submit, 1);

    term_write(format!(
        "{}{} {}\n",
        theme.dim.apply_to(symbols.bar.as_str()),
        rainbow_step_submit,
        message,
    ))
}

fn render_box_section(title: &str, lines: &[&str], min_content_width: usize) -> io::Result<()> {
    let theme = THEME.read().unwrap();
    let symbols = &*SYMBOLS;

    let max_line_width = lines
        .iter()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0);
    let content_width = max_line_width.max(min_content_width);

    let title_with_spaces = format!(" {} ", title);
    let title_total_len = title_with_spaces.chars().count();
    let remaining_horizontal = (content_width + 2).saturating_sub(title_total_len);

    term_write(format!(
        "{}{}{}{}",
        theme.dim.apply_to(symbols.connect_left.as_str()),
        title_with_spaces,
        theme
            .dim
            .apply_to(symbols.box_horizontal.repeat(remaining_horizontal)),
        theme.dim.apply_to(symbols.corner_top_right.as_str())
    ))?;
    term_write("\n")?;

    for line in lines {
        let line_width = line.chars().count();
        let padding = content_width.saturating_sub(line_width);
        term_write(format!(
            "{} {}{} {}\n",
            theme.dim.apply_to(symbols.box_vertical.as_str()),
            line,
            " ".repeat(padding),
            theme.dim.apply_to(symbols.box_vertical.as_str())
        ))?;
    }

    let total_bottom_width = content_width + 2;
    term_write(format!(
        "{}{}{}",
        theme.dim.apply_to(symbols.connect_left.as_str()),
        theme
            .dim
            .apply_to(symbols.box_horizontal.repeat(total_bottom_width)),
        theme.dim.apply_to(symbols.corner_bottom_right.as_str())
    ))?;
    term_write("\n")?;
    term_write(format!("{}\n", theme.dim.apply_to(symbols.bar.as_str())))?;

    Ok(())
}

pub fn section_with_width<F>(title: &str, content_width: usize, build: F) -> io::Result<()>
where
    F: FnOnce(&mut Vec<String>),
{
    let mut lines: Vec<String> = Vec::new();
    build(&mut lines);
    let line_refs: Vec<&str> = lines.iter().map(String::as_str).collect();
    render_box_section(title, &line_refs, content_width)
}

pub fn box_section(title: &str, lines: &[&str]) -> io::Result<()> {
    render_box_section(title, lines, 83)
}

pub fn confirm(prompt: impl Into<String>) -> Confirm {
    Confirm::new(prompt.into())
}

pub fn select<T: Clone>(prompt: impl Into<String>) -> Select<T> {
    Select::new(prompt.into())
}

pub fn multiselect<T: Clone>(prompt: impl Into<String>) -> MultiSelect<T> {
    MultiSelect::new(prompt.into())
}

pub mod log {
    use super::*;
    use owo_colors::OwoColorize;

    pub fn info(text: impl Display) -> io::Result<()> {
        let theme = THEME.read().unwrap();
        let symbols = &*SYMBOLS;
        eprintln!("{} {}", theme.dim.apply_to(symbols.info.as_str()), text);
        eprintln!("{}", theme.dim.apply_to(symbols.bar.as_str()));
        Ok(())
    }

    pub fn success(text: impl Display) -> io::Result<()> {
        let theme = THEME.read().unwrap();
        let symbols = &*SYMBOLS;
        eprintln!(
            "{} {}",
            theme.success.apply_to(symbols.checkmark.as_str()).bold(),
            text
        );
        eprintln!("{}", theme.dim.apply_to(symbols.bar.as_str()));
        Ok(())
    }

    pub fn warning(text: impl Display) -> io::Result<()> {
        let theme = THEME.read().unwrap();
        let symbols = &*SYMBOLS;
        eprintln!(
            "{} {}",
            theme.warning.apply_to(symbols.step_error.as_str()).bold(),
            text
        );
        eprintln!("{}", theme.dim.apply_to(symbols.bar.as_str()));
        Ok(())
    }

    pub fn step(text: impl Display) -> io::Result<()> {
        let theme = THEME.read().unwrap();
        let symbols = &*SYMBOLS;
        let rainbow_step = rainbow_symbol(&symbols.step_active, 1);
        eprintln!(
            "{} {} {}",
            theme.dim.apply_to(symbols.bar.as_str()),
            rainbow_step,
            text
        );
        Ok(())
    }
}
