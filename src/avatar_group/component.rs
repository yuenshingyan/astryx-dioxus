//! Group geometry and keyboard ownership adapted from Astryx AvatarGroup.

use dioxus::core::{AttributeValue, current_scope_id};
use dioxus::prelude::*;

use super::avatar_group_context::{AvatarGroupContext, AvatarGroupSettings};
use crate::avatar::{AvatarShape, AvatarSize};
use crate::visually_hidden::VisuallyHidden;

/// A named group of overlapping avatars. The caller chooses which members to render.
#[component]
pub fn AvatarGroup(
    /// Avatar and optional AvatarGroupOverflow children.
    children: Element,
    /// Uniform size, overriding each member's size.
    #[props(default)]
    size: AvatarSize,
    /// Uniform shape, overriding each member's shape.
    #[props(default)]
    shape: AvatarShape,
    /// Localizable accessible group name.
    #[props(default = "Avatars".into())]
    label: String,
    /// Localizable hint shown to assistive technology for interactive groups.
    #[props(default = "Use arrow keys to move between avatars".into())]
    keyboard_hint: String,
    /// Classes on the group root.
    #[props(default)]
    class: String,
    /// Inline declarations on the group root.
    #[props(default)]
    style: String,
    /// Mounted callback on the group root.
    #[props(default)]
    onmounted: Option<EventHandler<MountedEvent>>,
    /// Keyboard handler; cancel the event to skip built-in navigation.
    #[props(default)]
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    /// Focus handler on the group root.
    #[props(default)]
    onfocus: Option<EventHandler<FocusEvent>>,
    /// Native attributes and events on the group root.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let owner = current_scope_id().0;
    let overlap = (size.pixels() + 2) / 4;
    let settings = AvatarGroupSettings {
        size,
        shape,
        overlap,
        owner: Some(owner),
    };
    let mut settings_signal = use_signal(|| settings);
    if *settings_signal.peek() != settings {
        settings_signal.set(settings);
    }
    use_context_provider(|| AvatarGroupContext(Some(settings_signal)));
    let mut controller = use_signal(|| None::<document::Eval>);
    use_drop(move || {
        if let Some(eval) = *controller.peek() {
            let _ = eval.send(false);
        }
    });
    let label = attributes
        .iter()
        .find_map(|attr| match &attr.value {
            AttributeValue::Text(value) if attr.name == "aria-label" => Some(value.clone()),
            _ => None,
        })
        .unwrap_or(label);
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attr| {
            !matches!(
                attr.name,
                "class"
                    | "style"
                    | "onmounted"
                    | "role"
                    | "aria-label"
                    | "data-size"
                    | "data-shape"
                    | "data-astryx-avatar-group"
            )
        })
        .collect();
    let hint_id = format!("astryx-avatar-group-hint-{owner}");
    rsx! {
        div { class: "astryx-avatar-group {class}", style: "--_avatar-group-overlap:{overlap}px;{style}",
            "data-astryx-avatar-group": owner.to_string(), "data-size": size.selector(), "data-shape": shape.selector(),
            role: "group", "aria-label": label,
            onkeydown: move |event| { if let Some(handler) = onkeydown { handler.call(event); } },
            onfocus: move |event| { if let Some(handler) = onfocus { handler.call(event); } },
            onmounted: move |event| {
                if let Some(previous) = *controller.peek() { let _ = previous.send(false); }
                let eval = document::eval(include_str!("avatar_group_focus.js"));
                let _ = eval.send(owner);
                controller.set(Some(eval));
                if let Some(callback) = onmounted { callback.call(event); }
            },
            ..attributes,
            {children}
            VisuallyHidden { id: hint_id, "data-astryx-avatar-group-hint": "", hidden: true, "{keyboard_hint}" }
        }
    }
}
