//! Layout-owned slot presence. A new layer starts without surrounding slots.

#[derive(Clone, Copy, Default)]
pub(crate) struct LayoutSlotsContext {
    pub(crate) has_header: bool,
    pub(crate) has_footer: bool,
    pub(crate) has_start: bool,
    pub(crate) has_end: bool,
}
