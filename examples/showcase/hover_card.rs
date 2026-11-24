use dioxus::prelude::*;

use astryx_dioxus::{Avatar, AvatarSize, Button, ButtonVariant, Heading, HeadingLevel, HoverCard};

#[component]
pub(super) fn HoverCardExamples() -> Element {
    rsx! {
        section { id: "hover-card-examples",
            h2 { "Hover Card" }
            HoverCard {
                content: rsx! {
                    div { style: "inline-size:240px;max-inline-size:100%",
                        div { style: "display:flex;align-items:center;gap:var(--spacing-2)",
                            Avatar { name: "Jane Doe", size: AvatarSize::Large }
                            div {
                                Heading { level: HeadingLevel::Five, "Jane Doe" }
                                span { "Software Engineer" }
                            }
                        }
                        p { "Building great products with great people." }
                    }
                },
                Button { id: "hover-card-trigger", label: "@janedoe", variant: ButtonVariant::Ghost }
            }
        }
    }
}
