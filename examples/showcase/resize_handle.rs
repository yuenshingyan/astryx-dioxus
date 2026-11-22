use dioxus::prelude::*;

use astryx_dioxus::{Card, ResizeDirection, ResizeHandle, SizeValue};

#[component]
pub(super) fn ResizeHandleExamples() -> Element {
    let mut width = use_signal(|| 180_u32);
    let mut height = use_signal(|| 100_u32);
    rsx! {
        section { id: "resize-handle-examples",
            h2 { "ResizeHandle" }
            p { "Focus a separator and use the arrow keys to resize." }
            div { style: "display:flex",
                div { id: "resize-preview-pane", Card { width: SizeValue::from(width()), "Preview" } }
                ResizeHandle { label: "Preview width", controls: "resize-preview-pane", value: width(), min: 100, max: 280,
                    has_divider: true, on_resize: move |value| width.set(value) }
            }
            div { style: "display:flex;flex-direction:column;max-inline-size:280px",
                div { id: "resize-vertical-pane", style: "block-size:{height}px", "Details" }
                ResizeHandle { label: "Details height", controls: "resize-vertical-pane", value: height(), min: 50, max: 200,
                    direction: ResizeDirection::Vertical, is_reversed: true, on_resize: move |value| height.set(value) }
            }
        }
    }
}
