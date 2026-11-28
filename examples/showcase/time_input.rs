use dioxus::prelude::*;

use astryx_dioxus::{FieldStatusInput, FieldStatusPlacement, InputStatusType, TimeInput};

#[component]
pub(super) fn TimeInputExamples() -> Element {
    let mut time = use_signal(|| None::<String>);
    let display = time().unwrap_or_else(|| "No time selected".to_owned());
    rsx! {
        section { id: "time-input-examples",
            h2 { "Time Input" }
            form { onsubmit: move |event| event.prevent_default(),
                TimeInput { label: "Time", value: time(), html_name: "time", width: Some(320.into()),
                    min: "09:00", max: "17:00", has_clear: true, description: "Choose a time from 09:00 to 17:00.",
                    on_change: move |value| time.set(value),
                }
                output { id: "time-input-value", "{display}" }
                TimeInput { label: "Time with seconds", value: "12:34:56", has_seconds: true, html_name: "precise" }
                TimeInput { label: "Unavailable time", value: "12:00", is_disabled: true, has_clear: true, html_name: "unavailable" }
            }
            div { id: "time-input-status-example",
                TimeInput { label: "Review time", value: "12:00", width: Some(320.into()),
                    description: "Choose a time outside the lunch break.", status_variant: FieldStatusPlacement::Detached,
                    status: FieldStatusInput { status_type: InputStatusType::Error,
                        message: Some("This time overlaps the lunch break.".into()), message_id: None },
                }
            }
        }
    }
}
