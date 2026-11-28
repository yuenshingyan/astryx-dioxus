use dioxus::core::current_scope_id;
#[cfg(target_arch = "wasm32")]
use dioxus::core::queue_effect;
use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use web_sys::{Element as DomElement, HtmlElement};

use super::collapsible_group_context::{
    CollapsibleGroupDensity, CollapsibleGroupPresentationContext, GroupPresentation,
};
use super::use_collapsible::use_collapsible;
use crate::{Icon, IconSize};

/// Logical placement of the disclosure indicator.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum CollapsibleChevronPosition {
    Start,
    #[default]
    End,
}

/// A standalone disclosure that keeps its content mounted when closed.
#[component]
pub fn Collapsible(
    /// Non-interactive label content for the native trigger button.
    trigger: Element,
    children: Element,
    /// If supplied, the caller owns the open state.
    #[props(default)]
    is_open: Option<bool>,
    #[props(default = true)] default_is_open: bool,
    #[props(default)] is_disabled: bool,
    #[props(default)] on_open_change: Option<EventHandler<bool>>,
    /// Identifier used by the nearest CollapsibleGroup, when present.
    #[props(default)]
    value: Option<String>,
    #[props(default)] chevron_position: CollapsibleChevronPosition,
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let (is_expanded, mut toggle) =
        use_collapsible(is_open, default_is_open, on_open_change, value);
    let presentation = try_consume_context::<CollapsibleGroupPresentationContext>()
        .and_then(|context| (context.0)());
    let density = presentation
        .and_then(|presentation| presentation.density)
        .map(CollapsibleGroupDensity::as_str);
    let has_dividers = presentation.is_some_and(|presentation| presentation.has_dividers);
    let id = use_hook(|| format!("astryx-collapsible-{}-content", current_scope_id().0));
    let mut trigger_node = use_signal(|| None::<MountedEvent>);
    let panel_focus = use_close_focus(is_expanded, trigger_node);
    let is_start = chevron_position == CollapsibleChevronPosition::Start;
    let path = if is_start {
        "M9 18l6-6-6-6"
    } else {
        "M6 9l6 6 6-6"
    };
    let chevron = rsx! {
        span { class: "astryx-collapsible-chevron", "aria-hidden": "true",
            Icon { size: IconSize::Small, icon: rsx! {
                svg { view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round",
                    path { d: path }
                }
            } }
        }
    };
    rsx! {
        div { class: format!("astryx-collapsible {class}"), style, "data-density": density, "data-divided": has_dividers.then_some("true"), ..attributes,
            button { class: "astryx-collapsible-trigger", r#type: "button",
                "aria-controls": id.clone(), "aria-expanded": is_expanded.to_string(),
                "aria-disabled": is_disabled.then_some("true"), tabindex: is_disabled.then_some(-1),
                "data-chevron-position": if is_start { "start" } else { "end" },
                onmounted: move |event| trigger_node.set(Some(event)),
                onclick: move |_| {
                    if is_disabled { return; }
                    toggle();
                },
                if is_start { {chevron.clone()} }
                span { class: "astryx-collapsible-trigger__label", {trigger} }
                if !is_start { {chevron} }
            }
            div { class: "astryx-collapsible-content", id, hidden: !is_expanded, ..panel_focus,
                div { CollapsibleGroupPresentationReset { {children} } }
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn use_close_focus(
    is_expanded: bool,
    trigger_node: Signal<Option<MountedEvent>>,
) -> Vec<Attribute> {
    let mut panel_node = use_signal(|| None::<MountedEvent>);
    if !is_expanded
        && let Some(panel) = panel_node
            .peek()
            .as_ref()
            .and_then(|event| event.data().downcast::<DomElement>().cloned())
        && let Some(trigger) = trigger_node
            .peek()
            .as_ref()
            .and_then(|event| event.data().downcast::<DomElement>().cloned())
            .and_then(|node| node.dyn_into::<HtmlElement>().ok())
    {
        queue_focus_return(panel, trigger);
    }
    vec![onmounted(move |event| panel_node.set(Some(event)))]
}

#[cfg(target_arch = "wasm32")]
fn queue_focus_return(panel: DomElement, trigger: HtmlElement) {
    let Some(document) = panel.owner_document() else {
        return;
    };
    // Read the committed DOM before `hidden` can clear focus. Descendants
    // need not let their focus events bubble to the disclosure.
    if panel.has_attribute("hidden")
        || !document
            .active_element()
            .is_some_and(|active| panel.contains(Some(active.as_ref())))
    {
        return;
    }
    queue_effect(move || {
        if !panel.has_attribute("hidden") || !trigger.is_connected() {
            return;
        }
        // A caller may have focused something else during the close. The
        // document body is the browser's fallback when focus was lost.
        if let Some(active) = document.active_element()
            && !panel.contains(Some(active.as_ref()))
            && !document.body().is_some_and(|body| {
                let body: &DomElement = body.as_ref();
                body == &active
            })
        {
            return;
        }
        let _ = trigger.focus();
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn use_close_focus(
    is_expanded: bool,
    trigger_node: Signal<Option<MountedEvent>>,
) -> Vec<Attribute> {
    // Retain event-based recovery for renderers without a browser DOM.
    let mut has_panel_focus = use_signal(|| false);
    if !is_expanded && *has_panel_focus.peek() {
        has_panel_focus.set(false);
        if let Some(trigger) = trigger_node.peek().clone() {
            spawn(async move {
                let _ = trigger.data().set_focus(true).await;
            });
        }
    }
    vec![
        onfocusin(move |_| has_panel_focus.set(true)),
        onfocusout(move |_| has_panel_focus.set(false)),
    ]
}

#[component]
fn CollapsibleGroupPresentationReset(children: Element) -> Element {
    use_context_provider(|| {
        CollapsibleGroupPresentationContext(Signal::new(None::<GroupPresentation>))
    });
    children
}
