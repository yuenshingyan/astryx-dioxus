use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use super::use_popover::use_popover;
use crate::layer::layer_inset_style;
use crate::layout::padding::spacing_step_css;
use crate::{LayerPlacement, SizeValue, SpacingStep};

/// Bindings for the button that anchors and opens the popover.
#[derive(Clone, PartialEq)]
pub struct PopoverTriggerRenderProps {
    /// Use as the button's `id`.
    pub anchor_id: String,
    /// Use as `aria-controls`.
    pub controls: String,
    /// Use as `aria-expanded`.
    pub is_expanded: bool,
    /// Toggle the requested state on activation.
    pub onclick: EventHandler<MouseEvent>,
}

/// Anchored, nonmodal popover with a caller-rendered trigger.
#[component]
pub fn Popover(
    trigger: Callback<PopoverTriggerRenderProps, Element>,
    content: Element,
    label: String,
    #[props(default)] is_open: Option<bool>,
    #[props(default)] on_open_change: Option<EventHandler<bool>>,
    #[props(default = LayerPlacement::Below)] placement: LayerPlacement,
    #[props(default)] width: Option<SizeValue>,
    #[props(default)] padding: Option<SpacingStep>,
    #[props(default)] class: String,
) -> Element {
    let id = use_hook(|| format!("astryx-popover-{}", current_scope_id().0));
    let (is_expanded, request, onmounted) = use_popover(id.clone(), is_open, on_open_change);
    let anchor_id = format!("{id}-trigger");
    let trigger = trigger.call(PopoverTriggerRenderProps {
        anchor_id,
        controls: id.clone(),
        is_expanded,
        onclick: Callback::new(move |event: MouseEvent| {
            if event.default_action_enabled() {
                request.call(!is_expanded);
            }
        }),
    });
    let padding = padding.map_or("var(--spacing-4)", spacing_step_css);
    let mut style = format!("--_popover-padding:{padding};position-anchor:--{id};");
    let inset = layer_inset_style();
    if let Some(inset) = &inset {
        style.push_str(inset);
    }
    if let Some(width) = width {
        let width = width.as_css()?;
        style.push_str(&format!("inline-size:{width};"));
    }
    rsx! {
        {trigger}
        div { id, class: format!("astryx-popover {class}"), style,
            popover: "manual", role: "dialog", "aria-label": label,
            "data-layer-inset": inset.is_some().then_some("true"),
            "data-open": is_expanded.to_string(), "data-placement": placement.as_str(), onmounted,
            tabindex: -1,
            {content}
        }
    }
}
