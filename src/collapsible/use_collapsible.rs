use dioxus::prelude::*;

pub(super) fn use_collapsible(
    is_open: Option<bool>,
    default_is_open: bool,
    on_open_change: Option<EventHandler<bool>>,
) -> (bool, impl FnMut()) {
    let mut internal_open = use_signal(|| is_open.unwrap_or(default_is_open));
    let is_expanded = is_open.unwrap_or_else(|| *internal_open.read());
    (is_expanded, move || {
        if is_open.is_none() {
            internal_open.set(!is_expanded);
        }
        if let Some(handler) = on_open_change {
            handler.call(!is_expanded);
        }
    })
}
