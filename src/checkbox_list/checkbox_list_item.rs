use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::i18n::use_message;
use crate::utils::is_renderable::is_renderable;
use crate::{CheckboxIndicatorState, CheckboxInput, ItemAlign, ItemLabel, ListItem};

/// A caller-controlled checkbox option in a List or CheckboxList.
#[component]
pub fn CheckboxListItem(
    #[props(into)] label: ItemLabel,
    /// Replaces the name derived from the visible label. Retain all visible label words.
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
        ItemLabel::Rich(content) => match accessible_label {
            Some(name) => (ItemLabel::Rich(content), name, None),
            None => (
                ItemLabel::Rich(rsx! { span { id: label_id.clone(), {content} } }),
                use_message("@astryx.checkboxList.item.checkbox", "Checkbox"),
                Some(label_id),
            ),
        },
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
    let is_selected =
        is_checked == CheckboxIndicatorState::Checked && !is_disabled && !is_read_only;
    let state_class = if is_disabled {
        "astryx-checkbox-list-item--disabled"
    } else if is_selected {
        "astryx-checkbox-list-item--selected"
    } else {
        ""
    };
    // Theme state follows the fill, not the checkbox's raw value.
    let mut attributes = attributes;
    attributes.retain(|attribute| attribute.name != "data-selected");
    attributes.push(Attribute::new(
        "data-selected",
        is_selected.then_some("selected"),
        None,
        false,
    ));
    rsx! {
        ListItem { label, description, end_content, align, style, attributes,
            class: format!("astryx-checkbox-list-item {state_class} {class}"),
            start_content: rsx! {
                CheckboxInput { label: input_label, labelled_by, described_by, value: is_checked,
                    is_label_hidden: true, is_disabled, is_read_only, on_change: on_check,
                }
            },
        }
    }
}
