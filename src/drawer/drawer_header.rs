use dioxus::prelude::*;

use crate::utils::is_renderable::is_renderable;
use crate::{
    Button, ButtonSize, ButtonVariant, Heading, HeadingLevel, Icon, LayoutHeader, Text, TextColor,
    TextSize,
};

/// Edges on which trailing content compensates for the close button's padding.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DrawerHeaderEdgeCompensation {
    Inline,
    Block,
    All,
}

/// Drawer title, optional subtitle, and caller-owned actions.
/// The enclosing drawer owns its accessible name and dismissal behavior.
#[component]
pub fn DrawerHeader(
    /// Inline, noninteractive title rendered in an h2.
    title: Element,
    /// Optional supporting inline content.
    #[props(default)]
    subtitle: Option<Element>,
    /// When supplied, renders a close button that requests `false`.
    #[props(default)]
    on_open_change: Option<EventHandler<bool>>,
    /// Localized close button label and tooltip.
    #[props(default = "Close".to_owned())]
    close_label: String,
    /// Content before the title.
    #[props(default)]
    start_content: Option<Element>,
    /// Content before the close button at the inline end.
    #[props(default)]
    end_content: Option<Element>,
    /// Overrides automatic compensation on both axes when a close button exists.
    #[props(default)]
    end_content_edge_compensation: Option<DrawerHeaderEdgeCompensation>,
    /// Overrides the enclosing Layout divider setting.
    #[props(default)]
    has_divider: Option<bool>,
    #[props(default)] class: String,
    #[props(default)] style: String,
    /// Native attributes on the outer LayoutHeader.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let end_content = end_content.filter(is_renderable);
    let compensation =
        end_content_edge_compensation.or(on_open_change.map(|_| DrawerHeaderEdgeCompensation::All));
    let compensate = match compensation {
        Some(DrawerHeaderEdgeCompensation::Inline) => Some("inline"),
        Some(DrawerHeaderEdgeCompensation::Block) => Some("block"),
        Some(DrawerHeaderEdgeCompensation::All) => Some("all"),
        None => None,
    };
    rsx! {
        LayoutHeader { has_divider, class, style, attributes,
            div { class: "astryx-drawer-header",
                if let Some(start) = start_content.filter(is_renderable) {
                    div { class: "astryx-drawer-header-start-content", {start} }
                }
                div { class: "astryx-drawer-header-title-block",
                    Heading { level: HeadingLevel::Two, {title} }
                    if let Some(subtitle) = subtitle.filter(is_renderable) {
                        Text { size: TextSize::Small, color: TextColor::SECONDARY, {subtitle} }
                    }
                }
                if end_content.is_some() || on_open_change.is_some() {
                    div { class: "astryx-drawer-header-end-content", "data-compensation": compensate,
                        {end_content}
                        if let Some(close) = on_open_change {
                            Button { label: close_label.clone(), tooltip: close_label, variant: ButtonVariant::Ghost,
                                size: ButtonSize::Medium, is_icon_only: true, onclick: move |_| close.call(false),
                                icon: rsx! { Icon { class: "astryx-drawer-header-close-icon", icon: rsx! {
                                    svg { view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round",
                                        path { d: "M6 6l12 12M18 6 6 18" }
                                    }
                                } } },
                            }
                        }
                    }
                }
            }
        }
    }
}
