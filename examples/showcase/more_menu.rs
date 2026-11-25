use dioxus::prelude::*;

use astryx_dioxus::{DropdownMenuItem, MoreMenu};

#[component]
pub(super) fn MoreMenuExamples() -> Element {
    let mut action = use_signal(|| "No action selected");
    rsx! {
        section { id: "more-menu-examples",
            h2 { "More Menu" }
            MoreMenu {
                for label in ["Edit", "Duplicate", "Delete"] {
                    DropdownMenuItem { key: "{label}", label, onclick: move |_| action.set(label) }
                }
            }
            MoreMenu { label: "Unavailable options", is_disabled: true,
                DropdownMenuItem { label: "Edit" }
            }
            output { "{action()}" }
        }
    }
}
