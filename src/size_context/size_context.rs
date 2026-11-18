// Adapted from Meta Astryx, baseline 476a3153417dbf75c565f7df05b0784e94ddd59e.
// See THIRD_PARTY_NOTICES.md.

use dioxus::prelude::*;

/// Shared control size, independent of any particular component's API.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ElementSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl ElementSize {
    /// Upstream size selector.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Small => "sm",
            Self::Medium => "md",
            Self::Large => "lg",
        }
    }
}

/// Reactive inherited control size. Read the signal to subscribe to changes.
#[derive(Clone, Copy, PartialEq)]
pub struct SizeContext(pub Signal<ElementSize>);

/// Supplies a default size without adding a DOM element. The nearest provider wins.
#[component]
pub fn SizeProvider(size: ElementSize, children: Element) -> Element {
    let mut context = use_context_provider(|| SizeContext(Signal::new(size)));
    if *context.0.peek() != size {
        context.0.set(size);
    }
    children
}

/// Resolves an explicit size, then the nearest provider, then the default.
pub fn use_size(size: Option<ElementSize>) -> ElementSize {
    let inherited = try_consume_context::<SizeContext>().map(|context| (context.0)());
    size.or(inherited).unwrap_or_default()
}
