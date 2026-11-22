use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::{Badge, BadgeVariant};

/// Inline supplementary content above a chat composer, not a modal drawer.
#[component]
pub fn ChatComposerDrawer(
    children: Element,
    /// Enables disclosure, including when the count is zero.
    #[props(default)]
    count: Option<u32>,
    #[props(default = "Attachments".to_owned())] label: String,
    /// Non-interactive visual summary shown on the collapsed trigger.
    #[props(default)]
    collapsed_summary: Option<Element>,
    #[props(default)] is_collapsed: Option<bool>,
    #[props(default)] default_is_collapsed: bool,
    #[props(default)] on_collapsed_change: Option<EventHandler<bool>>,
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let mut internal = use_signal(|| default_is_collapsed);
    let can_collapse = count.is_some();
    let is_closed = can_collapse && is_collapsed.unwrap_or_else(|| *internal.read());
    let id = use_hook(|| format!("astryx-chat-composer-drawer-{}", current_scope_id().0));
    let toggle_label = if is_closed {
        format!("Expand {label}")
    } else {
        format!("Collapse {label}")
    };
    rsx! {
        div { class: format!("astryx-chat-composer-drawer {class}"), style, "data-collapsed": is_closed.then_some("true"), ..attributes,
            if let Some(count) = count {
                button { class: "astryx-chat-composer-drawer__toggle", r#type: "button", "aria-expanded": (!is_closed).to_string(),
                    "aria-controls": id.clone(), "aria-label": toggle_label,
                    onclick: move |_| {
                        if is_collapsed.is_none() { internal.set(!is_closed); }
                        if let Some(handler) = on_collapsed_change { handler.call(!is_closed); }
                    },
                    span { class: "astryx-chat-composer-drawer__summary", "aria-hidden": "true", inert: true,
                        {collapsed_summary.unwrap_or_else(|| rsx! { Badge { label: count, variant: BadgeVariant::NEUTRAL } span { "{label}" } })}
                    }
                    span { class: "astryx-chat-composer-drawer__grip", "aria-hidden": "true" }
                }
            }
            div { class: "astryx-chat-composer-drawer__region", id, inert: is_closed.then_some(true),
                div { class: "astryx-chat-composer-drawer__content", {children} }
            }
        }
    }
}
