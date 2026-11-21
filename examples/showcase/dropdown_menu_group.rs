use dioxus::prelude::*;

use astryx_dioxus::{DropdownMenuDivider, DropdownMenuGroup};

#[component]
pub(super) fn DropdownMenuGroupExamples() -> Element {
    rsx! {
        section { id: "dropdown-menu-group-examples", "data-astryx-theme": "neutral",
            h2 { "DropdownMenuGroup" }
            p { "These examples use native controls until the menu components are available." }
            h3 { "Actions" }
            DropdownMenuGroup { id: "menu-group-history", title: rsx! { "Version history" },
                button { r#type: "button", "Restore this version" }
                button { r#type: "button", "Compare with current" }
            }
            DropdownMenuDivider {}
            DropdownMenuGroup { title: rsx! { "Add to message" },
                button { r#type: "button", "As a link" }
                button { r#type: "button", "As a snippet" }
            }
            h3 { "With selectable rows" }
            DropdownMenuGroup { title: rsx! { "Model" },
                for option in ["Fast", "Balanced", "Thorough"] {
                    label { key: "{option}", input { r#type: "radio", name: "demo-menu-model", initial_checked: option == "Balanced" }, "{option}" }
                }
            }
            DropdownMenuDivider {}
            DropdownMenuGroup { title: rsx! { "On completion" },
                label { input { r#type: "checkbox", initial_checked: true }, "Notify me" }
                label { input { r#type: "checkbox" }, "Retry on failure" }
            }
        }
    }
}
