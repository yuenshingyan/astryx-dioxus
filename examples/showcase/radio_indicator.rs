use dioxus::prelude::*;

use astryx_dioxus::{HStack, IndicatorSize, RadioIndicator, RadioIndicatorState, SpacingStep};

#[component]
pub(super) fn RadioIndicatorExamples() -> Element {
    rsx! {
        section { id: "radio-indicator-examples", "data-astryx-theme": "neutral",
            h2 { "RadioIndicator" }
            p { "Decorative states. Hosts supply selection and focus behavior." }
            HStack { id: "radio-indicator-scope", gap: SpacingStep::Two, "data-astryx-indicator-scope": "",
                RadioIndicator { id: "radio-unchecked", state: RadioIndicatorState::Unchecked }
                RadioIndicator { id: "radio-checked", state: RadioIndicatorState::Checked, size: IndicatorSize::Small }
            }
            HStack { gap: SpacingStep::Two,
                RadioIndicator { id: "radio-disabled", state: RadioIndicatorState::Unchecked, is_disabled: true }
                RadioIndicator { id: "radio-replacement", state: RadioIndicatorState::Checked, replacement: rsx! { "…" } }
            }
        }
    }
}
