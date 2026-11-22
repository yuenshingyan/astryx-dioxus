use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::Pagination;

#[test]
fn large_ranges_keep_the_current_page_and_both_endpoints() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            Pagination { page: 7, total_pages: 12, on_change: |_| {} }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    for page in [1, 6, 7, 8, 12] {
        assert!(
            html.contains(&format!("aria-label=\"Go to page {page}\"")),
            "{html}"
        );
    }
    assert!(!html.contains("aria-label=\"Go to page 5\""), "{html}");
    assert_eq!(html.matches("aria-current=\"page\"").count(), 1);
}
