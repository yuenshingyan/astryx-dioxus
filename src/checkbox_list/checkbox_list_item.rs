use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::utils::is_renderable::is_renderable;
use crate::{CheckboxIndicatorState, CheckboxInput, ItemAlign, ItemLabel, ListItem};

/// A caller-controlled checkbox option in a List or CheckboxList.
#[component]
pub fn CheckboxListItem(
    #[props(into)] label: ItemLabel,
    /// Replaces the name derived from the visible label.
    #[props(default)]
    accessible_label: Option<String>,
    #[props(default)] description: Option<ItemLabel>,
    #[props(default)] end_content: Option<Element>,
    #[props(default)] align: ItemAlign,
    #[props(default = CheckboxIndicatorState::Unchecked)] is_checked: CheckboxIndicatorState,
    #[props(default)] on_check: Option<EventHandler<bool>>,
    #[props(default)] is_disabled: bool,
    #[props(default)] is_read_only: bool,
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let id = use_hook(|| format!("astryx-checkbox-list-item-{}", current_scope_id().0));
    let label_id = format!("{id}-label");
    let description_id = format!("{id}-description");
    let (label, input_label, labelled_by) = match label {
        ItemLabel::Text(label) => {
            let input_label = accessible_label.unwrap_or_else(|| label.clone());
            (ItemLabel::Text(label), input_label, None)
        }
        ItemLabel::Rich(content) => {
            let labelled_by = accessible_label.is_none().then(|| label_id.clone());
            let label = rsx! { span { id: label_id, {content} } };
            (
                ItemLabel::Rich(label),
                accessible_label.unwrap_or_default(),
                labelled_by,
            )
        }
    };
    let description = description.and_then(|description| match description {
        ItemLabel::Text(text) if !text.is_empty() => Some(rsx! {
            span { id: description_id.clone(), class: "astryx-checkbox-list-item__description", "{text}" }
        }),
        ItemLabel::Rich(content) if is_renderable(&content) => Some(rsx! { span { id: description_id.clone(), {content} } }),
        _ => None,
    });
    let described_by = description.as_ref().map(|_| description_id);
    let description = description.map(ItemLabel::Rich);
    let selection_class =
        if is_checked == CheckboxIndicatorState::Checked && !is_disabled && !is_read_only {
            "astryx-checkbox-list-item--selected"
        } else {
            ""
        };
    rsx! {
        ListItem { label, description, end_content, align, style, attributes,
            class: format!("astryx-checkbox-list-item {selection_class} {class}"),
            start_content: rsx! {
                CheckboxInput { label: input_label, labelled_by, described_by, value: is_checked,
                    is_label_hidden: true, is_disabled, is_read_only, on_change: on_check,
                }
            },
        }
    }
}
