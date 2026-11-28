use dioxus::prelude::*;

use astryx_dioxus::{Button, Lightbox, LightboxMedia};

const COAST: Asset = asset!("/tests/fixtures/lightbox/lightbox.png");
const COAST_ALT: &str = "Coastal shoreline with ocean waves";
const COAST_CAPTION: &str =
    "A scenic coastline with waves rolling onto a sandy beach beneath a clear sky.";

#[component]
pub(super) fn LightboxExamples() -> Element {
    let mut open = use_signal(|| false);
    let mut long_caption_open = use_signal(|| false);
    rsx! {
        section { id: "lightbox-examples",
            h2 { "Lightbox" }
            Button { id: "lightbox-trigger", label: "View image", onclick: move |_| open.set(true) }
            Lightbox { is_open: open(), on_open_change: move |next| open.set(next),
                media: LightboxMedia {
                    src: COAST.to_string(),
                    alt: COAST_ALT.into(),
                    caption: Some(rsx! { "{COAST_CAPTION}" }),
                },
            }
            Button { id: "lightbox-long-caption-trigger", label: "View image with a long caption",
                onclick: move |_| long_caption_open.set(true),
            }
            Lightbox { is_open: long_caption_open(), on_open_change: move |next| long_caption_open.set(next),
                media: LightboxMedia {
                    src: COAST.to_string(),
                    alt: COAST_ALT.into(),
                    caption: Some(rsx! {
                        "{COAST_CAPTION} The view follows the beach down to a rocky headland. "
                        "Bands of white foam mark where waves have broken, and the wet sand reflects "
                        "the open sky. Farther offshore, the water changes from turquoise to deep blue. "
                        "At low tide a narrow path opens below the cliffs, linking two coves that are "
                        "otherwise separated by the sea. From the headland you can see the curve of "
                        "the bay and the thin line where the swells meet the shore. The path is visible "
                        "only briefly; when the water returns, the two coves are cut off from each other. "
                        "The trail follows the rise and fall of the tide."
                    }),
                },
            }
        }
    }
}
