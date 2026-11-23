use dioxus::prelude::*;

use astryx_dioxus::{
    Button, Dialog, DialogHeader, DialogPurpose, Layout, LayoutContent, LayoutHeight,
};

#[component]
pub(super) fn DialogExamples() -> Element {
    let mut open = use_signal(|| false);
    let mut required = use_signal(|| false);
    rsx! {
        section { id: "dialog-examples",
            h2 { "Dialog" }
            Button { id: "dialog-preview-trigger", label: "Open dialog", onclick: move |_| open.set(true) }
            Dialog { is_open: open(), on_open_change: move |next| open.set(next), labelled_by: "dialog-preview-title",
                Layout { height: LayoutHeight::Auto,
                    header: rsx! { DialogHeader { title: rsx! { "Modal Title" }, title_id: "dialog-preview-title", on_open_change: move |next| open.set(next) } },
                    LayoutContent { "Review the workspace details." }
                }
            }
            Button { id: "required-dialog-preview-trigger", label: "Open required dialog", onclick: move |_| required.set(true) }
            Dialog { is_open: required(), purpose: DialogPurpose::Required, label: "Confirm details", on_open_change: move |next| required.set(next),
                p { "Confirm before continuing." }
                Button { label: "Confirm", onclick: move |_| required.set(false) }
            }
        }
    }
}
