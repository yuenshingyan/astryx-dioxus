use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::LayerPlacement;
use crate::popover::use_popover;

/// An anchored rich preview for a trigger that already supports keyboard focus.
#[component]
pub fn HoverCard(
    children: Element,
    content: Element,
    #[props(default)] label: Option<String>,
    #[props(default)] placement: LayerPlacement,
    #[props(default = 300)] delay: u32,
    #[props(default = 200)] hide_delay: u32,
    #[props(default)] on_open_change: Option<EventHandler<bool>>,
) -> Element {
    let id = use_hook(|| format!("astryx-hover-card-{}", current_scope_id().0));
    let (is_open, request, mount_popover) = use_popover(id.clone(), None, on_open_change);
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
