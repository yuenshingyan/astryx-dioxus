use astryx_dioxus::{
    ClickableCard, STYLES, SelectableCard, SizeValue, StatusDot, StatusDotVariant,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

/// Deliberately nested regression probes, not recommended application layouts.
#[component]
fn App() -> Element {
    let mut events = use_signal(Vec::<String>::new);
    let mut is_inner_disabled = use_signal(|| false);
    let mut is_inner_selected = use_signal(|| false);
    let mut is_mounted = use_signal(|| true);
    rsx! {
        document::Style { "{STYLES}" }
        document::Style { "
            body {{ margin: 24px; font: 16px/1.5 system-ui; }}
            main {{ color-scheme: light; }}
            section {{ margin-block: 24px; }}
            .boundary-controls {{ display: flex; gap: 12px; }}
            .boundary-content {{ display: block; padding: 12px; }}
        " }
        main { "data-astryx-theme": "neutral",
            h1 { "Interactive card boundary regression probes" }
            p { "Nested cards exercise ownership boundaries; applications should prefer independent surfaces." }
            div { class: "boundary-controls",
                button { id: "clear-events", onclick: move |_| events.write().clear(), "Clear events" }
                button { id: "disable-inner", onclick: move |_| is_inner_disabled.toggle(), "Toggle inner disabled" }
                button { id: "mount-cases", onclick: move |_| is_mounted.toggle(), "Toggle mounted cases" }
            }
            output { id: "events", "{events.read().join(\"|\")}" }
            output { id: "inner-selected", "{is_inner_selected}" }
            if is_mounted() {
                section {
                    ClickableCard {
                        id: "outer-clickable", label: "Outer action", width: SizeValue::from(420),
                        onclick: move |_| events.write().push("outer-action".to_owned()),
                        span { id: "outer-clickable-surface", class: "boundary-content", "Outer action surface" }
                        SelectableCard {
                            id: "inner-selectable", label: "Inner option", is_selected: is_inner_selected(),
                            is_disabled: is_inner_disabled(),
                            onchange: move |next| {
                                events.write().push(format!("inner-change:{next}"));
                                is_inner_selected.set(next);
                            },
                            span { id: "inner-selectable-surface", class: "boundary-content", "Inner option surface" }
                        }
                    }
                }
                section {
                    SelectableCard {
                        id: "outer-selectable", label: "Outer option", is_selected: false,
                        width: SizeValue::from(420),
                        onchange: move |_| events.write().push("outer-change".to_owned()),
                        onclick: move |_| events.write().push("outer-click".to_owned()),
                        span { id: "outer-selectable-surface", class: "boundary-content", "Outer option surface" }
                        ClickableCard {
                            id: "inner-clickable", label: "Inner action", is_disabled: is_inner_disabled(),
                            onclick: move |_| events.write().push("inner-action".to_owned()),
                            span { id: "inner-clickable-surface", class: "boundary-content", "Inner action surface" }
                        }
                    }
                }
                section {
                    SelectableCard {
                        id: "layer-outer", label: "Layer outer option", is_selected: false,
                        width: SizeValue::from(420),
                        onchange: move |_| events.write().push("layer-outer-change".to_owned()),
                        onclick: move |_| events.write().push("layer-outer-click".to_owned()),
                        SelectableCard {
                            id: "layer-inner", label: "Layer inner option", is_selected: false,
                            onchange: move |_| events.write().push("layer-inner-change".to_owned()),
                            onclick: move |_| events.write().push("layer-inner-click".to_owned()),
                            StatusDot { id: "nested-help", variant: StatusDotVariant::NEUTRAL, label: "Nested help", tooltip: "Nested portal help" }
                        }
                    }
                }
            }
            section {
                StatusDot { id: "outside-help", variant: StatusDotVariant::NEUTRAL, label: "Outside help", tooltip: "Unowned portal help" }
            }
        }
    }
}
