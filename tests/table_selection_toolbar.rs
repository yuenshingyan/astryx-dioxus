use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{TableSelectionSummary, TableSelectionToolbar};

#[test]
fn zero_selection_omits_toolbar() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            TableSelectionToolbar { selection: TableSelectionSummary { selected_count: 0, clear_selection: EventHandler::new(|()| {}) } }
        }
    });
    dom.rebuild_in_place();
    assert!(!render(&dom).contains("toolbar"));
}

#[test]
fn selected_rows_show_named_toolbar_and_custom_count() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            TableSelectionToolbar { selection: TableSelectionSummary { selected_count: 7, clear_selection: EventHandler::new(|()| {}) },
                render_selection_label: Callback::new(|count| rsx! { "{count} reports" }), clear_label: "Clear selection",
            }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    for expected in [
        "aria-label=\"Bulk actions\"",
        "data-size=\"sm\"",
        "data-variant=\"muted\"",
        "7 reports",
        "Clear selection",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
}
