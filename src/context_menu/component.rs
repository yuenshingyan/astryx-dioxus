use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::NavHeadingMenu;

/// Desktop context menu for a focusable region and caller-composed menu rows.
#[component]
pub fn ContextMenu(
    label: String,
    children: Element,
    menu_content: Element,
    #[props(default)] is_disabled: bool,
    #[props(default)] on_open_change: Option<EventHandler<bool>>,
) -> Element {
    let id = use_hook(|| format!("astryx-context-menu-{}", current_scope_id().0));
    let mount_id = id.clone();
    let mut controller = use_signal(|| None::<document::Eval>);
    let notify = use_callback(move |open| {
        if let Some(handler) = on_open_change {
            handler.call(open);
        }
    });
    use_drop(move || {
        if let Some(eval) = *controller.peek() {
            let _ = eval.send(());
        }
    });
    rsx! {
        div { id: "{id}-trigger", tabindex: 0, "data-context-menu-disabled": is_disabled.to_string(), {children} }
        NavHeadingMenu { id: id.clone(), class: "astryx-context-menu", popover: "auto", tabindex: -1,
            "aria-label": label,
            onmounted: move |_: MountedEvent| {
                if let Some(eval) = *controller.peek() { let _ = eval.send(()); }
                let mut eval = document::eval(include_str!("context_menu.js"));
                let _ = eval.send(mount_id.clone());
                controller.set(Some(eval));
                spawn(async move {
                    while let Ok(open) = eval.recv::<bool>().await { notify.call(open); }
                });
            },
            {menu_content}
        }
    }
}
