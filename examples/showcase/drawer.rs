use dioxus::prelude::*;

use astryx_dioxus::{Button, Drawer, DrawerHeader, Layout, LayoutContent};

#[component]
pub(super) fn DrawerExamples() -> Element {
    let mut open = use_signal(|| false);
    rsx! {
        section { id: "drawer-examples", dir: "rtl",
            h2 { "Drawer" }
            Button { id: "drawer-preview-trigger", label: "Open drawer", onclick: move |_| open.set(true) }
            Drawer { is_open: open(), on_open_change: move |next| open.set(next), label: "Details", width: Some(360.into()),
                Layout {
                    header: rsx! { DrawerHeader { title: rsx! { "Details" }, on_open_change: move |next| open.set(next) } },
                    LayoutContent { "Review the workspace details." }
                }
            }
        }
    }
}
