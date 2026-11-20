//! Side region adapted from Astryx Layout/LayoutPanel.tsx.

use dioxus::core::{AttributeValue, current_scope_id};
use dioxus::prelude::*;

use super::layout_area_context::LayoutAreaContext;
use super::layout_slots_context::LayoutSlotsContext;
use super::padding::spacing_step_css;
use crate::hooks::use_scroll_region_props::use_scroll_region;
use crate::utils::types::{SizeValue, SpacingStep};

/// Non-shrinking side panel with optional scrolling and an inset divider.
#[component]
pub fn LayoutPanel(
    /// Panel width in pixels or CSS. Numeric zero clips the content.
    #[props(default, into)]
    width: Option<SizeValue>,
    /// Draw a divider toward the content when placed in a Layout side slot.
    #[props(default)]
    has_divider: bool,
    /// Uniform inset; zero removes padding.
    #[props(default)]
    padding: Option<SpacingStep>,
    /// Scroll overflowing content rather than clipping it.
    #[props(default = true)]
    is_scrollable: bool,
    /// Caller-provided accessible name.
    #[props(default)]
    label: Option<String>,
    /// Localizable fallback name for an unnamed overflowing panel.
    #[props(default = "Content".into())]
    scroll_label: String,
    /// Caller CSS classes on the panel.
    #[props(default)]
    class: String,
    /// Caller inline declarations on the panel.
    #[props(default)]
    style: String,
    /// Panel content.
    #[props(default = rsx! {})]
    children: Element,
    /// Native attributes and callbacks on the panel, including `role` and `onmounted`.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    const MARKER: &str = "data-astryx-panel-scroll";
    let area = try_consume_context::<LayoutAreaContext>().unwrap_or_default();
    let slots = try_consume_context::<LayoutSlotsContext>().unwrap_or_default();
    let is_zero_width = matches!(width, Some(SizeValue::Pixels(value)) if value == 0.0);
    let width = width.map(|value| value.as_css()).transpose()?;
    let has_attribute = |name| {
        attributes
            .iter()
            .any(|attr| attr.name == name && !matches!(attr.value, AttributeValue::None))
    };
    let id = format!("layout-panel-{}", current_scope_id().0);
    let can_scroll = is_scrollable && !is_zero_width;
    let should_measure = can_scroll && !has_attribute("role");
    let is_group = use_scroll_region(MARKER, id.clone(), false, should_measure);
    let label = if has_attribute("aria-label") {
        None
    } else {
        label.or_else(|| is_group.then_some(scroll_label))
    };
    let mut classes = String::from("astryx-layout-panel");
    for (name, enabled) in [
        ("start", area.is_end_panel == Some(false)),
        ("end", area.is_end_panel == Some(true)),
        ("open-header", !slots.has_header),
        ("open-footer", !slots.has_footer),
        ("divider", has_divider),
        (
            "collapse-gap",
            !has_divider && padding.is_none() && !is_zero_width,
        ),
        ("scrollable", can_scroll),
        ("zero-width", is_zero_width),
    ] {
        if enabled {
            classes.push_str(" astryx-layout-panel--");
            classes.push_str(name);
        }
    }
    if !class.is_empty() {
        classes.push(' ');
        classes.push_str(&class);
    }
    let mut css = width
        .map(|value| format!("width:{value};"))
        .unwrap_or_default();
    if let Some(padding) = padding {
        css.push_str(&format!(
            "--_layout-panel-padding:{};",
            spacing_step_css(padding)
        ));
    }
    css.push_str(&style);
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attr| {
            !matches!(attr.name, "class" | "style" | MARKER)
                && !(matches!(attr.value, AttributeValue::None)
                    && matches!(attr.name, "role" | "aria-label"))
        })
        .collect();
    rsx! {
        div {
            class: classes, style: (!css.is_empty()).then_some(css),
            "data-astryx-panel-scroll": should_measure.then_some(id),
            role: is_group.then_some("group"), "aria-label": label,
            ..attributes,
            {children}
        }
    }
}
