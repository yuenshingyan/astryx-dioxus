// Adapted from Meta Astryx Text/textClampContext.ts, baseline
// 476a3153417dbf75c565f7df05b0784e94ddd59e. See THIRD_PARTY_NOTICES.md.

use dioxus::prelude::*;

/// Whether an ancestor Text clips its line boxes. Reset to false at layer boundaries.
#[derive(Clone, Copy, PartialEq)]
pub(crate) struct TextClampContext(pub Signal<bool>);

/// Reads the nearest reactive clamp scope; outside Text the default is false.
pub(crate) fn use_text_clamp_context() -> bool {
    try_consume_context::<TextClampContext>().is_some_and(|context| (context.0)())
}

/// Always mounted, including when clamping toggles, to preserve child identity.
#[component]
pub(crate) fn TextClampProvider(is_clamped: bool, children: Element) -> Element {
    let mut context = use_context_provider(|| TextClampContext(Signal::new(is_clamped)));
    if *context.0.peek() != is_clamped {
        context.0.set(is_clamped);
    }
    children
}
