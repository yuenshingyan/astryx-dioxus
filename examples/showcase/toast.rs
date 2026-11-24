use dioxus::prelude::*;

use astryx_dioxus::{Button, Toast, ToastType};

#[component]
pub(super) fn ToastExamples() -> Element {
    let mut show = use_signal(|| true);
    let mut show_timed = use_signal(|| false);
    rsx! {
        section { id: "toast-examples",
            h2 { "Toast" }
            if show() {
                Toast { on_dismiss: move |_| show.set(false), body: rsx! { "Document saved successfully" },
                    end_content: rsx! { Button { label: "Show toast", onclick: move |_| show_timed.set(true) } },
                }
            }
            Button { id: "toast-timed-trigger", label: "Show timed error toast", onclick: move |_| show_timed.set(true) }
            if show_timed() {
                Toast { r#type: ToastType::Error, is_auto_hide: true, auto_hide_duration: 1500,
                    body: rsx! { "Could not save document" }, on_dismiss: move |_| show_timed.set(false),
                }
            }
        }
    }
}
