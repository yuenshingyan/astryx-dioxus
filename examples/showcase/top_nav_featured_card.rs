use dioxus::prelude::*;

use astryx_dioxus::TopNavMegaMenuFeaturedCard;

const ART: Asset = asset!("/tests/fixtures/media-theme/light-scene-horizontal-1.png");

#[component]
pub(super) fn TopNavFeaturedCardExamples() -> Element {
    rsx! {
        section { id: "top-nav-featured-card-examples", "data-astryx-theme": "neutral",
            h2 { "TopNavMegaMenuFeaturedCard" }
            h3 { "Basic" }
            TopNavMegaMenuFeaturedCard { id: "changelog", style: "max-width:320px",
                title: "What's New", description: "Check out the latest features and improvements in the Q2 release.",
                link_label: "Read the changelog", link_href: "#changelog",
            }
            h3 { "Showcase" }
            TopNavMegaMenuFeaturedCard { id: "release", style: "max-width:320px",
                title: "Introducing v2.0", description: "Our biggest release yet with new components, faster builds, and a refreshed design language.",
                link_label: "Explore the release", link_href: "#release",
            }
            h3 { "Image and RTL" }
            TopNavMegaMenuFeaturedCard { id: "featured-card-rtl", dir: "rtl", style: "width:240px;max-width:100%",
                title: "أخبار المنتج", link_label: "اقرأ المزيد", link_href: "#release", image: ART.to_string(),
            }
        }
    }
}
