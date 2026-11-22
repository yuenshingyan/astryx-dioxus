use dioxus::prelude::*;

use astryx_dioxus::{Badge, BadgeVariant, Tab, TabList, TabListMode, TabMode};

#[component]
pub(super) fn TabExamples() -> Element {
    let mut current = use_signal(|| "inbox".to_owned());
    let mut panel = use_signal(|| "overview".to_owned());
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
            h3 { "Panel tabs" }
            div { dir: "rtl",
                TabList { label: "Project views", mode: TabListMode::Panels, value: panel(), has_divider: true, on_change: move |value| panel.set(value),
                    Tab { id: "overview-tab", value: "overview", label: "Overview", mode: TabMode::Panel { panel_id: "overview-panel".into() } }
                    Tab { value: "draft", label: "Draft", is_disabled: true }
                    Tab { id: "activity-tab", value: "activity", label: "Activity", mode: TabMode::Panel { panel_id: "activity-panel".into() } }
                }
                div { id: "overview-panel", role: "tabpanel", "aria-labelledby": "overview-tab", hidden: panel() != "overview", "Project overview" }
                div { id: "activity-panel", role: "tabpanel", "aria-labelledby": "activity-tab", hidden: panel() != "activity", "Recent activity" }
            }
        }
    }
}
