//! Layout-owned logical placement context, reset at a layer boundary.

/// `None` is outside a side slot. `Some(true)` identifies the end slot.
#[derive(Clone, Copy, Default)]
pub(crate) struct LayoutAreaContext {
    pub(crate) is_end_panel: Option<bool>,
}
