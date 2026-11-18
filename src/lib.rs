//! Reusable Astryx components for Dioxus.
//!
//! Include [`STYLES`] once using `document::Style` in your application root.

mod button;
mod button_group;
mod compatibility;
mod icon;
mod icon_button;
mod input_group;
mod layer;
mod link;
mod size_context;
mod spinner;

pub use button::{Button, ButtonAction, ButtonElevation, ButtonSize, ButtonType, ButtonVariant};
pub use button_group::{ButtonGroup, ButtonGroupElevation, ButtonGroupOrientation};
pub use icon::{Icon, IconDefaultSizeContext, IconDefaultSizeProvider, IconSize, use_icon_size};
pub use icon_button::IconButton;
pub use layer::LayerContentBoundary;
pub use link::{LinkComponentType, LinkProvider, LinkRenderProps, use_link_component};
pub use size_context::{ElementSize, SizeContext, SizeProvider, use_size};
pub use spinner::{Spinner, SpinnerShade, SpinnerSize};

// Compatibility spellings of provisional APIs. Core code uses upstream owners.
pub use compatibility::{
    ButtonEnvironment, ButtonEnvironmentContext, ButtonEnvironmentProvider, use_button_environment,
};
pub use link::{
    ButtonLinkProvider, LinkComponentType as ButtonLinkRenderer, LinkRenderProps as ButtonLinkProps,
};
pub use size_context::{ElementSize as ControlSize, SizeContext as ButtonSizeContext};

/// Base styles and design tokens. Include once in your application root.
pub const STYLES: &str = include_str!("../assets/astryx.css");
