use dioxus::prelude::*;

use crate::IndicatorSize;

#[derive(Clone, PartialEq)]
pub(super) struct RadioListSettings {
    pub(super) name: String,
    pub(super) value: Option<String>,
    pub(super) on_change: Option<EventHandler<String>>,
    pub(super) size: IndicatorSize,
    pub(super) is_disabled: bool,
    pub(super) is_required: bool,
}

#[derive(Clone, Copy)]
pub(super) struct RadioListContext(pub(super) Signal<RadioListSettings>);
