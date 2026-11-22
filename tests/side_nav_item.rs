use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{LinkProvider, LinkRenderProps, SideNavItem};

#[test]
fn selected_link_uses_router_and_disabled_item_is_a_button() {
    let mut dom = VirtualDom::new(|| {
        let router = |props: LinkRenderProps| rsx! { a { href: props.href, "data-router": "yes", ..props.attributes, {props.children} } };
        rsx! {
            LinkProvider { component: router,
                SideNavItem { label: "Inbox", href: "/inbox", is_selected: true, selected_icon: rsx! { "Filled" }, icon: rsx! { "Outline" } }
                SideNavItem { label: "Settings", href: "/settings", is_disabled: true }
            }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    assert_eq!(html.matches("data-router=\"yes\"").count(), 1, "{html}");
    for fragment in [
        "aria-current=\"page\"",
        "Filled",
        "<button",
        "aria-disabled=\"true\"",
    ] {
        assert!(html.contains(fragment), "{html}");
    }
    assert!(
        !html.contains("/settings") && !html.contains("Outline"),
        "{html}"
    );
}
