use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{
    ButtonSize, LayerContentBoundary, ToggleButton, ToggleButtonGroup, ToggleButtonGroupSelection,
};

#[test]
fn group_owns_member_state_and_stops_at_layer_boundary() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            ToggleButtonGroup { label: "Views", role: "toolbar", "aria-label": "Wrong", size: ButtonSize::Small, is_disabled: true,
                selection: ToggleButtonGroupSelection::Single { value: Some("grid".into()), onchange: EventHandler::new(|_| {}) },
                ToggleButton { label: "Grid", value: "grid" }
                LayerContentBoundary { ToggleButton { label: "Layer grid", value: "grid" } }
            }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    assert!(
        html.contains("role=\"group\"")
            && html.contains("aria-label=\"Views\"")
            && !html.contains("Wrong"),
        "{html}"
    );
    let buttons: Vec<_> = html
        .match_indices("<button ")
        .map(|(index, _)| html[index..].split('>').next().unwrap())
        .collect();
    assert_eq!(buttons.len(), 2);
    assert!(
        buttons[0].contains("aria-pressed=\"true\"")
            && buttons[0].contains("aria-disabled=\"true\""),
        "{html}"
    );
    assert!(
        buttons[1].contains("aria-pressed=\"false\"")
            && !buttons[1].contains("aria-disabled=\"true\""),
        "{html}"
    );
}
