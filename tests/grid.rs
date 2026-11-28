use astryx_dioxus::{Grid, GridAlignment, GridColumns, GridRepeat, SizeValue, SpacingStep};
use dioxus::prelude::*;

fn render(component: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(component);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[test]
fn native_root_keeps_order_and_adds_no_accessibility_semantics() {
    let html = render(|| {
        rsx! { Grid { id: "native-grid", "data-caller": "kept",
            button { "First" } a { href: "#native-grid", "Second" }
        } }
    });
    assert_eq!(html.matches("<div").count(), 1);
    assert!(html.contains("--_grid-columns:1fr"));
    assert!(html.contains("data-caller=\"kept\""));
    assert!(html.find("First") < html.find("Second"));
    for absent in [
        "role=",
        "tabindex=",
        "aria-",
        "data-columns=",
        "data-align=",
    ] {
        assert!(!html.contains(absent), "{html}");
    }
}

#[test]
fn caller_root_props_compose_without_replacing_alignment_or_dimensions() {
    let html = render(|| {
        rsx! { Grid {
            id: "named-grid", "aria-label": "Results", class: "caller-layout",
            align: GridAlignment::Start, justify: GridAlignment::Stretch,
            "data-align": "spoof", "data-justify": "spoof", "data-gap": "spoof",
            style: "color:red;width:1px;height:2px;max-width:3px;min-height:4px",
            width: SizeValue::from("100%"), height: SizeValue::from(80),
            max_width: SizeValue::from("40rem"), min_height: SizeValue::from(40),
            "Caller content"
        } }
    });
    for expected in [
        "id=\"named-grid\"",
        "aria-label=\"Results\"",
        "class=\"astryx-grid caller-layout\"",
        "data-align=\"start\"",
        "data-justify=\"stretch\"",
        "color:red;width:1px;height:2px;max-width:3px;min-height:4px;width:100%;height:80px;max-width:40rem;min-height:40px;",
        "Caller content",
    ] {
        assert!(html.contains(expected), "missing {expected:?}: {html}");
    }
    assert!(!html.contains("spoof"), "{html}");
    assert!(!html.contains("data-gap="), "{html}");
}

#[test]
fn responsive_cap_uses_column_gap_and_reflects_only_numeric_columns() {
    let html = render(|| {
        rsx! { Grid {
            columns: GridColumns::Responsive { min_width: 120.0, max: Some(4), repeat: GridRepeat::Fit },
            gap: SpacingStep::Four, column_gap: SpacingStep::Zero, row_gap: SpacingStep::Two,
            align: GridAlignment::End, justify: GridAlignment::Center, row_height: 72.5,
            style: "width:1px", width: SizeValue::from(500),
        } }
    });
    for expected in [
        "repeat(auto-fit, minmax(min(100%, max(120px, calc((100% - 3 * var(--spacing-0)) / 4))), 1fr))",
        "--_grid-rows:72.5px",
        "--_grid-row-gap:var(--spacing-2)",
        "--_grid-column-gap:var(--spacing-0)",
        "data-align=\"end\"",
        "data-justify=\"center\"",
        "width:1px;width:500px",
    ] {
        assert!(html.contains(expected), "{html}");
    }
    assert!(!html.contains("data-columns="));
}

#[test]
fn fixed_counts_and_browser_invalid_input_fallbacks() {
    let html = render(|| {
        rsx! {
            Grid { columns: GridColumns::Fixed(4), "data-columns": "spoof" }
            Grid { columns: GridColumns::Fixed(-2) }
            Grid { columns: GridColumns::Responsive { min_width: f64::NAN, max: None, repeat: GridRepeat::Fill }, row_height: -1.0 }
            Grid { columns: GridColumns::Responsive { min_width: 100.0, max: Some(0), repeat: GridRepeat::Fill } }
        }
    });
    for expected in [
        "repeat(4, 1fr)",
        "data-columns=\"4\"",
        "data-columns=\"-2\"",
        "--_grid-columns:1fr",
        "repeat(auto-fill, minmax(100px, 1fr))",
    ] {
        assert!(html.contains(expected), "{html}");
    }
    for absent in ["spoof", "NaN", "--_grid-rows"] {
        assert!(!html.contains(absent), "{html}");
    }
}
