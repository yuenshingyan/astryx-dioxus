//! Grouping and header visibility for SideNav sections.

use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::utils::is_renderable::is_renderable;
use crate::{VisuallyHidden, VisuallyHiddenRenderProps};

/// Groups related navigation items under a title that also names the group.
#[component]
pub fn SideNavSection(
    /// Visible title and accessible group name.
    title: String,
    /// Optional line under the title.
    #[props(default)]
    subtitle: Option<String>,
    /// Content at the inline end of the header. Do not supply focusable content
    /// when the header is visually hidden.
    #[props(default)]
    end_content: Option<Element>,
    /// Visually clip the header without removing the accessible group name.
    #[props(default)]
    is_header_hidden: bool,
    /// Navigation items owned by the caller.
    children: Element,
    /// Additional root classes.
    #[props(default)]
    class: String,
    /// Inline declarations on the root.
    #[props(default)]
    style: Option<String>,
    /// Native root attributes. The component owns the group role and name.
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let title_id = format!("astryx-side-nav-section-{}-title", current_scope_id().0);
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attr| !matches!(attr.name, "role" | "aria-labelledby" | "aria-label"))
        .collect();
    let header = rsx! {
        span { class: "astryx-side-nav-section__titles",
            span { id: title_id.clone(), class: "astryx-side-nav-section__title", "{title}" }
            if let Some(subtitle) = subtitle.filter(|text| !text.is_empty()) {
                span { class: "astryx-side-nav-section__subtitle", "{subtitle}" }
            }
        }
        if let Some(end) = end_content.filter(is_renderable) {
            span { class: "astryx-side-nav-section__end", {end} }
        }
    };
    rsx! {
        div { class: "astryx-side-nav-section {class}", style, role: "group", "aria-labelledby": title_id, ..attributes,
            if is_header_hidden {
                VisuallyHidden { as_element: |props: VisuallyHiddenRenderProps| rsx! { div { ..props.attributes, {props.children} } }, {header} }
            } else {
                div { class: "astryx-side-nav-section__header", {header} }
            }
            div { class: "astryx-side-nav-section__items", {children} }
        }
    }
}
