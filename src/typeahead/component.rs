use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use super::{BaseTypeahead, BaseTypeaheadOption};
use crate::{Field, FieldStatusInput, FieldStatusPlacement, InputStatusType, SizeValue};

/// A labeled single-selection field with local typeahead results.
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
    rsx! {
        Field { label: label.clone(), input_id: id.clone(), description, description_id,
            status, status_variant, label_tooltip, width, is_label_hidden, is_disabled,
            BaseTypeahead { label, options, value, on_change, placeholder, input_id: id,
                described_by: (!descriptions.is_empty()).then_some(descriptions), is_invalid,
                is_disabled, has_entries_on_focus,
            }
        }
    }
}
