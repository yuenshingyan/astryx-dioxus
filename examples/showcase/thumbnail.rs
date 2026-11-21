use dioxus::prelude::*;

use astryx_dioxus::{Thumbnail, ThumbnailRemoveVisibility};

const FOREST: Asset = asset!("/tests/fixtures/thumbnail/night_forest.png");
const VALLEY: Asset = asset!("/tests/fixtures/thumbnail/misty_valley.png");
const SUNSET: Asset = asset!("/tests/fixtures/thumbnail/golden_sunset.png");
const PEAKS: Asset = asset!("/tests/fixtures/thumbnail/snowy_peaks.png");
const ATTACHMENTS: [(Asset, &str, &str); 4] = [
    (FOREST, "Dark forest at night", "forest-night.jpg"),
    (VALLEY, "Misty mountain valley", "misty-valley.jpg"),
    (SUNSET, "Golden sunset over mountains", "golden-sunset.jpg"),
    (PEAKS, "Snowy mountain peaks", "snowy-peaks.jpg"),
];

#[component]
fn AttachmentList(id: &'static str, #[props(default)] can_open: bool) -> Element {
    let mut removed = use_signal(|| [false; 4]);
    let mut selected = use_signal(|| None::<&'static str>);
    rsx! {
        div { id,
            div { style: "display:flex;gap:var(--spacing-2);flex-wrap:wrap",
                for (index, (asset, alt, label)) in ATTACHMENTS.into_iter().enumerate() {
                    if !removed()[index] {
                        Thumbnail { key: "{label}", src: asset.to_string(), alt, label,
                            show_remove_on: ThumbnailRemoveVisibility::Always,
                            on_click: can_open.then(|| EventHandler::new(move |_| selected.set(Some(label)))),
                            on_remove: move |_| removed.write()[index] = true,
                        }
                    }
                }
            }
            if let Some(label) = selected() { p { "Selected: {label}" } }
        }
    }
}

#[component]
pub(super) fn ThumbnailExamples() -> Element {
    let mut phase = use_signal(|| 0);
    let mut opens = use_signal(|| 0);
    rsx! {
        section { id: "thumbnail-examples", "data-astryx-theme": "neutral",
            h2 { "Thumbnail" }
            h3 { "Showcase" }
            Thumbnail { src: PEAKS.to_string(), alt: "Snowy mountain peaks", label: "snowy-peaks.jpg" }
            h3 { "States" }
            div { style: "display:flex;gap:var(--spacing-2)",
                Thumbnail { label: "report.pdf" }
                Thumbnail { label: "uploading.jpg", is_loading: true }
                Thumbnail { src: VALLEY.to_string(), alt: "Misty mountain valley", label: "misty-valley.jpg", is_loading: true }
                Thumbnail { src: VALLEY.to_string(), alt: "Misty mountain valley", label: "misty-valley.jpg" }
            }
            h3 { "Removable" }
            AttachmentList { id: "thumbnail-removable" }
            h3 { "Gallery" }
            AttachmentList { id: "thumbnail-gallery", can_open: true }
            h3 { "Disabled" }
            div { style: "display:flex;gap:var(--spacing-2)",
                for is_disabled in [false, true] {
                    Thumbnail { src: SUNSET.to_string(), alt: "Golden sunset over mountains", label: "golden-sunset.jpg", is_disabled,
                        show_remove_on: if is_disabled { ThumbnailRemoveVisibility::Hover } else { ThumbnailRemoveVisibility::Always }, on_remove: |_| {} }
                    Thumbnail { label: "document.pdf", is_disabled,
                        show_remove_on: if is_disabled { ThumbnailRemoveVisibility::Hover } else { ThumbnailRemoveVisibility::Always }, on_remove: |_| {} }
                }
            }
            h3 { "Source and upload changes" }
            button { id: "thumbnail-cycle", onclick: move |_| phase.set((phase() + 1) % 3), "Change preview" }
            Thumbnail { id: "thumbnail-state", src: if phase() == 0 { "/missing-thumbnail.png".to_owned() } else { PEAKS.to_string() },
                alt: "Snowy mountain peaks", label: "Test preview", is_loading: phase() == 2,
                on_click: move |_| opens += 1, on_remove: |_| {}, }
            output { id: "thumbnail-opens", "{opens}" }
        }
    }
}
