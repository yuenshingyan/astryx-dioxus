use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::SpacingStep;
use crate::layout::padding::spacing_step_css;

/// One carousel slide with a stable key for reconciliation.
#[derive(Clone, PartialEq)]
pub struct CarouselSlide {
    /// Unique identity within this carousel.
    pub key: String,
    /// Slide content.
    pub content: Element,
}

/// Horizontally scrollable slides with optional snapping and navigation buttons.
#[component]
pub fn Carousel(
    label: String,
    slides: Vec<CarouselSlide>,
    #[props(default = SpacingStep::Two)] gap: SpacingStep,
    #[props(default)] padding: Option<SpacingStep>,
    #[props(default)] has_snap: bool,
    #[props(default = true)] has_buttons: bool,
    /// Fade edges that have more slides beyond them.
    #[props(default = true)]
    has_edge_fade: bool,
    #[props(default = "Previous slides".to_owned())] previous_label: String,
    #[props(default = "Next slides".to_owned())] next_label: String,
    #[props(default)] slide_label: Option<Callback<(usize, usize), String>>,
    #[props(default)] class: String,
) -> Element {
    let id = use_hook(|| format!("astryx-carousel-{}", current_scope_id().0));
    let mount_id = id.clone();
    let mut controller = use_signal(|| None::<document::Eval>);
    use_drop(move || {
        if let Some(eval) = *controller.peek() {
            let _ = eval.send(());
        }
    });
    let count = slides.len();
    let gap = spacing_step_css(gap);
    let padding = padding.map_or("0px", spacing_step_css);
    rsx! {
        div { id, class: "astryx-carousel {class}", role: "region", "aria-roledescription": "carousel",
            "aria-label": label.clone(), style: "--_carousel-gap:{gap};--_carousel-padding:{padding}",
            onmounted: move |_| {
                if let Some(eval) = *controller.peek() { let _ = eval.send(()); }
                let eval = document::eval(include_str!("carousel_scroll.js"));
                let _ = eval.send(mount_id.clone());
                controller.set(Some(eval));
            },
            div { class: "astryx-carousel__scroller", role: "group", "aria-label": label, tabindex: -1,
                "data-snap": has_snap.then_some("true"), "data-edge-fade": has_edge_fade.then_some("true"),
                for (index, slide) in slides.into_iter().enumerate() {
                    div { key: "{slide.key}", class: "astryx-carousel__slide", role: "group",
                        "aria-roledescription": "slide",
                        "aria-label": slide_label.map_or_else(|| format!("{} of {count}", index + 1), |label| label.call((index + 1, count))),
                        {slide.content}
                    }
                }
            }
            if has_buttons {
                button { class: "astryx-carousel__button", r#type: "button", "data-carousel-step": "previous", hidden: true,
                    "aria-label": previous_label, span { "aria-hidden": "true", "‹" } }
                button { class: "astryx-carousel__button", r#type: "button", "data-carousel-step": "next", hidden: true,
                    "aria-label": next_label, span { "aria-hidden": "true", "›" } }
            }
        }
    }
}
