use dioxus::prelude::*;

use astryx_dioxus::{AlertDialog, Button};

#[component]
pub(super) fn AlertDialogExamples() -> Element {
    let mut open = use_signal(|| false);
    let mut count = use_signal(|| 0);
    rsx! {
        section { id: "alert-dialog-examples",
            h2 { "AlertDialog" }
            Button { id: "alert-preview-trigger", label: "Review deletion", onclick: move |_| open.set(true) }
            AlertDialog { is_open: open(), on_open_change: move |next| open.set(next),
                title: "Delete item?",
                description: "This action cannot be undone. The item and all its data will be permanently removed.",
                action_label: "Delete", on_action: move |_| count += 1,
            }
            output { id: "alert-preview-action", "Requests: {count}" }
        }
    }
}
