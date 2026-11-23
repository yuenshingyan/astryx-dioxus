use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::app_shell::use_app_shell_mobile;
use crate::{Button, ButtonVariant};

/// Logical edge at which the mobile drawer opens.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum MobileNavSide {
    #[default]
    Start,
    End,
}

/// A controlled modal navigation drawer. Without explicit state it reads AppShell mobile state.
#[component]
pub fn MobileNav(
    children: Element,
    #[props(default)] is_open: Option<bool>,
    #[props(default)] on_open_change: Option<EventHandler<bool>>,
    #[props(default)] id: Option<String>,
    #[props(default = "Navigation".to_owned())] label: String,
    #[props(default)] header: Option<Element>,
    #[props(default = 320)] width: u32,
    #[props(default)] side: MobileNavSide,
) -> Element {
    let shell = use_app_shell_mobile();
    let open = is_open.unwrap_or(shell.is_mobile_nav_open);
    let owner = use_hook(|| format!("astryx-mobile-nav-{}", current_scope_id().0));
    let mount_owner = owner.clone();
    let id = id.or(shell.mobile_nav_id).unwrap_or_else(|| owner.clone());
    let request_close = use_callback(move |()| {
        if let Some(handler) = on_open_change {
            handler.call(false);
        } else if is_open.is_none()
            && open
            && let Some(toggle) = shell.toggle_mobile_nav
        {
            toggle.call(());
        }
    });
    let mut controller = use_signal(|| None::<document::Eval>);
    use_drop(move || {
        if let Some(eval) = *controller.peek() {
            let _ = eval.send(());
        }
    });
    rsx! {
        dialog { class: "astryx-mobile-nav", id, "aria-label": label.clone(),
            "data-astryx-mobile-nav-owner": owner, "data-open": open.to_string(),
            "data-side": if side == MobileNavSide::Start { "start" } else { "end" },
            style: format!("--_mobile-nav-width:{}px", width.max(1)),
            onclick: move |event| event.stop_propagation(),
            onmounted: move |_| {
                if let Some(eval) = *controller.peek() { let _ = eval.send(()); }
                let mut eval = document::eval(include_str!("mobile_nav.js"));
                let _ = eval.send(mount_owner.clone());
                controller.set(Some(eval));
                spawn(async move {
                    while eval.recv::<bool>().await.is_ok() { request_close.call(()); }
                });
            },
            div { class: "astryx-mobile-nav__drawer", tabindex: -1, autofocus: true,
                div { class: "astryx-mobile-nav__header", {header}
                    Button { label: "Close navigation", variant: ButtonVariant::Ghost,
                        onclick: move |_| request_close.call(()),
                    }
                }
                nav { class: "astryx-mobile-nav__content", "aria-label": label, {children} }
            }
        }
    }
}
