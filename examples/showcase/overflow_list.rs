use dioxus::prelude::*;

use astryx_dioxus::{Button, ButtonSize, ButtonVariant, OverflowItem, OverflowList};

#[component]
pub(super) fn OverflowListExamples() -> Element {
    let items = ["Edit", "Duplicate", "Share", "Archive", "Delete"]
        .into_iter()
        .map(|label| rsx! { Button { label, size: ButtonSize::Small } })
        .collect();
    rsx! {
        section { id: "overflow-list-examples",
            h2 { "Overflow List" }
            div { id: "overflow-list-frame", style: "width:400px;max-width:100%;border:1px dashed var(--color-border);padding:8px;box-sizing:border-box",
                OverflowList { items, overflow_renderer: |items: Vec<OverflowItem>| rsx! {
                    Button { label: format!("+{} more", items.len()), variant: ButtonVariant::Ghost, size: ButtonSize::Small }
                } }
            }
        }
    }
}
