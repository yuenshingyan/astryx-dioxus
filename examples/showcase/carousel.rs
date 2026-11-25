use dioxus::prelude::*;

use astryx_dioxus::{Card, Carousel, CarouselSlide, Heading, HeadingLevel, SpacingStep};

#[component]
pub(super) fn CarouselExamples() -> Element {
    let slides = [
        ("Design", "Create wireframes and prototypes."),
        ("Develop", "Build components and pages."),
        ("Test", "Write tests and fix bugs."),
        ("Deploy", "Ship to production."),
        ("Monitor", "Track performance and errors."),
    ]
    .into_iter()
    .map(|(title, body)| CarouselSlide {
        key: title.to_owned(),
        content: rsx! {
            Card { padding: SpacingStep::Three, style: "width:200px",
                Heading { level: HeadingLevel::Five, "{title}" }
                p { "{body}" }
            }
        },
    })
    .collect();
    rsx! {
        section { id: "carousel-examples",
            h2 { "Carousel" }
            div { style: "max-width:500px",
                Carousel { label: "Workflow steps", slides, has_snap: true, has_buttons: false }
            }
            div { style: "width:220px;max-width:100%", dir: "rtl",
                Carousel { label: "Navigation example", slides: ["One", "Two", "Three"].into_iter().map(|name| CarouselSlide {
                    key: name.to_owned(), content: rsx! { div { style: "width:200px;padding-block:16px", "{name}" } },
                }).collect::<Vec<_>>() }
            }
        }
    }
}
