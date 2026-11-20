use dioxus::prelude::*;

use astryx_dioxus::{FieldStatus, FieldStatusVariant, InputStatusType, SpacingStep, VStack};

#[component]
pub(super) fn FieldStatusExamples() -> Element {
    let mut phase = use_signal(|| 0);
    let (status, message) = match phase() {
        1 => (InputStatusType::Warning, "Review your postal code"),
        2 => (InputStatusType::Error, "Review your postal code"),
        3 => (InputStatusType::Error, "Enter a valid postal code"),
        4 => (InputStatusType::Success, ""),
        _ => (InputStatusType::Success, "Postal code verified"),
    };
    rsx! {
        section { id: "field-status-examples", "data-astryx-theme": "neutral",
            h2 { "FieldStatus" }
            h3 { "Basic" }
            VStack { gap: SpacingStep::Two,
                FieldStatus { r#type: InputStatusType::Error, variant: FieldStatusVariant::DETACHED, message: "This field is required" }
                FieldStatus { r#type: InputStatusType::Success, variant: FieldStatusVariant::DETACHED, message: "Your changes have been saved" }
            }
            h3 { "Showcase" }
            VStack { gap: SpacingStep::Two,
                FieldStatus { r#type: InputStatusType::Error, variant: FieldStatusVariant::DETACHED, message: "This field is required" }
                FieldStatus { r#type: InputStatusType::Warning, variant: FieldStatusVariant::DETACHED, message: "This username is already taken by another team" }
                FieldStatus { r#type: InputStatusType::Success, variant: FieldStatusVariant::DETACHED, message: "Your changes have been saved" }
            }
            h3 { "Attached to a control" }
            button { id: "field-status-change", onclick: move |_| phase.set((phase() + 1) % 6), "Change validation status" }
            div { style: "width:200px",
                label { r#for: "field-status-input", "Postal code" }
                input { id: "field-status-input", "aria-describedby": (phase() != 0 && phase() != 5).then_some("field-status-current"), "aria-invalid": (status == InputStatusType::Error).then_some("true"), style: "position:relative;display:block;width:100%;box-sizing:border-box" }
                if phase() != 0 && phase() != 5 {
                    FieldStatus { id: "field-status-current", r#type: status, message }
                }
            }
        }
    }
}
