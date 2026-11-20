use astryx_dioxus::{Heading, HeadingLevel, Text};
use dioxus::html::events::onanimationiteration as iteration_attribute;
use dioxus::prelude::*;

// Isolated opt-in fixture: the ordinary gallery must not register iteration
// listeners merely to demonstrate callback forwarding.
pub fn is_requested() -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window()
            .and_then(|window| window.location().search().ok())
            .is_some_and(|query| query == "?typography-animation")
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        false
    }
}

#[component]
pub fn TypographyAnimation() -> Element {
    let mut phase = use_signal(|| 0_u8);
    let mut delivered = use_signal(String::new);
    let start = (phase() == 1 || phase() == 2).then(|| {
        EventHandler::new(move |event: AnimationEvent| {
            delivered
                .write()
                .push_str(&format!("start:{};", event.animation_name()));
        })
    });
    let iteration = (phase() == 2).then(|| {
        EventHandler::new(move |event: AnimationEvent| {
            delivered
                .write()
                .push_str(&format!("iteration:{};", event.animation_name()));
        })
    });
    let end = (phase() == 2).then(|| {
        EventHandler::new(move |event: AnimationEvent| {
            delivered
                .write()
                .push_str(&format!("end:{};", event.animation_name()));
        })
    });
    let mut attributes = vec![Attribute::new(
        "data-phase",
        phase().to_string(),
        None,
        false,
    )];
    if phase() == 3 {
        attributes.push(iteration_attribute(move |event: AnimationEvent| {
            delivered
                .write()
                .push_str(&format!("raw:{};", event.animation_name()));
        }));
    }
    rsx! {
        button { id: "next", onclick: move |_| phase += 1, "Next" }
        output { id: "delivered", "{delivered}" }
        Text { id: "animation-text", onanimationstart: start,
            onanimationiteration: iteration, onanimationend: end,
            attributes: attributes.clone(), span { "Text descendant" } }
        Heading { id: "animation-heading", level: HeadingLevel::Two,
            onanimationstart: start, onanimationiteration: iteration,
            onanimationend: end, attributes, span { "Heading descendant" } }
    }
}
