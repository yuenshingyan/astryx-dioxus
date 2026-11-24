use dioxus::prelude::*;

use super::stepper_context::{Settings, StepperContext};

/// Direction of the ordered progress track.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StepperOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/// Ordered, caller-controlled progress with optional navigation between steps.
#[component]
pub fn Stepper(
    active_step: usize,
    children: Element,
    #[props(default = "Progress".to_owned())] label: String,
    #[props(default)] orientation: StepperOrientation,
    #[props(default)] on_step_click: Option<EventHandler<usize>>,
) -> Element {
    let settings = Settings {
        active_step,
        on_step_click,
    };
    let StepperContext(mut ctx) =
        use_context_provider(|| StepperContext(Signal::new(settings.clone())));
    if *ctx.peek() != settings {
        ctx.set(settings);
    }
    let direction = match orientation {
        StepperOrientation::Horizontal => "horizontal",
        StepperOrientation::Vertical => "vertical",
    };
    rsx! { ol { class: "astryx-stepper", role: "list", "aria-label": label, "data-orientation": direction,
        {children}
    } }
}
