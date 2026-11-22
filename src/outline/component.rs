use std::fmt::Write;

use dioxus::prelude::*;

use crate::link::{LinkComponentType, render_navigation_anchor, use_link_component};

/// A heading destination in document order. IDs must be unique in the outline.
#[derive(Clone, PartialEq, Eq)]
pub struct OutlineItem {
    pub id: String,
    pub label: String,
    pub level: u8,
}

/// Vertical spacing between outline links.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum OutlineDensity {
    #[default]
    Default,
    Compact,
}

/// Table of contents with native fragment links and caller-owned current location.
#[component]
pub fn Outline(
    items: Vec<OutlineItem>,
    #[props(default)] active_id: Option<String>,
    #[props(default = "Table of contents".to_owned())] label: String,
    #[props(default)] density: OutlineDensity,
    #[props(default)] class: String,
    #[props(default)] style: String,
) -> Element {
    let renderer = use_link_component(None);
    let is_compact = density == OutlineDensity::Compact;
    rsx! {
        nav { class: "astryx-outline {class}", style, "aria-label": label, "data-density": if is_compact { "compact" } else { "default" },
            ul { class: "astryx-outline__list", role: "list",
                for item in items.into_iter().filter(|item| !item.id.is_empty()) {
                    li { key: "{item.id}",
                        {render_item(item, active_id.as_deref(), renderer)}
                    }
                }
            }
        }
    }
}

fn render_item(
    item: OutlineItem,
    active_id: Option<&str>,
    renderer: Option<LinkComponentType>,
) -> Element {
    let mut attributes = vec![
        Attribute::new("class", "astryx-outline-item", None, false),
        Attribute::new(
            "data-indent",
            item.level.saturating_sub(1).clamp(1, 4).to_string(),
            None,
            false,
        ),
    ];
    if active_id == Some(item.id.as_str()) {
        attributes.push(Attribute::new("aria-current", "location", None, false));
    }
    let label = item.label;
    render_navigation_anchor(
        fragment(&item.id),
        renderer,
        attributes,
        rsx! { span { "{label}" } },
        None,
    )
}

fn fragment(id: &str) -> String {
    let mut href = String::from("#");
    for byte in id.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
            href.push(char::from(byte));
        } else {
            let _ = write!(href, "%{byte:02X}");
        }
    }
    href
}
