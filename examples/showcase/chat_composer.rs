use dioxus::prelude::*;

use astryx_dioxus::ChatComposer;

#[component]
pub(super) fn ChatComposerExamples() -> Element {
    let mut message = use_signal(String::new);
    let mut is_running = use_signal(|| false);
    rsx! {
        section { id: "chat-composer-examples",
            h2 { "Chat Composer" }
            div { style: "inline-size:450px;max-inline-size:100%",
                ChatComposer { placeholder: "Type a message…", is_stop_shown: is_running(),
                    is_disabled: is_running(),
                    on_submit: move |text| { message.set(text); is_running.set(true); },
                    on_stop: move |_| is_running.set(false),
                }
                output { "{message()}" }
            }
        }
    }
}
