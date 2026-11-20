use std::collections::BTreeSet;
use std::future::Future;
use std::pin::Pin;

use astryx_dioxus::{
    Button, CardVariant, Elevation, SelectableCard, SizeValue, StatusDot, StatusDotVariant, Text,
    TextColor, TextType, TextWeight,
};
use dioxus::events::{
    onclick as click_attribute, onkeydown as keydown_attribute, onmouseup as mouseup_attribute,
};
#[cfg(target_arch = "wasm32")]
use dioxus::html::Modifiers;
use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{JsCast, prelude::wasm_bindgen};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(
    inline_js = "export function selectableFixtureIsOriginal(event) { return event === globalThis.__selectableOriginalEvent; }"
)]
extern "C" {
    #[wasm_bindgen(js_name = selectableFixtureIsOriginal)]
    fn is_original_event(event: &web_sys::MouseEvent) -> bool;
}

#[cfg(target_arch = "wasm32")]
fn record_mouse_event(event: &MouseEvent) {
    let data = event.data();
    let Some(native) = data.downcast::<web_sys::MouseEvent>() else {
        return;
    };
    let owner = native
        .current_target()
        .and_then(|target| target.dyn_into::<web_sys::Element>().ok());
    let root = owner.clone().or_else(|| {
        native
            .target()
            .and_then(|target| target.dyn_into::<web_sys::Element>().ok())
            .and_then(|target| target.closest(".astryx-selectable-card").ok().flatten())
    });
    let Some(root) = root else {
        return;
    };
    for (name, value) in [
        (
            "data-caller-default",
            event.default_action_enabled().to_string(),
        ),
        (
            "data-native-identity",
            is_original_event(native).to_string(),
        ),
        (
            "data-native-position",
            format!("{},{}", native.client_x(), native.client_y()),
        ),
        (
            "data-wrapped-position",
            format!(
                "{},{}",
                event.client_coordinates().x,
                event.client_coordinates().y
            ),
        ),
        (
            "data-wrapped-ctrl",
            event.modifiers().contains(Modifiers::CONTROL).to_string(),
        ),
        (
            "data-native-owner",
            owner.map_or_else(|| "document".to_owned(), |target| target.id()),
        ),
        ("data-native-type", native.type_()),
        ("data-native-trusted", native.is_trusted().to_string()),
        ("data-native-ctrl", native.ctrl_key().to_string()),
    ] {
        let _ = root.set_attribute(name, &value);
    }
}

#[component]
pub fn SelectableCardExamples() -> Element {
    let mut is_dark = use_signal(|| false);
    let mut plan = use_signal(|| "Pro".to_owned());
    let mut elevated = use_signal(|| "Pro".to_owned());
    let mut tags = use_signal(|| BTreeSet::from(["React", "TypeScript"]));
    let mut is_selected = use_signal(|| false);
    let mut is_disabled = use_signal(|| false);
    let mut is_shown = use_signal(|| true);
    let mut log = use_signal(Vec::<String>::new);
    let mut is_layer_cancelled = use_signal(|| false);
    let mut is_child_prevented = use_signal(|| false);
    let mut has_changed = use_signal(|| false);
    let mut should_remove_on_click = use_signal(|| false);
    let mut should_stop_capture = use_signal(|| false);
    let mut should_dispatch_unrelated = use_signal(|| false);
    rsx! {
        document::Style { "
          .demo-selectable-context {{font-family:var(--font-family-body);font-size:var(--text-body-size);line-height:var(--text-body-leading);font-weight:400;}}
          .demo-selectable-row {{display:flex;flex-direction:row;align-items:stretch;gap:var(--spacing-3);width:max-content;}}
          .demo-selectable-scroll {{max-width:100%;overflow:auto;padding:var(--spacing-4);}}
          .demo-selectable-stack {{display:flex;flex-direction:column;align-items:stretch;gap:var(--spacing-1);}}
          .demo-selectable-heading {{margin:0;display:block;font-family:var(--font-family-heading);font-size:var(--text-heading-4-size);line-height:var(--text-heading-4-leading);font-weight:var(--text-heading-4-weight);color:var(--color-text-primary);}}
          .demo-selectable-class {{margin-inline:7px;}}
          .demo-selectable-custom[data-variant=brand] {{background:rgb(50,80,120);--selectable-card-ring-color:rgb(255,210,90);}}
          @media (pointer:coarse) and (width < 1024px) {{
            #selectable-card-examples[data-astryx-theme=neutral] {{
              --text-body-size:var(--font-size-base);--text-body-leading:1.5;
              --text-supporting-size:.875rem;--text-supporting-leading:1.4286;
              --text-large-size:1.1875rem;--text-large-leading:1.4737;
              --text-heading-4-size:var(--font-size-base);--text-heading-4-leading:1.5;
            }}
          }}
        " }
        section { id: "selectable-card-examples", class: "demo-selectable-context", "data-astryx-theme": "neutral",
            style: if is_dark() {"color-scheme:dark;background:#1b1b1b;color:var(--color-text-primary);padding:1.5rem"} else {"color-scheme:light;background:#fff;color:var(--color-text-primary);padding:1.5rem"},
            h2 { "SelectableCard" }
            p { "Original single-choice recipes below still expose checkboxes, not a radio group. Prefer the multi-select recipe for checkbox choices." }
            Button { id: "sc-theme", label: "Switch SelectableCard light/dark", onclick: move |_| is_dark.toggle() }
            h3 { "SelectableCardShowcase" }
            div { class: "demo-selectable-scroll", div { id: "sc-showcase", class: "demo-selectable-row",
                for (name, price, description) in [("Basic", "$9/mo", "For individuals"), ("Pro", "$29/mo", "For small teams"), ("Enterprise", "$99/mo", "For organizations")] {
                    SelectableCard { label: name, is_selected: plan() == name,
                        onchange: move |_| plan.set(name.to_owned()), width: SizeValue::from(180), "data-selectable-named": "showcase",
                        div { class: "demo-selectable-stack",
                            h4 { class: "demo-selectable-heading", "{name}" }
                            Text { text_type: TextType::LARGE, weight: TextWeight::Bold, "{price}" }
                            Text { text_type: TextType::SUPPORTING, color: TextColor::SECONDARY, "{description}" }
                        }
                    }
                }
            } }
            h3 { "SelectableCardElevated" }
            div { class: "demo-selectable-scroll", div { style: "width:max-content", div { id: "sc-elevated", style: "display:grid;grid-template-columns:repeat(2,minmax(0,1fr));gap:var(--spacing-3);width:420px;max-width:100%",
                for name in ["Starter", "Pro"] {
                    SelectableCard { label: name, is_selected: elevated() == name, elevation: Elevation::Low,
                        onchange: move |_| elevated.set(name.to_owned()), "data-selectable-named": "elevated",
                        Text { weight: TextWeight::Bold, "{name}" }
                        Text { text_type: TextType::SUPPORTING, color: TextColor::SECONDARY, "The resting shadow stays put — the selection ring layers on top." }
                    }
                }
            } } }
            h3 { "SelectableCardMulti" }
            div { class: "demo-selectable-scroll", div { style: "width:max-content", div { id: "sc-multi", style: "display:grid;grid-template-columns:repeat(3,minmax(0,1fr));gap:var(--spacing-2);width:400px;max-width:100%",
                for (name, variant) in [("React", CardVariant::BLUE), ("TypeScript", CardVariant::CYAN), ("Node.js", CardVariant::GREEN), ("Python", CardVariant::YELLOW), ("Rust", CardVariant::ORANGE), ("Go", CardVariant::TEAL)] {
                    SelectableCard { label: name, is_selected: tags.read().contains(name), variant,
                        onchange: move |is_now| { if is_now { tags.write().insert(name); } else { tags.write().remove(name); } },
                        "data-selectable-named": "multi", Text { weight: TextWeight::Bold, "{name}" }
                    }
                }
            } } }
            h3 { "Interaction contracts" }
            Button { id: "sc-clear", label: "Clear event log", onclick: move |_| log.write().clear() }
            Button { id: "sc-disable", label: "Toggle disabled", onclick: move |_| is_disabled.toggle() }
            Button { id: "sc-drop", label: "Mount/remove card", onclick: move |_| is_shown.toggle() }
            Button { id: "sc-layer-cancel", label: "Toggle layer caller cancellation", onclick: move |_| is_layer_cancelled.toggle() }
            Button { id: "sc-child-cancel", label: "Toggle nested child cancellation", onclick: move |_| is_child_prevented.toggle() }
            Button { id: "sc-change-props", label: "Change Card props", onclick: move |_| has_changed.toggle() }
            Button { id: "sc-remove-on-click", label: "Remove on next caller click", onclick: move |_| should_remove_on_click.set(true) }
            Button { id: "sc-stop-capture", label: "Toggle capture propagation stop", onclick: move |_| should_stop_capture.toggle() }
            Button { id: "sc-unrelated-on-click", label: "Toggle unrelated native dispatch", onclick: move |_| should_dispatch_unrelated.toggle() }
            button { id: "sc-unrelated-target", r#type: "button", onclick: move |_| log.write().push("unrelated".to_owned()), "Unrelated native action" }
            output { id: "sc-log", "{log.read().join(\"|\")}" }
            if is_shown() {
                SelectableCard { id: "sc-controlled", label: if has_changed() { "Renamed option" } else { "Controlled option" }, is_selected: is_selected(), is_disabled: is_disabled(),
                    width: SizeValue::from(if has_changed() { 320 } else { 360 }), elevation: if has_changed() { Elevation::High } else { Elevation::Med }, variant: if has_changed() { CardVariant::GREEN } else { CardVariant::DEFAULT },
                    class: "demo-selectable-class", lang: "fr", "data-caller": "kept",
                    onchange: move |is_now_selected| { log.write().push(format!("change:{is_now_selected}")); is_selected.set(is_now_selected); },
                    onclick: move |event: MouseEvent| {
                        log.write().push("click".to_owned());
                        #[cfg(target_arch = "wasm32")]
                        record_mouse_event(&event);
                        if should_dispatch_unrelated() {
                            #[cfg(target_arch = "wasm32")]
                            if let Some(target) = event.data().downcast::<web_sys::MouseEvent>().and_then(|event| event.current_target()).and_then(|owner| owner.dyn_into::<web_sys::Element>().ok()).and_then(|owner| owner.owner_document()).and_then(|doc| doc.get_element_by_id("sc-unrelated-target")).and_then(|target| target.dyn_into::<web_sys::HtmlElement>().ok()) { target.click(); }
                        }
                        if is_layer_cancelled() { event.stop_propagation(); event.prevent_default(); }
                        if should_remove_on_click() { is_shown.set(false); }
                    },
                    onmouseup: move |_| log.write().push("mouseup".to_owned()),
                    onclick_capture: move |event: MouseEvent| { log.write().push("capture".to_owned()); if should_stop_capture() { event.stop_propagation(); } },
                    onpointerdown: move |_| log.write().push("pointerdown".to_owned()),
                    onmounted: move |_| log.write().push("mounted".to_owned()),
                    p { id: "sc-surface", "Selectable body text" }
                    span { id: "sc-child-prevent", onclick: move |event| { log.write().push("span-prevent".to_owned()); event.prevent_default(); }, "Noninteractive child preventing default" }
                    span { id: "sc-child-stop", onclick: move |event| { log.write().push("span-stop".to_owned()); event.stop_propagation(); }, "Noninteractive child stopping propagation" }
                    Button { id: "sc-nested", label: "Nested action", click_action: move |_| -> Pin<Box<dyn Future<Output = ()>>> { log.write().push("nested-action".to_owned()); Box::pin(async {}) }, onclick: move |event: MouseEvent| { log.write().push("nested".to_owned()); if is_child_prevented() { event.prevent_default(); } } }
                    a { id: "sc-nested-link", href: "#sc-destination", onclick: move |_| log.write().push("link".to_owned()), "Nested link" }
                    input { id: "sc-nested-input", aria_label: "Nested input" }
                    StatusDot { id: "sc-tooltip-trigger", variant: StatusDotVariant::NEUTRAL, label: "Selection details", tooltip: "Source-owned tooltip inside card" }
                    SelectableCard { id: "sc-nested-card", label: "Nested card", is_selected: false,
                        onchange: move |is_now_selected| log.write().push(format!("nested-card-change:{is_now_selected}")),
                        onclick: move |event: MouseEvent| { log.write().push("nested-card-click".to_owned());
                            #[cfg(target_arch = "wasm32")] record_mouse_event(&event);
                            #[cfg(not(target_arch = "wasm32"))] let _ = event;
                        }, "Nested selectable surface"
                        StatusDot { id: "sc-inner-tooltip-trigger", variant: StatusDotVariant::NEUTRAL, label: "Nested details", tooltip: "Nested source-owned tooltip" }
                    }
                    SelectableCard { id: "sc-disabled-card", label: "Disabled nested card", is_selected: false, is_disabled: true,
                        onchange: move |_| log.write().push("ERROR nested disabled".to_owned()), "Disabled nested selectable surface"
                    }
                    div { id: "sc-layer", "data-astryx-layer-root": "", onclick: move |event| { log.write().push("layer-boundary".to_owned()); event.stop_propagation(); },
                        Button { id: "sc-layer-button", label: "Layer action", onclick: move |event: MouseEvent| log.write().push(format!("layer:{}", event.default_action_enabled())) }
                        a { id: "sc-layer-link", href: "#sc-native-default", "Layer destination" }
                    }
                    div { id: "sc-unmarked", popover: "manual", "Unmarked popover is not a layer" }
                }
            }
            SelectableCard { id: "sc-raw", label: "Attribute builder option", is_selected: false, is_disabled: is_disabled(), onchange: move |is_now_selected| log.write().push(format!("raw-change:{is_now_selected}")),
                attributes: vec![
                    click_attribute(move |_| log.write().push("raw-click".to_owned())),
                    mouseup_attribute(move |_| log.write().push("raw-mouseup".to_owned())),
                    keydown_attribute(move |_| log.write().push("raw-keydown".to_owned())),
                ],
                "Non-reserved keydown builders retain renderer payloads; use named card callbacks"
            }
            SelectableCard { id: "sc-rejected", label: "Rejected change", is_selected: false, onchange: move |is_now_selected| log.write().push(format!("rejected:{is_now_selected}")), "Caller does not accept requests" }
            SelectableCard { id: "sc-disabled-selected", label: "Disabled selected", is_selected: true, is_disabled: true, onchange: move |_| log.write().push("ERROR disabled".to_owned()), "Selected and disabled" }
            dialog { id: "sc-native-modal",
                SelectableCard { id: "sc-modal-card", label: "Modal option", is_selected: false,
                    onchange: move |is_now_selected| log.write().push(format!("modal-change:{is_now_selected}")),
                    onclick: move |_| log.write().push("modal-click".to_owned()),
                    onclick_capture: move |_| log.write().push("modal-capture".to_owned()),
                    "Card inside native modal"
                    StatusDot { id: "sc-modal-trigger", variant: StatusDotVariant::NEUTRAL, label: "Modal details", tooltip: "Modal source-owned tooltip" }
                }
            }
            SelectableCard { id: "sc-custom", class: "demo-selectable-custom", label: "Custom theme", is_selected: true, onchange: move |_| {}, variant: CardVariant::new("brand")?, "Theme-owned ring" }
            div { id: "sc-variants",
                for variant in [CardVariant::DEFAULT, CardVariant::TRANSPARENT, CardVariant::MUTED, CardVariant::BLUE, CardVariant::CYAN, CardVariant::GRAY, CardVariant::GREEN, CardVariant::ORANGE, CardVariant::PINK, CardVariant::PURPLE, CardVariant::RED, CardVariant::TEAL, CardVariant::YELLOW] {
                    SelectableCard { label: variant.as_str().to_owned(), variant: variant.clone(), is_selected: true, onchange: move |_| {}, "{variant.as_str()}" }
                }
            }
        }
    }
}
