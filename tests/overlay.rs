use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{Overlay, OverlayAlign, OverlayPosition, OverlayScrimMode, OverlayShowOn};

#[test]
fn defaults_mount_base_and_dark_scrim_in_order() {
    let html = render_element(
        rsx! { Overlay { content: rsx! { span { "Details" } }, img { alt: "Landscape" } } },
    );
    assert!(
        html.contains("data-position=\"fill\"") && html.contains("data-align=\"end\""),
        "{html}"
    );
    assert!(
        html.contains("data-astryx-media=\"dark\"") && html.contains("data-reveal=\"open\""),
        "{html}"
    );
    assert!(html.find("Landscape").unwrap() < html.find("Details").unwrap());
}

#[test]
fn explicit_close_wins_over_reveal_and_disables_interaction() {
    let html = render_element(
        rsx! { Overlay { scrim: OverlayScrimMode::None, position: OverlayPosition::Top, align: OverlayAlign::Start,
            show_on: OverlayShowOn::Always, is_open: false, content: rsx! { button { "Details" } }, id: "media",
        } },
    );
    for expected in [
        "id=\"media\"",
        "data-reveal=\"closed\"",
        "data-position=\"top\"",
        "data-align=\"start\"",
        " inert",
        "<button>Details</button>",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
    assert!(!html.contains("data-astryx-media"));
}
