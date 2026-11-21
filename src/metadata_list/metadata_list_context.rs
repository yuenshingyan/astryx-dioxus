use dioxus::prelude::*;

/// Resolved presentation of the parent description list.
#[derive(Clone, Copy)]
pub(super) struct MetadataListContext {
    pub(super) is_stacked: Signal<bool>,
}
