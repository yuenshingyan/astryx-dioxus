use dioxus::prelude::*;

use super::collapsible_group_context::CollapsibleGroupContext;

pub(super) fn use_collapsible(
    is_open: Option<bool>,
    default_is_open: bool,
    on_open_change: Option<EventHandler<bool>>,
    value: Option<String>,
) -> (bool, impl FnMut()) {
    let mut internal_open = use_signal(|| is_open.unwrap_or(default_is_open));
    let group = try_consume_context::<CollapsibleGroupContext>();
    let group = group.zip(value).map(|(group, value)| ((group.0)(), value));
    let is_expanded = group.as_ref().map_or_else(
        || is_open.unwrap_or_else(|| *internal_open.read()),
        |(group, value)| group.open_values.contains(value),
    );
    (is_expanded, move || {
        if let Some((group, value)) = &group {
            group.toggle.call(value.clone());
            return;
        }
        if is_open.is_none() {
            internal_open.set(!is_expanded);
        }
        if let Some(handler) = on_open_change {
            handler.call(!is_expanded);
        }
    })
}
