use dioxus::prelude::*;

use astryx_dioxus::{DropdownMenu, DropdownMenuItem, DropdownMenuItemTarget};

#[component]
pub(super) fn DropdownMenuExamples() -> Element {
    let mut action = use_signal(|| "No action selected");
    let mut is_pinned = use_signal(|| false);
    rsx! {
        section { id: "dropdown-menu-examples",
            h2 { "Dropdown Menu" }
            DropdownMenu { label: "Actions",
                for label in ["Edit", "Duplicate", "Delete"] {
                    DropdownMenuItem { key: "{label}", label, is_disabled: label == "Duplicate", onclick: move |_| action.set(label) }
                }
            }
            output { "{action()}" }
            button { r#type: "button", id: "dropdown-menu-after", "After menu" }
            div { id: "dropdown-menu-keep-open",
                DropdownMenu { label: "Preview options",
                    DropdownMenuItem { label: if is_pinned() { "Unpin preview" } else { "Pin preview" },
                        has_close_on_select: false, onclick: move |_| is_pinned.toggle(),
                    }
                    DropdownMenuItem { label: "Done" }
                }
            }
            div { id: "dropdown-menu-link-rows",
                DropdownMenu { label: "Destinations",
                    DropdownMenuItem { label: "Calendar examples", href: "#calendar-examples", onclick: move |_| action.set("Opened calendar") }
                    DropdownMenuItem { label: "Unavailable destination", href: "#calendar-examples", is_disabled: true }
                    DropdownMenuItem { label: "Blocked destination", href: "javascript:void(0)" }
                    DropdownMenuItem { label: "Carousel in a new tab", href: "#carousel-examples", target: DropdownMenuItemTarget::NewTab }
                }
            }
        }
    }
}
