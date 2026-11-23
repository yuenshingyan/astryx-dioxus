use dioxus::prelude::*;

use astryx_dioxus::DateTimeInput;

#[component]
pub(super) fn DateTimeInputExamples() -> Element {
    let mut meeting = use_signal(|| None::<String>);
    let display = meeting().unwrap_or_else(|| "No date and time selected".to_owned());
    rsx! {
        section { id: "date-time-input-examples",
            h2 { "Date and Time Input" }
            form { onsubmit: move |event| event.prevent_default(),
                DateTimeInput { label: "Meeting time", value: meeting(), html_name: "meeting",
                    width: Some(320.into()), on_change: move |next| meeting.set(next),
                }
                output { id: "date-time-input-value", "{display}" }
            }
        }
    }
}
