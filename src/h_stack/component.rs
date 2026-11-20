//! Thin horizontal Stack wrapper, adapted from Astryx HStack.

use dioxus::prelude::*;

use crate::stack::{
    Stack, StackCrossAlignment, StackDirection, StackElement, StackMainAlignment, StackWrap,
};
use crate::utils::types::{SizeValue, SpacingStep};

/// Arranges children along the inline axis using the Stack layout contract.
#[component]
pub fn HStack(
    /// Physical horizontal alignment; takes precedence over the matching alias.
    #[props(default)]
    h_align: Option<StackMainAlignment>,
    /// Physical vertical alignment; takes precedence over the matching alias.
    #[props(default)]
    v_align: Option<StackCrossAlignment>,
    /// Main-axis alias, applied only when the corresponding physical prop is absent.
    #[props(default)]
    justify: Option<StackMainAlignment>,
    /// Cross-axis alias, applied only when the corresponding physical prop is absent.
    #[props(default)]
    align: Option<StackCrossAlignment>,
    /// Equal row and column gap; omitted by default.
    #[props(default)]
    gap: Option<SpacingStep>,
    /// Uniform local padding; omitted by default.
    #[props(default)]
    padding: Option<SpacingStep>,
    /// Overrides uniform padding on the inline axis.
    #[props(default)]
    padding_inline: Option<SpacingStep>,
    /// Overrides axis and uniform padding on this edge, including explicit zero.
    #[props(default)]
    padding_inline_start: Option<SpacingStep>,
    /// Overrides axis and uniform padding on this edge, including explicit zero.
    #[props(default)]
    padding_inline_end: Option<SpacingStep>,
    /// Overrides uniform padding on the block axis.
    #[props(default)]
    padding_block: Option<SpacingStep>,
    /// Overrides axis and uniform padding on this edge, including explicit zero.
    #[props(default)]
    padding_block_start: Option<SpacingStep>,
    /// Overrides axis and uniform padding on this edge, including explicit zero.
    #[props(default)]
    padding_block_end: Option<SpacingStep>,
    /// Enable overflow auto; no implicit role, name, tabindex or flex minimum reset.
    #[props(default)]
    is_scrollable: bool,
    /// Optional width; numbers become pixels, strings remain CSS dimensions.
    #[props(default)]
    width: Option<SizeValue>,
    /// Optional height; does not enable scrolling by itself.
    #[props(default)]
    height: Option<SizeValue>,
    /// Optional maximum width.
    #[props(default)]
    max_width: Option<SizeValue>,
    /// Optional minimum height.
    #[props(default)]
    min_height: Option<SizeValue>,
    /// Wrapping; omission leaves the browser's nowrap default and no reflection.
    #[props(default)]
    wrap: Option<StackWrap>,
    /// Upstream `as`: caller renderer for any native or application-owned element.
    /// Omission renders one div. Forward the supplied attributes and children.
    #[props(default)]
    as_element: Option<StackElement>,
    /// Caller CSS classes, replacing StyleX xstyle/className with native CSS.
    #[props(default)]
    class: String,
    /// Caller inline CSS, before explicit sizing props.
    #[props(default)]
    style: String,
    /// Native click callback, composed with attribute-builder listeners.
    #[props(default)]
    onclick: Option<EventHandler<MouseEvent>>,
    /// Native ref bridge, composed with attribute-builder mounted listeners.
    #[props(default)]
    onmounted: Option<EventHandler<MountedEvent>>,
    /// Free-form caller content.
    #[props(default = rsx! {})]
    children: Element,
    /// Native attributes/events, including onmounted as the native ref callback.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        Stack {
            direction: StackDirection::Horizontal,
            justify: h_align.or(justify),
            align: v_align.or(align),
            gap,
            padding,
            padding_inline,
            padding_inline_start,
            padding_inline_end,
            padding_block,
            padding_block_start,
            padding_block_end,
            is_scrollable,
            width,
            height,
            max_width,
            min_height,
            wrap,
            as_element,
            class,
            style,
            onclick,
            onmounted,
            children,
            attributes,
        }
    }
}
