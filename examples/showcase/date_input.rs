use dioxus::prelude::*;

use astryx_dioxus::{DateInput, FieldStatusInput, FieldStatusPlacement, InputStatusType};

#[component]
pub(super) fn DateInputExamples() -> Element {
    let mut date = use_signal(|| None::<String>);
    let display = date().unwrap_or_else(|| "No date selected".to_owned());
    rsx! {
        section { id: "date-input-examples",
            h2 { "Date Input" }
            form { onsubmit: move |event| event.prevent_default(),
                DateInput { label: "Start date", value: date(), html_name: "start", width: Some(320.into()),
                    min: "2024-01-01", max: "2024-12-31", is_required: true, has_clear: true,
                    description: "Choose a date in 2024.", on_change: move |value| date.set(value),
                }
                output { id: "date-input-value", "{display}" }
                DateInput { label: "Unavailable date", value: "2024-03-01", is_disabled: true, has_clear: true, html_name: "unavailable" }
            }
            div { id: "date-input-status-example",
                DateInput { label: "Review date", value: "2024-10-12", width: Some(320.into()),
                    description: "Choose a working day.", status_variant: FieldStatusPlacement::Detached,
                    status: FieldStatusInput { status_type: InputStatusType::Error,
                        message: Some("This date falls on a weekend.".into()), message_id: None },
                }
            }
        }
    }
}
