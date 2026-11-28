use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::layout::padding::spacing_step_css;
use crate::{DialogPurpose, SizeValue, SpacingStep};

/// Logical edge of the drawer panel.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum DrawerSide {
    Start,
    #[default]
    End,
}

/// A controlled modal panel. Compose a DrawerHeader for an explicit close action.
/// Mark a focusable descendant with `data-autofocus` to focus it when the drawer opens.
#[component]
pub fn Drawer(
    is_open: bool,
    on_open_change: EventHandler<bool>,
    children: Element,
    #[props(default)] side: DrawerSide,
    #[props(default)] purpose: DialogPurpose,
    #[props(default = "Drawer".to_owned())] label: String,
    #[props(default)] labelled_by: Option<String>,
    #[props(default)] width: Option<SizeValue>,
    #[props(default)] padding: Option<SpacingStep>,
) -> Element {
    let owner = use_hook(|| format!("astryx-drawer-{}", current_scope_id().0));
    let mount_owner = owner.clone();
    let request_close = use_callback(move |()| on_open_change.call(false));
    let mut controller = use_signal(|| None::<document::Eval>);
    use_drop(move || {
        if let Some(eval) = *controller.peek() {
            let _ = eval.send(());
        }
    });
    let width = width
        .as_ref()
        .map(SizeValue::as_css)
        .transpose()?
        .unwrap_or_else(|| "400px".into());
    let padding = padding.map_or("var(--spacing-4)", spacing_step_css);
    let style = format!("--_drawer-width:{width};--_drawer-padding:{padding}");
    let role = (purpose == DialogPurpose::Required).then_some("alertdialog");
    let purpose = match purpose {
        DialogPurpose::Info => "info",
        DialogPurpose::Form => "form",
        DialogPurpose::Required => "required",
    };
    let side = match side {
        DrawerSide::Start => "start",
        DrawerSide::End => "end",
    };
    rsx! {
        dialog { class: "astryx-drawer", "data-side": side, "data-purpose": purpose,
            "data-open": is_open.to_string(), "data-astryx-dialog-owner": owner, style,
            role, "aria-label": label, "aria-labelledby": labelled_by,
            onclick: move |event| event.stop_propagation(),
            onmounted: move |_| {
                if let Some(eval) = *controller.peek() { let _ = eval.send(()); }
                let mut eval = document::eval(include_str!("../dialog/dialog.js"));
                let _ = eval.send(mount_owner.clone());
                controller.set(Some(eval));
                spawn(async move {
                    while eval.recv::<bool>().await.is_ok() { request_close.call(()); }
                });
            },
            // Keep the native fallback separate from the controller's caller-marker search.
            div { class: "astryx-drawer__content", tabindex: -1, autofocus: true, {children} }
        }
    }
}
