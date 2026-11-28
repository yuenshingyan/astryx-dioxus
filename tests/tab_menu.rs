use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{Tab, TabList, TabMenu, TabMenuOption};

#[component]
fn Example(value: String) -> Element {
    let options = vec![
        TabMenuOption {
            value: "settings".into(),
            label: "Settings".into(),
            icon: Some(rsx! { span { "Decorative artwork" } }),
        },
        TabMenuOption {
            value: "billing".into(),
            label: "Billing".into(),
            icon: None,
        },
    ];
    rsx! {
        TabList { label: "Workspace", value, on_change: |_| {},
            Tab { value: "overview", label: "Overview" }
            TabMenu { label: "More", options }
        }
    }
}

fn render_menu(value: &str) -> String {
    let mut dom = VirtualDom::new_with_props(
        Example,
        ExampleProps {
            value: value.into(),
        },
    );
    dom.rebuild_in_place();
    render(&dom)
}

#[test]
fn menu_heading_names_the_picker_without_becoming_a_menu_item() {
    for (value, trigger_text, tab_index) in [("overview", "More", -1), ("settings", "Settings", 0)]
    {
        let html = render_menu(value);
        let (trigger, rest) = html
            .split_once("astryx-tab-menu__trigger\"")
            .unwrap()
            .1
            .split_once("</button>")
            .unwrap();
        let menu_id = trigger
            .split_once("aria-controls=\"")
            .unwrap()
            .1
            .split('"')
            .next()
            .unwrap();
        assert!(
            trigger.contains(&format!("tabindex={tab_index} ")),
            "{trigger}"
        );
        assert!(trigger.contains(&format!(">{trigger_text}")), "{trigger}");
        assert!(trigger.contains("aria-haspopup=\"menu\""), "{trigger}");
        assert!(trigger.contains("aria-expanded=\"false\""), "{trigger}");
        let (menu, items) = rest.split_once("astryx-tab-menu__heading").unwrap();
        assert!(menu.contains(&format!("id=\"{menu_id}\"")), "{menu}");
        assert!(menu.contains("role=\"menu\""), "{menu}");
        assert!(menu.contains("aria-label=\"More\""), "{menu}");
        let (heading, items) = items.split_once("</span>").unwrap();
        let (heading_attributes, heading_text) = heading.split_once('>').unwrap();
        assert!(
            heading_attributes.contains("role=\"presentation\""),
            "{heading}"
        );
        assert!(!heading_attributes.contains("tabindex="), "{heading}");
        assert_eq!(heading_text, "More");
        assert_eq!(items.matches("role=\"menuitemradio\"").count(), 2);
    }
}

#[test]
fn only_the_selected_radio_option_has_a_decorative_check() {
    for value in ["settings", "billing", "overview"] {
        let html = render_menu(value);
        for (option_value, label) in [("settings", "Settings"), ("billing", "Billing")] {
            let row = html
                .split("<button")
                .filter_map(|button| button.split_once("</button>"))
                .map(|(button, _)| button)
                .find(|button| button.contains(&format!("aria-label=\"{label}\"")))
                .unwrap();
            let (row, content) = row.split_once('>').unwrap();
            let checked = option_value == value;
            assert!(row.contains("role=\"menuitemradio\""), "{row}");
            assert!(
                row.contains(&format!("aria-checked=\"{checked}\"")),
                "{row}"
            );
            assert!(content.contains(&format!(">{label}</span>")), "{content}");
            let check = content
                .split('<')
                .filter_map(|tag| tag.split_once('>'))
                .map(|(tag, _)| tag)
                .find(|tag| tag.contains("astryx-tab-menu__check"));
            assert_eq!(check.is_some(), checked, "{content}");
            if let Some(check) = check {
                assert!(check.contains("aria-hidden=\"true\""), "{check}");
            }
            if option_value == "settings" {
                assert!(
                    content.contains(
                        "<span aria-hidden=\"true\"><span>Decorative artwork</span></span>"
                    ),
                    "{content}"
                );
            }
        }
    }
}
