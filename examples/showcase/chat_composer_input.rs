use dioxus::prelude::*;

use astryx_dioxus::ChatComposerInput;

#[component]
pub(super) fn ChatComposerInputExamples() -> Element {
    let mut submitted = use_signal(String::new);
    rsx! {
        section { id: "chat-composer-input-examples",
            h2 { "Chat Composer Input" }
            div { style: "inline-size:450px;max-inline-size:100%",
                ChatComposerInput { placeholder: "Ask me anything about Astryx...",
                    on_submit: move |message| submitted.set(message),
                }
                output { "{submitted()}" }
            }
        }
    }
}
