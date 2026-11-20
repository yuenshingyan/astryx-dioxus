// Adapted from Stack/StackItem.tsx and Stack/stackItem.stylex.ts.
// Private static/fill div slice for StackFillItem; no scroll-region API exposed.
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum PatternItemSize {
    Static,
    Fill,
}

#[component]
pub(crate) fn PatternStackItem(size: PatternItemSize, children: Element) -> Element {
    let size = match size {
        PatternItemSize::Static => "static",
        PatternItemSize::Fill => "fill",
    };
    rsx! { div { class: "astryx-stack-item", "data-size": size, {children} } }
}
