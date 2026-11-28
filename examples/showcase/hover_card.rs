use dioxus::prelude::*;

use astryx_dioxus::{
    Avatar, AvatarSize, Button, ButtonVariant, ControlledHoverCard, Heading, HeadingLevel,
    HoverCard,
};

#[component]
pub(super) fn HoverCardExamples() -> Element {
    let mut is_open = use_signal(|| false);
    let mut requests = use_signal(|| 0);
    let mut last_request = use_signal(|| None);
    let request_summary = match last_request() {
        Some(true) => " (open)",
        Some(false) => " (close)",
        None => "",
    };
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
            div { id: "controlled-hover-card-example",
                h3 { "Caller-controlled preview" }
                p { "Pointer and focus don't change visibility. Escape requests closure; the caller keeps the preview open until you choose Close preview." }
                button { id: "controlled-hover-card-show", onclick: move |_| is_open.set(true),
                    "Show preview"
                }
                ControlledHoverCard {
                    label: "Project preview",
                    is_open: is_open(),
                    on_open_change: move |next: bool| {
                        requests += 1;
                        last_request.set(Some(next));
                    },
                    content: rsx! {
                        p { "Design review is on Friday." }
                        button { id: "controlled-hover-card-close", onclick: move |_| is_open.set(false),
                            "Close preview"
                        }
                    },
                    button { id: "controlled-hover-card-trigger", "Project notes" }
                }
                output { id: "controlled-hover-card-requests", "Visibility requests: {requests}{request_summary}" }
            }
        }
    }
}
