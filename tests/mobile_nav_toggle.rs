use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{AppShellMobileProvider, AppShellMobileState, MobileNavToggle};

#[test]
fn renders_only_when_mobile_navigation_is_available() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            MobileNavToggle { id: "no-provider" }
            AppShellMobileProvider { state: AppShellMobileState { is_mobile_nav_enabled: true, ..Default::default() }, MobileNavToggle { id: "desktop" } }
            AppShellMobileProvider { state: AppShellMobileState { is_mobile: true, ..Default::default() }, MobileNavToggle { id: "disabled" } }
        }
    });
    dom.rebuild_in_place();
    assert!(!render(&dom).contains("<button"));
}

#[test]
fn shell_owns_expanded_state_and_control_reference() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            AppShellMobileProvider { state: AppShellMobileState {
                is_mobile: true, is_mobile_nav_enabled: true, is_mobile_nav_open: true,
                mobile_nav_id: Some("drawer".into()), ..Default::default()
            },
                MobileNavToggle { "aria-expanded": "false", "aria-controls": "wrong", "data-testid": "custom-toggle" }
            }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    for expected in [
        "aria-label=\"Open navigation\"",
        "aria-expanded=\"true\"",
        "aria-controls=\"drawer\"",
        "data-testid=\"custom-toggle\"",
        "data-variant=\"ghost\"",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
    assert!(!html.contains("wrong"), "{html}");
}
