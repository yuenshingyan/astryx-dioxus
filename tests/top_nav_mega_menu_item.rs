use dioxus::prelude::*;
use dioxus_ssr::{render, render_element};

use astryx_dioxus::TopNavMegaMenuItem;

#[test]
fn static_and_link_forms_have_native_roots() {
    let html =
        render_element(rsx! { TopNavMegaMenuItem { title: "Storage", description: "Files" } });
    assert!(
        html.starts_with("<div ") && html.contains("Files") && !html.contains("tabindex"),
        "{html}"
    );
    let html = render_element(
        rsx! { TopNavMegaMenuItem { title: "Storage", href: "/files", tabindex: "-1" } },
    );
    assert!(
        html.starts_with("<a ")
            && html.contains("href=\"/files\"")
            && html.contains("tabindex=\"-1\""),
        "{html}"
    );
}

#[test]
fn standalone_action_does_not_submit_forms() {
    let mut dom =
        VirtualDom::new(|| rsx! { TopNavMegaMenuItem { title: "Refresh", onclick: |_| {} } });
    dom.rebuild_in_place();
    let html = render(&dom);
    assert!(
        html.starts_with("<button ") && html.contains("type=\"button\""),
        "{html}"
    );
}
