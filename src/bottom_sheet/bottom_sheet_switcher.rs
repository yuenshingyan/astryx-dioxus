use dioxus::prelude::*;

use super::bottom_sheet_switcher_context::BottomSheetSwitcherContext;

/// Coordinates modal BottomSheet children by their `sheet_id`.
/// The upstream API is deprecated; prefer standalone sheets for new flows.
#[component]
pub fn BottomSheetSwitcher(
    active_sheet: Option<String>,
    on_active_sheet_change: EventHandler<Option<String>>,
    children: Element,
) -> Element {
    let mut state = use_context_provider(|| {
        Signal::new(BottomSheetSwitcherContext {
            active_sheet: active_sheet.clone(),
            on_active_sheet_change,
        })
    });
    let next = BottomSheetSwitcherContext {
        active_sheet,
        on_active_sheet_change,
    };
    if *state.peek() != next {
        state.set(next);
    }
    children
}
