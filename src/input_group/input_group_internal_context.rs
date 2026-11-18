// Adapted from Meta Astryx InputGroup/InputGroupInternalContext.ts.
// See THIRD_PARTY_NOTICES.md.

use dioxus::prelude::*;

/// Input-owned disabled state, independent of ButtonGroup membership.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct InputGroupInternalState {
    pub is_disabled: bool,
}

#[derive(Clone, Copy)]
pub(crate) struct InputGroupInternalContext(pub Signal<InputGroupInternalState>);

pub(crate) fn use_input_group_internal() -> InputGroupInternalState {
    try_consume_context::<InputGroupInternalContext>()
        .map(|context| (context.0)())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use dioxus::prelude::*;

    use crate::{Button, ButtonElevation, ButtonGroup};

    use super::{InputGroupInternalContext, InputGroupInternalState};

    #[component]
    fn InputState(is_disabled: bool, children: Element) -> Element {
        let mut context = use_context_provider(|| {
            InputGroupInternalContext(Signal::new(InputGroupInternalState { is_disabled }))
        });
        if context.0.peek().is_disabled != is_disabled {
            context.0.set(InputGroupInternalState { is_disabled });
        }
        children
    }

    fn render(component: fn() -> Element) -> String {
        let mut dom = VirtualDom::new(component);
        dom.rebuild_in_place();
        dioxus_ssr::render(&dom)
    }

    #[test]
    fn input_disabled_state_does_not_create_button_group_membership() {
        let html = render(|| {
            rsx! {
                InputState { is_disabled: true,
                    Button { label: "Input-owned", elevation: ButtonElevation::High }
                }
            }
        });
        assert!(html.contains("aria-disabled=\"true\""), "{html}");
        assert!(html.contains("data-elevation=\"high\""), "{html}");
        assert!(!html.contains("data-astryx-grouped"), "{html}");
    }

    #[test]
    fn independent_input_state_cannot_clear_a_disabled_button_group() {
        let html = render(|| {
            rsx! {
                ButtonGroup { label: "Actions", is_disabled: true,
                    InputState { is_disabled: false,
                        Button { label: "Still blocked" }
                    }
                }
            }
        });
        assert_eq!(html.matches("aria-disabled=\"true\"").count(), 2, "{html}");
        assert!(html.contains("data-astryx-grouped=\"true\""), "{html}");
    }
}
