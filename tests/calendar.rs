use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::Calendar;

#[test]
fn selected_day_button_name_includes_selection() {
    let mut dom = VirtualDom::new(|| {
        rsx! { Calendar { default_focused_date: "2026-04-15", value: "2026-04-16" } }
    });
    dom.rebuild_in_place();
    let html = render(&dom);

    assert!(html.contains("aria-label=\"Thursday, April 16, 2026, selected\""));
    assert!(html.contains("aria-label=\"Wednesday, April 15, 2026\""));
    assert_eq!(html.matches("aria-selected=\"true\"").count(), 1);
    assert_eq!(html.matches(", selected\"").count(), 1);
}

#[test]
fn an_empty_selection_keeps_date_only_button_names() {
    let mut dom = VirtualDom::new(|| {
        rsx! { Calendar { default_focused_date: "2026-04-15" } }
    });
    dom.rebuild_in_place();
    let html = render(&dom);

    assert!(html.contains("aria-label=\"Wednesday, April 15, 2026\""));
    assert!(!html.contains("aria-selected=\"true\""));
    assert!(!html.contains(", selected\""));
}
