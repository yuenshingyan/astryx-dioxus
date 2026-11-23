use dioxus::prelude::*;

use astryx_dioxus::NumberInput;

#[component]
pub(super) fn NumberInputExamples() -> Element {
    let mut quantity = use_signal(|| Some(0.0_f64));
    let display = quantity().map_or_else(|| "Empty".to_owned(), |value| value.to_string());
    rsx! {
        section { id: "number-input-examples",
            h2 { "Number Input" }
            form { onsubmit: move |event| event.prevent_default(),
                NumberInput { label: "Quantity", value: quantity(), placeholder: "Enter quantity",
                    width: Some(300.into()), html_name: "quantity", min: 0.0, max: 10.0, step: 1.0,
                    description: "Enter a whole number from 0 to 10.",
                    on_change: move |value| quantity.set(value),
                }
                output { id: "number-input-value", "{display}" }
                NumberInput { label: "Read-only quantity", value: 3.0, html_name: "fixed", is_read_only: true }
                NumberInput { label: "Unavailable quantity", value: 4.0, html_name: "unavailable", is_disabled: true }
            }
        }
    }
}
