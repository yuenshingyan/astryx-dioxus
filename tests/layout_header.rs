use dioxus::core::{AttributeValue, Mutation};
use dioxus::events::{onclick, onmounted};
use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{LayoutHeader, SizeValue, SpacingStep};

#[test]
fn default_band_has_no_landmark_or_divider() {
    let html = render_element(rsx! { LayoutHeader { "Dashboard" } });
    assert_eq!(html.matches("<div").count(), 2);
    assert!(html.contains("astryx-layout-header__inner") && html.contains("Dashboard"));
    for absent in ["role=", "aria-label=", "tabindex=", "data-divider="] {
        assert!(!html.contains(absent), "{html}");
    }
}

#[test]
fn caller_attributes_stay_on_the_band_and_zero_overrides_the_block_end() {
    let html = render_element(rsx! {
        LayoutHeader { id: "header", class: "caller", label: "Site tools", role: "banner",
            "data-divider": "spoof", "data-testid": "band", "aria-describedby": "hint",
            has_divider: true, height: SizeValue::from(72), style: "height:80px",
            padding: SpacingStep::Two, padding_block_end: SpacingStep::Zero, "Tools"
        }
    });
    let (outer, inner) = html.split_once('>').unwrap();
    for expected in [
        "id=\"header\"",
        "class=\"astryx-layout-header caller\"",
        "data-testid=\"band\"",
        "aria-describedby=\"hint\"",
        "aria-label=\"Site tools\"",
        "role=\"banner\"",
        "data-divider=\"true\"",
        "--_layout-header-height:72px;height:80px",
    ] {
        assert!(outer.contains(expected), "{html}");
    }
    assert!(inner.contains(
        "--_layout-header-padding:var(--spacing-2);--_layout-header-padding-end:var(--spacing-0)"
    ));
    assert!(!html.contains("spoof"));
}

#[test]
fn padding_inputs_distinguish_automatic_full_bleed_and_an_isolated_edge_override() {
    for (padding, padding_block_end, expected) in [
        (None, None, None),
        (
            Some(SpacingStep::Zero),
            None,
            Some("--_layout-header-padding:var(--spacing-0);"),
        ),
        (
            None,
            Some(SpacingStep::Two),
            Some("--_layout-header-padding-end:var(--spacing-2);"),
        ),
    ] {
        let html = render_element(rsx! { LayoutHeader { padding, padding_block_end, "Tools" } });
        let (outer, rest) = html.split_once('>').unwrap();
        let (inner, _) = rest.split_once('>').unwrap();
        assert!(!outer.contains("style="), "{html}");
        assert_eq!(
            inner
                .split_once("style=\"")
                .map(|(_, style)| style.trim_end_matches('"')),
            expected,
            "{html}"
        );
    }
}

#[test]
fn native_aria_label_overrides_label_and_disabled_divider_cannot_be_spoofed() {
    let html = render_element(rsx! {
        LayoutHeader {
            has_divider: false,
            label: "Fallback",
            "aria-label": "Workspace tools",
            "data-divider": "true",
            role: "region",
            nav { "Tools" }
        }
    });
    let (outer, inner) = html.split_once('>').unwrap();
    assert!(outer.starts_with("<div "), "{html}");
    assert!(outer.contains("role=\"region\""), "{html}");
    assert!(outer.contains("aria-label=\"Workspace tools\""), "{html}");
    assert_eq!(html.matches("aria-label=").count(), 1, "{html}");
    assert!(!html.contains("data-divider="), "{html}");
    assert!(
        inner.starts_with("<div class=\"astryx-layout-header__inner\"><nav>Tools</nav>"),
        "{html}"
    );
}

#[test]
fn native_mount_and_click_listeners_target_the_band() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            LayoutHeader { id: "native-header", attributes: vec![onmounted(|_| {}), onclick(|_| {})],
                button { "New item" }
            }
        }
    });
    let mutations = dom.rebuild_to_vec();
    let root = mutations.edits.iter().find_map(|edit| match edit {
        Mutation::SetAttribute {
            name: "id",
            value: AttributeValue::Text(value),
            id,
            ..
        } if value == "native-header" => Some(*id),
        _ => None,
    });
    let root = root.expect("header ID mutation");
    for event in ["mounted", "click"] {
        let targets: Vec<_> = mutations
            .edits
            .iter()
            .filter_map(|edit| match edit {
                Mutation::NewEventListener { name, id } if name == event => Some(*id),
                _ => None,
            })
            .collect();
        assert_eq!(targets, [root], "{event} should target the outer band");
    }
}
