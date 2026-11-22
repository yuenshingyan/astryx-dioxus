use dioxus::prelude::*;

use astryx_dioxus::{RadioList, RadioListItem, RadioListOrientation};

#[component]
pub(super) fn RadioListExamples() -> Element {
    let mut size = use_signal(|| "Medium".to_owned());
    rsx! {
        section { id: "radio-list-examples",
            h2 { "RadioList" }
            p { "Notification preference uses caller-owned radios; Size uses controlled RadioListItem rows." }
            RadioList { label: "Notification preference", id: "notification-radio-group",
                description: "Choose one channel for alerts.",
                for (index, label) in ["Email", "SMS", "Push notification"].into_iter().enumerate() {
                    label { key: "{label}", input { r#type: "radio", name: "notification-channel", checked: index == 0 }, " {label}" }
                }
            }
            RadioList { label: "Size", id: "size-radio-group", orientation: RadioListOrientation::Horizontal,
                value: size(), on_change: move |value| size.set(value), html_name: "radio-preview-size",
                RadioListItem { value: "Small", label: "Small", is_disabled: true }
                RadioListItem { value: "Medium", label: "Medium" }
                RadioListItem { value: "Large", label: rsx! { strong { "Large" } }, description: Some("Extra room".into()) }
            }
        }
    }
}
