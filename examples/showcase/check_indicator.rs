use dioxus::prelude::*;

use astryx_dioxus::{CheckIndicator, CheckIndicatorState, HStack, IndicatorSize, SpacingStep};

#[component]
pub(super) fn CheckIndicatorExamples() -> Element {
    rsx! {
        section { id: "check-indicator-examples", "data-astryx-theme": "neutral",
            h2 { "CheckIndicator" }
            p { "A decorative selection mark. The composing control owns selection and focus." }
            HStack { gap: SpacingStep::Two, "Checked" CheckIndicator { id: "check-mark", state: CheckIndicatorState::Checked, class: "example-check" } }
            HStack { gap: SpacingStep::Two, "Unchecked" CheckIndicator { id: "check-absent", state: CheckIndicatorState::Unchecked } }
            HStack { gap: SpacingStep::Two, "Disabled" CheckIndicator { id: "check-disabled", state: CheckIndicatorState::Checked, size: IndicatorSize::Small, is_disabled: true } }
            HStack { gap: SpacingStep::Two, "Replacement" CheckIndicator { id: "check-slot", state: CheckIndicatorState::Unchecked, replacement: rsx! { "…" } } }
        }
    }
}
