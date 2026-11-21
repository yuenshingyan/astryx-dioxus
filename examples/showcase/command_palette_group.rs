use dioxus::prelude::*;

use astryx_dioxus::CommandPaletteGroup;

#[component]
fn Results(
    heading: &'static str,
    labels: Vec<&'static str>,
    onselect: EventHandler<&'static str>,
) -> Element {
    rsx! {
        CommandPaletteGroup { heading,
            for label in labels {
                button { key: "{label}", r#type: "button", onclick: move |_| onselect.call(label), "{label}" }
            }
        }
    }
}

#[component]
pub(super) fn CommandPaletteGroupExamples() -> Element {
    let mut selected = use_signal(|| "No command selected");
    let onselect = EventHandler::new(move |label| selected.set(label));
    rsx! {
        section { id: "command-palette-group-examples", "data-astryx-theme": "neutral",
            h2 { "CommandPaletteGroup" }
            h3 { "Basic" }
            Results { heading: "Navigation", labels: vec!["Go to Home", "Go to Dashboard"], onselect }
            Results { heading: "Actions", labels: vec!["New File", "Save All"], onselect }
            h3 { "Showcase" }
            Results { heading: "Recent Files", labels: vec!["index.tsx", "App.tsx"], onselect }
            Results { heading: "Commands", labels: vec!["Open Settings", "Toggle Terminal"], onselect }
            Results { heading: "Preferences", labels: vec!["Color Theme", "Font Size"], onselect }
            p { id: "command-palette-group-selection", "{selected}" }
        }
    }
}
