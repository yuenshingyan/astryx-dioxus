use dioxus::prelude::*;

use crate::SizeValue;

/// Logical viewport edges reserved for persistent UI.
#[derive(Clone, Default, PartialEq)]
pub struct LayerInset {
    pub block_start: Option<SizeValue>,
    pub block_end: Option<SizeValue>,
    pub inline_start: Option<SizeValue>,
    pub inline_end: Option<SizeValue>,
}

#[derive(Clone, Copy)]
struct LayerContext(Signal<Option<LayerInset>>);

/// Supplies viewport insets without adding a DOM wrapper. An omitted inset
/// inherits from the nearest provider; a supplied inset replaces all edges.
#[component]
pub fn LayerProvider(children: Element, #[props(default)] inset: Option<LayerInset>) -> Element {
    // Capture the parent before this scope installs its own context.
    let parent = use_hook(try_consume_context::<LayerContext>);
    let inherited = parent.and_then(|ctx| ctx.0.read().clone());
    let inset = inset.or(inherited);
    let mut ctx = use_context_provider(|| LayerContext(Signal::new(inset.clone())));
    if *ctx.0.peek() != inset {
        ctx.0.set(inset);
    }
    children
}

fn consume_inset() -> Option<LayerInset> {
    try_consume_context::<LayerContext>().and_then(|ctx| ctx.0.read().clone())
}

pub(crate) fn layer_inset_style() -> Option<String> {
    let inset = consume_inset()?;
    Some(
        [
            ("block-start", inset.block_start),
            ("block-end", inset.block_end),
            ("inline-start", inset.inline_start),
            ("inline-end", inset.inline_end),
        ]
        .into_iter()
        .filter_map(|(edge, value)| {
            let value = value?.as_css().ok()?;
            Some(format!("--_astryx-layer-inset-{edge}:{value};"))
        })
        .collect(),
    )
}
