use dioxus::prelude::*;

use astryx_dioxus::{CommandPaletteEmpty, Text, TextColor, TextType};

#[component]
pub(super) fn CommandPaletteEmptyExamples() -> Element {
    rsx! {
        section { id: "command-palette-empty-examples", "data-astryx-theme": "neutral",
            h2 { "CommandPaletteEmpty" }
            h3 { "Basic" }
            CommandPaletteEmpty {
                Text { text_type: TextType::SUPPORTING, color: TextColor::SECONDARY, "No commands available" }
            }
            h3 { "Showcase" }
            CommandPaletteEmpty {
                Text { text_type: TextType::SUPPORTING, color: TextColor::SECONDARY, "No commands available yet" }
            }
            h3 { "Long search term" }
            CommandPaletteEmpty { id: "command-palette-empty-wrap", style: "width:160px;box-sizing:border-box",
                Text { text_type: TextType::SUPPORTING, "No results for /workspace/projects/archive/quarterly/verylongunbrokenfilename" }
            }
        }
    }
}
