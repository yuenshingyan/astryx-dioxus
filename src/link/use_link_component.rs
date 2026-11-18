use dioxus::prelude::*;

use super::link_context::LinkContext;
use super::types::LinkComponentType;

/// Resolves an explicit renderer before the nearest provider. None uses an anchor.
pub fn use_link_component(component: Option<LinkComponentType>) -> Option<LinkComponentType> {
    let inherited = try_consume_context::<LinkContext>().map(|context| (context.0)());
    component.or(inherited)
}
