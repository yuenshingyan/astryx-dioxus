use dioxus::prelude::*;

use super::{ChatComposerInput, ChatSendButton};

/// Plain-text composer with a shared draft for keyboard and button submission.
#[component]
pub fn ChatComposer(
    on_submit: EventHandler<String>,
    #[props(default)] value: Option<String>,
    #[props(default)] on_change: Option<EventHandler<String>>,
    #[props(default)] placeholder: Option<String>,
    #[props(default = "Message".to_owned())] input_label: String,
    #[props(default)] is_disabled: bool,
    #[props(default)] is_stop_shown: bool,
    #[props(default)] on_stop: Option<EventHandler<()>>,
    #[props(default)] drawer: Option<Element>,
    #[props(default)] header_actions: Option<Element>,
    #[props(default)] header_context: Option<Element>,
    #[props(default)] footer_actions: Option<Element>,
    #[props(default)] send_actions: Option<Element>,
) -> Element {
    let mut local = use_signal(String::new);
    let is_local = value.is_none();
    let draft = value.unwrap_or_else(|| local.read().clone());
    let can_send = !is_disabled && !draft.trim().is_empty();
    let update = use_callback(move |text: String| {
        if is_local {
            local.set(text.clone());
        }
        if let Some(change) = on_change {
            change.call(text);
        }
    });
    rsx! {
        div { class: "astryx-chat-composer",
            {drawer}
            div { class: "astryx-chat-composer__body",
                if header_actions.is_some() || header_context.is_some() {
                    div { class: "astryx-chat-composer__row",
                        div { {header_actions} }
                        div { {header_context} }
                    }
                }
                ChatComposerInput { label: input_label, placeholder, value: draft.clone(), is_disabled,
                    on_change: update, on_submit,
                }
                div { class: "astryx-chat-composer__row",
                    div { {footer_actions} }
                    div { class: "astryx-chat-composer__send-actions",
                        {send_actions}
                        ChatSendButton { is_stop_shown, is_disabled: !can_send, on_stop,
                            on_send: move |_| {
                                if can_send {
                                    on_submit.call(draft.trim().to_owned());
                                    update.call(String::new());
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}
