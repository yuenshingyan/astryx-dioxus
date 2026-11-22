use dioxus::prelude::*;

use crate::utils::is_renderable::is_renderable;
use crate::{Icon, IconSize};

/// Search field for a command palette. Standalone use leaves result ownership to the caller.
#[component]
pub fn CommandPaletteInput(
    #[props(default)] value: Option<String>,
    #[props(default)] on_value_change: Option<EventHandler<String>>,
    #[props(default)] oninput: Option<EventHandler<FormEvent>>,
    #[props(default)] onkeydown: Option<EventHandler<KeyboardEvent>>,
    #[props(default)] onmounted: Option<EventHandler<MountedEvent>>,
    #[props(default)] id: Option<String>,
    #[props(default = "Search commands".to_owned())] label: String,
    #[props(default = "Search…".to_owned())] placeholder: String,
    #[props(default = true)] has_auto_focus: bool,
    #[props(default)] end_content: Option<Element>,
    #[props(default)] class: String,
    #[props(default)] style: String,
) -> Element {
    let label = if label.trim().is_empty() {
        "Search commands".to_owned()
    } else {
        label
    };
    rsx! {
        div { class: format!("astryx-command-palette-input {class}"), style,
            span { class: "astryx-command-palette-input__icon", "aria-hidden": "true",
                Icon { size: IconSize::Small, icon: rsx! {
                    svg { view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round",
                        circle { cx: "10.5", cy: "10.5", r: "6" }
                        path { d: "m15 15 5 5" }
                    }
                } }
            }
            input { class: "astryx-command-palette-input__control", id, r#type: "text",
                "aria-label": label, placeholder, value, "enterkeyhint": "search", "autocorrect": "off",
                "data-autofocus": has_auto_focus.then_some("true"), autofocus: has_auto_focus,
                oninput: move |event| {
                    if let Some(handler) = on_value_change { handler.call(event.value()); }
                    if let Some(handler) = oninput { handler.call(event); }
                },
                onkeydown: move |event| { if let Some(handler) = onkeydown { handler.call(event); } },
                onmounted: move |event| { if let Some(handler) = onmounted { handler.call(event); } },
            }
            if let Some(content) = end_content.filter(is_renderable) {
                span { class: "astryx-command-palette-input__end", {content} }
            }
        }
    }
}
