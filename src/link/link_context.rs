use dioxus::prelude::*;

use super::types::LinkComponentType;

#[derive(Clone, Copy)]
pub(crate) struct LinkContext(pub Signal<LinkComponentType>);
