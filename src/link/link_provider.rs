use dioxus::prelude::*;

use super::link_context::LinkContext;
use super::types::LinkComponentType;

/// Supplies the default router-aware link component without adding a DOM wrapper.
/// Individual controls can override it with their explicit renderer.
#[component]
pub fn LinkProvider(#[props(into)] component: LinkComponentType, children: Element) -> Element {
    let mut context = use_context_provider(|| LinkContext(Signal::new(component)));
    if *context.0.peek() != component {
        context.0.set(component);
    }
    children
}

/// Compatibility spelling of LinkProvider with the provisional `render` prop.
/// New code should use `LinkProvider { component: ... }`.
#[component]
pub fn ButtonLinkProvider(#[props(into)] render: LinkComponentType, children: Element) -> Element {
    rsx! { LinkProvider { component: render, {children} } }
}
