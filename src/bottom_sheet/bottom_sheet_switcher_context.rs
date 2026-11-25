use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub(super) struct BottomSheetSwitcherContext {
    pub(super) active_sheet: Option<String>,
    pub(super) on_active_sheet_change: EventHandler<Option<String>>,
}
