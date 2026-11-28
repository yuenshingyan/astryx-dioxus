use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{CheckboxIndicatorState, CheckboxListItem, ItemAlign, List};

fn render_item(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    render(&dom)
}

fn attribute<'a>(attrs: &'a str, name: &str) -> Option<&'a str> {
    attrs
        .split_once(&format!(" {name}=\""))?
        .1
        .split('"')
        .next()
}

fn opening_tag<'a>(html: &'a str, tag: &str) -> &'a str {
    let start = html.find(&format!("<{tag} ")).unwrap() + tag.len() + 1;
    html[start..].split_once('>').unwrap().0
}

#[test]
fn visible_rich_label_names_the_native_checkbox_and_description_stays_separate() {
    let html = render_item(|| {
        rsx! {
            List {
                CheckboxListItem { label: rsx! { strong { "Team access" } },
                    description: Some("Some teams have access".into()), align: ItemAlign::FirstLine,
                    is_checked: CheckboxIndicatorState::Indeterminate,
                    end_content: rsx! { button { "Manage team" } },
                }
            }
        }
    });
    assert_eq!(html.matches("<input ").count(), 1, "{html}");
    let row = opening_tag(&html, "li");
    let input = opening_tag(&html, "input");
    assert!(
        !row.contains("tabindex") && !row.contains("aria-label"),
        "{row}"
    );
    assert_eq!(attribute(row, "data-align"), Some("first-line"));
    assert_eq!(attribute(input, "type"), Some("checkbox"));
    assert_eq!(attribute(input, "aria-checked"), Some("mixed"));
    let label_id = attribute(input, "aria-labelledby").unwrap();
    let description_id = attribute(input, "aria-describedby").unwrap();
    assert!(
        html.contains(&format!("id=\"{label_id}\"><strong>Team access</strong>")),
        "{html}"
    );
    assert!(html.contains(&format!("id=\"{description_id}\" class=\"astryx-checkbox-list-item__description\">Some teams have access")), "{html}");
    assert!(html.contains("<button>Manage team</button>"), "{html}");
    let hidden_label = opening_tag(&html, "label");
    assert_eq!(attribute(hidden_label, "for"), attribute(input, "id"));
    assert!(html.contains(">Checkbox</label>"), "{html}");
}

#[test]
fn explicit_name_replaces_rich_label_reference_without_moving_the_visible_label() {
    let html = render_item(|| {
        rsx! {
            List {
                CheckboxListItem { label: rsx! { strong { "Pro plan Recommended" } },
                    accessible_label: "Pro plan Recommended option".to_owned(), description: Some("".into()),
                }
            }
        }
    });
    let input = opening_tag(&html, "input");
    assert_eq!(attribute(input, "aria-labelledby"), None);
    assert_eq!(attribute(input, "aria-describedby"), None);
    assert_eq!(
        attribute(opening_tag(&html, "label"), "for"),
        attribute(input, "id")
    );
    assert!(
        html.contains(">Pro plan Recommended option</label>"),
        "{html}"
    );
    assert!(
        html.contains("<strong>Pro plan Recommended</strong>"),
        "{html}"
    );
}

#[test]
fn checked_row_reflection_excludes_disabled_read_only_and_mixed_options() {
    let cases = [
        (CheckboxIndicatorState::Checked, false, false, true),
        (CheckboxIndicatorState::Checked, true, false, false),
        (CheckboxIndicatorState::Checked, false, true, false),
        (CheckboxIndicatorState::Indeterminate, false, false, false),
        (CheckboxIndicatorState::Unchecked, false, false, false),
    ];
    for (state, is_disabled, is_read_only, is_selected) in cases {
        let mut dom = VirtualDom::new_with_props(
            |(state, is_disabled, is_read_only)| {
                rsx! {
                    List {
                        CheckboxListItem { label: "Guest access", is_checked: state, is_disabled, is_read_only,
                            id: "guest", "data-example": "preserved",
                        }
                    }
                }
            },
            (state, is_disabled, is_read_only),
        );
        dom.rebuild_in_place();
        let html = render(&dom);
        let row = opening_tag(&html, "li");
        let input = opening_tag(&html, "input");
        assert_eq!(attribute(row, "id"), Some("guest"));
        assert_eq!(attribute(row, "data-example"), Some("preserved"));
        assert_eq!(
            attribute(row, "data-selected"),
            is_selected.then_some("selected")
        );
        assert_eq!(
            row.contains("astryx-checkbox-list-item--selected"),
            is_selected,
            "{row}"
        );
        assert_eq!(
            row.contains("astryx-checkbox-list-item--disabled"),
            is_disabled,
            "{row}"
        );
        assert_eq!(
            attribute(input, "aria-disabled"),
            is_disabled.then_some("true")
        );
        assert_eq!(
            attribute(input, "aria-readonly"),
            is_read_only.then_some("true")
        );
        assert_eq!(
            attribute(input, "data-checked"),
            Some(if state == CheckboxIndicatorState::Checked {
                "true"
            } else {
                "false"
            })
        );
        assert!(
            !input.contains(" disabled") && !input.contains(" tabindex"),
            "{input}"
        );
        assert!(html.contains(">Guest access</label>"), "{html}");
    }
}
