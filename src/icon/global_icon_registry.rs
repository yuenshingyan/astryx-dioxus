// Adapted from Astryx Icon/globalIconRegistry.tsx. See THIRD_PARTY_NOTICES.md.

use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use dioxus::prelude::*;

use super::IconSize;

/// Size and optional application-defined variant supplied to registry artwork.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IconRequest {
    /// Resolved Icon size.
    pub size: IconSize,
    /// Optional renderer-specific variant.
    pub variant: Option<String>,
}

/// Artwork or a pure synchronous size-aware renderer. Empty output is final.
#[derive(Clone)]
pub enum IconEntry {
    /// Caller-provided decorative artwork.
    Node(Element),
    /// Produce artwork for the requested size/variant.
    Renderer(Rc<dyn Fn(IconRequest) -> Element>),
}

/// A global icon key was empty or contained control characters.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvalidIconName;

impl Display for InvalidIconName {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(
            "icon registration failed: keys must be nonempty and contain no control characters",
        )
    }
}
impl Error for InvalidIconName {}

thread_local! {
    static REGISTRY: RefCell<HashMap<String, IconEntry>> = RefCell::new(HashMap::new());
}

/// Merge global overrides on the UI thread, preserving other registered keys.
/// Register before mounting; this setter does not trigger a renderer update.
/// Keys are open, including library namespaces such as `editor:bold`.
/// This is the global extension seam, not a Theme provider or a builtin catalog.
pub fn register_icons(
    icons: impl IntoIterator<Item = (String, IconEntry)>,
) -> Result<(), InvalidIconName> {
    let entries: Vec<_> = icons.into_iter().collect();
    if entries
        .iter()
        .any(|(name, _)| name.is_empty() || name.chars().any(char::is_control))
    {
        return Err(InvalidIconName);
    }
    REGISTRY.with(|registry| registry.borrow_mut().extend(entries));
    Ok(())
}

/// Resolve a global extension at medium size, or use the caller's fallback.
/// Registered empty artwork never silently falls back.
pub fn get_extended_icon(name: &str, fallback: Element) -> Element {
    resolve_icon(
        name,
        IconRequest {
            size: IconSize::Medium,
            variant: None,
        },
        fallback,
    )
}

pub(crate) fn resolve_icon(name: &str, request: IconRequest, fallback: Element) -> Element {
    let entry = REGISTRY.with(|registry| registry.borrow().get(name).cloned());
    match entry {
        Some(IconEntry::Node(node)) => node,
        Some(IconEntry::Renderer(renderer)) => renderer(request),
        None => fallback,
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use dioxus::prelude::*;

    use super::{IconEntry, IconRequest, get_extended_icon, register_icons, resolve_icon};
    use crate::icon::IconSize;

    #[test]
    fn global_renderers_receive_the_actual_size_and_empty_output_is_final() {
        assert!(
            register_icons([
                (
                    "registry-test:renderer".into(),
                    IconEntry::Renderer(Rc::new(
                        |request| rsx! { span { "{request.size.as_str()}" } }
                    ))
                ),
                ("registry-test:empty".into(), IconEntry::Node(rsx! {})),
            ])
            .is_ok()
        );
        let mut dom = VirtualDom::new(|| {
            resolve_icon(
                "registry-test:renderer",
                IconRequest {
                    size: IconSize::ExtraSmall,
                    variant: None,
                },
                rsx! { "Fallback" },
            )
        });
        dom.rebuild_in_place();
        assert!(dioxus_ssr::render(&dom).contains("xsm"));
        let mut empty =
            VirtualDom::new(|| get_extended_icon("registry-test:empty", rsx! { "Fallback" }));
        empty.rebuild_in_place();
        assert!(!dioxus_ssr::render(&empty).contains("Fallback"));
    }

    #[test]
    fn invalid_registration_is_atomic_and_fallback_remains_available() {
        assert!(
            register_icons([
                (
                    "registry-test:valid".into(),
                    IconEntry::Node(rsx! { span { "Should not commit" } })
                ),
                (String::new(), IconEntry::Node(rsx! {}))
            ])
            .is_err()
        );
        let mut dom = VirtualDom::new(|| {
            get_extended_icon("registry-test:valid", rsx! { span { "Fallback" } })
        });
        dom.rebuild_in_place();
        assert!(dioxus_ssr::render(&dom).contains("Fallback"));
    }
}
