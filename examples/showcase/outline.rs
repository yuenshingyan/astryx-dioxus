use dioxus::prelude::*;

use astryx_dioxus::{Outline, OutlineDensity, OutlineItem};

#[component]
pub(super) fn OutlineExamples() -> Element {
    let entries = [
        ("outline-overview", "Overview", 2),
        ("outline-installation", "Installation", 2),
        ("outline-theming", "Theming", 2),
        ("outline-tokens", "Tokens", 3),
        ("outline-accessibility", "Accessibility", 2),
    ];
    let items = entries
        .map(|(id, label, level)| OutlineItem {
            id: id.into(),
            label: label.into(),
            level,
        })
        .to_vec();
    rsx! {
        section { id: "outline-examples",
            h2 { "Outline" }
            div { style: "max-inline-size:18rem",
                Outline { items, active_id: "outline-theming", density: OutlineDensity::Compact }
            }
            for (id, label, _) in entries {
                h3 { id, "{label}" }
            }
        }
    }
}
