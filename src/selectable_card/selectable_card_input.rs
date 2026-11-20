// Adapted from Astryx SelectableCard/selectableCardInput.ts.
// This channel belongs to SelectableCard, not to a combined card environment.
use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SelectableCardInputType {
    Checkbox,
    Radio,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SelectableCardInput {
    pub input_type: SelectableCardInputType,
    pub name: String,
    pub value: String,
}

#[derive(Clone, Copy)]
pub(crate) struct SelectableCardInputContext(pub Signal<Option<SelectableCardInput>>);

pub(super) fn requested_selection(is_radio: bool, is_selected: bool) -> Option<bool> {
    if is_radio && is_selected {
        None
    } else {
        Some(!is_selected)
    }
}

#[cfg(test)]
#[path = "../../tests/internal/selectable_card_input.rs"]
mod tests;
