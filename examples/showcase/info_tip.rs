use dioxus::prelude::*;

use astryx_dioxus::InfoTip;

#[component]
pub(super) fn InfoTipExamples() -> Element {
    rsx! {
        section { id: "info-tip-examples",
            h2 { "InfoTip" }
            h3 { "Showcase" }
            p { "Average response time " InfoTip { label: "About this metric", content: "30-day rolling average, recomputed nightly." } }
            h3 { "Metric definitions" }
            dl {
                dt { "Active users " InfoTip { label: "About active users", content: "People who opened the app at least once in the last 28 days." } }
                dd { "48,210" }
                dt { "Retention " InfoTip { label: "About retention", content: "Share of last month’s active users who returned this month." } }
                dd { "62%" }
                dt { "Median session " InfoTip { label: "About median session", content: "Half of all sessions this week were shorter than this." } }
                dd { "6m 40s" }
            }
            h3 { "Permission notes" }
            dl {
                dt { "Owner " InfoTip { label: "Who can change owner", content: "Only admins can change the owner." } }
                dd { "Design Systems" }
                dt { "Visibility " InfoTip { label: "Who can change visibility", content: "Editors can change this field; viewers cannot." } }
                dd { "Everyone in the workspace" }
            }
        }
    }
}
