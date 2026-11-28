use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::Pagination;

fn render_pages(page: u32, total_pages: u32) -> String {
    let mut dom = VirtualDom::new_with_props(
        |(page, total_pages)| rsx! { Pagination { page, total_pages, on_change: |_| {} } },
        (page, total_pages),
    );
    dom.rebuild_in_place();
    render(&dom)
}

fn assert_page_window(page: u32, total_pages: u32, expected: &str) {
    let html = render_pages(page, total_pages);
    let window = html
        .split('<')
        .filter_map(|part| {
            let (tag, text) = part.split_once('>')?;
            if text == "…" {
                assert!(tag.starts_with("span "), "{tag}");
                assert!(tag.contains("aria-hidden=\"true\""), "{tag}");
                assert!(!tag.contains("tabindex="), "{tag}");
                return Some(text);
            }
            if !tag.starts_with("button ") {
                return None;
            }
            let (_, label) = tag.split_once("aria-label=\"Go to page ")?;
            let (number, _) = label.split_once('"').unwrap();
            assert_eq!(
                tag.contains("aria-current=\"page\""),
                number.parse::<u32>().unwrap() == page,
                "{tag}"
            );
            Some(number)
        })
        .collect::<Vec<_>>()
        .join(" ");
    assert_eq!(window, expected, "page {page} of {total_pages}");
    assert_eq!(html.matches("aria-current=\"page\"").count(), 1);
}

#[test]
fn totals_through_seven_fit_without_ellipsis() {
    assert_page_window(1, 1, "1");
    assert_page_window(4, 7, "1 2 3 4 5 6 7");
}

#[test]
fn eight_pages_switch_between_one_and_two_ellipses() {
    for (page, expected) in [
        (1, "1 2 3 4 5 … 8"),
        (3, "1 2 3 4 5 … 8"),
        (4, "1 … 3 4 5 … 8"),
        (5, "1 … 4 5 6 … 8"),
        (6, "1 … 4 5 6 7 8"),
        (8, "1 … 4 5 6 7 8"),
    ] {
        assert_page_window(page, 8, expected);
    }
}

#[test]
fn large_totals_keep_the_window_bounded_without_overflow() {
    assert_page_window(7, 12, "1 … 6 7 8 … 12");
    assert_page_window(
        u32::MAX,
        u32::MAX,
        "1 … 4294967291 4294967292 4294967293 4294967294 4294967295",
    );
}

#[test]
fn zero_pages_render_no_navigation() {
    assert!(!render_pages(1, 0).contains("<nav"));
}
