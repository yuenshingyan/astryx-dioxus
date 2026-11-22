use dioxus::prelude::*;

use astryx_dioxus::{Badge, BadgeVariant, Tab};

#[component]
pub(super) fn TabExamples() -> Element {
    let mut current = use_signal(|| "inbox".to_owned());
    rsx! {
        section { id: "tab-examples",
            h2 { "Tab" }
            div { class: "showcase-row",
                Tab { value: "inbox", label: "Inbox", is_selected: current() == "inbox", tabindex: 0,
                    on_select: move |value| current.set(value),
                    end_content: rsx! { Badge { label: "3", variant: BadgeVariant::INFO } },
                }
                Tab { value: "archive", label: "Archive", is_selected: current() == "archive", tabindex: 0,
                    on_select: move |value| current.set(value),
                }
                Tab { value: "drafts", label: "Drafts", is_disabled: true, tabindex: 0,
                    on_select: move |value| current.set(value),
                }
            }
        }
    }
}
