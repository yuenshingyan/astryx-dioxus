use dioxus::prelude::*;

use crate::utils::is_renderable::is_renderable;
use crate::{Divider, DividerLabel};

/// Presentation of a system message in a chat transcript.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ChatSystemMessageVariant {
    /// Centered notice, optionally with a leading icon.
    #[default]
    Default,
    /// Labeled divider, typically a date marker. The icon slot is not used.
    Divider,
}

/// A non-sender message. Live announcements belong to the enclosing transcript.
#[component]
pub fn ChatSystemMessage(
    children: Element,
    #[props(default)] variant: ChatSystemMessageVariant,
    #[props(default)] icon: Option<Element>,
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let variant_name = match variant {
        ChatSystemMessageVariant::Default => "default",
        ChatSystemMessageVariant::Divider => "divider",
    };
    let attributes = attributes
        .into_iter()
        .filter(|attr| !matches!(attr.name, "class" | "style" | "data-variant"))
        .collect::<Vec<_>>();
    rsx! {
        div { class: "astryx-chat-system-message {class}", style, "data-variant": variant_name, ..attributes,
            match variant {
                ChatSystemMessageVariant::Default => rsx! {
                    span { class: "astryx-chat-system-message__content",
                        if let Some(icon) = icon.filter(is_renderable) {
                            span { class: "astryx-chat-system-message__icon", {icon} }
                        }
                        {children}
                    }
                },
                ChatSystemMessageVariant::Divider => {
                    let label = if is_renderable(&children) { DividerLabel::Rich(children) } else { DividerLabel::Empty };
                    rsx! { Divider { label } }
                },
            }
        }
    }
}
