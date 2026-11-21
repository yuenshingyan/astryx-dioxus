use dioxus::prelude::*;

use astryx_dioxus::TopNavMegaMenuItem;

const BOLT: &str = "m3.75 13.5 10.5-11.25L12 10.5h8.25L9.75 21.75 12 13.5H3.75Z";
const CUBE: &str =
    "m21 7.5-9-5.25L3 7.5m18 0-9 5.25m9-5.25v9l-9 5.25M3 7.5l9 5.25M3 7.5v9l9 5.25m0-9v9";
const GLOBE: &str = "M12 21a9.004 9.004 0 0 0 8.716-6.747M12 21a9.004 9.004 0 0 1-8.716-6.747M12 21c2.485 0 4.5-4.03 4.5-9S14.485 3 12 3m0 18c-2.485 0-4.5-4.03-4.5-9S9.515 3 12 3m0 0a8.997 8.997 0 0 1 7.843 4.582M12 3a8.997 8.997 0 0 0-7.843 4.582m15.686 0A11.953 11.953 0 0 1 12 10.5c-2.998 0-5.74-1.1-7.843-2.918m15.686 0A8.959 8.959 0 0 1 21 12c0 .778-.099 1.533-.284 2.253m0 0A17.919 17.919 0 0 1 12 16.5a17.92 17.92 0 0 1-8.716-2.247m0 0A8.966 8.966 0 0 1 3 12c0-1.264.26-2.466.732-3.558";
const WRENCH: &str = "M11.42 15.17 17.25 21A2.652 2.652 0 0 0 21 17.25l-5.877-5.877M11.42 15.17l2.496-3.03c.317-.384.74-.626 1.208-.766M11.42 15.17l-4.655 5.653a2.548 2.548 0 1 1-3.586-3.586l6.837-5.63m5.108-.233c.55-.164 1.163-.188 1.743-.14a4.5 4.5 0 0 0 4.486-6.336l-3.276 3.277a3.004 3.004 0 0 1-2.25-2.25l3.276-3.276a4.5 4.5 0 0 0-6.336 4.486c.049.58.025 1.192-.14 1.743";

#[component]
fn Glyph(path: &'static str) -> Element {
    rsx! { svg { width: "20", height: "20", view_box: "0 0 24 24", fill: "none", stroke: "currentColor",
        stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round", "aria-hidden": "true",
        path { d: path }
    } }
}

#[component]
fn Tiles(#[props(default)] is_showcase: bool) -> Element {
    rsx! {
        TopNavMegaMenuItem { title: "Edge Functions", description: "Run serverless code at the network edge", href: "#edge", icon: rsx! { Glyph { path: BOLT } } }
        TopNavMegaMenuItem { title: "Storage", description: "Object and file storage for your application", href: "#storage",
            icon: rsx! { Glyph { path: if is_showcase { CUBE } else { "M21 6c0 1.657-4.029 3-9 3S3 7.657 3 6s4.029-3 9-3 9 1.343 9 3ZM3 6v12c0 1.657 4.029 3 9 3s9-1.343 9-3V6M3 12c0 1.657 4.029 3 9 3s9-1.343 9-3" } } }
        }
        if is_showcase {
            TopNavMegaMenuItem { title: "CDN", description: "Global content delivery with instant purging", href: "#cdn", icon: rsx! { Glyph { path: GLOBE } } }
            TopNavMegaMenuItem { title: "Build Tools", description: "Optimized bundling and compilation pipeline", href: "#build", icon: rsx! { Glyph { path: WRENCH } } }
        }
    }
}

#[component]
pub(super) fn TopNavMegaMenuItemExamples() -> Element {
    let mut count = use_signal(|| 0);
    rsx! {
        section { id: "top-nav-mega-menu-item-examples", "data-astryx-theme": "neutral",
            h2 { "TopNavMegaMenuItem" }
            h3 { "Basic" }
            div { id: "mega-item-basic", style: "max-width:360px", Tiles {} }
            h3 { "Showcase" }
            div { id: "mega-item-showcase", style: "max-width:360px", Tiles { is_showcase: true } }
            h3 { "Action and static tile" }
            div { style: "max-width:360px",
                TopNavMegaMenuItem { id: "mega-item-action", title: "Refresh", onclick: move |_| count += 1 }
                TopNavMegaMenuItem { id: "mega-item-static", title: "Updates: {count}" }
            }
        }
    }
}
