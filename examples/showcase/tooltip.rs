use dioxus::prelude::*;

use astryx_dioxus::Tooltip;

#[component]
pub(super) fn TooltipExamples() -> Element {
    rsx! {
        section { id: "tooltip-examples",
            h2 { "Tooltip" }
            h3 { "Inline text tooltips" }
            p {
                "Learn more about our "
                Tooltip { text: "privacy policy", content: "Your data is encrypted and never shared" }
                " and "
                Tooltip { text: "terms of service", content: "Standard 30-day agreement" }
                "."
            }
        }
    }
}
