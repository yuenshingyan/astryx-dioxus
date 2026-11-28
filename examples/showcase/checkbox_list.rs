use dioxus::prelude::*;

use astryx_dioxus::{
    Badge, CheckboxIndicatorState, CheckboxList, CheckboxListItem, FieldStatusInput,
    InputStatusType,
};

#[component]
pub(super) fn CheckboxListExamples() -> Element {
    let mut preferences = use_signal(|| [true, false, false]);
    let mut disabled_requests = use_signal(|| 0);
    rsx! {
        section { id: "checkbox-list-examples",
            h2 { "CheckboxList" }
            p { "CheckboxListItem rows use caller-owned selection." }
            CheckboxList { id: "notification-preferences", label: "Notification preferences",
                description: "Choose how you would like to be notified",
                status: FieldStatusInput { status_type: InputStatusType::Warning, message: Some("SMS availability varies by region.".into()), message_id: None },
                for (index, (label, description)) in [
                    ("Email", "Weekly digest every Monday"), ("Push notification", "Instant alerts on your device"), ("SMS", "Standard messaging rates apply"),
                ].into_iter().enumerate() {
                    CheckboxListItem { key: "{label}", label, description: Some(description.into()),
                        is_checked: if preferences.read()[index] { CheckboxIndicatorState::Checked } else { CheckboxIndicatorState::Unchecked },
                        on_check: move |checked| preferences.write()[index] = checked,
                    }
                }
            }
            h3 { "CheckboxListItem states" }
            CheckboxList { label: "Access options", id: "checkbox-list-item-states",
                CheckboxListItem { label: rsx! { strong { "Team access" } }, description: Some("Some teams have access".into()),
                    is_checked: CheckboxIndicatorState::Indeterminate, end_content: rsx! { Badge { label: "Partial" } },
                }
                CheckboxListItem { label: "Guest access", description: Some("Managed by your administrator".into()),
                    is_checked: CheckboxIndicatorState::Checked, is_disabled: true,
                    on_check: move |_| disabled_requests += 1,
                }
                CheckboxListItem { label: rsx! { span { "Pro plan " Badge { label: "Recommended" } } },
                    accessible_label: "Pro plan Recommended option".to_owned(),
                }
            }
            p { id: "checkbox-list-disabled-requests", "Guest access change requests: {disabled_requests}" }
        }
    }
}
