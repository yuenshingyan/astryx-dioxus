use std::cell::Cell;
use std::rc::Rc;

use dioxus::core::{AttributeValue, Mutation};
use dioxus::html::PlatformEventData;
use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{TableCellScope, TableHeaderCell};

#[test]
fn standalone_header_cell_adds_only_its_theme_target() {
    let html = render_element(rsx! { TableHeaderCell { "Hours" } });
    assert_eq!(html, "<th class=\"astryx-table-header-cell\">Hours</th>");
}

#[test]
fn header_cell_forwards_scope_and_caller_attributes() {
    for (scope, value) in [
        (TableCellScope::Column, "col"),
        (TableCellScope::Row, "row"),
        (TableCellScope::ColumnGroup, "colgroup"),
        (TableCellScope::RowGroup, "rowgroup"),
    ] {
        let html = render_element(rsx! {
            TableHeaderCell { scope, id: "hours", class: "custom", style: "color:inherit", colspan: 2,
                "data-testid": "heading", "Hours"
            }
        });
        for expected in [
            format!("scope=\"{value}\""),
            "id=\"hours\"".into(),
            "class=\"astryx-table-header-cell custom\"".into(),
            "style=\"color:inherit\"".into(),
            "colspan=2 ".into(),
            "data-testid=\"heading\"".into(),
        ] {
            assert!(html.contains(&expected), "missing {expected}: {html}");
        }
    }
}

#[test]
fn header_cell_composes_spread_attributes_with_native_table_content() {
    let html = render_element(rsx! {
        table {
            thead {
                tr { th { id: "totals", scope: "col", "Totals" } }
                tr {
                    TableHeaderCell {
                        scope: TableCellScope::Column,
                        class: "align-end",
                        style: "color:inherit",
                        attributes: vec![
                            Attribute::new("class", "consumer", None, false),
                            Attribute::new("style", "color:red", None, false),
                            Attribute::new("headers", "totals", None, false),
                        ],
                        aria_sort: "ascending",
                        button { r#type: "button", "Hours" }
                    }
                }
            }
        }
    });
    let (_, header_and_children) = html
        .split_once("</th></tr><tr><th ")
        .expect("header cell in the second row");
    assert!(
        html.ends_with("<button type=\"button\">Hours</button></th></tr></thead></table>"),
        "{html}"
    );
    let (header, _) = header_and_children.split_once('>').expect("th opening tag");
    for expected in [
        "class=\"astryx-table-header-cell align-end consumer\"",
        "style=\"color:inherit;color:red\"",
        "scope=\"col\"",
        "aria-sort=\"ascending\"",
        "headers=\"totals\"",
    ] {
        assert!(header.contains(expected), "missing {expected}: {header}");
    }
    for attribute in ["class=", "style="] {
        assert_eq!(header.matches(attribute).count(), 1, "{header}");
    }
    assert!(!header.contains("tabindex="), "{header}");
    assert!(!header.contains("role="), "{header}");
}

#[test]
fn caller_key_listener_on_header_receives_bubbling_child_key() {
    let calls = Rc::new(Cell::new(0));
    let mut dom = VirtualDom::new_with_props(
        |calls: Rc<Cell<usize>>| {
            rsx! {
                TableHeaderCell {
                    attributes: vec![Attribute::new(
                        "onkeydown",
                        AttributeValue::listener(move |event: Event<PlatformEventData>| {
                            assert_eq!(event.downcast::<Key>(), Some(&Key::Enter));
                            calls.set(calls.get() + 1);
                        }),
                        None,
                        false,
                    )],
                    button { r#type: "button", onkeyup: |_| {}, "Hours" }
                }
            }
        },
        calls.clone(),
    );
    let mutations = dom.rebuild_to_vec();
    let button = mutations
        .edits
        .iter()
        .find_map(|edit| match edit {
            Mutation::NewEventListener { name, id } if *name == "keyup" => Some(*id),
            _ => None,
        })
        .expect("child button listener");
    dom.runtime().handle_event(
        "keydown",
        Event::new(Rc::new(PlatformEventData::new(Box::new(Key::Enter))), true).into_any(),
        button,
    );
    assert_eq!(calls.get(), 1);
}
