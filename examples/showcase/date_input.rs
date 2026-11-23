use dioxus::prelude::*;

use astryx_dioxus::DateInput;

#[component]
pub(super) fn DateInputExamples() -> Element {
    let mut date = use_signal(|| None::<String>);
    let display = date().unwrap_or_else(|| "No date selected".to_owned());
    rsx! {
        section { id: "date-input-examples",
            h2 { "Date Input" }
            form { onsubmit: move |event| event.prevent_default(),
                DateInput { label: "Start date", value: date(), html_name: "start", width: Some(320.into()),
                    min: "2024-01-01", max: "2024-12-31", is_required: true,
                    description: "Choose a date in 2024.", on_change: move |value| date.set(value),
                }
                output { id: "date-input-value", "{display}" }
                DateInput { label: "Unavailable date", value: "2024-03-01", is_disabled: true, html_name: "unavailable" }
            }
        }
    }
}
