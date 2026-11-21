//! Overflow marker adapted from Astryx AvatarGroup/AvatarGroupOverflow.tsx.

use dioxus::core::{AttributeValue, current_scope_id};
use dioxus::prelude::*;

use super::avatar_group_context::avatar_group_settings;
use crate::hooks::use_press_feedback::{PressFeedbackConfig, use_feedback_controller};

fn count_label(count: u64) -> String {
    let digits = count.to_string();
    let mut label = String::new();
    for (index, digit) in digits.bytes().enumerate() {
        if index != 0 && (digits.len() - index).is_multiple_of(3) {
            label.push(',');
        }
        label.push(char::from(digit));
    }
    format!("{label} more")
}

/// Shows the number of additional avatars, optionally as a native button.
#[component]
pub fn AvatarGroupOverflow(
    /// Overflow count. Negative counts display as zero.
    count: i64,
    /// Optional activation callback; the caller owns what opens.
    #[props(default)]
    on_click: Option<EventHandler<()>>,
    /// Custom visible content instead of `+N`.
    #[props(default)]
    children: Option<Element>,
    /// Full localized accessible name, if supplied.
    #[props(default)]
    label: Option<String>,
    /// Classes on the root.
    #[props(default)]
    class: String,
    /// Inline declarations on the root.
    #[props(default)]
    style: String,
    /// Mounted callback on the native root.
    #[props(default)]
    onmounted: Option<EventHandler<MountedEvent>>,
    /// Native attributes on the root.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let settings = avatar_group_settings();
    let size = settings.size.selector();
    let shape = settings.shape.selector();
    let pixels = settings.size.pixels();
    let count = count.max(0).unsigned_abs();
    let label = label.unwrap_or_else(|| count_label(count));
    let id = format!("avatar-group-overflow-{}", current_scope_id().0);
    let is_button = on_click.is_some();
    let feedback = use_feedback_controller(
        PressFeedbackConfig {
            id: id.clone(),
            label: label.clone(),
            tooltip: None,
            is_icon_only: false,
            is_loading: false,
            is_disabled: false,
        },
        false,
        is_button,
    );
    let mounted = EventHandler::new(move |event: MountedEvent| {
        feedback.call(event.clone());
        if let Some(callback) = onmounted {
            callback.call(event);
        }
    });
    let has_role = attributes
        .iter()
        .any(|attr| attr.name == "role" && !matches!(attr.value, AttributeValue::None));
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attr| {
            !matches!(
                attr.name,
                "class"
                    | "style"
                    | "onmounted"
                    | "aria-label"
                    | "data-size"
                    | "data-shape"
                    | "data-avatar-item"
                    | "data-astryx-avatar-group-member"
                    | "data-astryx-feedback-id"
                    | "data-astryx-container-press"
            ) && !(attr.name == "role" && matches!(attr.value, AttributeValue::None))
        })
        .collect();
    let classes = if class.is_empty() {
        "astryx-avatar-group-overflow".into()
    } else {
        format!("astryx-avatar-group-overflow {class}")
    };
    let overlap = settings.overlap;
    let style =
        format!("--_avatar-overflow-size:{pixels}px;--_avatar-group-overlap:-{overlap}px;{style}");
    let content = children.unwrap_or_else(|| rsx! { "+{count}" });
    if let Some(on_click) = on_click {
        return rsx! { button {
            r#type: "button", class: classes, style, "data-size": size, "data-shape": shape,
            "aria-label": label, "data-avatar-item": "", "data-astryx-avatar-group-member": settings.owner.map(|owner| owner.to_string()), "data-astryx-container-press": "true",
            "data-astryx-feedback-id": id, onmounted: mounted, onclick: move |_| on_click.call(()),
            ..attributes, {content}
        } };
    }
    rsx! { span { class: classes, style, "data-size": size, "data-shape": shape,
        role: (!has_role).then_some("img"), "aria-label": label, onmounted: mounted, ..attributes, {content}
    } }
}
