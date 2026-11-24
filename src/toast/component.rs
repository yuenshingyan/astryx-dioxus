use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::{Button, ButtonSize, ButtonVariant};

/// Severity of a notification.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToastType {
    #[default]
    Info,
    Error,
}

/// Why a toast requests dismissal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToastDismissReason {
    Manual,
    Auto,
}

/// Standalone notification surface. The caller removes it after dismissal.
#[component]
pub fn Toast(
    body: Element,
    on_dismiss: EventHandler<ToastDismissReason>,
    #[props(default)] r#type: ToastType,
    #[props(default)] end_content: Option<Element>,
    #[props(default)] is_auto_hide: bool,
    #[props(default = 5000)] auto_hide_duration: u32,
    #[props(default = "Dismiss".to_owned())] dismiss_label: String,
) -> Element {
    let id = use_hook(|| format!("astryx-toast-{}", current_scope_id().0));
    let mount_id = id.clone();
    let mut dismissed = use_signal(|| false);
    let dismiss = use_callback(move |reason| {
        if !*dismissed.peek() {
            dismissed.set(true);
            on_dismiss.call(reason);
        }
    });
    let mut controller = use_signal(|| None::<document::Eval>);
    use_drop(move || {
        if let Some(eval) = *controller.peek() {
            let _ = eval.send(());
        }
    });
    let is_error = r#type == ToastType::Error;
    rsx! {
        div { id, class: "astryx-toast-surface", "data-error": is_error.then_some("true"),
            "data-duration": (is_auto_hide && !dismissed()).then(|| auto_hide_duration.to_string()),
            onmounted: move |_| {
                if let Some(eval) = *controller.peek() { let _ = eval.send(()); }
                let mut eval = document::eval(include_str!("toast_timer.js"));
                let _ = eval.send(mount_id.clone());
                controller.set(Some(eval));
                spawn(async move {
                    while eval.recv::<bool>().await.is_ok() { dismiss.call(ToastDismissReason::Auto); }
                });
            },
            div { class: "astryx-toast-surface__body", role: if is_error { "alert" } else { "status" }, {body} }
            div { class: "astryx-toast-surface__actions", {end_content}
                Button { label: dismiss_label, variant: ButtonVariant::Ghost, size: ButtonSize::Small,
                    is_icon_only: true, icon: rsx! { span { "aria-hidden": "true", "×" } },
                    onclick: move |_| dismiss.call(ToastDismissReason::Manual),
                }
            }
        }
    }
}
