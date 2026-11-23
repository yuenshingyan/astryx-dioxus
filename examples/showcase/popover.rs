use dioxus::prelude::*;

use astryx_dioxus::{Button, Popover, PopoverTriggerRenderProps};

#[component]
pub(super) fn PopoverExamples() -> Element {
    let mut open = use_signal(|| false);
    rsx! {
        section { id: "popover-examples",
            h2 { "Popover" }
            Popover { label: "Settings", width: Some(280.into()), is_open: open(),
                on_open_change: move |next| open.set(next),
                trigger: |props: PopoverTriggerRenderProps| rsx! {
                    Button { id: props.anchor_id, label: "Settings", onclick: props.onclick,
                        "aria-haspopup": "dialog", "aria-controls": props.controls,
                        "aria-expanded": props.is_expanded.to_string(),
                    }
                },
                content: rsx! {
                    h3 { "Settings" }
                    p { "Notifications, dark mode, and sound preferences." }
                    Button { label: "Done", onclick: move |_| open.set(false) }
                },
            }
            button { id: "popover-preview-outside", r#type: "button", "Outside popover" }
        }
    }
}
