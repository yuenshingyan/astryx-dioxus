//! Reusable Astryx components for Dioxus.
//!
//! Include [`STYLES`] once using `document::Style` in your application root.

mod components;

pub use components::{
    Button, ButtonAction, ButtonElevation, ButtonSize, ButtonType, ButtonVariant,
};

/// Base styles and design tokens. Include once in your application root.
pub const STYLES: &str = include_str!("../assets/astryx.css");
