//! Five-slot composition adapted from Astryx Layout/Layout.tsx.

use std::fmt::Write;

use dioxus::prelude::*;

use super::layout_area_context::LayoutAreaContext;
use super::layout_divider_context::LayoutDividerContext;
use super::layout_slots_context::{LayoutSlots, LayoutSlotsContext};
use super::padding::spacing_step_css;
use crate::utils::is_renderable::is_renderable;
use crate::utils::types::{SizeValue, SpacingStep};

/// How Layout takes up vertical space.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum LayoutHeight {
    /// Fill the containing height.
    #[default]
    Fill,
    /// Grow with the content.
    Auto,
}

fn has_slot(slot: &Option<Element>) -> bool {
    slot.as_ref().is_some_and(is_renderable)
}

fn supports_alignment(width: &SizeValue) -> bool {
    let SizeValue::Css(value) = width else {
        return true;
    };
    let value = value.trim().to_ascii_lowercase();
    if value.contains('%') {
        return false;
    }
    if value == "0"
        || ["calc(", "min(", "max(", "clamp("]
            .iter()
            .any(|prefix| value.starts_with(prefix))
    {
        return true;
    }
    let length = value.strip_prefix('-').unwrap_or(&value);
    let Some(unit_start) = length.find(|ch: char| ch.is_ascii_alphabetic()) else {
        return false;
    };
    let (number, unit) = length.split_at(unit_start);
    !number.ends_with('.')
        && number
            .bytes()
            .all(|byte| byte.is_ascii_digit() || byte == b'.')
        && number.parse::<f64>().is_ok()
        && unit.bytes().all(|byte| byte.is_ascii_alphabetic())
}

#[component]
fn PanelSlot(is_end: bool, children: Element) -> Element {
    use_context_provider(|| LayoutAreaContext {
        is_end_panel: Some(is_end),
    });
    children
}

/// Compose a header, footer, two side panels, and a central content slot.
#[component]
pub fn Layout(
    /// Content slot, taking precedence over children.
    #[props(default)]
    content: Option<Element>,
    /// Maximum width of aligned content.
    #[props(default, into)]
    content_width: Option<SizeValue>,
    /// Logical inline-end slot.
    #[props(default)]
    end: Option<Element>,
    /// Footer slot.
    #[props(default)]
    footer: Option<Element>,
    /// Header slot.
    #[props(default)]
    header: Option<Element>,
    /// Whether to fill the containing height or grow with content.
    #[props(default)]
    height: LayoutHeight,
    /// Outer-edge inset for this Layout's own regions.
    #[props(default)]
    padding: Option<SpacingStep>,
    /// Logical inline-start slot.
    #[props(default)]
    start: Option<Element>,
    /// Default divider for header and footer descendants in this layer.
    #[props(default)]
    default_has_dividers: Option<bool>,
    /// Caller classes on the root.
    #[props(default)]
    class: String,
    /// Caller inline CSS on the root.
    #[props(default)]
    style: String,
    /// Shorthand for the content slot.
    #[props(default = rsx! {})]
    children: Element,
    /// Native root attributes, events, and mounted callback.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    use_context_provider(LayoutAreaContext::default);
    let slots = LayoutSlots {
        has_header: has_slot(&header),
        has_footer: has_slot(&footer),
        has_start: has_slot(&start),
        has_end: has_slot(&end),
    };
    let mut slot_context = use_context_provider(|| LayoutSlotsContext(Signal::new(slots)));
    if *slot_context.0.peek() != slots {
        slot_context.0.set(slots);
    }
    // Capture the ancestor before providing this Layout's own reactive value.
    let inherited = use_hook(try_consume_context::<LayoutDividerContext>);
    let divider = default_has_dividers.unwrap_or_else(|| inherited.is_some_and(|ctx| (ctx.0)()));
    let mut divider_context = use_context_provider(|| LayoutDividerContext(Signal::new(divider)));
    if *divider_context.0.peek() != divider {
        divider_context.0.set(divider);
    }
    let mode = match height {
        LayoutHeight::Fill => "fill",
        LayoutHeight::Auto => "auto",
    };
    let mut inner_style = String::new();
    if let Some(padding) = padding {
        let value = spacing_step_css(padding);
        let _ = write!(
            inner_style,
            "--layout-padding-own-outer-x:{value};--layout-padding-own-outer-y:{value};"
        );
    }
    let mut middle = String::from("astryx-layout__middle");
    if let Some(width) = content_width {
        let value = width.as_css()?;
        let _ = write!(inner_style, "--layout-content-width:{value};");
        if supports_alignment(&width) && !(slots.has_start && slots.has_end) {
            let _ = write!(inner_style, "--layout-alignment-width:{value};");
            middle.push_str(" astryx-layout__middle--align");
            if slots.has_start {
                middle.push_str(" astryx-layout__middle--start");
            }
            if slots.has_end {
                middle.push_str(" astryx-layout__middle--end");
            }
        } else {
            middle.push_str(" astryx-layout__middle--constrained");
        }
    }
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attr| !matches!(attr.name, "class" | "style" | "data-height"))
        .collect();
    rsx! {
        div { class: "astryx-layout {class}", "data-height": mode, style: (!style.is_empty()).then_some(style), ..attributes,
            div { class: "astryx-layout__inner", style: (!inner_style.is_empty()).then_some(inner_style),
                if slots.has_header { {header} }
                div { class: middle,
                    if slots.has_start { PanelSlot { is_end: false, {start} } }
                    div { class: "astryx-layout__content-slot", {content.unwrap_or(children)} }
                    if slots.has_end { PanelSlot { is_end: true, {end} } }
                }
                if slots.has_footer { {footer} }
            }
        }
    }
}
