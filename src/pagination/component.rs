use dioxus::prelude::*;

use crate::{Button, ButtonVariant};

/// Presentation of the page position between the navigation controls.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum PaginationVariant {
    /// Numbered page buttons with ellipses for omitted ranges.
    #[default]
    Pages,
    /// A textual current-page and total-page summary.
    Compact,
}

/// Controlled navigation for a finite set of one-based pages.
#[component]
pub fn Pagination(
    page: u32,
    total_pages: u32,
    on_change: EventHandler<u32>,
    #[props(default)] variant: PaginationVariant,
    #[props(default)] is_disabled: bool,
    #[props(default = "Pagination".to_owned())] label: String,
) -> Element {
    if total_pages == 0 {
        return VNode::empty();
    }
    let current = page.clamp(1, total_pages);
    rsx! {
        nav { class: "astryx-pagination", "aria-label": label,
            Button { label: "Previous page", variant: ButtonVariant::Ghost,
                is_disabled: is_disabled || current == 1,
                onclick: move |_| on_change.call(current.saturating_sub(1)),
            }
            match variant {
                PaginationVariant::Compact => rsx! { span { class: "astryx-pagination__summary", "Page {current} of {total_pages}" } },
                PaginationVariant::Pages => rsx! {
                    for (index, entry) in page_range(current, total_pages).into_iter().enumerate() {
                        PageEntry { key: "{entry_key(entry, index)}", entry, current, is_disabled, on_change }
                    }
                },
            }
            Button { label: "Next page", variant: ButtonVariant::Ghost,
                is_disabled: is_disabled || current == total_pages,
                onclick: move |_| on_change.call(current.saturating_add(1)),
            }
        }
    }
}

#[component]
fn PageEntry(
    entry: Option<u32>,
    current: u32,
    is_disabled: bool,
    on_change: EventHandler<u32>,
) -> Element {
    if let Some(number) = entry {
        rsx! {
            Button { label: "Go to page {number}", variant: ButtonVariant::Ghost,
                is_disabled, "aria-current": (number == current).then_some("page"),
                "aria-label": "Go to page {number}",
                onclick: move |_| { if number != current { on_change.call(number); } },
                children: rsx! { "{number}" },
            }
        }
    } else {
        rsx! { span { "aria-hidden": "true", "…" } }
    }
}

fn entry_key(entry: Option<u32>, index: usize) -> String {
    entry.map_or_else(|| format!("gap-{index}"), |number| format!("page-{number}"))
}

// The pinned default is one sibling on each side of the current page.
fn page_range(current: u32, total: u32) -> Vec<Option<u32>> {
    if total <= 7 {
        return (1..=total).map(Some).collect();
    }
    if current <= 3 {
        return (1..=5).map(Some).chain([None, Some(total)]).collect();
    }
    if current >= total - 2 {
        return [Some(1), None]
            .into_iter()
            .chain((total - 4..=total).map(Some))
            .collect();
    }
    vec![
        Some(1),
        None,
        Some(current - 1),
        Some(current),
        Some(current + 1),
        None,
        Some(total),
    ]
}
