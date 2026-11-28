use std::cell::Cell;
use std::rc::Rc;

use dioxus::core::{AttributeValue, Mutation};
use dioxus::html::PlatformEventData;
use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{
    BreadcrumbMenuDivider, ContextMenuDivider, DropdownMenu, DropdownMenuDivider, DropdownMenuItem,
};

#[test]
fn aliases_share_the_same_unfocusable_separator() {
    let html = render_element(rsx! { DropdownMenuDivider {} });
    assert_eq!(html, render_element(rsx! { ContextMenuDivider {} }));
    assert_eq!(html, render_element(rsx! { BreadcrumbMenuDivider {} }));
    for expected in [
        "class=\"astryx-divider astryx-dropdown-menu-divider\"",
        "data-orientation=\"horizontal\"",
        "data-variant=\"subtle\"",
        "role=\"separator\"",
        "aria-orientation=\"horizontal\"",
    ] {
        assert!(html.contains(expected), "{html}");
    }
    assert_eq!(html.matches("astryx-divider__line").count(), 1, "{html}");
    for forbidden in ["astryx-divider__label", "aria-label", "tabindex="] {
        assert!(!html.contains(forbidden), "{html}");
    }
}

#[test]
fn divider_sits_between_real_compound_menu_rows() {
    let html = render_element(rsx! {
        DropdownMenu { label: "Actions",
            DropdownMenuItem { label: "Edit" }
            DropdownMenuDivider { id: "actions-divider" }
            DropdownMenuItem { label: "Delete" }
        }
    });
    let (before, after) = html
        .split_once("id=\"actions-divider\"")
        .expect("divider root");
    assert!(before.contains("role=\"menu\""), "{html}");
    assert!(
        before.contains("Edit") && after.contains("Delete"),
        "{html}"
    );
    assert_eq!(html.matches("role=\"menuitem\"").count(), 2, "{html}");
    assert_eq!(html.matches("role=\"separator\"").count(), 1, "{html}");
}

#[test]
fn host_attributes_pass_through_without_replacing_owned_semantics() {
    let html = render_element(rsx! { DropdownMenuDivider {
        id: "actions-divider", class: "custom", style: "margin-block:8px",
        "data-testid": "separator", "aria-describedby": "actions-description",
        role: "button", "aria-orientation": "vertical", tabindex: "0", contenteditable: "true",
        attributes: vec![
            Attribute::new("ROLE", "menuitem", None, false),
            Attribute::new("Aria-Orientation", "vertical", None, false),
            Attribute::new("tabIndex", "-1", None, false),
            Attribute::new("contentEditable", "true", None, false),
            Attribute::new("autoFocus", "", None, false),
        ],
    } });
    let (root, _) = html.split_once('>').expect("separator opening tag");
    for expected in [
        "id=\"actions-divider\"",
        "data-testid=\"separator\"",
        "aria-describedby=\"actions-description\"",
        "class=\"astryx-divider astryx-dropdown-menu-divider custom\"",
        "style=\"margin-block:8px\"",
    ] {
        assert!(root.contains(expected), "{html}");
    }
    assert!(
        html.contains("role=\"separator\"") && html.contains("aria-orientation=\"horizontal\"")
    );
    assert_eq!(
        root.to_ascii_lowercase().matches("role=").count(),
        1,
        "{html}"
    );
    assert_eq!(
        root.to_ascii_lowercase()
            .matches("aria-orientation=")
            .count(),
        1,
        "{html}"
    );
    for forbidden in ["tabindex=", "contenteditable=", "autofocus"] {
        assert!(!root.to_ascii_lowercase().contains(forbidden), "{html}");
    }
}

#[test]
fn host_handler_and_mount_callback_are_registered_on_the_same_element() {
    let clicks = Rc::new(Cell::new(0));
    let mut dom = VirtualDom::new_with_props(
        |clicks: Rc<Cell<usize>>| {
            rsx! {
                DropdownMenuDivider {
                    onmounted: |_| {},
                    attributes: vec![Attribute::new(
                        "onclick",
                        AttributeValue::listener(move |_: Event<PlatformEventData>| clicks.set(clicks.get() + 1)),
                        None,
                        false,
                    )],
                }
            }
        },
        clicks.clone(),
    );
    let mutations = dom.rebuild_to_vec();
    let listener_element = |event_name| {
        let listeners: Vec<_> = mutations
            .edits
            .iter()
            .filter_map(|edit| match edit {
                Mutation::NewEventListener { name, id } if *name == event_name => Some(*id),
                _ => None,
            })
            .collect();
        assert_eq!(listeners.len(), 1, "{event_name}");
        listeners[0]
    };
    let separator = listener_element("click");
    assert_eq!(separator, listener_element("mounted"));
    dom.runtime().handle_event(
        "click",
        Event::new(Rc::new(PlatformEventData::new(Box::new(()))), false).into_any(),
        separator,
    );
    assert_eq!(clicks.get(), 1);
}
