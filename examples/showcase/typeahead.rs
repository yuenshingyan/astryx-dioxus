use dioxus::prelude::*;

use astryx_dioxus::{BaseTypeaheadOption, Typeahead};

#[component]
pub(super) fn TypeaheadExamples() -> Element {
    let mut value = use_signal(|| None::<BaseTypeaheadOption>);
    let mut should_keep_selection = use_signal(|| false);
    let mut clear_requests = use_signal(|| 0);
    let options = [
        "Apple",
        "Banana",
        "Cherry",
        "Date",
        "Elderberry",
        "Fig",
        "Grape",
        "Honeydew",
    ]
    .into_iter()
    .map(|fruit| BaseTypeaheadOption {
        id: fruit.to_lowercase(),
        label: fruit.into(),
        description: None,
    })
    .collect();
    rsx! {
        section { id: "typeahead-examples",
            h2 { "Typeahead" }
            Typeahead { label: "Fruit", placeholder: "Search fruits...", width: Some(320.into()),
                options, value: value(), on_change: move |next: Option<BaseTypeaheadOption>| {
                    if next.is_none() {
                        clear_requests += 1;
                        if should_keep_selection() { return; }
                    }
                    value.set(next);
                },
                description: "Choose a fruit from the results.",
            }
            label {
                input { id: "typeahead-keep-selection", r#type: "checkbox", checked: should_keep_selection(),
                    onchange: move |event| should_keep_selection.set(event.checked()),
                }
                "Keep the current selection when Clear is requested"
            }
            p { id: "typeahead-clear-requests", "Clear requests: {clear_requests}" }
        }
    }
}
