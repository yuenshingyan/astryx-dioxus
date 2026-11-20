// Copyright (c) Meta Platforms, Inc. and affiliates.
// Adapted from Astryx 476a3153417dbf75c565f7df05b0784e94ddd59e.
use dioxus::prelude::*;

use super::stack_style::{
    StackAlignment, StackCrossAlignment, StackDirection, StackMainAlignment, StackWrap,
};
use crate::layout::padding::spacing_step_css;
use crate::utils::types::{SizeValue, SpacingStep};

/// Polymorphic root inputs. Forward both fields to one native root, without a wrapper.
/// `attributes` includes native events and mounted callbacks (the Dioxus ref bridge).
#[derive(Clone, PartialEq)]
pub struct StackRenderProps {
    /// Merged layout, native, ARIA, data and event attributes.
    pub attributes: Vec<Attribute>,
    /// Caller content, retaining its own semantics.
    pub children: Element,
}

/// Dioxus equivalent of upstream `ElementType`, following Link's renderer contract.
/// For example: `|p: StackRenderProps| rsx! { nav { ..p.attributes, {p.children} } }`.
pub type StackElement = Callback<StackRenderProps, Element>;

/// One-dimensional flex layout, with no added role, focus target, state or effects.
/// Explicit physical alignment wins over aliases, even when invalid for that axis
/// (an inapplicable value is ignored, not replaced by its alias). Padding is local:
/// unlike Card it never publishes container-inset variables. Caller CSS overrides
/// layout, but explicit sizing props follow and override caller inline dimensions.
#[component]
pub fn Stack(
    /// Flow axis; horizontal follows the inherited writing direction.
    #[props(default)]
    direction: StackDirection,
    /// Physical horizontal alignment; takes precedence over the matching alias.
    #[props(default)]
    h_align: Option<StackAlignment>,
    /// Physical vertical alignment; takes precedence over the matching alias.
    #[props(default)]
    v_align: Option<StackAlignment>,
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
    /// Free-form caller content.
    #[props(default = rsx! {})]
    children: Element,
    /// Native attributes/events, including onmounted as the native ref callback.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let (main, cross) = match direction {
        StackDirection::Horizontal => (
            h_align.or(justify.map(Into::into)),
            v_align.or(align.map(Into::into)),
        ),
        StackDirection::Vertical => (
            v_align.or(justify.map(Into::into)),
            h_align.or(align.map(Into::into)),
        ),
    };
    let mut classes = format!("astryx-stack astryx-stack--{}", direction.as_str());
    if let Some(value) = main.and_then(StackAlignment::main_css) {
        classes.push_str(&format!(" astryx-stack--justify-{value}"));
    }
    if let Some(value) = cross.and_then(StackAlignment::cross_css) {
        classes.push_str(&format!(" astryx-stack--align-{value}"));
    }
    if let Some(wrap) = wrap {
        classes.push_str(&format!(" astryx-stack--{}", wrap.as_str()));
    }
    if is_scrollable {
        classes.push_str(" astryx-stack--scrollable");
    }
    classes.push(' ');
    classes.push_str(&class);
    let mut generated_style = String::new();
    if let Some(gap) = gap {
        generated_style.push_str(&format!("--_stack-gap:{};", spacing_step_css(gap)));
        classes.push_str(" astryx-stack--gap");
    }
    for (edge, value) in [
        (
            "inline-start",
            padding_inline_start.or(padding_inline).or(padding),
        ),
        (
            "inline-end",
            padding_inline_end.or(padding_inline).or(padding),
        ),
        (
            "block-start",
            padding_block_start.or(padding_block).or(padding),
        ),
        ("block-end", padding_block_end.or(padding_block).or(padding)),
    ] {
        if let Some(value) = value {
            classes.push_str(&format!(" astryx-stack--padding-{edge}"));
            generated_style.push_str(&format!(
                "--_stack-padding-{edge}:{};",
                spacing_step_css(value)
            ));
        }
    }
    generated_style.push_str(&style);
    generated_style.push(';');
    for (property, size) in [
        ("width", width),
        ("height", height),
        ("max-width", max_width),
        ("min-height", min_height),
    ] {
        if let Some(size) = size {
            generated_style.push_str(&format!("{property}:{};", size.as_css()?));
        }
    }
    // Passthrough follows generated attributes upstream. Keep native events intact,
    // including multiple mounted listeners supplied through the attribute builder.
    let mut root_attributes = vec![
        Attribute::new("class", classes, None, false),
        Attribute::new("style", generated_style, None, false),
        Attribute::new("data-direction", direction.as_str(), None, false),
    ];
    if let Some(gap) = gap {
        root_attributes.push(Attribute::new("data-gap", gap.as_str(), None, false));
    }
    if let Some(wrap) = wrap {
        root_attributes.push(Attribute::new("data-wrap", wrap.as_str(), None, false));
    }
    root_attributes.extend(attributes);
    if let Some(element) = as_element {
        return element.call(StackRenderProps {
            attributes: root_attributes,
            children,
        });
    }
    rsx! { div { ..root_attributes, {children} } }
}
