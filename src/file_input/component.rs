use dioxus::core::current_scope_id;
use dioxus::html::FileData;
use dioxus::prelude::*;

use crate::{
    Field, FieldStatusInput, FieldStatusPlacement, InputStatusType, SizeValue, Spinner, SpinnerSize,
};

/// Native file picker. The browser owns the selection and the caller owns what
/// happens to the selected files. The component does not upload or read them.
#[component]
pub fn FileInput(
    label: String,
    #[props(default)] on_change: Option<EventHandler<Vec<FileData>>>,
    #[props(default)] html_name: Option<String>,
    #[props(default)] accept: Option<String>,
    #[props(default)] description: Option<String>,
    /// Caller-supplied feedback; does not filter files or change native validation.
    #[props(default)]
    status: Option<FieldStatusInput>,
    /// Placement of the validation message.
    #[props(default)]
    status_variant: FieldStatusPlacement,
    #[props(default)] is_multiple: bool,
    #[props(default)] is_required: bool,
    #[props(default)] is_disabled: bool,
    /// Block new selections while caller-owned work is in progress.
    /// The current selection remains part of the form.
    #[props(default)]
    is_loading: bool,
    #[props(default)] is_label_hidden: bool,
    #[props(default)] width: Option<SizeValue>,
) -> Element {
    let id = use_hook(|| format!("astryx-file-input-{}", current_scope_id().0));
    let description = description.filter(|text| !text.is_empty());
    let description_id = description.as_ref().map(|_| format!("{id}-description"));
    let severity = status.as_ref().map(|status| status.status_type);
    let status_id = status
        .as_ref()
        .filter(|status| {
            status
                .message
                .as_ref()
                .is_some_and(|message| !message.is_empty())
        })
        .map(|status| {
            status
                .message_id
                .clone()
                .unwrap_or_else(|| format!("{id}-status"))
        });
    let described_by = [description_id.as_deref(), status_id.as_deref()]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join(" ");
    rsx! {
        Field { class: "astryx-file-input", label, input_id: id.clone(), is_label_hidden, is_required, is_disabled,
            width, description, description_id, status, status_variant,
            div { class: "astryx-file-input__entry", "data-loading": is_loading.then_some("true"),
            input { class: "astryx-file-input__control", id, r#type: "file", name: html_name,
                accept, multiple: is_multiple, required: is_required, disabled: is_disabled,
                "aria-describedby": (!described_by.is_empty()).then_some(described_by),
                "aria-invalid": (severity == Some(InputStatusType::Error)).then_some("true"),
                "data-status": severity.map(InputStatusType::as_str),
                "aria-busy": is_loading.then_some("true"),
                onclick: move |event| { if is_loading { event.prevent_default(); } },
                ondrop: move |event| { if is_loading { event.prevent_default(); } },
                onchange: move |event| {
                    if !is_disabled && !is_loading && let Some(handler) = on_change { handler.call(event.files()); }
                },
            }
            if is_loading {
                Spinner { size: SpinnerSize::Small, is_decorative: true }
            }
            }
        }
    }
}
