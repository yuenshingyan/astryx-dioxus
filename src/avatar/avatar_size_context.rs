//! Avatar-owned size, scoped to the nearest layer.

use dioxus::prelude::*;

#[derive(Clone, Copy)]
pub(crate) struct AvatarSizeContext(pub Signal<u32>);

impl Default for AvatarSizeContext {
    fn default() -> Self {
        Self(Signal::new(36))
    }
}
