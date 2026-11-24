use dioxus::prelude::*;

use astryx_dioxus::{BaseTypeaheadOption, Typeahead};

#[component]
pub(super) fn TypeaheadExamples() -> Element {
    let mut value = use_signal(|| None::<BaseTypeaheadOption>);
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
                options, value: value(), on_change: move |next| value.set(next),
                description: "Choose a fruit from the results.",
            }
        }
    }
}
