use dioxus::prelude::*;

/// Resolved presentation of the parent description list.
#[derive(Clone, Copy)]
pub(crate) struct MetadataListContext {
    pub(crate) is_stacked: Signal<bool>,
}
