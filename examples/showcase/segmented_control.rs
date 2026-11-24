use dioxus::prelude::*;

use astryx_dioxus::{SegmentedControl, SegmentedControlItem, SegmentedControlLayout};

#[component]
pub(super) fn SegmentedControlExamples() -> Element {
    let mut mode = use_signal(|| "grid".to_owned());
    let mut board = use_signal(|| "board".to_owned());
    rsx! {
        section { id: "segmented-control-examples",
            h2 { "Segmented Control" }
            SegmentedControl { label: "View mode", value: mode(), on_change: move |next| mode.set(next),
                SegmentedControlItem { value: "grid", label: "Grid" }
                SegmentedControlItem { value: "list", label: "List" }
                SegmentedControlItem { value: "table", label: "Table" }
            }
            div { dir: "rtl", style: "max-width:320px",
                SegmentedControl { label: "Board views", value: board(), on_change: move |next| board.set(next), layout: SegmentedControlLayout::Fill,
                    SegmentedControlItem { value: "board", label: "Board" }
                    SegmentedControlItem { value: "chart", label: "Chart", is_disabled: true }
                    SegmentedControlItem { value: "timeline", label: "Timeline", is_label_hidden: true, icon: rsx! { "▦" } }
                    SegmentedControlItem { value: "list", label: "List", onclick: move |event: MouseEvent| event.prevent_default() }
                }
            }
        }
    }
}
