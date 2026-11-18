use dioxus::prelude::*;

/// Native link-root contract shared by router adapters and link-like controls.
///
/// Forward native properties, attributes, handlers, and children to one focusable
/// link root. Preserve modified clicks and normal browser navigation.
#[derive(Clone, PartialEq)]
pub struct LinkRenderProps {
    pub href: String,
    pub target: Option<String>,
    pub rel: Option<String>,
    pub attributes: Vec<Attribute>,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
    pub onmounted: EventHandler<MountedEvent>,
    pub children: Element,
}

/// Application-supplied link component, used unless a control overrides it.
pub type LinkComponentType = Callback<LinkRenderProps, Element>;
