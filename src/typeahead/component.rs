use dioxus::core::current_scope_id;
use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use web_sys::{Element as DomElement, HtmlElement, MouseEvent as DomMouseEvent, window};

use super::{BaseTypeahead, BaseTypeaheadOption};
use crate::field::input_clear_button::InputClearButton;
use crate::{Field, FieldStatusInput, FieldStatusPlacement, InputStatusType, SizeValue};

/// A labeled single-selection field with local typeahead results.
/// Clearing an enabled selection requests `on_change(None)`.
#[component]
pub fn Typeahead(
    label: String,
    options: Vec<BaseTypeaheadOption>,
    value: Option<BaseTypeaheadOption>,
    on_change: EventHandler<Option<BaseTypeaheadOption>>,
    #[props(default)] placeholder: Option<String>,
    #[props(default)] description: Option<String>,
    #[props(default)] status: Option<FieldStatusInput>,
    #[props(default)] status_variant: FieldStatusPlacement,
    #[props(default)] label_tooltip: Option<String>,
    #[props(default)] width: Option<SizeValue>,
    #[props(default)] is_label_hidden: bool,
    #[props(default)] is_disabled: bool,
    #[props(default)] has_entries_on_focus: bool,
) -> Element {
    let id = use_hook(|| format!("astryx-typeahead-input-{}", current_scope_id().0));
    let description_id = description
        .as_ref()
        .filter(|text| !text.is_empty())
        .map(|_| format!("{id}-desc"));
    let status_id = status
        .as_ref()
        .filter(|status| status.message.as_ref().is_some_and(|text| !text.is_empty()))
        .map(|status| {
            status
                .message_id
                .clone()
                .unwrap_or_else(|| format!("{id}-status"))
        });
    let descriptions = [description_id.as_deref(), status_id.as_deref()]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join(" ");
    let is_invalid = status
        .as_ref()
        .is_some_and(|status| status.status_type == InputStatusType::Error);
    let should_show_clear = value.is_some() && !is_disabled;
    let clear_label = format!("Clear {label} selection");
    #[cfg(target_arch = "wasm32")]
    let input_id = id.clone();
    rsx! {
        Field { label: label.clone(), input_id: id.clone(), description, description_id,
            status, status_variant, label_tooltip, width, is_label_hidden, is_disabled,
            div { class: "astryx-typeahead__control",
                BaseTypeahead { label, options, value, on_change, placeholder, input_id: id,
                    described_by: (!descriptions.is_empty()).then_some(descriptions), is_invalid,
                    is_disabled, has_entries_on_focus,
                }
                if should_show_clear {
                    InputClearButton { label: clear_label, onclick: move |event: MouseEvent| {
                        event.stop_propagation();
                        #[cfg(target_arch = "wasm32")]
                        let input = prepare_clear(&input_id, &event);
                        on_change.call(None);
                        #[cfg(target_arch = "wasm32")]
                        if let Some(input) = input { let _ = input.focus(); }
                    } }
                }
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn prepare_clear(id: &str, event: &MouseEvent) -> Option<HtmlElement> {
    let document = window()?.document()?;
    let input = document
        .get_element_by_id(id)?
        .dyn_into::<HtmlElement>()
        .ok()?;
    if document.active_element().as_ref() != Some(input.as_ref()) {
        return Some(input);
    }
    // Pointer presses on the shared clear button keep input focus. Move focus
    // within the field on activation so BaseTypeahead can reset its draft on blur.
    if let Some(button) = event
        .data()
        .downcast::<DomMouseEvent>()
        .and_then(|event| event.target())
        .and_then(|target| target.dyn_into::<DomElement>().ok())
        .and_then(|target| target.closest("button").ok().flatten())
        .and_then(|button| button.dyn_into::<HtmlElement>().ok())
    {
        let _ = button.focus();
    }
    Some(input)
}
