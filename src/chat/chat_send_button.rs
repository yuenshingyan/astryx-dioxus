use dioxus::prelude::*;

use crate::{Button, ButtonSize, ButtonVariant, Icon};

/// Supported sizes for a chat send or stop action.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ChatSendButtonSize {
    /// Compact control.
    Small,
    /// Standard control.
    #[default]
    Medium,
}

/// An icon-only send or stop action. The caller owns readiness and state transitions.
#[component]
pub fn ChatSendButton(
    /// Selects the stop action, independently of send readiness.
    #[props(default)]
    is_stop_shown: bool,
    /// Disables send; has no effect on stop.
    #[props(default = true)]
    is_disabled: bool,
    #[props(default)] on_send: Option<EventHandler<()>>,
    #[props(default)] on_stop: Option<EventHandler<()>>,
    /// Runs after the current action.
    #[props(default)]
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(default)] send_icon: Option<Element>,
    #[props(default)] stop_icon: Option<Element>,
    #[props(default = "Send".to_owned())] send_label: String,
    #[props(default = "Stop".to_owned())] stop_label: String,
    #[props(default)] size: ChatSendButtonSize,
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(default)] onmounted: EventHandler<MountedEvent>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let (label, variant, icon, action) = if is_stop_shown {
        (stop_label, ButtonVariant::Secondary, stop_icon, on_stop)
    } else {
        (send_label, ButtonVariant::Primary, send_icon, on_send)
    };
    let size = match size {
        ChatSendButtonSize::Small => ButtonSize::Small,
        ChatSendButtonSize::Medium => ButtonSize::Medium,
    };
    let icon = icon.unwrap_or_else(|| rsx! {
        Icon { icon: rsx! {
            if is_stop_shown {
                svg { view_box: "0 0 24 24", fill: "currentColor",
                    rect { x: "5", y: "5", width: "14", height: "14", rx: "2" }
                }
            } else {
                svg { view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                    path { d: "M12 19V5m-6 6 6-6 6 6" }
                }
            }
        } }
    });
    rsx! {
        Button { label, variant, size, icon, is_icon_only: true, is_disabled: !is_stop_shown && is_disabled,
            class: "astryx-chat-send-button {class}", style, attributes, onmounted,
            onclick: move |event| {
                if let Some(action) = action { action.call(()); }
                if let Some(onclick) = onclick { onclick.call(event); }
            },
        }
    }
}
