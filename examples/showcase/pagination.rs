use dioxus::prelude::*;

use astryx_dioxus::{Pagination, PaginationVariant};

#[component]
pub(super) fn PaginationExamples() -> Element {
    let mut page = use_signal(|| 7);
    let mut edge_page = use_signal(|| 1);
    let mut compact_page = use_signal(|| 1);
    rsx! {
        section { id: "pagination-examples",
            h2 { "Pagination" }
            h3 { "Pages — middle window" }
            Pagination { page: page(), total_pages: 12, on_change: move |next| page.set(next) }
            h3 { "Pages — first page" }
            Pagination { label: "Pagination from the first page", page: edge_page(), total_pages: 8,
                on_change: move |next| edge_page.set(next) }
            h3 { "Compact" }
            Pagination { label: "Compact pagination", page: compact_page(), total_pages: 3,
                variant: PaginationVariant::Compact, on_change: move |next| compact_page.set(next) }
        }
    }
}
