use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::form_layout::form_layout_context::{FormLayoutContext, FormOptionality};
use crate::utils::is_renderable::is_renderable;
use crate::{
    ElementSize, Field, FieldStatusInput, FieldStatusPlacement, InputStatusType, SizeValue,
    use_size,
};

/// A labeled, controlled, native multiline text field.
#[component]
pub fn TextArea(
    label: String,
    value: String,
    /// Requests an updated value after native input.
    #[props(default)]
    on_change: Option<EventHandler<String>>,
    #[props(default)] id: Option<String>,
    #[props(default)] html_name: Option<String>,
    #[props(default)] placeholder: Option<String>,
    #[props(default)] description: Option<String>,
    #[props(default)] label_tooltip: Option<String>,
    #[props(default)] status: Option<FieldStatusInput>,
    #[props(default)] status_variant: FieldStatusPlacement,
    #[props(default)] start_icon: Option<Element>,
    #[props(default)] is_label_hidden: bool,
    #[props(default)] is_required: bool,
    #[props(default)] is_optional: bool,
    #[props(default)] is_disabled: bool,
    #[props(default)] is_read_only: bool,
    /// Native UTF-16 limit; it does not truncate a caller-supplied value.
    #[props(default)]
    html_max_length: Option<u32>,
    #[props(default = 3)] rows: u32,
    #[props(default)] size: Option<ElementSize>,
    #[props(default)] width: Option<SizeValue>,
    #[props(default)] class: String,
    #[props(default)] style: String,
) -> Element {
    let generated_id = use_hook(|| format!("astryx-text-area-{}", current_scope_id().0));
    let id = id.unwrap_or(generated_id);
    let input_id = id.clone();
    let size = use_size(size).as_str();
    let description = description.filter(|text| !text.is_empty());
    let description_id = description.as_ref().map(|_| format!("{id}-desc"));
    let severity = status.as_ref().map(|status| status.status_type);
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
    let is_effectively_required = !is_optional
        && (is_required
            || try_consume_context::<FormLayoutContext>().is_some_and(|context| {
                context().default_optionality == Some(FormOptionality::Required)
            }));
    let start_icon = start_icon.filter(is_renderable);
    let has_start_icon = start_icon.is_some();
    let name = if is_disabled { None } else { html_name };
    let rows = rows.max(1);
    // HTML strips one leading newline when parsing a textarea's initial content.
    let initial_text = if value.starts_with('\n') {
        format!("\n{value}")
    } else {
        value.clone()
    };
    rsx! {
        Field { class: format!("astryx-text-area-field {class}"), style, width,
            label, input_id: id, description, description_id, label_tooltip, status, status_variant,
            is_label_hidden, is_required, is_optional, is_disabled,
            div { class: "astryx-text-area", "data-size": size, "data-status": severity.map(InputStatusType::as_str),
                "data-has-icon": has_start_icon.then_some("true"), "data-disabled": is_disabled.then_some("true"),
                if let Some(icon) = start_icon {
                    span { class: "astryx-text-area__icon", "aria-hidden": "true", {icon} }
                }
                textarea { class: "astryx-text-area-control", id: input_id, name, placeholder, rows,
                    maxlength: html_max_length, readonly: is_disabled || is_read_only,
                    "aria-required": is_effectively_required.then_some("true"), "aria-disabled": is_disabled.then_some("true"),
                    "aria-invalid": (severity == Some(InputStatusType::Error)).then_some("true"),
                    "aria-describedby": (!descriptions.is_empty()).then_some(descriptions),
                    value,
                    oninput: move |event| {
                        if !is_disabled && !is_read_only && let Some(handler) = on_change { handler.call(event.value()); }
                    },
                    "{initial_text}"
                }
            }
        }
    }
}
