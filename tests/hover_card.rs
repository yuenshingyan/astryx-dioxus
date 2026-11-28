use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{ControlledHoverCard, HoverCard, HoverCardProps, LayerPlacement};

#[test]
fn renders_caller_owned_visibility_and_preserves_uncontrolled_default() {
    let mut dom = VirtualDom::new(|| {
        let uncontrolled = HoverCardProps {
            children: rsx! { button { "Uncontrolled trigger" } },
            content: rsx! { "Uncontrolled preview" },
            label: None,
            placement: LayerPlacement::default(),
            delay: 300,
            hide_delay: 200,
            on_open_change: None,
        };
        rsx! {
            HoverCard { ..uncontrolled }
            ControlledHoverCard {
                is_open: false,
                content: rsx! { "Held closed" },
                button { "Closed trigger" }
            }
            ControlledHoverCard {
                is_open: true,
                label: "Profile preview",
                content: rsx! { "Held open" },
                button { "Open trigger" }
            }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    let surfaces: Vec<_> = html
        .split("<div ")
        .skip(1)
        .filter(|part| part.contains("class=\"astryx-popover astryx-hover-card\""))
        .collect();
    assert_eq!(surfaces.len(), 3, "{html}");
    for (surface, state, content) in [
        (surfaces[0], "false", "Uncontrolled preview"),
        (surfaces[1], "false", "Held closed"),
        (surfaces[2], "true", "Held open"),
    ] {
        assert!(
            surface.contains(&format!("data-open=\"{state}\"")),
            "{surface}"
        );
        assert!(surface.contains(content), "{surface}");
    }
    assert!(surfaces[0].contains("role=\"group\""), "{html}");
    assert!(surfaces[2].contains("role=\"dialog\""), "{html}");
    assert!(
        surfaces[2].contains("aria-label=\"Profile preview\""),
        "{html}"
    );
}
