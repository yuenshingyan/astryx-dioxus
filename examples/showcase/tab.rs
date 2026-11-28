use dioxus::prelude::*;

use astryx_dioxus::{Badge, BadgeVariant, Tab, TabList, TabListMode, TabMode};

#[component]
pub(super) fn TabExamples() -> Element {
    let mut current = use_signal(|| "inbox".to_owned());
    let mut panel = use_signal(|| "overview".to_owned());
    let mut section = use_signal(|| "summary".to_owned());
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
            h3 { "Scrolling tabs" }
            div { id: "overflow-tabs", dir: "ltr", style: "inline-size: 16rem; max-inline-size: 100%;",
                TabList { label: "Project sections", value: section(), has_divider: true, on_change: move |value| section.set(value),
                    Tab { value: "summary", label: "Summary" }
                    Tab { value: "milestones", label: "Milestones" }
                    Tab { value: "contributors", label: "Contributors" }
                    Tab { value: "resources", label: "Resources" }
                    Tab { value: "settings", label: "Project settings" }
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
