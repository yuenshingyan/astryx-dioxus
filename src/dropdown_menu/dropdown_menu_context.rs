use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub(super) struct RadioGroupSettings {
    pub(super) value: Option<String>,
    pub(super) on_change: EventHandler<String>,
    pub(super) has_close_on_select: bool,
}

#[derive(Clone, Copy)]
pub(super) struct DropdownMenuRadioGroupContext(pub(super) Signal<RadioGroupSettings>);
