// Adapted from Meta Astryx Code/Code.tsx, baseline
// 476a3153417dbf75c565f7df05b0784e94ddd59e. See THIRD_PARTY_NOTICES.md.

use dioxus::core::AttributeValue;
use dioxus::events::{onclick as click_attribute, onmounted as mounted_attribute};
use dioxus::prelude::*;

/// Closed primary/secondary/inherited subset of Text colors.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CodeColor {
    /// Shared primary text ink (default).
    #[default]
    Primary,
    /// Shared secondary text ink.
    Secondary,
    /// Surrounding text color.
    Inherit,
}

impl CodeColor {
    fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
            Self::Inherit => "inherit",
        }
    }
}

/// Explicit size override. Omit `size` to use the responsive code type scale.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CodeSize {
    /// Adopt the surrounding font size (line height always inherits).
    Inherit,
}

impl CodeSize {
    fn as_str(self) -> &'static str {
        match self {
            Self::Inherit => "inherit",
        }
    }
}

/// Short technical content in a single native `<code>` root.
///
/// Defaults to left-to-right direction; a caller's `dir` replaces it. No role,
/// tab stop, accessible name, interaction, state or resource is added. Rich
/// children remain caller-owned. This is not a syntax highlighter or CodeBlock.
///
/// `class` composes with the stable `astryx-code` target; native inline `style`
/// replaces the React/StyleX escape hatch. `onmounted` is Dioxus's root-ref
/// adapter. Other native events can be supplied with Dioxus event builders in
/// `attributes`; no event is intercepted. Owned class/style and visual-axis
/// reflection cannot be replaced by the attribute spread.
#[component]
pub fn Code(
    children: Element,
    #[props(default)] color: CodeColor,
    #[props(default)] size: Option<CodeSize>,
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
    #[props(default)] onmounted: Option<EventHandler<MountedEvent>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    attributes.retain(|attribute| match attribute.name {
        "class" | "style" | "data-color" | "data-size" => false,
        "onmounted" => onmounted.is_none(),
        "onclick" => onclick.is_none(),
        "dir" => !matches!(attribute.value, AttributeValue::None),
        _ => true,
    });
    // An absent caller value must not let Code inherit the prose direction.
    if !attributes.iter().any(|attribute| attribute.name == "dir") {
        attributes.push(Attribute::new("dir", "ltr", None, false));
    }
    if let Some(handler) = onmounted {
        attributes.push(mounted_attribute(move |event| handler.call(event)));
    }
    if let Some(handler) = onclick {
        attributes.push(click_attribute(move |event| handler.call(event)));
    }
    rsx! {
        code {
            class: "astryx-code {class}",
            style,
            "data-color": color.as_str(),
            "data-size": size.map(CodeSize::as_str),
            ..attributes,
            {children}
        }
    }
}
