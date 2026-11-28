use std::cell::RefCell;
use std::rc::Rc;

use dioxus::core::{AttributeValue, HasAttributes, Mutation};
use dioxus::html::PlatformEventData;
use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{
    ContextMenuItem, DropdownMenuItem, DropdownMenuItemProps, DropdownMenuItemVariant,
};

#[test]
fn action_row_owns_menu_semantics_and_preserves_content() {
    let html = render_element(rsx! {
        DropdownMenuItem { id: "archive", label: "Archive", description: Some("Move to storage".into()),
            variant: DropdownMenuItemVariant::Destructive, is_disabled: true,
            role: "button", tabindex: "0",
        }
    });
    for expected in [
        "id=\"archive\"",
        "role=\"menuitem\"",
        "tabindex=\"-1\"",
        "aria-disabled=\"true\"",
        "data-variant=\"destructive\"",
        "Move to storage",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
    assert!(!html.contains("role=\"button\"") && !html.contains("tabindex=\"0\""));
}

#[test]
fn disabled_rows_compose_host_mouse_down_before_preventing_pointer_focus() {
    type ObservedMouseDown = Rc<RefCell<Option<Event<PlatformEventData>>>>;

    for href in [None, Some("#archive".to_owned())] {
        let observed = Rc::new(RefCell::new(None));
        let mut dom = VirtualDom::new_with_props(
            |(href, observed): (Option<String>, ObservedMouseDown)| {
                let is_link = href.is_some();
                let props = DropdownMenuItemProps::builder()
                    .label("Archive")
                    .is_disabled(true)
                    .href(href)
                    .push_attribute(
                        "onmousedown",
                        None,
                        AttributeValue::listener(move |event: Event<PlatformEventData>| {
                            assert!(event.default_action_enabled());
                            assert!(observed.borrow_mut().replace(event).is_none());
                        }),
                        false,
                    )
                    .build();
                if is_link {
                    rsx! { ContextMenuItem { ..props } }
                } else {
                    rsx! { DropdownMenuItem { ..props } }
                }
            },
            (href, observed.clone()),
        );
        let mutations = dom.rebuild_to_vec();
        let mut listeners = mutations
            .edits
            .iter()
            .filter_map(|mutation| match mutation {
                Mutation::NewEventListener { name, id } if name == "mousedown" => Some(*id),
                _ => None,
            });
        let row = listeners
            .next()
            .expect("the row should listen for mousedown");
        assert!(listeners.next().is_none());
        let event = Event::new(Rc::new(PlatformEventData::new(Box::new(()))), true);
        dom.runtime()
            .handle_event("mousedown", event.into_any(), row);
        let event = observed.take().expect("the host listener should run");
        assert!(!event.default_action_enabled());
    }
}
