//! Reusable Astryx components for Dioxus.
//!
//! Include [`STYLES`] once using `document::Style` in your application root.

mod aspect_ratio;
mod badge;
mod blockquote;
mod button;
mod button_group;
mod card;
mod clickable_card;
mod code;
mod compatibility;
mod divider;
mod heading;
mod hooks;
mod i18n;
mod icon;
mod icon_button;
mod input_group;
mod interactive_role_context;
mod kbd;
mod layer;
mod layout;
mod link;
mod private_attributes;
mod progress_bar;
mod selectable_card;
mod size_context;
mod skeleton;
mod spinner;
mod stack;
mod status_dot;
mod text;
mod theme;
mod tooltip;
mod utils;
mod visually_hidden;

pub use aspect_ratio::{AspectRatio, AspectRatioFit, AspectRatioShape};
pub use badge::{Badge, BadgeLabel, BadgeVariant, InvalidBadgeVariant};
pub use blockquote::{Blockquote, BlockquoteCite};
pub use button::{Button, ButtonAction, ButtonElevation, ButtonSize, ButtonType, ButtonVariant};
pub use button_group::{ButtonGroup, ButtonGroupElevation, ButtonGroupOrientation};
pub use card::{Card, CardVariant, InvalidCardVariant};
pub use clickable_card::{ClickableCard, ClickableCardError};
pub use code::{Code, CodeColor, CodeProps, CodeSize};
pub use divider::{
    Divider, DividerLabel, DividerOrientation, DividerVariant, InvalidDividerVariant,
};
pub use heading::{Heading, HeadingLevel, HeadingProps, HeadingType, InvalidHeadingLevel};
pub use hooks::use_interactive_role::{InteractiveRole, use_interactive_role};
pub use icon::{
    Icon, IconDefaultSizeContext, IconDefaultSizeProvider, IconEntry, IconRequest, IconSize,
    InvalidIconName, get_extended_icon, register_icons, use_icon_size,
};
pub use icon_button::IconButton;
pub use interactive_role_context::{
    InteractiveRoleContext, InteractiveRoleProvider, use_interactive_role_context,
};
pub use kbd::{Kbd, KbdError, KbdText};
pub use layer::{LayerContentBoundary, LayerPlacement};
pub use link::{
    Link, LinkComponentType, LinkDownload, LinkProvider, LinkRenderProps, LinkifyCaptures,
    LinkifyError, LinkifyMatch, LinkifyNode, LinkifyPattern, UseLinkifyOptions, linkify,
    use_link_component, use_linkify,
};
pub use progress_bar::{
    InvalidProgressBarVariant, ProgressBar, ProgressBarError, ProgressBarMark, ProgressBarState,
    ProgressBarVariant,
};
pub use selectable_card::{SelectableCard, SelectableCardError, SelectableCardProps};
pub use size_context::{ElementSize, SizeContext, SizeProvider, use_size};
pub use skeleton::{Skeleton, SkeletonError, SkeletonLength, SkeletonRadius};
pub use spinner::{Spinner, SpinnerError, SpinnerShade, SpinnerSize};
pub use stack::{
    Stack, StackAlignment, StackCrossAlignment, StackDirection, StackElement, StackMainAlignment,
    StackProps, StackRenderProps, StackWrap,
};
pub use status_dot::{
    InvalidStatusDotVariant, StatusDot, StatusDotError, StatusDotIcon, StatusDotVariant,
};
pub use text::{Text, TextTag, Truncation, use_truncation};
pub use theme::{
    InvalidTextName, TextColor, TextDisplay, TextJustify, TextSize, TextType, TextWeight, TextWrap,
    WordBreak,
};

pub use utils::types::{Elevation, SizeValue, SizeValueError, SpacingStep};

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
