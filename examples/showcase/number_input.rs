use dioxus::prelude::*;

use astryx_dioxus::{FieldStatusInput, FieldStatusPlacement, InputStatusType, NumberInput};

#[component]
pub(super) fn NumberInputExamples() -> Element {
    let mut quantity = use_signal(|| Some(0.0_f64));
    let mut budget = use_signal(|| Some(-5.0));
    let mut headcount = use_signal(|| Some(150.0));
    let mut completion = use_signal(|| Some(25.0));
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
            div { id: "number-input-statuses",
                h3 { "Statuses" }
                NumberInput { label: "Budget", value: budget(), width: Some(300.into()),
                    description: "Enter the proposed budget.", on_change: move |next| budget.set(next),
                    status: FieldStatusInput { status_type: InputStatusType::Error, message: Some("Must be a positive amount".into()), message_id: Some("budget-status".into()) },
                }
                NumberInput { label: "Headcount", value: headcount(), width: Some(300.into()),
                    on_change: move |next| headcount.set(next), status_variant: FieldStatusPlacement::Detached,
                    status: FieldStatusInput { status_type: InputStatusType::Warning, message: Some("Exceeds typical team size".into()), message_id: None },
                }
                NumberInput { label: "Completion (%)", value: completion(), width: Some(300.into()),
                    on_change: move |next| completion.set(next),
                    status: FieldStatusInput { status_type: InputStatusType::Success, message: Some("On track".into()), message_id: None },
                }
            }
        }
    }
}
