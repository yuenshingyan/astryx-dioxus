use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::{Item, ItemLabel};

/// A desktop flyout within a DropdownMenu.
#[component]
pub fn DropdownMenuSubMenu(
    #[props(into)] label: ItemLabel,
    children: Element,
    #[props(default)] description: Option<ItemLabel>,
    #[props(default)] icon: Option<Element>,
    #[props(default)] is_disabled: bool,
) -> Element {
    let id = use_hook(|| format!("astryx-dropdown-sub-menu-{}", current_scope_id().0));
    let trigger = format!("{id}-trigger");
    let mount_id = id.clone();
    let mut controller = use_signal(|| None::<document::Eval>);
    use_drop(move || {
        if let Some(eval) = *controller.peek() {
            let _ = eval.send(());
        }
    });
    rsx! {
        Item { id: trigger.clone(), label, description, start_content: icon,
            class: "astryx-dropdown-menu-item astryx-dropdown-sub-menu__trigger",
            style: "anchor-name:--{id}", role: "menuitem", tabindex: -1,
            "aria-haspopup": "menu", "aria-expanded": "false", "aria-controls": id.clone(),
            "aria-disabled": is_disabled.then_some("true"),
            "data-dropdown-sub-menu-trigger": "true",
            end_content: rsx! { span { class: "astryx-dropdown-sub-menu__chevron", "aria-hidden": "true", "›" } },
        }
        div { id: id.clone(), class: "astryx-dropdown-menu astryx-dropdown-sub-menu", role: "menu", tabindex: -1,
            popover: "auto", "aria-labelledby": trigger, "data-dropdown-sub-menu": "true", style: "position-anchor:--{id}",
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
