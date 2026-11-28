use dioxus::prelude::*;

use astryx_dioxus::{FieldStatusInput, InputStatusType, MultiSelector, SelectorChoice};

#[component]
pub(super) fn MultiSelectorExamples() -> Element {
    let mut value = use_signal(Vec::<String>::new);
    let mut is_loading = use_signal(|| false);
    let mut is_rejected = use_signal(|| false);
    let status = is_rejected().then(|| FieldStatusInput {
        status_type: InputStatusType::Error,
        message: Some("Created is not available for this report.".into()),
        message_id: Some("multi-selector-columns-error".into()),
    });
    let options = ["Name", "Email", "Role", "Status", "Created"]
        .into_iter()
        .map(|column| SelectorChoice {
            value: column.into(),
            label: column.into(),
            is_disabled: false,
        })
        .collect();
    let selected = value().join(", ");
    rsx! {
        section { id: "multi-selector-examples",
            h2 { "Multi Selector" }
            form { onsubmit: move |event| event.prevent_default(),
                MultiSelector { label: "Columns", options, value: value(), html_name: "columns",
                    is_required: true, width: Some(300.into()), is_loading: is_loading(),
                    description: "Choose columns for the report.", status,
                    on_change: move |next: Vec<String>| {
                        let is_unavailable = next.iter().any(|column| column == "Created");
                        is_rejected.set(is_unavailable);
                        if !is_unavailable { value.set(next); }
                    },
                }
                output { "{selected}" }
            }
            button { id: "multi-selector-toggle-loading", r#type: "button", onclick: move |_| is_loading.toggle(), "Toggle simulated loading" }
            p { "This demo does not accept Created." }
        }
    }
}
