use dioxus::prelude::*;

use crate::{Dialog, DialogPurpose, SizeValue, SpacingStep};

/// Height budget for a standalone bottom sheet.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum BottomSheetHeight {
    /// Fit the content up to the tall viewport limit.
    Hug,
    /// Fit the content up to the shorter default viewport limit.
    #[default]
    Capped,
    /// Fill the tall viewport budget.
    Tall,
    /// Use the supplied CSS size as the height budget.
    Custom(SizeValue),
}

/// A standalone modal sheet anchored to the bottom of the viewport.
#[component]
pub fn BottomSheet(
    is_open: bool,
    on_open_change: EventHandler<bool>,
    label: String,
    children: Element,
    #[props(default)] height: BottomSheetHeight,
    #[props(default)] purpose: DialogPurpose,
    #[props(default)] padding: Option<SpacingStep>,
) -> Element {
    let (height, is_tall) = match height {
        BottomSheetHeight::Hug => ("92dvh".into(), false),
        BottomSheetHeight::Capped => ("62dvh".into(), false),
        BottomSheetHeight::Tall => ("92dvh".into(), true),
        BottomSheetHeight::Custom(height) => (height, false),
    };
    rsx! {
        Dialog { is_open, on_open_change, label, purpose, padding, max_height: Some(height),
            class: "astryx-bottom-sheet", "data-tall": is_tall.then_some("true"),
            div { class: "astryx-bottom-sheet__content", tabindex: -1, "data-autofocus": "sheet-content", {children} }
        }
    }
}
