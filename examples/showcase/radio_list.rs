use dioxus::prelude::*;

use astryx_dioxus::{IndicatorSize, RadioList, RadioListItem, RadioListOrientation};

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
            ShippingMethod { id: "radio-list-item-basic", title: "RadioListItem — Basic" }
            ShippingMethod { id: "radio-list-item-showcase", title: "RadioListItem", show_disabled: true }
            RadioListItemInteractions {}
        }
    }
}

#[component]
fn ShippingMethod(
    id: &'static str,
    title: &'static str,
    #[props(default)] show_disabled: bool,
) -> Element {
    let mut selected = use_signal(|| "standard".to_owned());
    rsx! {
        section { id,
            h3 { "{title}" }
            RadioList { label: "Shipping method", value: selected(), on_change: move |value| selected.set(value),
                RadioListItem { label: "Standard", value: "standard", description: Some("5–7 business days".into()) }
                RadioListItem { label: "Express", value: "express", description: Some("2–3 business days".into()) }
                RadioListItem { label: "Overnight", value: "overnight", description: Some("Next business day".into()) }
                if show_disabled {
                    RadioListItem { label: "Same day", value: "sameday", description: Some("Not available in your area".into()), is_disabled: true }
                }
            }
        }
    }
}

#[component]
fn RadioListItemInteractions() -> Element {
    let mut value = use_signal(|| "standard".to_owned());
    let mut is_disabled = use_signal(|| false);
    let mut clicks = use_signal(|| 0u32);
    let mut changes = use_signal(|| 0u32);
    let mut nested = use_signal(|| 0u32);
    rsx! {
        section { id: "radio-list-item-interactions",
            h3 { "RadioListItem interaction coverage" }
            button { id: "radio-list-item-disable", onclick: move |_| is_disabled.toggle(), "Toggle group availability" }
            RadioList { label: "Delivery options", value: value(), size: IndicatorSize::Small, is_disabled: is_disabled(),
                on_change: move |next| { changes += 1; value.set(next); },
                RadioListItem { value: "standard", label: "Standard" }
                RadioListItem { id: "radio-list-item-rich", value: "express", accessible_label: "Express delivery",
                    label: rsx! { strong { "Express" } },
                    description: Some(rsx! { span { "Two days ", a { href: "#radio-list-item-interactions", onclick: move |event| { event.prevent_default(); nested += 1; }, "See details" } } }.into()),
                    onclick: move |_| clicks += 1,
                }
                RadioListItem { id: "radio-list-item-disabled", value: "unavailable", label: "Unavailable", is_disabled: true,
                    onclick: move |_| clicks += 1,
                }
            }
            output { id: "radio-list-item-counts", "{value}:{clicks}:{changes}:{nested}" }
        }
    }
}
