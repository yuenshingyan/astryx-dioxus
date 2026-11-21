use dioxus::prelude::*;

use crate::{Button, ButtonElevation, ButtonSize, ButtonVariant};

/// Controlled, standalone toggle with a persistent pressed state.
#[component]
pub fn ToggleButton(
    /// Visible label, or accessible name if the button is icon-only.
    label: String,
    /// Current caller-owned pressed state.
    #[props(default)]
    is_pressed: bool,
    /// Receives the requested state and original click event.
    #[props(default)]
    on_pressed_change: Option<EventHandler<(bool, MouseEvent)>>,
    /// Runs before the toggle request and can cancel it with `prevent_default`.
    #[props(default)]
    onclick: Option<EventHandler<MouseEvent>>,
    /// Button size override.
    #[props(default)]
    size: Option<ButtonSize>,
    /// Resting shadow depth.
    #[props(default)]
    elevation: ButtonElevation,
    /// Suppresses activation.
    #[props(default)]
    is_disabled: bool,
    /// Shows Button's loading indicator.
    #[props(default)]
    is_loading: bool,
    /// Shows artwork instead of a visible label.
    #[props(default)]
    is_icon_only: bool,
    /// Default icon.
    #[props(default)]
    icon: Option<Element>,
    /// Replacement icon in the pressed state.
    #[props(default)]
    pressed_icon: Option<Element>,
    /// Custom visible label content. Use noninteractive content.
    #[props(default)]
    children: Option<Element>,
    /// Tooltip text override.
    #[props(default)]
    tooltip: Option<String>,
    /// Additional root classes.
    #[props(default)]
    class: String,
    /// Inline root styles.
    #[props(default)]
    style: String,
    /// Callback for the mounted button.
    #[props(default)]
    onmounted: EventHandler<MountedEvent>,
    /// Native root attributes. Pressed-state attributes are component-owned.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let icon = if is_pressed {
        pressed_icon.or(icon)
    } else {
        icon
    };
    let children = (!is_icon_only).then(|| {
        let text = children.unwrap_or_else(|| rsx! { "{label}" });
        rsx! {
            span { class: "astryx-toggle-button__label",
                span { class: "astryx-toggle-button__visible", {text.clone()} }
                span { class: "astryx-toggle-button__reserve", "aria-hidden": "true", inert: true, {text} }
            }
        }
    });
    let mut attributes = attributes;
    attributes.retain(|attr| !matches!(attr.name, "aria-pressed" | "data-is-pressed"));
    let pressed = if is_pressed { "true" } else { "false" };
    attributes.extend([
        Attribute::new("aria-pressed", pressed, None, false),
        Attribute::new("data-is-pressed", pressed, None, false),
    ]);
    rsx! {
        Button { label, variant: ButtonVariant::Ghost, elevation, size, is_disabled, is_loading,
            is_interruptible: true, is_icon_only, icon, children, tooltip, onmounted,
            class: "astryx-toggle-button {class}", style, attributes,
            onclick: move |event: MouseEvent| {
                let was_allowed = event.default_action_enabled();
                if let Some(handler) = onclick { handler.call(event.clone()); }
                if (!was_allowed || event.default_action_enabled()) && let Some(handler) = on_pressed_change {
                    handler.call((!is_pressed, event));
                }
            },
        }
    }
}
