use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::{Button, LayerPlacement};

/// Single-level menu with a native popover and caller-owned menu rows.
#[component]
pub fn DropdownMenu(
    label: String,
    children: Element,
    #[props(default = LayerPlacement::Below)] placement: LayerPlacement,
    #[props(default)] is_disabled: bool,
) -> Element {
    let id = use_hook(|| format!("astryx-dropdown-menu-{}", current_scope_id().0));
    let trigger = format!("{id}-trigger");
    let mount_id = id.clone();
    let mut controller = use_signal(|| None::<document::Eval>);
    use_drop(move || {
        if let Some(eval) = *controller.peek() {
            let _ = eval.send(());
        }
    });
    rsx! {
        Button { id: trigger.clone(), label, is_disabled, "aria-haspopup": "menu", "aria-expanded": "false", "aria-controls": id.clone(),
            style: "anchor-name:--{id}",
        }
        div { id, class: "astryx-dropdown-menu", role: "menu", tabindex: -1, popover: "auto",
            "aria-labelledby": trigger, "data-placement": placement.as_str(), style: "position-anchor:--{mount_id}",
            onmounted: move |_| {
                if let Some(eval) = *controller.peek() { let _ = eval.send(()); }
                let eval = document::eval(include_str!("dropdown_menu.js"));
                let _ = eval.send(mount_id.clone());
                controller.set(Some(eval));
            },
            {children}
        }
    }
}
