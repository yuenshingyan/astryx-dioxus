use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{AppShell, AppShellVariant, LayoutHeight};

#[test]
fn layout_boundary_is_on_the_shell_root() {
    let mut dom = VirtualDom::new(|| rsx! { AppShell { class: "caller-shell", "Workspace" } });
    dom.rebuild_in_place();
    let html = render(&dom);
    let root = html.split('>').next().unwrap();
    assert!(
        root.contains("class=\"astryx-app-shell astryx-layout-root caller-shell\""),
        "{html}"
    );
    assert_eq!(html.matches("astryx-layout-root").count(), 1, "{html}");
}

#[test]
fn skip_link_targets_main_with_or_without_navigation() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            AppShell { variant: AppShellVariant::Section, top_nav: rsx! { "Navigation" }, "Workspace" }
            AppShell { height: LayoutHeight::Auto, "Standalone content" }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    let targets: Vec<_> = html
        .split("href=\"#")
        .skip(1)
        .map(|tail| tail.split('"').next().unwrap())
        .collect();
    assert_eq!(targets.len(), 2, "{html}");
    assert_ne!(targets[0], targets[1]);
    for id in targets {
        assert!(html.contains(&format!("id=\"{id}\"")), "{html}");
    }
    assert_eq!(html.matches("role=\"main\"").count(), 2, "{html}");
    assert_eq!(html.matches("role=\"banner\"").count(), 1, "{html}");
}
