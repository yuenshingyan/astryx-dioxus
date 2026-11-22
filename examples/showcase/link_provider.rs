use dioxus::html::input_data::MouseButton;
use dioxus::html::{ModifiersInteraction, PointerInteraction};
use dioxus::prelude::*;

use astryx_dioxus::{Link, LinkProvider, LinkRenderProps};

#[component]
pub(super) fn LinkProviderExamples() -> Element {
    let mut destination = use_signal(|| None::<String>);
    let router = move |props: LinkRenderProps| {
        let LinkRenderProps {
            href,
            target,
            rel,
            attributes,
            onclick,
            onkeydown,
            onmounted,
            children,
        } = props;
        let route = href
            .as_ref()
            .filter(|href| href.as_str() == "/dashboard")
            .cloned();
        rsx! {
            a { href, target, rel, onkeydown, onmounted,
                onclick: move |event: MouseEvent| {
                    onclick.call(event.clone());
                    if event.default_action_enabled() && event.modifiers().is_empty()
                        && event.trigger_button() == Some(MouseButton::Primary)
                        && let Some(route) = route.as_ref()
                    {
                        event.prevent_default();
                        destination.set(Some(route.clone()));
                    }
                },
                ..attributes,
                {children}
            }
        }
    };
    rsx! {
        section { id: "link-provider-examples",
            h2 { "LinkProvider" }
            h3 { "Custom Link Component" }
            LinkProvider { component: router,
                Link { href: "/dashboard", is_standalone: true, "Go to dashboard" }
            }
            output { id: "link-provider-result",
                if let Some(destination) = destination() {
                    "Demo: your router would navigate to {destination} here"
                }
            }
        }
    }
}
