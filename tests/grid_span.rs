use dioxus::core::{AttributeValue, Mutation};
use dioxus::events::{onclick, onmounted};
use dioxus::prelude::*;

use astryx_dioxus::{Grid, GridColumns, GridSpan, GridSpanColumns};

#[path = "../examples/showcase/grid_span.rs"]
mod showcase;

use showcase::GridSpanExamples;

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
    let button = html.find("<button>First</button>").expect("native button");
    let input = html.find("<input value=\"Second\"").expect("native input");
    assert!(button < input);
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

#[test]
fn spans_replace_only_the_requested_axis_inside_grid() {
    let html = render(|| {
        rsx! {
            Grid { columns: GridColumns::Fixed(2), row_height: 80.0,
                GridSpan { columns: GridSpanColumns::Count(19), style: "grid-row:2", "Wide" }
                GridSpan { rows: 21, style: "grid-column:1", "Tall" }
                GridSpan { columns: GridSpanColumns::Count(3), rows: 4, "Combined" }
                GridSpan { style: "grid-column:2;grid-row:3", "Caller placement" }
            }
        }
    });
    for expected in [
        "--_grid-columns:repeat(2, 1fr);--_grid-rows:80px;",
        "style=\"grid-row:2;grid-column:span 19;\">Wide</div>",
        "style=\"grid-column:1;grid-row:span 21;\">Tall</div>",
        "style=\"grid-column:span 3;grid-row:span 4;\">Combined</div>",
        "style=\"grid-column:2;grid-row:3;\">Caller placement</div>",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
}

#[test]
fn named_showcase_patterns_render_the_pinned_compositions() {
    let html = render(|| rsx! { GridSpanExamples {} });
    let mut grids = html.split("<div class=\"astryx-grid ").skip(1);
    for (id, count, spans, card_count) in [
        (
            "grid-span-showcase",
            4,
            vec![
                "grid-column:span 3;",
                "grid-row:span 2;",
                "grid-column:span 3;",
                "grid-column:span 2;",
            ],
            6,
        ),
        (
            "grid-span-columns",
            3,
            vec!["grid-column:span 2;", "grid-column:span 2;"],
            4,
        ),
    ] {
        let pattern = grids.next().expect("named Grid example");
        assert!(pattern.contains(&format!("id=\"{id}\"")));
        assert!(pattern.contains(&format!("--_grid-columns:repeat({count}, 1fr);")));
        assert!(pattern.contains("--_grid-column-gap:var(--spacing-3);"));
        assert!(pattern.contains("width:400px;"));
        assert_eq!(pattern.matches("class=\"astryx-card ").count(), card_count);
        let placement: Vec<_> = pattern
            .split("style=\"")
            .skip(1)
            .filter(|value| value.starts_with("grid-"))
            .filter_map(|value| value.split_once('"').map(|(style, _)| style))
            .collect();
        assert_eq!(placement, spans);
    }
}

#[test]
fn native_mount_and_click_listeners_target_the_wrapper() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            Grid { columns: GridColumns::Fixed(2),
                GridSpan { id: "native-span", attributes: vec![onmounted(|_| {}), onclick(|_| {})],
                    button { id: "native-child", "Child" }
                }
            }
        }
    });
    let mutations = dom.rebuild_to_vec();
    let wrapper = mutations.edits.iter().find_map(|edit| match edit {
        Mutation::SetAttribute {
            name: "id",
            value: AttributeValue::Text(value),
            id,
            ..
        } if value == "native-span" => Some(*id),
        _ => None,
    });
    let wrapper = wrapper.expect("wrapper ID mutation");
    for event in ["mounted", "click"] {
        assert_eq!(
            mutations
                .edits
                .iter()
                .filter_map(|edit| match edit {
                    Mutation::NewEventListener { name, id } if name == event => Some(*id),
                    _ => None,
                })
                .collect::<Vec<_>>(),
            [wrapper],
            "listener should only be installed on the wrapper: {event}"
        );
    }
}
