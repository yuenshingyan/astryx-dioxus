use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::layout::padding::spacing_step_css;
use crate::{SizeValue, SpacingStep};

/// Which automatic dismissal methods a dialog allows.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum DialogPurpose {
    /// Escape and backdrop clicks request dismissal.
    #[default]
    Info,
    /// Only Escape requests dismissal.
    Form,
    /// No implicit dismissal. The caller supplies explicit actions.
    Required,
}

/// Surface geometry of a dialog.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum DialogVariant {
    #[default]
    Standard,
    Fullscreen,
}

/// A caller-controlled native modal dialog, or a nonmodal inline preview.
#[component]
pub fn Dialog(
    is_open: bool,
    on_open_change: EventHandler<bool>,
    children: Element,
    #[props(default)] purpose: DialogPurpose,
    #[props(default)] variant: DialogVariant,
    #[props(default)] is_inline: bool,
    #[props(default = "Dialog".to_owned())] label: String,
    #[props(default)] labelled_by: Option<String>,
    #[props(default)] width: Option<SizeValue>,
    #[props(default)] max_height: Option<SizeValue>,
    #[props(default)] padding: Option<SpacingStep>,
) -> Element {
    let owner = use_hook(|| format!("astryx-dialog-{}", current_scope_id().0));
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
    let max_height = max_height
        .as_ref()
        .map(SizeValue::as_css)
        .transpose()?
        .unwrap_or_else(|| "75dvh".into());
    let padding = padding.map_or("var(--spacing-4)", spacing_step_css);
    let style = format!(
        "--_dialog-width:{width};--_dialog-max-height:{max_height};--_dialog-padding:{padding}"
    );
    let variant = if variant == DialogVariant::Fullscreen {
        "fullscreen"
    } else {
        "standard"
    };
    if is_inline {
        return rsx! { div { class: "astryx-dialog", "data-variant": variant, style, hidden: !is_open, {children} } };
    }
    let role = (purpose == DialogPurpose::Required).then_some("alertdialog");
    let purpose = match purpose {
        DialogPurpose::Info => "info",
        DialogPurpose::Form => "form",
        DialogPurpose::Required => "required",
    };
    rsx! {
        dialog { class: "astryx-dialog", "data-variant": variant, "data-purpose": purpose, "data-open": is_open.to_string(),
            "data-astryx-dialog-owner": owner, style, role, "aria-label": label, "aria-labelledby": labelled_by,
            onclick: move |event| event.stop_propagation(),
            onmounted: move |_| {
                if let Some(eval) = *controller.peek() { let _ = eval.send(()); }
                let mut eval = document::eval(include_str!("dialog.js"));
                let _ = eval.send(mount_owner.clone());
                controller.set(Some(eval));
                spawn(async move {
                    while eval.recv::<bool>().await.is_ok() { request_close.call(()); }
                });
            },
            {children}
        }
    }
}
