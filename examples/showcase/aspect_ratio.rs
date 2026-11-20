use astryx_dioxus::{AspectRatio, AspectRatioFit, AspectRatioShape, Skeleton};
use dioxus::events::onclick;
use dioxus::prelude::*;

// Self-contained replacement artwork, not a copy of upstream catalog assets.
const IMAGE: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='100' height='100'%3E%3Cpath fill='%239ac' d='M0 0h100v100H0z'/%3E%3Ccircle fill='%23345' cx='50' cy='50' r='30'/%3E%3C/svg%3E";

#[component]
pub fn AspectRatioExamples() -> Element {
    let mut events = use_signal(|| 0);
    let mut is_loading = use_signal(|| true);
    rsx! {
        document::Style { "
            #aspect-ratio-examples .ar-stage {{ overflow-x:auto; margin-block:16px; }}
            #ar-gallery {{ display:grid;grid-template-columns:repeat(3,1fr);gap:var(--spacing-4);width:600px; }}
            #ar-equal {{ display:flex;gap:var(--spacing-4);align-items:start; }}
            .ar-responsive-stage {{ container:gallery / inline-size; width:600px;max-width:100%; }}
            @container gallery (max-width:400px) {{ .ar-responsive {{ aspect-ratio:3 / 2; }} }}
            @media (max-width:500px) {{ .ar-media {{ aspect-ratio:1; }} }}
        " }
        section { id: "aspect-ratio-examples",
            h2 { "AspectRatio — pinned patterns, local replacement artwork" }
            div { class: "ar-stage",
                div { id: "ar-equal",
                    for (ratio, label) in [(1.0,"1 : 1"),(4.0/3.0,"4 : 3"),(16.0/9.0,"16 : 9")] {
                        div { style: "display:flex;flex-direction:column;align-items:center;gap:var(--spacing-2);",
                            AspectRatio { ratio, fit: AspectRatioFit::Cover, style: "height:120px;width:auto;border-radius:var(--radius-container)", img { src: IMAGE, alt: "{label}" } }
                            span { "{label}" }
                        }
                    }
                }
            }
            div { class: "ar-stage", div { style: "width:600px;", AspectRatio { id: "ar-wide", ratio:16.0/9.0, fit:AspectRatioFit::Cover, img { src:IMAGE, alt:"16:9 widescreen" } } } }
            div { class: "ar-stage", div { style: "width:300px;", AspectRatio { id: "ar-square", ratio:1.0, fit:AspectRatioFit::Cover, img { src:IMAGE, alt:"1:1 square" } } } }
            div { class: "ar-stage", div { style: "width:300px;", AspectRatio { id: "ar-circle", ratio:1.0, shape:AspectRatioShape::Ellipse, fit:AspectRatioFit::Cover, img { src:IMAGE, alt:"Circular image" } } } }
            div { class: "ar-stage", div { id:"ar-gallery",
                for alt in ["Mountain landscape","Ocean sunset","Forest trail","City skyline","Desert dunes","Snowy peaks"] {
                    AspectRatio { ratio:4.0/3.0, fit:AspectRatioFit::Cover, img { src:IMAGE, alt, style:"border-radius:var(--radius-element)" } }
                }
            } }
            div { class:"ar-stage", div { style:"width:600px;", "aria-busy":is_loading(),
                AspectRatio { id:"ar-skeleton", ratio:16.0/9.0,
                    if is_loading() { Skeleton { width:"100%", height:"100%" } } else { img { src:IMAGE, alt:"Loaded preview", style:"width:100%;height:100%;object-fit:cover" } }
                }
            } }
            button { id:"ar-load", onclick:move |_| is_loading.set(false), "Replace caller-owned placeholder" }
            h3 { "Contract probes" }
            div { style:"width:320px;",
                for (id,fit) in [("ar-cover",Some(AspectRatioFit::Cover)),("ar-contain",Some(AspectRatioFit::Contain)),("ar-center",Some(AspectRatioFit::Center)),("ar-unset",None)] {
                    AspectRatio { id, ratio:16.0/9.0, fit, img { src:IMAGE, alt:"Fit probe" } }
                }
                AspectRatio { id:"ar-oval", ratio:2.0, shape:AspectRatioShape::Ellipse, fit:AspectRatioFit::Cover, img { src:IMAGE, alt:"Oval" } }
                AspectRatio { id:"ar-native", ratio:2.0, class:"caller-ratio", lang:"fr", "data-test":"native", "aria-label":"Caller label", tabindex:0,
                    onmounted:move |_| events += 1, attributes:vec![onclick(move |_| events += 1)],
                    button { id:"ar-child", "Child action" }
                }
                p { id:"ar-events", "{events}" }
                AspectRatio { id:"ar-inline", ratio:3.0, style:"aspect-ratio:3 / 2", div {} }
                AspectRatio { id:"ar-media", class:"ar-media", ratio:3.0, div {} }
                AspectRatio { id:"ar-video", ratio:16.0/9.0, fit:AspectRatioFit::Contain, video { "aria-label":"Caller video" } }
            }
            div { class:"ar-responsive-stage", AspectRatio { id:"ar-responsive", class:"ar-responsive", ratio:3.0, div {} } }
        }
    }
}
