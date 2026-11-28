use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{Tab, TabList, TabListMode};

#[test]
fn navigation_strip_has_no_extra_tab_stop_or_server_guessed_fades() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            TabList { label: "Project sections", value: "settings", has_divider: true, on_change: |_| {},
                Tab { value: "summary", label: "Summary", href: "/summary" }
                Tab { value: "settings", label: "Settings", href: "/settings" }
            }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    assert!(html.starts_with("<nav "), "{html}");
    assert!(html.contains("aria-label=\"Project sections\""), "{html}");
    assert!(html.contains("data-divider=\"true\""), "{html}");
    assert!(html.contains("aria-current=\"true\""), "{html}");
    assert_eq!(html.matches("tabindex=0").count(), 1, "{html}");
    assert!(!html.contains("data-tab-overflow"), "{html}");
}

#[test]
fn panel_strip_keeps_the_widget_role_on_the_named_element() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            TabList { mode: TabListMode::Panels, label: "Project views", value: "activity", on_change: |_| {},
                Tab { value: "overview", label: "Overview" }
                Tab { value: "activity", label: "Activity" }
            }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    let strip = html.split('>').next().unwrap();
    assert!(strip.contains("role=\"tablist\""), "{html}");
    assert!(strip.contains("aria-label=\"Project views\""), "{html}");
    assert_eq!(html.matches("role=\"tab\"").count(), 2, "{html}");
    assert_eq!(html.matches("aria-selected=\"true\"").count(), 1, "{html}");
    assert_eq!(html.matches("tabindex=0").count(), 1, "{html}");
}
