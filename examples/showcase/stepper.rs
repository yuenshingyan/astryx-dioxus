use dioxus::prelude::*;

use astryx_dioxus::{Step, Stepper, StepperOrientation};

#[component]
pub(super) fn StepperExamples() -> Element {
    let mut active = use_signal(|| 2_usize);
    rsx! {
        section { id: "stepper-examples",
            h2 { "Stepper" }
            div { style: "max-width:640px",
                Stepper { label: "Checkout progress", active_step: active(), on_step_click: move |next| active.set(next),
                    Step { step: 0_usize, label: "Cart" }
                    Step { step: 1_usize, label: "Shipping" }
                    Step { step: 2_usize, label: "Payment" }
                    Step { step: 3_usize, label: "Review", is_optional: true }
                    Step { step: 4_usize, label: "Confirm", is_disabled: true }
                }
            }
            div { style: "max-width:380px",
                Stepper { label: "Setup progress", active_step: 1_usize, orientation: StepperOrientation::Vertical,
                    Step { step: 1_usize, label: "Connect a repository", is_optional: true,
                        description: "Astryx reads your build settings from the default branch",
                        end_content: rsx! { span { "2 min" } },
                    }
                }
            }
        }
    }
}
