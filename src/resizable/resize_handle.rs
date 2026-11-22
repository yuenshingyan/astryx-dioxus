use dioxus::prelude::*;

/// Axis along which the controlled region grows.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ResizeDirection {
    #[default]
    Horizontal,
    Vertical,
}

/// A keyboard-operable splitter for a caller-sized region.
#[component]
pub fn ResizeHandle(
    /// Accessible name of the splitter.
    #[props(default = "Resize handle".to_owned())]
    label: String,
    /// ID of the region controlled by the splitter.
    controls: String,
    /// Region size in pixels.
    value: u32,
    min: u32,
    max: u32,
    on_resize: EventHandler<u32>,
    #[props(default)] direction: ResizeDirection,
    /// Reverse which physical arrow grows the panel.
    #[props(default)]
    is_reversed: bool,
    /// Mirror horizontal arrows for a right-to-left layout.
    #[props(default)]
    is_rtl: bool,
    #[props(default)] is_disabled: bool,
    #[props(default)] has_divider: bool,
    #[props(default = true)] is_always_visible: bool,
    #[props(default)] children: Option<Element>,
) -> Element {
    let has_valid_bounds = min <= max;
    let is_disabled = is_disabled || !has_valid_bounds;
    let current = if has_valid_bounds {
        value.clamp(min, max)
    } else {
        value
    };
    let is_horizontal = direction == ResizeDirection::Horizontal;
    let should_reverse = is_reversed ^ (is_horizontal && is_rtl);
    rsx! {
        div { class: "astryx-resize-handle", role: "separator", tabindex: if is_disabled { -1 } else { 0 },
            "aria-label": label, "aria-controls": controls,
            "aria-orientation": if is_horizontal { "vertical" } else { "horizontal" },
            "aria-valuemin": has_valid_bounds.then(|| min.to_string()), "aria-valuemax": has_valid_bounds.then(|| max.to_string()),
            "aria-valuenow": has_valid_bounds.then(|| current.to_string()), "aria-disabled": is_disabled.then_some("true"),
            "data-divider": has_divider.then_some("true"), "data-auto-hide": (!is_always_visible).then_some("true"),
            onkeydown: move |event| {
                if is_disabled || event.modifiers().intersects(Modifiers::ALT | Modifiers::CONTROL | Modifiers::META) { return; }
                let step = if event.modifiers().contains(Modifiers::SHIFT) { 50 } else { 10 };
                let next = match (direction, event.key()) {
                    (_, Key::Home) => min,
                    (_, Key::End) => max,
                    (ResizeDirection::Horizontal, Key::ArrowRight) | (ResizeDirection::Vertical, Key::ArrowDown) => stepped(current, step, should_reverse, min, max),
                    (ResizeDirection::Horizontal, Key::ArrowLeft) | (ResizeDirection::Vertical, Key::ArrowUp) => stepped(current, step, !should_reverse, min, max),
                    _ => return,
                };
                event.prevent_default();
                if next != current { on_resize.call(next); }
            },
            if let Some(children) = children { {children} }
            else { span { class: "astryx-resize-handle-pill", "aria-hidden": "true" } }
        }
    }
}

fn stepped(current: u32, step: u32, should_shrink: bool, min: u32, max: u32) -> u32 {
    (if should_shrink {
        current.saturating_sub(step)
    } else {
        current.saturating_add(step)
    })
    .clamp(min, max)
}
