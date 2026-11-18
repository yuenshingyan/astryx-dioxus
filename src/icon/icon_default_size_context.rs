// Adapted from Meta Astryx, baseline 476a3153417dbf75c565f7df05b0784e94ddd59e.
// See THIRD_PARTY_NOTICES.md.

use dioxus::prelude::*;

use super::icon_size::IconSize;

/// Reactive default for unsized Icon components in an owning component's slot.
#[derive(Clone, Copy, PartialEq)]
pub struct IconDefaultSizeContext(pub Signal<IconSize>);

/// Sets the nearest icon-slot default without a wrapper element.
#[component]
pub fn IconDefaultSizeProvider(size: IconSize, children: Element) -> Element {
    let mut context = use_context_provider(|| IconDefaultSizeContext(Signal::new(size)));
    if *context.0.peek() != size {
        context.0.set(size);
    }
    children
}

/// Resolves an explicit icon size, then the nearest slot default, then Medium.
pub fn use_icon_size(size: Option<IconSize>) -> IconSize {
    let inherited = try_consume_context::<IconDefaultSizeContext>().map(|context| (context.0)());
    size.or(inherited).unwrap_or_default()
}
