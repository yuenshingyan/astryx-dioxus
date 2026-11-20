//! Avatar-owned size, scoped to the nearest layer.

#[derive(Clone, Copy)]
pub(crate) struct AvatarSizeContext(pub u32);

impl Default for AvatarSizeContext {
    fn default() -> Self {
        Self(36)
    }
}
