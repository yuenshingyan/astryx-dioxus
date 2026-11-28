use dioxus::prelude::*;

use astryx_dioxus::{
    FieldStatusInput, FieldStatusPlacement, InputStatusType, Selector, SelectorChoice,
};

#[component]
pub(super) fn SelectorExamples() -> Element {
    let mut value = use_signal(|| None::<String>);
    let mut is_rejected = use_signal(|| false);
    let status = is_rejected().then(|| FieldStatusInput {
        status_type: InputStatusType::Error,
        message: Some("Mango is not available for this order.".into()),
        message_id: Some("selector-fruit-error".into()),
    });
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
                    description: "Choose a fruit for this order.", status, status_variant: FieldStatusPlacement::Detached,
                    on_change: move |next: String| {
                        // Keep Mango uncommitted in this demo to exercise caller rejection.
                        is_rejected.set(next == "Mango");
                        if next != "Mango" { value.set(Some(next)); }
                    },
                }
            }
            p { "This demo does not accept Mango." }
        }
    }
}
