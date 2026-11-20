use dioxus::prelude::*;

use astryx_dioxus::{GridSpan, GridSpanColumns};

fn render(component: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(component);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[test]
fn default_wrapper_preserves_native_child_order_and_semantics() {
    let html =
        render(|| rsx! { GridSpan { id: "span", button { "First" } input { value: "Second" } } });
    assert_eq!(html.matches("<div").count(), 1);
    assert!(html.contains("astryx-grid-span"));
    assert!(html.find("First") < html.find("Second"));
    for absent in ["role=", "tabindex=", "aria-", "grid-column", "grid-row"] {
        assert!(!html.contains(absent), "{html}");
    }
}

#[test]
fn span_props_compose_with_class_and_override_caller_placement() {
    let html = render(|| {
        rsx! { GridSpan {
            class: "caller", style: "grid-column:2;grid-row:4;color:red",
            columns: GridSpanColumns::Full, rows: 3,
            "aria-label": "Caller label", "data-case": "full",
        } }
    });
    for expected in [
        "astryx-grid-span caller",
        "grid-column:2;grid-row:4;color:red;grid-column:1 / -1;grid-row:span 3;",
        "aria-label=\"Caller label\"",
        "data-case=\"full\"",
    ] {
        assert!(html.contains(expected), "{html}");
    }
}

#[test]
fn numeric_columns_and_unset_or_zero_placement() {
    let html = render(|| {
        rsx! {
            GridSpan { columns: GridSpanColumns::Count(2) }
            GridSpan { columns: GridSpanColumns::Count(0), rows: 0, style: "grid-column:2;grid-row:4;" }
        }
    });
    assert!(html.contains("grid-column:span 2;"));
    assert!(html.contains("grid-column:2;grid-row:4;"));
    assert!(!html.contains("span 0"));
}
