//! Sizing and overlap published by the AvatarGroup owner.

use dioxus::prelude::*;

use crate::avatar::{AvatarShape, AvatarSize};

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct AvatarGroupSettings {
    pub(crate) size: AvatarSize,
    pub(crate) shape: AvatarShape,
    pub(crate) overlap: u32,
    pub(crate) owner: Option<usize>,
}

#[derive(Clone, Copy)]
pub(crate) struct AvatarGroupContext(pub(crate) Option<Signal<AvatarGroupSettings>>);

pub(crate) fn avatar_group_settings() -> AvatarGroupSettings {
    try_consume_context::<AvatarGroupContext>()
        .and_then(|context| context.0)
        .map_or_else(AvatarGroupSettings::default, |settings| settings())
}
