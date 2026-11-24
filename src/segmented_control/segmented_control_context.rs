use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub(super) struct Settings {
    pub value: String,
    pub on_change: EventHandler<String>,
    pub is_disabled: bool,
}

#[derive(Clone, Copy)]
pub(super) struct SegmentedControlContext(pub Signal<Settings>);
