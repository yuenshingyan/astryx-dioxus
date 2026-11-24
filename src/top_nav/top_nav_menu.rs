use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::popover::use_popover;
use crate::{NavHeadingMenu, NavHeadingMenuItem};

/// A link or action in a Top Nav menu.
#[derive(Clone, PartialEq)]
pub struct TopNavMenuItemData {
    /// Stable, unique title within the menu.
    pub title: String,
    /// Optional supporting text below the title.
    pub description: Option<String>,
    /// Optional leading artwork.
    pub icon: Option<Element>,
    /// Destination of a navigation row.
    pub href: Option<String>,
    /// Caller-owned action, which may cancel default activation.
    pub onclick: Option<EventHandler<MouseEvent>>,
    /// Keep the popup open after activation.
    pub should_keep_open: bool,
}

/// Desktop Top Nav trigger for a compact menu of links and actions.
#[component]
pub fn TopNavMenu(label: String, items: Vec<TopNavMenuItemData>) -> Element {
    let id = use_hook(|| format!("astryx-top-nav-menu-{}", current_scope_id().0));
    let (is_open, request, mount_popover) = use_popover(id.clone(), None, None);
    let trigger_id = format!("{id}-trigger");
    let focus_id = id.clone();
    let mut controller = use_signal(|| None::<document::Eval>);
    use_drop(move || {
        if let Some(eval) = *controller.peek() {
            let _ = eval.send(());
        }
    });
    rsx! {
        button { id: trigger_id.clone(), class: "astryx-top-nav-menu__trigger", r#type: "button",
            "aria-haspopup": "menu", "aria-expanded": is_open.to_string(), "aria-controls": id.clone(),
            onclick: move |_| request.call(!is_open),
            "{label}"
            span { "aria-hidden": "true", "⌄" }
        }
        NavHeadingMenu { id: id.clone(), class: "astryx-top-nav-menu", popover: "manual", tabindex: -1,
            "aria-labelledby": trigger_id.clone(), "data-open": is_open.to_string(), style: "position-anchor:--{id}",
            onmounted: move |event: MountedEvent| {
                if let Some(eval) = *controller.peek() { let _ = eval.send(()); }
                let mut eval = document::eval(include_str!("top_nav_menu_focus.js"));
                let _ = eval.send(focus_id.clone());
                controller.set(Some(eval));
                spawn(async move {
                    while eval.recv::<bool>().await.is_ok() { request.call(false); }
                });
                mount_popover.call(event);
            },
            for item in items {
                NavHeadingMenuItem { key: "{item.title}", label: rsx! { "{item.title}" },
                    description: item.description.map(|text| rsx! { "{text}" }), icon: item.icon, href: item.href,
                    on_select: move |event: MouseEvent| {
                        if let Some(handler) = item.onclick { handler.call(event.clone()); }
                        if event.default_action_enabled() && !item.should_keep_open { request.call(false); }
                    },
                }
            }
        }
    }
}
