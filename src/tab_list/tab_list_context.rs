use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub(super) struct TabListState {
    pub value: String,
    pub on_change: EventHandler<String>,
    pub is_panel_list: bool,
}

#[derive(Clone, Copy)]
pub(super) struct TabListContext(pub Signal<TabListState>);
