use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{SideNavItem, SideNavSection};

#[path = "../examples/showcase/side_nav_section.rs"]
mod showcase;

use showcase::SideNavSectionExamples;

fn title_reference(group: &str) -> &str {
    group
        .split_once("aria-labelledby=\"")
        .and_then(|(_, tail)| tail.split_once('"'))
        .map(|(id, _)| id)
        .expect("group name reference")
}

#[test]
fn catalog_examples_nest_named_groups_and_current_links_in_the_rail() {
    let html = render_element(rsx! { SideNavSectionExamples {} });
    let mut ids = Vec::new();
    for (label, has_settings) in [
        ("Basic navigation groups", false),
        ("Navigation groups showcase", true),
    ] {
        let nav = html
            .split("<nav ")
            .filter_map(|tail| tail.split_once("</nav>"))
            .map(|(nav, _)| nav)
            .find(|nav| nav.contains(&format!("aria-label=\"{label}\"")))
            .expect("named rail landmark");
        let groups: Vec<_> = nav.split("role=\"group\"").skip(1).collect();
        assert_eq!(groups.len(), 2, "{nav}");
        for (group, title, item) in [
            (groups[0], "Overview", "Dashboard"),
            (groups[1], "Account", "Profile"),
        ] {
            let id = title_reference(group);
            assert!(!ids.contains(&id), "duplicate title ID {id}");
            ids.push(id);
            let (_, title_and_items) = group
                .split_once(&format!("id=\"{id}\""))
                .expect("title target inside group");
            let (title_markup, items) = title_and_items
                .split_once("<a ")
                .expect("items follow the title");
            assert!(title_markup.contains(&format!(">{title}</span>")));
            let (first_link, _) = items.split_once("</a>").expect("first item link");
            assert!(first_link.contains(&format!(">{item}</span>")));
            assert_eq!(
                first_link.contains("aria-current=\"page\""),
                item == "Dashboard"
            );
        }
        assert_eq!(nav.matches("aria-current=\"page\"").count(), 1);
        assert_eq!(groups[1].contains(">Settings</span>"), has_settings);
        assert!(!nav.contains("clip:rect(0,0,0,0)"), "{nav}");
    }
}

#[test]
fn hidden_group_preserves_its_title_and_rejects_name_overrides() {
    let html = render_element(rsx! {
        SideNavSection { title: "Account", subtitle: "Production", is_header_hidden: true,
            id: "account", role: "menu", "aria-labelledby": "incorrect", "aria-label": "incorrect",
            end_content: rsx! { span { "2" } },
            a { href: "/profile", "Profile" }
            SideNavItem { label: "Settings", href: "/settings", is_disabled: true }
        }
    });
    let label = title_reference(&html);
    assert!(html.contains(&format!("id=\"{label}\"")));
    for expected in [
        "role=\"group\"",
        "id=\"account\"",
        "clip:rect(0,0,0,0)",
        "Account",
        "Production",
        "Profile",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
    assert!(
        !html.contains("incorrect") && !html.contains("role=\"menu\""),
        "{html}"
    );
    let (header, items) = html
        .split_once("class=\"astryx-side-nav-section__items\"")
        .expect("separate items container");
    assert!(header.contains("</div>") && !header.contains("/settings"));
    let (button, _) = items
        .split_once("<button ")
        .and_then(|(_, tail)| tail.split_once("</button>"))
        .expect("disabled item remains in the group");
    assert!(button.contains("aria-disabled=\"true\"") && button.contains("Settings"));
    assert!(!items.contains("/settings"), "{items}");
}
