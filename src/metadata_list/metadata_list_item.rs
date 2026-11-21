//! Native metadata term/value pair adapted from Astryx MetadataListItem.tsx.

use dioxus::core::AttributeValue;
use dioxus::prelude::*;

use super::metadata_list_context::MetadataListContext;
use crate::utils::is_renderable::is_renderable;

/// A labeled value inside a description list. Its parent controls stacking.
#[component]
pub fn MetadataListItem(
    /// Visible term for the value.
    label: String,
    /// Value associated with the term; may contain links or other rich content.
    children: Element,
    /// Caller-owned artwork before the term.
    #[props(default)]
    icon: Option<Element>,
    /// Test ID for the stacked wrapper, or prefix for the inline label and value IDs.
    #[props(default)]
    test_id: Option<String>,
    /// Classes on the stacked wrapper or inline term.
    #[props(default)]
    class: String,
    /// Inline CSS on the stacked wrapper or inline term.
    #[props(default)]
    style: String,
    /// Attributes and events on the stacked wrapper or inline term.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let is_stacked =
        try_consume_context::<MetadataListContext>().is_some_and(|ctx| (ctx.is_stacked)());
    let mut test_id = test_id;
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attr| {
            if attr.name == "data-testid" {
                if let AttributeValue::Text(value) = &attr.value {
                    test_id = Some(value.clone());
                }
                return false;
            }
            !matches!(attr.name, "class" | "style")
        })
        .collect();
    let term = rsx! {
        if let Some(icon) = icon.filter(is_renderable) {
            span { class: "astryx-metadata-list-item__icon", {icon} }
        }
        {label}
    };
    if is_stacked {
        return rsx! {
            div { class: "astryx-metadata-list-item astryx-metadata-list-item--stacked {class}",
                style: (!style.is_empty()).then_some(style), "data-testid": test_id, ..attributes,
                dt { class: "astryx-metadata-list-item__label", {term} }
                dd { class: "astryx-metadata-list-item__value", {children} }
            }
        };
    }
    let label_id = test_id
        .as_ref()
        .filter(|id| !id.is_empty())
        .map(|id| format!("{id}-label"));
    let value_id = test_id
        .filter(|id| !id.is_empty())
        .map(|id| format!("{id}-value"));
    rsx! {
        dt { class: "astryx-metadata-list-item astryx-metadata-list-item__label {class}",
            style: (!style.is_empty()).then_some(style), "data-testid": label_id, ..attributes,
            {term}
        }
        dd { class: "astryx-metadata-list-item__value", "data-testid": value_id, {children} }
    }
}

#[cfg(test)]
mod tests {
    use dioxus::prelude::*;
    use dioxus_ssr::render_element;

    use super::MetadataListContext;
    use super::MetadataListItem;

    #[component]
    fn StackedList() -> Element {
        use_context_provider(|| MetadataListContext {
            is_stacked: Signal::new(true),
        });
        rsx! { dl { MetadataListItem { label: "Name", id: "group", "data-testid": "item", "Project" } } }
    }

    #[test]
    fn stacked_item_wraps_the_pair_and_owns_passthrough() {
        let html = render_element(rsx! { StackedList {} });
        assert!(html.starts_with("<dl><div"));
        assert!(
            html.contains("data-testid=\"item\" id=\"group\"><dt"),
            "{html}"
        );
        assert!(html.contains("</dt><dd") && html.ends_with("</dd></div></dl>"));
    }
}
