use dioxus::prelude::*;

use crate::utils::is_renderable::is_renderable;

/// Default-mode top navigation with heading, start, center and end slots.
#[component]
pub fn TopNav(
    #[props(default = "Top navigation".to_owned())] label: String,
    #[props(default)] heading: Option<Element>,
    #[props(default)] start_content: Option<Element>,
    #[props(default)] center_content: Option<Element>,
    #[props(default)] end_content: Option<Element>,
    /// Alias for start content when no explicit start slot is supplied.
    #[props(default)]
    children: Option<Element>,
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let heading = heading.filter(is_renderable);
    let start = start_content.or(children).filter(is_renderable);
    let center = center_content.filter(is_renderable);
    let end = end_content.filter(is_renderable);
    let has_center = center.is_some();
    let attributes = attributes
        .into_iter()
        .filter(|attr| !matches!(attr.name, "class" | "style" | "aria-label" | "role"))
        .collect::<Vec<_>>();
    rsx! {
        nav { class: format!("astryx-top-nav {class}"), style, "aria-label": label, ..attributes,
            div { class: "astryx-top-nav__leading",
                if let Some(heading) = heading { div { class: "astryx-top-nav__heading", {heading} } }
                if let Some(start) = start { div { class: "astryx-top-nav__start", {start} } }
            }
            if let Some(center) = center { div { class: "astryx-top-nav__center", {center} } }
            if has_center || end.is_some() {
                div { class: "astryx-top-nav__end", "data-balanced": has_center.then_some("true"), {end} }
            }
        }
    }
}
