//! Reusable Astryx components for Dioxus.
//!
//! Include [`STYLES`] once using `document::Style` in your application root.

mod components;

pub use components::{
    Button, ButtonAction, ButtonElevation, ButtonEnvironment, ButtonEnvironmentContext,
    ButtonEnvironmentProvider, ButtonGroup, ButtonGroupElevation, ButtonGroupOrientation,
    ButtonLinkProps, ButtonLinkProvider, ButtonLinkRenderer, ButtonSize, ButtonSizeContext,
    ButtonType, ButtonVariant, ControlSize, Icon, IconDefaultSizeContext, IconDefaultSizeProvider,
    IconSize, SizeProvider, Spinner, SpinnerShade, SpinnerSize, use_button_environment,
};

/// Base styles and design tokens. Include once in your application root.
pub const STYLES: &str = include_str!("../assets/astryx.css");
