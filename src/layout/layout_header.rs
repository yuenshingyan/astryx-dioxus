//! Top layout region adapted from Astryx Layout/LayoutHeader.tsx.

use dioxus::core::AttributeValue;
use dioxus::prelude::*;

use super::layout_divider_context::LayoutDividerContext;
use super::padding::spacing_step_css;
use crate::utils::types::{SizeValue, SpacingStep};

/// Non-shrinking header band with a centered content area and optional divider.
#[component]
pub fn LayoutHeader(
    /// Overrides the nearest Layout's divider default.
    #[props(default)]
    has_divider: Option<bool>,
    /// Height of the outer band.
    #[props(default, into)]
    height: Option<SizeValue>,
    /// Overrides the inherited inset on every edge.
    #[props(default)]
    padding: Option<SpacingStep>,
    /// Overrides the block-end inset, including explicit zero.
    #[props(default)]
    padding_block_end: Option<SpacingStep>,
    /// Accessible name when a caller supplies a landmark role.
    #[props(default)]
    label: Option<String>,
    /// Caller CSS classes on the outer band.
    #[props(default)]
    class: String,
    /// Caller inline declarations on the outer band.
    #[props(default)]
    style: String,
    /// Content of the inner wrapper.
    #[props(default = rsx! {})]
    children: Element,
    /// Native outer attributes such as `role`, events, and `onmounted`.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let has_divider = has_divider.unwrap_or_else(|| {
        try_consume_context::<LayoutDividerContext>().is_some_and(|ctx| (ctx.0)())
    });
    let mut outer_style = match height {
        Some(height) => format!("--_layout-header-height:{};", height.as_css()?),
        None => String::new(),
    };
    outer_style.push_str(&style);
    let mut inner_style = String::new();
    if let Some(padding) = padding {
        inner_style.push_str(&format!(
            "--_layout-header-padding:{};",
            spacing_step_css(padding)
        ));
    }
    if let Some(padding) = padding_block_end {
        inner_style.push_str(&format!(
            "--_layout-header-padding-end:{};",
            spacing_step_css(padding)
        ));
    }
    let mut aria_label = label.map_or(AttributeValue::None, AttributeValue::Text);
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter_map(|attr| match attr.name {
            "class" | "style" | "data-divider" => None,
            "aria-label" => {
                // Native attributes follow `label` upstream. Emit only one name
                // so SSR and client DOM agree on which value wins.
                aria_label = attr.value;
                None
            }
            _ => Some(attr),
        })
        .collect();
    rsx! {
        div {
            class: "astryx-layout-header {class}",
            style: (!outer_style.is_empty()).then_some(outer_style),
            "aria-label": aria_label,
            "data-divider": has_divider.then_some("true"),
            ..attributes,
            div { class: "astryx-layout-header__inner", style: (!inner_style.is_empty()).then_some(inner_style), {children} }
        }
    }
}

#[cfg(test)]
mod tests {
    use dioxus::prelude::*;
    use dioxus_ssr::render_element;

    use super::{LayoutDividerContext, LayoutHeader};
    use crate::layer::LayerContentBoundary;

    #[test]
    fn divider_default_is_overridable_and_stops_at_a_layer_boundary() {
        #[component]
        fn Parent(children: Element) -> Element {
            use_context_provider(|| LayoutDividerContext(Signal::new(true)));
            children
        }
        let html = render_element(rsx! {
            Parent {
                LayoutHeader { "Inherited" }
                LayoutHeader { has_divider: false, "Opt out" }
                LayerContentBoundary { LayoutHeader { "New layer" } }
            }
        });
        assert_eq!(html.matches("data-divider=\"true\"").count(), 1, "{html}");
    }
}
