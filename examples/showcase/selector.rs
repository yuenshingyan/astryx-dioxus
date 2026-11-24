use dioxus::prelude::*;

use astryx_dioxus::{Selector, SelectorChoice};

#[component]
pub(super) fn SelectorExamples() -> Element {
    let mut value = use_signal(|| None::<String>);
    let options = ["Apple", "Banana", "Orange", "Mango", "Pineapple"]
        .into_iter()
        .map(|fruit| SelectorChoice {
            value: fruit.into(),
            label: fruit.into(),
            is_disabled: false,
        })
        .collect();
    rsx! {
        section { id: "selector-examples",
            h2 { "Selector" }
            form { onsubmit: move |event| event.prevent_default(),
                Selector { label: "Fruit", placeholder: "Select a fruit...", width: Some(300.into()),
                    options, value: value(), html_name: "fruit", is_required: true,
                    on_change: move |next: String| {
                        // Keep Mango uncommitted in this demo to exercise caller rejection.
                        if next != "Mango" { value.set(Some(next)); }
                    },
                }
            }
            p { "This demo does not accept Mango." }
        }
    }
}
