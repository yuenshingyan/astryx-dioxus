use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{ElementSize, NavHeadingMenu, NavHeadingMenuItem};

#[test]
fn menu_owns_role_and_cascades_size_to_rows() {
    let html = render_element(rsx! {
        NavHeadingMenu { size: ElementSize::Large, min_width: Some("260px".into()),
            "aria-label": "Products", role: "navigation",
            NavHeadingMenuItem { label: rsx! { "Dashboard" } }
        }
    });
    assert!(
        html.contains("role=\"menu\"") && !html.contains("role=\"navigation\""),
        "{html}"
    );
    assert!(
        html.contains("min-width:260px") && html.contains("aria-label=\"Products\""),
        "{html}"
    );
    assert_eq!(html.matches("data-size=\"lg\"").count(), 2, "{html}");
}
