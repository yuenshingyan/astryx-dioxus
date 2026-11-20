// Adapted from Astryx VisuallyHidden. See THIRD_PARTY_NOTICES.md.
use dioxus::prelude::*;

/// Inputs to a caller-rendered hidden root. Forward the attributes unchanged.
#[derive(Clone, PartialEq)]
pub struct VisuallyHiddenRenderProps {
    /// Fixed clipping styles and native attributes or callbacks.
    pub attributes: Vec<Attribute>,
    /// Content retained in the accessibility tree unless hidden by the caller.
    pub children: Element,
}

/// Renders the hidden content in one caller-selected element.
pub type VisuallyHiddenElement = Callback<VisuallyHiddenRenderProps, Element>;

/// Keep content in the accessibility tree with the original fixed clip block.
/// There is no styling target, and raw caller styling cannot replace the block.
#[component]
pub fn VisuallyHidden(
    /// Content to clip visually.
    children: Element,
    /// Render a different root instead of the default span.
    #[props(default)]
    as_element: Option<VisuallyHiddenElement>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attr| !matches!(attr.name, "style" | "class" | "className" | "xstyle"))
        .collect();
    let mut attributes = attributes;
    attributes.push(Attribute::new("style", "position:absolute;width:1px;height:1px;margin:-1px;padding:0;overflow:hidden;clip:rect(0,0,0,0);white-space:nowrap;border-style:none;inset-block-start:0;inset-inline-start:0;pointer-events:none;user-select:none", None, false));
    attributes.sort_by_key(|attribute| attribute.name);
    if let Some(render) = as_element {
        return render.call(VisuallyHiddenRenderProps {
            attributes,
            children,
        });
    }
    rsx! { span { ..attributes, {children} } }
}
