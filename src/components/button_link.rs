use dioxus::prelude::*;

/// Button's native-link rendering contract for application router adapters.
///
/// Forward `attributes`, `onclick`, `onkeydown`, and `onmounted` to the same
/// focusable link element. Render `children` inside it. Keeping the native href,
/// target, and rel preserves modified-click, download, and browser navigation.
#[derive(Clone, PartialEq)]
pub struct ButtonLinkProps {
    pub href: String,
    pub target: Option<String>,
    pub rel: Option<String>,
    pub attributes: Vec<Attribute>,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
    pub onmounted: EventHandler<MountedEvent>,
    pub children: Element,
}

/// Caller-supplied link renderer; explicit Button renderers override the provider.
pub type ButtonLinkRenderer = Callback<ButtonLinkProps, Element>;

#[derive(Clone, Copy)]
pub(crate) struct ButtonLinkContext(pub Signal<ButtonLinkRenderer>);

/// Supplies a router-aware link renderer without adding a DOM wrapper.
#[component]
pub fn ButtonLinkProvider(#[props(into)] render: ButtonLinkRenderer, children: Element) -> Element {
    let mut context = use_context_provider(|| ButtonLinkContext(Signal::new(render)));
    if *context.0.peek() != render {
        context.0.set(render);
    }
    children
}
