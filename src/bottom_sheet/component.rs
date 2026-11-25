use dioxus::prelude::*;

use super::bottom_sheet_switcher_context::BottomSheetSwitcherContext;
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
    #[props(default)] is_open: bool,
    #[props(default)] on_open_change: EventHandler<bool>,
    label: String,
    children: Element,
    #[props(default)] height: BottomSheetHeight,
    #[props(default)] purpose: DialogPurpose,
    #[props(default)] padding: Option<SpacingStep>,
    /// Identifies a child step within a BottomSheetSwitcher. Omit for standalone sheets.
    #[props(default)]
    sheet_id: Option<String>,
) -> Element {
    let step = sheet_id.filter(|id| !id.trim().is_empty());
    let switcher = step
        .as_ref()
        .and_then(|_| try_consume_context::<Signal<BottomSheetSwitcherContext>>());
    let is_open = switcher.map_or(is_open, |state| state.read().active_sheet == step);
    let on_change = move |open: bool| {
        if let Some(state) = switcher {
            let callback = {
                let current = state.peek();
                (!open && current.active_sheet == step).then_some(current.on_active_sheet_change)
            };
            if let Some(callback) = callback {
                callback.call(None);
            }
        } else {
            on_open_change.call(open);
        }
    };
    let (height, is_tall) = match height {
        BottomSheetHeight::Hug => ("92dvh".into(), false),
        BottomSheetHeight::Capped => ("62dvh".into(), false),
        BottomSheetHeight::Tall => ("92dvh".into(), true),
        BottomSheetHeight::Custom(height) => (height, false),
    };
    rsx! {
        Dialog { is_open, on_open_change: on_change, label, purpose, padding, max_height: Some(height),
            class: "astryx-bottom-sheet", "data-tall": is_tall.then_some("true"),
            div { class: "astryx-bottom-sheet__content", tabindex: -1, "data-autofocus": "sheet-content", {children} }
        }
    }
}
