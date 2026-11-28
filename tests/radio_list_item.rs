use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{IndicatorSize, ItemAlign, RadioList, RadioListItem};

fn attribute<'a>(tag: &'a str, name: &str) -> Option<&'a str> {
    tag.split_once(&format!(" {name}=\""))?.1.split('"').next()
}

#[test]
fn painted_rows_reflect_alignment_size_selection_and_availability() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            RadioList { label: "Delivery", value: "standard", html_name: "delivery", size: IndicatorSize::Small,
                RadioListItem { id: "standard-row", label: "Standard", value: "standard", class: "shipping", style: "gap: 4px;" }
                RadioListItem { label: "Express", value: "express", align: ItemAlign::Start, is_disabled: true }
            }
            RadioList { label: "Pickup", is_disabled: true,
                RadioListItem { label: "Store", value: "store", align: ItemAlign::FirstLine }
            }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    assert_eq!(html.matches("type=\"radio\"").count(), 3, "{html}");
    for (row, (align, size, is_selected, is_disabled)) in html
        .split("class=\"astryx-item astryx-radio-list-item ")
        .skip(1)
        .zip([
            ("center", "sm", true, false),
            ("start", "sm", false, true),
            ("first-line", "md", false, true),
        ])
    {
        let tag = row.split('>').next().unwrap();
        assert_eq!(attribute(tag, "data-align"), Some(align));
        assert_eq!(attribute(tag, "data-size"), Some(size));
        assert_eq!(attribute(tag, "data-selected").is_some(), is_selected);
        assert_eq!(attribute(tag, "data-disabled").is_some(), is_disabled);
        assert_eq!(
            attribute(tag, "data-astryx-indicator-scope").is_some(),
            !is_disabled
        );
        assert_eq!(attribute(tag, "tabindex"), None);
        let input = row
            .split_once("<input ")
            .unwrap()
            .1
            .split('>')
            .next()
            .unwrap();
        assert_eq!(input.contains(" checked"), is_selected);
        assert_eq!(input.contains(" disabled"), is_disabled);
        if size == "sm" {
            assert_eq!(attribute(input, "name"), Some("delivery"));
        }
    }
    assert_eq!(
        html.matches("class=\"astryx-item astryx-radio-list-item ")
            .count(),
        3
    );
    assert!(html.contains("astryx-radio-list-item shipping\""), "{html}");
    assert!(html.contains("style=\"gap: 4px;\""), "{html}");
    assert!(html.contains("id=\"standard-row\""), "{html}");
}

#[test]
fn rich_content_names_and_describes_input_unless_a_name_is_overridden() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            RadioList { label: "Delivery",
                RadioListItem { label: rsx! { strong { "Standard" } }, value: "standard",
                    description: Some(rsx! { a { href: "#details", "Delivery details" } }.into()),
                    start_content: rsx! { span { "Start accessory" } }, end_content: rsx! { button { "End accessory" } },
                }
                RadioListItem { label: rsx! { strong { "Express" } }, value: "express",
                    accessible_label: "Express delivery", description: Some(rsx! {}.into()),
                }
            }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    let mut inputs = html
        .split("<input ")
        .skip(1)
        .map(|rest| rest.split('>').next().unwrap());
    let standard = inputs.next().unwrap();
    let express = inputs.next().unwrap();
    let label_id = attribute(standard, "aria-labelledby").unwrap();
    let description_id = attribute(standard, "aria-describedby").unwrap();
    assert!(
        html.contains(&format!("id=\"{label_id}\"><strong>Standard</strong>")),
        "{html}"
    );
    assert!(
        html.contains(&format!(
            "id=\"{description_id}\"><a href=\"#details\">Delivery details</a>"
        )),
        "{html}"
    );
    assert!(html.find("<input ").unwrap() < html.find("Start accessory").unwrap());
    assert!(html.find("Start accessory").unwrap() < html.find("<strong>Standard").unwrap());
    assert!(html.find("Delivery details").unwrap() < html.find("<button>End accessory").unwrap());
    assert_eq!(attribute(express, "aria-label"), Some("Express delivery"));
    assert_eq!(attribute(express, "aria-labelledby"), None);
    assert_eq!(attribute(express, "aria-describedby"), None);
    assert_eq!(html.matches("aria-label=\"Express delivery\"").count(), 1);
}
