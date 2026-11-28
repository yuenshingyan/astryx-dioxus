use dioxus::prelude::*;

#[cfg(target_arch = "wasm32")]
use web_sys::Element as WebElement;

use astryx_dioxus::{
    DropdownMenu, DropdownMenuDivider, DropdownMenuItem, DropdownMenuItemTarget,
    DropdownMenuItemVariant,
};

#[component]
pub(super) fn DropdownMenuExamples() -> Element {
    let mut action = use_signal(|| "No action selected");
    let mut divider_clicks = use_signal(|| 0);
    let mut is_pinned = use_signal(|| false);
    rsx! {
        section { id: "dropdown-menu-examples",
            h2 { "Dropdown Menu" }
            DropdownMenu { label: "Actions",
                DropdownMenuItem { label: "Edit", onclick: move |_| action.set("Edit") }
                DropdownMenuItem { label: "Duplicate", is_disabled: true }
                DropdownMenuDivider {
                    id: "dropdown-menu-actions-divider", class: "actions-divider", style: "margin-block: 8px",
                    onmounted: move |_event: MountedEvent| {
                        #[cfg(target_arch = "wasm32")]
                        if let Some(element) = _event.data().downcast::<WebElement>() {
                            element.set_attribute("data-mounted", "true").expect("valid mount marker");
                        }
                    },
                    attributes: vec![onclick(move |_| divider_clicks += 1)],
                }
                DropdownMenuItem { label: "Delete", variant: DropdownMenuItemVariant::Destructive,
                    onclick: move |_| action.set("Delete"),
                }
            }
            output { "data-divider-clicks": "{divider_clicks()}", "{action()}" }
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
