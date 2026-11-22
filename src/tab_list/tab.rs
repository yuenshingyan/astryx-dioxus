use dioxus::prelude::*;

use super::tab_list_context::TabListContext;
use crate::link::{LinkComponentType, render_navigation_anchor_with_click, use_link_component};
use crate::utils::disabled_interaction::{cancel_disabled_click, cancel_disabled_key};
use crate::utils::is_renderable::is_renderable;
use crate::utils::safe_url::is_safe_url;
use crate::{ElementSize, use_size};

/// Semantics supplied by the owner of the strip.
#[derive(Clone, Default, PartialEq, Eq)]
pub enum TabMode {
    /// Link or action with a generic current-item marker.
    #[default]
    Navigation,
    /// A panel-switching tab. The owner supplies the matching tabpanel.
    Panel { panel_id: String },
}

/// A single tab. The strip owner manages selection and arrow-key navigation.
#[component]
pub fn Tab(
    value: String,
    label: String,
    #[props(default)] mode: TabMode,
    #[props(default)] id: Option<String>,
    #[props(default)] href: Option<String>,
    #[props(default)] render_link: Option<LinkComponentType>,
    #[props(default)] is_selected: bool,
    #[props(default)] is_disabled: bool,
    #[props(default)] is_label_hidden: bool,
    #[props(default)] icon: Option<Element>,
    #[props(default)] selected_icon: Option<Element>,
    #[props(default)] end_content: Option<Element>,
    #[props(default)] size: Option<ElementSize>,
    #[props(default)] tabindex: Option<i32>,
    #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
    #[props(default)] on_select: Option<EventHandler<String>>,
) -> Element {
    let renderer = use_link_component(render_link);
    let context = try_consume_context::<TabListContext>().map(|context| (context.0)());
    let is_selected = context
        .as_ref()
        .map_or(is_selected, |state| state.value == value);
    let mode = match context.as_ref() {
        Some(state) if !state.is_panel_list => TabMode::Navigation,
        Some(_) if mode == TabMode::Navigation => TabMode::Panel {
            panel_id: String::new(),
        },
        _ => mode,
    };
    let on_select = context.map(|state| state.on_change).or(on_select);
    let href = if mode == TabMode::Navigation {
        href
    } else {
        None
    };
    let is_disabled = is_disabled || href.as_ref().is_some_and(|url| !is_safe_url(url));
    let size = use_size(size).as_str();
    let tabindex = tabindex.unwrap_or(if is_selected { 0 } else { -1 });
    let icon = selected_icon
        .filter(|icon| is_selected && is_renderable(icon))
        .or_else(|| icon.filter(is_renderable));
    let is_label_hidden = is_label_hidden && icon.is_some();
    let mut attributes = vec![
        Attribute::new("class", "astryx-tab", None, false),
        Attribute::new("data-size", size, None, false),
        Attribute::new("data-tab-value", value.clone(), None, false),
        Attribute::new("tabindex", tabindex, None, false),
    ];
    let mut attr = |name, value| attributes.push(Attribute::new(name, value, None, false));
    if let Some(id) = id {
        attr("id", id);
    }
    if is_label_hidden {
        attr("aria-label", label.clone());
    }
    if is_disabled {
        attr("aria-disabled", "true".into());
    }
    match mode {
        TabMode::Panel { panel_id } => {
            attr("role", "tab".into());
            attr("aria-selected", is_selected.to_string());
            if !panel_id.is_empty() {
                attr("aria-controls", panel_id);
            }
        }
        TabMode::Navigation if is_selected => attr("aria-current", "true".into()),
        TabMode::Navigation => {}
    }
    let content = rsx! {
        if let Some(icon) = icon { span { class: "astryx-tab__icon", "aria-hidden": "true", {icon} } }
        if !is_label_hidden {
            span { class: "astryx-tab__label", span { "{label}" } span { "aria-hidden": "true", class: "astryx-tab__sizer", "{label}" } }
        }
        if let Some(end_content) = end_content.filter(is_renderable) { span { class: "astryx-tab__end", {end_content} } }
        span { class: "astryx-tab-indicator", "aria-hidden": "true" }
    };
    let activate = move |event: MouseEvent| {
        if is_disabled {
            cancel_disabled_click(&event);
            return;
        }
        if let Some(handler) = onclick {
            handler.call(event.clone());
        }
        if event.default_action_enabled()
            && !is_selected
            && let Some(handler) = on_select
        {
            handler.call(value.clone());
        }
    };
    if let Some(href) = href {
        if !is_disabled {
            return render_navigation_anchor_with_click(
                href,
                renderer,
                attributes,
                content,
                None,
                Some(Callback::new(activate)),
            );
        }
        return rsx! { a { role: "link", onclick: activate, onkeydown: move |event| cancel_disabled_key(&event), ..attributes, {content} } };
    }
    rsx! { button { r#type: "button", onclick: activate, onkeydown: move |event| { if is_disabled { cancel_disabled_key(&event); } }, ..attributes, {content} } }
}
