use dioxus::prelude::*;

use astryx_dioxus::{RadioList, RadioListOrientation};

#[component]
pub(super) fn RadioListExamples() -> Element {
    rsx! {
        section { id: "radio-list-examples",
            h2 { "RadioList" }
            p { "These arrangements use caller-owned native radios." }
            RadioList { label: "Notification preference", id: "notification-radio-group",
                description: "Choose one channel for alerts.",
                for (index, label) in ["Email", "SMS", "Push notification"].into_iter().enumerate() {
                    label { key: "{label}", input { r#type: "radio", name: "notification-channel", checked: index == 0 }, " {label}" }
                }
            }
            RadioList { label: "Size", id: "size-radio-group", orientation: RadioListOrientation::Horizontal,
                for (index, label) in ["Small", "Medium", "Large"].into_iter().enumerate() {
                    label { key: "{label}", input { r#type: "radio", name: "radio-preview-size", checked: index == 1 }, " {label}" }
                }
            }
        }
    }
}
