//! Reusable Astryx components for Dioxus.
//!
//! Include [`STYLES`] once using `document::Style` in your application root.

mod badge;
mod button;
mod button_group;
mod compatibility;
mod hooks;
mod i18n;
mod icon;
mod icon_button;
mod input_group;
mod interactive_role_context;
mod layer;
mod link;
mod size_context;
mod spinner;
mod text;
mod theme;
mod tooltip;
mod utils;

pub use badge::{Badge, BadgeLabel, BadgeVariant, InvalidBadgeVariant};
pub use button::{Button, ButtonAction, ButtonElevation, ButtonSize, ButtonType, ButtonVariant};
pub use button_group::{ButtonGroup, ButtonGroupElevation, ButtonGroupOrientation};
pub use hooks::use_interactive_role::{InteractiveRole, use_interactive_role};
pub use icon::{
    Icon, IconDefaultSizeContext, IconDefaultSizeProvider, IconEntry, IconRequest, IconSize,
    InvalidIconName, get_extended_icon, register_icons, use_icon_size,
};
pub use icon_button::IconButton;
pub use interactive_role_context::{
    InteractiveRoleContext, InteractiveRoleProvider, use_interactive_role_context,
};
pub use layer::LayerContentBoundary;
pub use link::{
    Link, LinkComponentType, LinkDownload, LinkProvider, LinkRenderProps, LinkifyCaptures,
    LinkifyError, LinkifyMatch, LinkifyNode, LinkifyPattern, UseLinkifyOptions, linkify,
    use_link_component, use_linkify,
};
pub use size_context::{ElementSize, SizeContext, SizeProvider, use_size};
pub use spinner::{Spinner, SpinnerError, SpinnerShade, SpinnerSize};
pub use text::{Text, TextTag};
pub use theme::{
    InvalidTextName, TextColor, TextDisplay, TextJustify, TextSize, TextType, TextWeight, TextWrap,
    WordBreak,
};

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
