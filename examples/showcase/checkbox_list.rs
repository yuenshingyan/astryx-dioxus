use dioxus::prelude::*;

use astryx_dioxus::{
    CheckboxIndicatorState, CheckboxInput, CheckboxList, FieldStatusInput, InputStatusType,
};

#[component]
pub(super) fn CheckboxListExamples() -> Element {
    let mut preferences = use_signal(|| [true, false, false]);
    rsx! {
        section { id: "checkbox-list-examples",
            h2 { "CheckboxList" }
            p { "CheckboxInput rows manage selection in the caller." }
            CheckboxList { id: "notification-preferences", label: "Notification preferences",
                description: "Choose how you would like to be notified",
                status: FieldStatusInput { status_type: InputStatusType::Warning, message: Some("SMS availability varies by region.".into()), message_id: None },
                for (index, (label, description)) in [
                    ("Email", "Weekly digest every Monday"), ("Push notification", "Instant alerts on your device"), ("SMS", "Standard messaging rates apply"),
                ].into_iter().enumerate() {
                    li { key: "{label}",
                        CheckboxInput { label, description,
                            value: if preferences.read()[index] { CheckboxIndicatorState::Checked } else { CheckboxIndicatorState::Unchecked },
                            on_change: move |checked| preferences.write()[index] = checked,
                        }
                    }
                }
            }
        }
    }
}
