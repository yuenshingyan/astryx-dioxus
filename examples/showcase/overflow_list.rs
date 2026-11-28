use dioxus::prelude::*;

use astryx_dioxus::{
    Button, ButtonSize, ButtonVariant, OverflowCollapseFrom, OverflowItem, OverflowListProps,
    OverflowListWithNotifications, SpacingStep,
};

#[component]
pub(super) fn OverflowListExamples() -> Element {
    let mut is_reordered = use_signal(|| false);
    let mut reported = use_signal(Vec::<String>::new);
    let mut reports = use_signal(|| 0);
    let mut labels = ["Edit", "Duplicate", "Share", "Archive", "Delete"];
    if is_reordered() {
        labels.swap(3, 4);
    }
    let items = labels
        .into_iter()
        .map(|label| rsx! { Button { key: "{label}", label, size: ButtonSize::Small } })
        .collect();
    let list = OverflowListProps {
        items,
        gap: SpacingStep::Two,
        min_visible_items: 0,
        max_visible_items: None,
        collapse_from: OverflowCollapseFrom::End,
        overflow_renderer: Some(Callback::new(|items: Vec<OverflowItem>| {
            rsx! {
                Button { label: format!("+{} more", items.len()), variant: ButtonVariant::Ghost, size: ButtonSize::Small }
            }
        })),
    };
    let hidden = if reported.read().is_empty() {
        "none".to_owned()
    } else {
        reported.read().join(", ")
    };
    rsx! {
        section { id: "overflow-list-examples",
            h2 { "Overflow List" }
            div { id: "overflow-list-frame", style: "width:400px;max-width:100%;border:1px dashed var(--color-border);padding:8px;box-sizing:border-box",
                OverflowListWithNotifications {
                    list,
                    on_overflow_change: move |items: Vec<OverflowItem>| {
                        reported.set(items.iter().map(|item| format!("{}: {}", item.index, labels[item.index])).collect());
                        reports += 1;
                    },
                }
            }
            Button {
                label: "Swap last two actions",
                onclick: move |_| is_reordered.toggle(),
            }
            output { id: "overflow-list-report", "Reports: {reports}. Hidden: {hidden}" }
        }
    }
}
