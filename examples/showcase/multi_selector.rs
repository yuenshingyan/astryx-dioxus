use dioxus::prelude::*;

use astryx_dioxus::{MultiSelector, SelectorChoice};

#[component]
pub(super) fn MultiSelectorExamples() -> Element {
    let mut value = use_signal(Vec::<String>::new);
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
                    is_required: true, width: Some(300.into()),
                    on_change: move |next: Vec<String>| {
                        if !next.iter().any(|column| column == "Created") { value.set(next); }
                    },
                }
                output { "{selected}" }
            }
            p { "This demo does not accept Created." }
        }
    }
}
