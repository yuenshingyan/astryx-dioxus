use dioxus::prelude::*;

use crate::utils::is_renderable::is_renderable;
use crate::{
    Button, ButtonSize, ButtonVariant, Heading, HeadingLevel, Icon, LayoutHeader, Text, TextColor,
    TextSize,
};

const DEFAULT_CLOSE_LABEL: &str = "Close";

/// Edges on which trailing content compensates for the close button's padding.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DialogHeaderEdgeCompensation {
    Inline,
    Block,
    All,
}

/// Dialog title, optional subtitle, and caller-owned actions.
/// The enclosing dialog owns dismissal and may reference `title_id` for its name.
#[component]
pub fn DialogHeader(
    /// Inline, noninteractive title rendered in an h2.
    title: Element,
    /// ID the enclosing dialog can reference from `aria-labelledby`.
    #[props(default)]
    title_id: Option<String>,
    /// Suppress title autofocus in an inline preview.
    #[props(default)]
    is_inline: bool,
    /// Optional supporting inline content.
    #[props(default)]
    subtitle: Option<Element>,
    /// When supplied, renders a close button that requests `false`.
    #[props(default)]
    on_open_change: Option<EventHandler<bool>>,
    /// Localized close button label and tooltip. Blank values fall back to "Close".
    #[props(default = DEFAULT_CLOSE_LABEL.to_owned())]
    close_label: String,
    /// Content before the title.
    #[props(default)]
    start_content: Option<Element>,
    /// Content before the close button at the inline end.
    #[props(default)]
    end_content: Option<Element>,
    /// Overrides automatic compensation on both axes when a close button exists.
    #[props(default)]
    end_content_edge_compensation: Option<DialogHeaderEdgeCompensation>,
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
    let close_label = if close_label.trim().is_empty() {
        DEFAULT_CLOSE_LABEL.to_owned()
    } else {
        close_label
    };
    let compensation =
        end_content_edge_compensation.or(on_open_change.map(|_| DialogHeaderEdgeCompensation::All));
    let compensate = match compensation {
        Some(DialogHeaderEdgeCompensation::Inline) => Some("inline"),
        Some(DialogHeaderEdgeCompensation::Block) => Some("block"),
        Some(DialogHeaderEdgeCompensation::All) => Some("all"),
        None => None,
    };
    rsx! {
        LayoutHeader { has_divider, class, style, attributes,
            div { class: "astryx-dialog-header",
                if let Some(start) = start_content.filter(is_renderable) {
                    div { class: "astryx-dialog-header-start-content", {start} }
                }
                div { class: "astryx-dialog-header-title-block",
                    Heading { level: HeadingLevel::Two, id: title_id, tabindex: "-1",
                        class: "astryx-dialog-header-focus-title",
                        "data-autofocus": (!is_inline).then_some("dialog-title"),
                        onmounted: move |event: MountedEvent| {
                            if !is_inline { spawn(async move { let _ = event.data().set_focus(true).await; }); }
                        },
                        {title}
                    }
                    if let Some(subtitle) = subtitle.filter(is_renderable) {
                        Text { size: TextSize::Small, color: TextColor::SECONDARY, {subtitle} }
                    }
                }
                if end_content.is_some() || on_open_change.is_some() {
                    div { class: "astryx-dialog-header-end-content", "data-compensation": compensate,
                        {end_content}
                        if let Some(close) = on_open_change {
                            Button { label: close_label.clone(), tooltip: close_label, variant: ButtonVariant::Ghost,
                                size: ButtonSize::Medium, is_icon_only: true, onclick: move |_| close.call(false),
                                icon: rsx! { Icon { class: "astryx-dialog-header-close-icon", icon: rsx! {
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
