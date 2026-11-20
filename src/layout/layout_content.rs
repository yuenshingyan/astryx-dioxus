//! Content region adapted from Astryx Layout/LayoutContent.tsx.

use dioxus::core::{AttributeValue, current_scope_id};
use dioxus::prelude::*;

use super::layout_slots_context::LayoutSlotsContext;
use super::padding::spacing_step_css;
use crate::hooks::use_scroll_region_props::use_scroll_region;
use crate::utils::types::SpacingStep;

/// A Layout content region that owns padding and optionally scrolls its children.
#[component]
pub fn LayoutContent(
    /// Uniform inset; zero enables full bleed.
    #[props(default)]
    padding: Option<SpacingStep>,
    /// Scroll overflow rather than clipping it.
    #[props(default = true)]
    is_scrollable: bool,
    /// Caller-provided accessible name.
    #[props(default)]
    label: Option<String>,
    /// Localizable fallback name for an unnamed overflowing region.
    #[props(default = "Content".into())]
    scroll_label: String,
    /// Caller classes on the region.
    #[props(default)]
    class: String,
    /// Caller inline CSS on the region.
    #[props(default)]
    style: String,
    /// Content of the region, with no intervening wrapper.
    #[props(default = rsx! {})]
    children: Element,
    /// Native root attributes, including role, events, and mounted callbacks.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    const MARKER: &str = "data-astryx-content-scroll";
    let slots = try_consume_context::<LayoutSlotsContext>().unwrap_or_default();
    let has_attribute = |name| {
        attributes
            .iter()
            .any(|attr| attr.name == name && !matches!(attr.value, AttributeValue::None))
    };
    let id = format!("layout-content-{}", current_scope_id().0);
    let should_measure = is_scrollable && !has_attribute("role");
    let is_group = use_scroll_region(MARKER, id.clone(), false, should_measure);
    let has_label = has_attribute("aria-label");
    let label = if has_label {
        None
    } else {
        label.or_else(|| is_group.then_some(scroll_label))
    };
    let mut classes = "astryx-layout-content".to_owned();
    for (name, enabled) in [
        ("scrollable", is_scrollable),
        ("full-bleed", padding == Some(SpacingStep::Zero)),
        ("open-start", !slots.has_start),
        ("open-end", !slots.has_end),
        ("open-header", !slots.has_header),
        ("open-footer", !slots.has_footer),
    ] {
        if enabled {
            classes.push_str(" astryx-layout-content--");
            classes.push_str(name);
        }
    }
    classes.push(' ');
    classes.push_str(&class);
    let mut css = padding
        .map(|step| format!("--_layout-content-padding:{};", spacing_step_css(step)))
        .unwrap_or_default();
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
            "data-astryx-content-scroll": should_measure.then_some(id),
            role: is_group.then_some("group"), "aria-label": label,
            ..attributes,
            {children}
        }
    }
}
