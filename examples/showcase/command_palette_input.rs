use dioxus::prelude::*;

use astryx_dioxus::{CommandPaletteInput, Kbd};

#[component]
pub(super) fn CommandPaletteInputExamples() -> Element {
    let mut query = use_signal(String::new);
    rsx! {
        section { id: "command-palette-input-examples",
            h2 { "CommandPaletteInput" }
            CommandPaletteInput { id: "command-query", value: query(), has_auto_focus: false,
                on_value_change: move |value| query.set(value),
                end_content: rsx! { Kbd { keys: "Esc" } },
            }
            output { id: "command-query-value", "{query}" }
        }
    }
}
