use dioxus::prelude::*;

use astryx_dioxus::{Button, ElementSize, SectionDivider, Toolbar, ToolbarOrientation};

#[component]
pub(super) fn ToolbarExamples() -> Element {
    rsx! {
        section { id: "toolbar-examples",
            h2 { "Toolbar" }
            h3 { "Three slots" }
            Toolbar { label: "Document actions", id: "toolbar-three", dividers: vec![SectionDivider::Bottom],
                start_content: rsx! { Button { id: "toolbar-edit", label: "Edit" } },
                center_content: rsx! { span { "Report" } },
                end_content: rsx! { Button { id: "toolbar-share", label: "Share" } Button { id: "toolbar-export", label: "Export" } },
            }
            h3 { "Table filter" }
            Toolbar { label: "Table filters", size: ElementSize::Small,
                start_content: rsx! { input { "aria-label": "Filter rows" } },
                end_content: rsx! { Button { label: "Apply" } },
            }
            h3 { "Vertical" }
            Toolbar { label: "Layout actions", orientation: ToolbarOrientation::Vertical,
                start_content: rsx! { Button { label: "Align" } },
                end_content: rsx! { Button { label: "Distribute" } },
            }
        }
    }
}
