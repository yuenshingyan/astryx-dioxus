use dioxus::prelude::*;

use astryx_dioxus::{Button, Drawer, DrawerHeader, Layout, LayoutContent};

#[component]
pub(super) fn DrawerExamples() -> Element {
    let mut open = use_signal(|| false);
    let mut should_focus_name = use_signal(|| false);
    rsx! {
        section { id: "drawer-examples", dir: "rtl",
            h2 { "Drawer" }
            Button { id: "drawer-preview-trigger", label: "Open drawer", onclick: move |_| {
                should_focus_name.set(false);
                open.set(true);
            } }
            Button { id: "drawer-preview-edit-trigger", label: "Edit workspace name", onclick: move |_| {
                should_focus_name.set(true);
                open.set(true);
            } }
            Drawer { is_open: open(), on_open_change: move |next| open.set(next), label: "Details", width: Some(360.into()),
                Layout {
                    header: rsx! { DrawerHeader { title: rsx! { "Details" }, on_open_change: move |next| open.set(next) } },
                    LayoutContent {
                        label {
                            "Workspace name"
                            input { id: "drawer-workspace-name", "data-autofocus": should_focus_name().then_some("true") }
                        }
                    }
                }
            }
        }
    }
}
