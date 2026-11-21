//! Native viewport adapted from Astryx ScrollableArea.

use std::fmt::Write;

use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::hooks::use_scrollable_area::use_scrollable_area;
use crate::layout::padding::spacing_step_css;
use crate::utils::types::{SizeValue, SpacingStep};

/// Logical axes enabled for scrolling.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ScrollAxis {
    /// Scroll along the block axis.
    #[default]
    Block,
    /// Scroll along the inline axis.
    Inline,
    /// Scroll along both logical axes.
    Both,
}

/// Accessible semantics of the viewport.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ScrollableAreaRole {
    /// Named group.
    #[default]
    Group,
    /// Named landmark region.
    Region,
}

/// Whether gestures can chain to an ancestor viewport.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ScrollOverscroll {
    /// Allow chaining.
    #[default]
    Allow,
    /// Contain gestures along effective axes.
    Contain,
}

/// Whether fitting content retains a sticky containing boundary.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ScrollableAreaStickyContainment {
    /// Let sticky children use an ancestor while this viewport fits.
    #[default]
    WhenScrollable,
    /// Retain the containing boundary even when content fits.
    Always,
}

/// A named native viewport with one normal-flow content box.
#[component]
pub fn ScrollableArea(
    /// Accessible viewport name.
    label: String,
    /// Logical scroll axes.
    #[props(default)]
    axis: ScrollAxis,
    /// Viewport semantics.
    #[props(default)]
    role: ScrollableAreaRole,
    /// Gesture chaining policy.
    #[props(default)]
    overscroll: ScrollOverscroll,
    /// Viewport width.
    #[props(default, into)]
    width: Option<SizeValue>,
    /// Viewport height.
    #[props(default, into)]
    height: Option<SizeValue>,
    /// Viewport maximum width.
    #[props(default, into)]
    max_width: Option<SizeValue>,
    /// Viewport minimum height.
    #[props(default, into)]
    min_height: Option<SizeValue>,
    /// Sticky containing-boundary policy.
    #[props(default)]
    sticky_containment: ScrollableAreaStickyContainment,
    /// Uniform content padding.
    #[props(default = SpacingStep::Zero)]
    padding: SpacingStep,
    /// Inline-axis padding override.
    #[props(default)]
    padding_inline: Option<SpacingStep>,
    /// Block-axis padding override.
    #[props(default)]
    padding_block: Option<SpacingStep>,
    /// Inline-start padding override.
    #[props(default)]
    padding_inline_start: Option<SpacingStep>,
    /// Inline-end padding override.
    #[props(default)]
    padding_inline_end: Option<SpacingStep>,
    /// Block-start padding override.
    #[props(default)]
    padding_block_start: Option<SpacingStep>,
    /// Block-end padding override.
    #[props(default)]
    padding_block_end: Option<SpacingStep>,
    /// Escape inherited container padding at eligible edges.
    #[props(default)]
    is_full_bleed: bool,
    /// Caller classes on the viewport.
    #[props(default)]
    class: String,
    /// Caller inline declarations on the viewport.
    #[props(default)]
    style: String,
    /// Content of the observed normal-flow box.
    #[props(default = rsx! {})]
    children: Element,
    /// Mounted callback on the viewport.
    #[props(default)]
    onmounted: Option<EventHandler<MountedEvent>>,
    /// Native viewport attributes and events.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let axis = match axis {
        ScrollAxis::Block => "block",
        ScrollAxis::Inline => "inline",
        ScrollAxis::Both => "both",
    };
    let role = match role {
        ScrollableAreaRole::Group => "group",
        ScrollableAreaRole::Region => "region",
    };
    let id = format!("scrollable-area-{}", current_scope_id().0);
    let mounted = use_scrollable_area(
        id.clone(),
        (
            axis,
            sticky_containment == ScrollableAreaStickyContainment::Always,
            overscroll == ScrollOverscroll::Contain,
        ),
    );
    let mut viewport_style = String::new();
    for (property, value) in [
        ("width", width),
        ("height", height),
        ("max-width", max_width),
        ("min-height", min_height),
    ] {
        if let Some(value) = value {
            let _ = write!(viewport_style, "{property}:{};", value.as_css()?);
        }
    }
    viewport_style.push_str(&style);
    let mut content_style = String::new();
    for (edge, value) in [
        ("inline-start", padding_inline_start.or(padding_inline)),
        ("inline-end", padding_inline_end.or(padding_inline)),
        ("block-start", padding_block_start.or(padding_block)),
        ("block-end", padding_block_end.or(padding_block)),
    ] {
        let _ = write!(
            content_style,
            "--container-padding-{edge}:{};",
            spacing_step_css(value.unwrap_or(padding))
        );
    }
    let classes = if is_full_bleed {
        format!("astryx-scrollable-area astryx-scrollable-area--full-bleed {class}")
    } else {
        format!("astryx-scrollable-area {class}")
    };
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attr| {
            !matches!(
                attr.name,
                "class"
                    | "style"
                    | "onmounted"
                    | "role"
                    | "aria-label"
                    | "tabindex"
                    | "data-axis"
                    | "data-scroll-axis"
                    | "data-astryx-scroll-viewport"
            )
        })
        .collect();
    rsx! {
        div { class: classes, style: viewport_style, role, "aria-label": label,
            "data-axis": axis, "data-scroll-axis": axis, "data-astryx-scroll-viewport": id,
            onmounted: move |event: MountedEvent| {
                mounted.call(event.clone());
                if let Some(callback) = onmounted { callback.call(event); }
            },
            ..attributes,
            div { class: "astryx-scrollable-area__content", "data-scroll-content": "", style: content_style, {children} }
        }
    }
}
