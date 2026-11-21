use dioxus::prelude::*;

use astryx_dioxus::{CommandPaletteGroup, CommandPaletteList};

#[component]
fn ResultPreview(
    label: &'static str,
    #[props(default)] is_selected: bool,
    #[props(default)] is_highlighted: bool,
    #[props(default)] is_disabled: bool,
) -> Element {
    let background = if is_highlighted || is_selected {
        "var(--color-overlay-hover)"
    } else {
        "transparent"
    };
    rsx! {
        div { role: "option", "aria-selected": is_selected.to_string(), "aria-disabled": is_disabled.then_some("true"),
            style: "padding:8px 12px;border-radius:var(--radius-element);background:{background}", "{label}"
        }
    }
}

#[component]
pub(super) fn CommandPaletteListExamples() -> Element {
    rsx! {
        section { id: "command-palette-list-examples", "data-astryx-theme": "neutral",
            h2 { "CommandPaletteList" }
            p { "Static previews of command results. Search and keyboard selection require the parent palette." }
            h3 { "Basic" }
            CommandPaletteList { id: "command-list-basic", style: "max-width:360px",
                ResultPreview { label: "Go Home" }
                ResultPreview { label: "Open Settings", is_highlighted: true }
                ResultPreview { label: "View Profile", is_selected: true }
                ResultPreview { label: "Help (unavailable)", is_disabled: true }
            }
            h3 { "Showcase" }
            CommandPaletteList { id: "command-list-grouped", label: "Grouped commands", style: "max-width:360px;max-height:160px",
                CommandPaletteGroup { heading: "Navigation",
                    ResultPreview { label: "Go Home" }
                    ResultPreview { label: "Open Settings" }
                    ResultPreview { label: "View Profile" }
                }
                CommandPaletteGroup { heading: "Actions",
                    ResultPreview { label: "New File" }
                    ResultPreview { label: "Search Files", is_highlighted: true }
                    ResultPreview { label: "Save All" }
                }
            }
        }
    }
}
