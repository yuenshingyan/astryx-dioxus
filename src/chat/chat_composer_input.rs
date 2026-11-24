use dioxus::core::current_scope_id;
use dioxus::prelude::*;

/// Plain-text chat entry with Enter to send and Shift+Enter for a newline.
/// When `value` is supplied, the caller applies changes through `on_change`.
#[component]
pub fn ChatComposerInput(
    #[props(default = "Message".to_owned())] label: String,
    #[props(default)] placeholder: Option<String>,
    #[props(default)] value: Option<String>,
    #[props(default)] on_change: Option<EventHandler<String>>,
    #[props(default)] on_submit: Option<EventHandler<String>>,
    #[props(default = 8)] max_rows: u16,
    #[props(default)] is_disabled: bool,
) -> Element {
    let id = use_hook(|| format!("astryx-chat-composer-input-{}", current_scope_id().0));
    let mut draft = use_signal(String::new);
    let is_local = value.is_none();
    let value = value.unwrap_or_else(|| draft.read().clone());
    let mut controller = use_signal(|| None::<document::Eval>);
    use_drop(move || {
        if let Some(eval) = *controller.peek() {
            let _ = eval.send(());
        }
    });
    let update = use_callback(move |text: String| {
        if is_local {
            draft.set(text.clone());
        }
        if let Some(change) = on_change {
            change.call(text);
        }
    });
    let submit = use_callback(move |text: String| {
        if !is_disabled && let Some(submit) = on_submit {
            submit.call(text);
            update.call(String::new());
        }
    });
    let mount_id = id.clone();
    rsx! {
        textarea { id, class: "astryx-chat-composer-input", rows: 1, value, placeholder,
            "aria-label": label, disabled: is_disabled, "data-submit": on_submit.is_some().then_some("true"),
            style: "--_composer-max-rows:{max_rows.max(1)}", enterkeyhint: "send",
            oninput: move |event| { if !is_disabled { update.call(event.value()); } },
            onmounted: move |_| {
                if let Some(eval) = *controller.peek() { let _ = eval.send(()); }
                let mut eval = document::eval(include_str!("chat_composer_input.js"));
                let _ = eval.send(mount_id.clone());
                controller.set(Some(eval));
                spawn(async move {
                    while let Ok(text) = eval.recv::<String>().await { submit.call(text); }
                });
            },
        }
    }
}
