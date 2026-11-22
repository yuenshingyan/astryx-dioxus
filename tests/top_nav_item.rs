use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{LinkProvider, LinkRenderProps, TopNavItem};

#[test]
fn router_receives_only_enabled_destinations() {
    let mut dom = VirtualDom::new(|| {
        let router = |props: LinkRenderProps| rsx! { a { href: props.href, target: props.target, "data-router": "yes", ..props.attributes, {props.children} } };
        rsx! {
            LinkProvider { component: router,
                TopNavItem { label: "Current", href: "/current", target: "_self", is_selected: true }
                TopNavItem { label: "Disabled", href: "/disabled", is_disabled: true }
                TopNavItem { label: "Rejected", href: "javascript:void(0)" }
            }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    assert_eq!(html.matches("data-router=\"yes\"").count(), 1, "{html}");
    assert_eq!(html.matches("aria-disabled=\"true\"").count(), 2, "{html}");
    assert!(
        html.contains("aria-current=\"page\"")
            && html.contains("href=\"/current\"")
            && html.contains("target=\"_self\""),
        "{html}"
    );
    assert!(
        !html.contains("/disabled") && !html.contains("javascript:"),
        "{html}"
    );
}
