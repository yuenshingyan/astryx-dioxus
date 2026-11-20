//! Layout-owned slot presence. A new layer starts without surrounding slots.

use dioxus::prelude::*;

#[derive(Clone, Copy)]
pub(crate) struct LayoutSlotsContext(pub Signal<LayoutSlots>);

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct LayoutSlots {
    pub(crate) has_header: bool,
    pub(crate) has_footer: bool,
    pub(crate) has_start: bool,
    pub(crate) has_end: bool,
}

pub(crate) fn layout_slots() -> LayoutSlots {
    try_consume_context::<LayoutSlotsContext>()
        .map(|ctx| (ctx.0)())
        .unwrap_or_default()
}
