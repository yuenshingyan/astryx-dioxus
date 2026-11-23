use dioxus::core::current_scope_id;
use dioxus::html::FileData;
use dioxus::prelude::*;

use crate::{FieldLabel, SizeValue};

/// Native file picker. The browser owns the selection and the caller owns what
/// happens to the selected files. The component does not upload or read them.
#[component]
pub fn FileInput(
    label: String,
    #[props(default)] on_change: Option<EventHandler<Vec<FileData>>>,
    #[props(default)] html_name: Option<String>,
    #[props(default)] accept: Option<String>,
    #[props(default)] description: Option<String>,
    #[props(default)] is_multiple: bool,
    #[props(default)] is_required: bool,
    #[props(default)] is_disabled: bool,
    #[props(default)] is_label_hidden: bool,
    #[props(default)] width: Option<SizeValue>,
) -> Element {
    let id = use_hook(|| format!("astryx-file-input-{}", current_scope_id().0));
    let description = description.filter(|text| !text.is_empty());
    let description_id = description.as_ref().map(|_| format!("{id}-description"));
    let style = width
        .map(|size| size.as_css().map(|width| format!("inline-size:{width}")))
        .transpose()?;
    rsx! {
        div { class: "astryx-file-input", style,
            FieldLabel { label, input_id: id.clone(), is_label_hidden, is_required, is_disabled,
                description: description.map(|text| rsx! { "{text}" }), description_id: description_id.clone(),
            }
            input { class: "astryx-file-input__control", id, r#type: "file", name: html_name,
                accept, multiple: is_multiple, required: is_required, disabled: is_disabled,
                "aria-describedby": description_id,
                onchange: move |event| {
                    if !is_disabled && let Some(handler) = on_change { handler.call(event.files()); }
                },
            }
        }
    }
}
