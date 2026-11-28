use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::LayerPlacement;
use crate::popover::use_popover;

const DEFAULT_DELAY: u32 = 300;
const DEFAULT_HIDE_DELAY: u32 = 200;

/// An anchored rich preview for a trigger that already supports keyboard focus.
/// Use [`ControlledHoverCard`] to drive visibility from caller state.
#[component]
pub fn HoverCard(
    children: Element,
    content: Element,
    #[props(default)] label: Option<String>,
    #[props(default)] placement: LayerPlacement,
    #[props(default = DEFAULT_DELAY)] delay: u32,
    #[props(default = DEFAULT_HIDE_DELAY)] hide_delay: u32,
    #[props(default)] on_open_change: Option<EventHandler<bool>>,
) -> Element {
    render_hover_card(
        HoverCardProps {
            children,
            content,
            label,
            placement,
            delay,
            hide_delay,
            on_open_change,
        },
        None,
    )
}

/// A [`HoverCard`] whose visibility is owned by the caller.
///
/// Hover and focus do not change visibility or emit requests. Escape from the
/// trigger or card and clicks outside request closure; only a caller update
/// to `is_open` hides the card. Prop changes do not fire `on_open_change`.
#[component]
pub fn ControlledHoverCard(
    children: Element,
    content: Element,
    #[props(default)] label: Option<String>,
    #[props(default)] placement: LayerPlacement,
    /// Whether to show the preview.
    is_open: bool,
    /// Called with `false` when the existing dismissal controller requests closure.
    #[props(default)]
    on_open_change: Option<EventHandler<bool>>,
) -> Element {
    render_hover_card(
        HoverCardProps {
            children,
            content,
            label,
            placement,
            delay: DEFAULT_DELAY,
            hide_delay: DEFAULT_HIDE_DELAY,
            on_open_change,
        },
        Some(is_open),
    )
}

fn render_hover_card(props: HoverCardProps, controlled_open: Option<bool>) -> Element {
    let HoverCardProps {
        children,
        content,
        label,
        placement,
        delay,
        hide_delay,
        on_open_change,
    } = props;
    let id = use_hook(|| format!("astryx-hover-card-{}", current_scope_id().0));
    let is_controlled = controlled_open.is_some();
    let (is_open, request, mount_popover) =
        use_popover(id.clone(), controlled_open, on_open_change);
    let mut controller = use_signal(|| None::<document::Eval>);
    use_drop(move || {
        if let Some(eval) = *controller.peek() {
            let _ = eval.send(());
        }
    });
    let mount_id = id.clone();
    let label = label.filter(|label| !label.trim().is_empty());
    let role = if label.is_some() { "dialog" } else { "group" };
    rsx! {
        span { id: "{id}-trigger", class: "astryx-hover-card__anchor", {children} }
        div { id: id.clone(), class: "astryx-popover astryx-hover-card", popover: "manual", tabindex: -1,
            role, "aria-label": label, "data-open": is_open.to_string(), "data-placement": placement.as_str(),
            "data-hover-controlled": is_controlled.then_some("true"),
            "data-hover-delay": delay.to_string(), "data-hide-delay": hide_delay.to_string(),
            style: "position-anchor:--{id}",
            onmounted: move |event: MountedEvent| {
                mount_popover.call(event);
                if let Some(eval) = *controller.peek() { let _ = eval.send(()); }
                let mut eval = document::eval(include_str!("hover_card.js"));
                let _ = eval.send(mount_id.clone());
                controller.set(Some(eval));
                spawn(async move {
                    while let Ok(next) = eval.recv::<bool>().await { request.call(next); }
                });
            },
            {content}
        }
    }
}
