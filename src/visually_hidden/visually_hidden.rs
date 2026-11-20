// Fixed-span internal use of upstream VisuallyHidden. Its public polymorphic
// API and standalone examples remain an independent migration.
use dioxus::prelude::*;

/// Keep content in the accessibility tree with the original fixed clip block.
/// There is no styling target, and raw caller styling cannot replace the block.
#[component]
pub(crate) fn VisuallyHidden(
    children: Element,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attr| !matches!(attr.name, "style" | "class" | "className" | "xstyle"))
        .collect();
    rsx! {span{
        style:"position:absolute;width:1px;height:1px;margin:-1px;padding:0;overflow:hidden;clip:rect(0,0,0,0);white-space:nowrap;border-style:none;inset-block-start:0;inset-inline-start:0;pointer-events:none;user-select:none",
        ..attributes,{children}
    }}
}
