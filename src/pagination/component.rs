use dioxus::prelude::*;

use crate::i18n::use_message;
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
    let previous = use_message("@astryx.pagination.previous", "Previous page");
    let next = use_message("@astryx.pagination.next", "Next page");
    rsx! {
        nav { class: "astryx-pagination", "aria-label": label,
            Button { label: previous, variant: ButtonVariant::Ghost,
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
            Button { label: next, variant: ButtonVariant::Ghost,
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
        rsx! { span { class: "astryx-pagination__ellipsis", "aria-hidden": "true", "…" } }
    }
}

fn entry_key(entry: Option<u32>, index: usize) -> String {
    entry.map_or_else(|| format!("gap-{index}"), |number| format!("page-{number}"))
}

fn page_range(current: u32, total: u32) -> Vec<Option<u32>> {
    const SIBLINGS: u32 = 1;
    const EDGE_PAGES: u32 = 3 + 2 * SIBLINGS;
    const SLOTS: u32 = EDGE_PAGES + 2;

    if total <= SLOTS {
        return (1..=total).map(Some).collect();
    }

    // Fill the window from an endpoint when the adjacent gap would hide at most one page.
    if current <= 2 + SIBLINGS {
        return (1..=EDGE_PAGES)
            .map(Some)
            .chain([None, Some(total)])
            .collect();
    }
    if current >= total - 1 - SIBLINGS {
        return [Some(1), None]
            .into_iter()
            .chain((total - EDGE_PAGES + 1..=total).map(Some))
            .collect();
    }
    [Some(1), None]
        .into_iter()
        .chain((current - SIBLINGS..=current + SIBLINGS).map(Some))
        .chain([None, Some(total)])
        .collect()
}
