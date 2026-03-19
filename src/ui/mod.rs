//! # UI Module - Interactive TUI Components
//!
//! A comprehensive terminal user interface library providing interactive prompt types
//! for building beautiful CLI experiences in Rust applications.
//!
//! ## Available Modules
//!
//! - [`prompts`] - Interactive prompt types (input, confirm, select, etc.)
//! - [`effects`] - Visual effects (rainbow colors)
//! - [`splash`] - ASCII art and animations

#![allow(dead_code)]

pub mod effects;
pub mod prompts;
pub mod splash;

pub use effects::RainbowEffect;
