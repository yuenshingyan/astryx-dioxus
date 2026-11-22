use dioxus::prelude::*;

use astryx_dioxus::{Banner, BannerStatus, Button, ButtonVariant};

#[component]
pub(super) fn BannerExamples() -> Element {
    let mut retried = use_signal(|| false);
    rsx! {
        section { id: "banner-examples",
            h2 { "Banner" }
            Banner { status: BannerStatus::INFO, title: "A new software update is available." }
            Banner { status: BannerStatus::SUCCESS, title: "Your changes have been saved." }
            Banner { status: BannerStatus::WARNING, title: "Your trial expires in 3 days.",
                description: rsx! { "Update your plan to keep access." },
            }
            Banner { id: "payment-banner", status: BannerStatus::ERROR, title: "Payment failed.", is_urgent: true, is_dismissable: true,
                description: rsx! { "Check your payment details." },
                end_content: rsx! { Button { label: "Retry", variant: ButtonVariant::Ghost, onclick: move |_| retried.set(true) } },
            }
            output { id: "banner-demo-result", if retried() { "Retry requested" } }
        }
    }
}
