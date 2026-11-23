use dioxus::prelude::*;

use astryx_dioxus::{Button, Lightbox, LightboxMedia};

const COAST: Asset = asset!("/tests/fixtures/lightbox/lightbox.png");

#[component]
pub(super) fn LightboxExamples() -> Element {
    let mut open = use_signal(|| false);
    rsx! {
        section { id: "lightbox-examples",
            h2 { "Lightbox" }
            Button { id: "lightbox-trigger", label: "View image", onclick: move |_| open.set(true) }
            Lightbox { is_open: open(), on_open_change: move |next| open.set(next),
                media: LightboxMedia {
                    src: COAST.to_string(),
                    alt: "Coastal shoreline with ocean waves".into(),
                    caption: Some(rsx! { "A scenic coastline with waves rolling onto a sandy beach beneath a clear sky." }),
                },
            }
        }
    }
}
