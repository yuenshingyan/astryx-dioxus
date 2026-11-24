use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub(super) struct Settings {
    pub active_step: usize,
    pub on_step_click: Option<EventHandler<usize>>,
}

#[derive(Clone, Copy)]
pub(super) struct StepperContext(pub Signal<Settings>);
