use std::future::Future;
use std::pin::Pin;

use astryx_dioxus::{
    Button, ButtonElevation, ButtonEnvironment, ButtonEnvironmentProvider, ButtonGroup,
    ButtonGroupElevation, ButtonGroupOrientation, ButtonSize, ElementSize, Icon, IconSize,
    LinkProvider, LinkRenderProps, SizeProvider,
};
use dioxus::prelude::*;

fn plus() -> Element {
    rsx! { svg { view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", path { d: "M12 5v14M5 12h14" } } }
}

fn router_link(props: LinkRenderProps) -> Element {
    render_router_link(props, "custom")
}

fn alternate_link(props: LinkRenderProps) -> Element {
    render_router_link(props, "alternate")
}

fn render_router_link(props: LinkRenderProps, marker: &'static str) -> Element {
    rsx! {
        a {
            href: props.href,
            target: props.target,
            rel: props.rel,
            onclick: props.onclick,
            onkeydown: props.onkeydown,
            onmounted: props.onmounted,
            "data-router": marker,
            ..props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn ButtonContractExamples() -> Element {
    let mut is_link_disabled = use_signal(|| false);
    let mut is_child_mounted = use_signal(|| true);
    let mut is_small = use_signal(|| true);
    let mut touch_count = use_signal(|| 0);
    let mut group_value = use_signal(|| "None".to_string());
    let mut completed_tasks = use_signal(|| 0);
    let mut callback_count = use_signal(|| 0);
    let mut is_alternate_router = use_signal(|| false);
    let default_router = use_callback(router_link);
    let alternate_router = use_callback(alternate_link);
    rsx! {
        section { "aria-labelledby": "tooltip-heading",
            h2 { id: "tooltip-heading", "Tooltips and keyboard truncation" }
            p { "Hover or focus for hints. Tab to the narrow button above to read its full label; Escape dismisses." }
            div { class: "showcase-row",
                Button { id: "tooltip", label: "Save", tooltip: "Save your changes" }
                Button { id: "tooltip-same", label: "Save", tooltip: "Save" }
                Button { id: "tooltip-disabled", label: "Unavailable", tooltip: "You need edit access", is_disabled: true }
                Button { id: "tooltip-description", label: "More", tooltip: "Additional details", "aria-describedby": "button-extra-description" }
                span { id: "button-extra-description", "Existing description" }
                Button { id: "tooltip-precedence", label: "A long explicitly described action", width: "100px", tooltip: "Caller tooltip wins" }
            }
        }
        section { "aria-labelledby": "providers-heading",
            h2 { id: "providers-heading", "Inherited sizes and icon defaults" }
            p { "The disabled input-slot example uses the legacy adapter; it is not a standalone InputGroup port." }
            div { class: "showcase-row",
                Button { id: "provider-toggle", label: "Change inherited size", onclick: move |_| is_small.toggle() }
                SizeProvider { size: if is_small() { ElementSize::Small } else { ElementSize::Large },
                    Button { id: "inherited-size", label: "Inherited" }
                    Button { id: "explicit-size", label: "Explicit medium", size: ButtonSize::Medium }
                }
                Button { id: "default-icon", label: "Default icon", tooltip: "Default icon", icon: rsx! { Icon { icon: plus() } } }
                Button { id: "large-icon", label: "Large button", size: ButtonSize::Large, icon: rsx! { Icon { icon: plus() } } }
                Button { id: "explicit-icon", label: "Explicit icon size", icon: rsx! { Icon { icon: plus(), size: IconSize::Large } } }
                ButtonEnvironmentProvider { environment: ButtonEnvironment { size: Some(ElementSize::Small), is_disabled: true, ..Default::default() },
                    div { class: "astryx-input-group", Button { id: "input-addon", label: "Disabled input addon" } }
                }
            }
        }
        section { "aria-labelledby": "group-heading",
            h2 { id: "group-heading", "Connected-group integration" }
            p { "One Tab stop per group. Use arrow keys, Home and End; touch can slide between members." }
            div { class: "showcase-row",
                ButtonGroup { label: "Editing actions", elevation: ButtonGroupElevation::Medium,
                    Button { id: "group-a", label: "First", elevation: ButtonElevation::High, onclick: move |_| group_value.set("First".to_string()) }
                    Button { id: "group-b", label: "Second", tooltip: "Second action", onclick: move |_| group_value.set("Second".to_string()) }
                    Button { id: "group-c", label: "Third", onclick: move |_| group_value.set("Third".to_string()) }
                }
                ButtonGroup { label: "Vertical actions", orientation: ButtonGroupOrientation::Vertical,
                    Button { id: "vertical-a", label: "Above" }
                    Button { id: "vertical-b", label: "Below" }
                }
                ButtonGroup { label: "Disabled actions", is_disabled: true,
                    Button { id: "group-disabled", label: "Group disabled" }
                }
            }
            p { id: "group-value", "Group action: {group_value}" }
        }
        section { "aria-labelledby": "routing-heading",
            h2 { id: "routing-heading", "Router adapters and focus retention" }
            div { class: "showcase-row",
                Button { id: "router-toggle", label: "Change provider adapter", onclick: move |_| is_alternate_router.toggle() }
                LinkProvider { component: if is_alternate_router() { alternate_router } else { default_router },
                    Button { id: "custom-link", label: "Provider link", href: "#providers-heading", onfocus: move |_| callback_count += 10 }
                    Button { id: "custom-link-disabled", label: "Disabled provider link", href: "#providers-heading", is_disabled: true }
                    Button { id: "explicit-overrides-provider", label: "Explicit overrides provider", href: "#providers-heading", render_link: alternate_link }
                }
                Button { id: "explicit-link", label: "Explicit adapter", href: "#providers-heading", render_link: router_link }
                Button { id: "focus-swap", label: "Link becoming disabled", tooltip: "Focus stays here", href: "#providers-heading", is_disabled: is_link_disabled() }
                Button { id: "focus-swap-toggle", label: "Toggle link availability", onclick: move |_| is_link_disabled.toggle() }
            }
        }
        section { "aria-labelledby": "touch-heading",
            h2 { id: "touch-heading", "Touch feedback and cleanup" }
            div { class: "showcase-row",
                Button { id: "touch-button", label: "Touch target", size: ButtonSize::Small, onclick: move |_| touch_count += 1 }
                Button { id: "callback-button", label: "Root callbacks", onfocus: move |_| callback_count += 1, onpointerdown: move |_| callback_count += 1, onkeyup: move |_| callback_count += 1 }
                Button { id: "popup-button", label: "Open popup anchor", "aria-haspopup": "menu", "aria-expanded": "true" }
                Button { id: "unmount-toggle", label: "Mount or unmount action", onclick: move |_| is_child_mounted.toggle() }
                if is_child_mounted() {
                    Button {
                        id: "unmount-action", label: "Unmount while pending", tooltip: "This layer must be cleaned up",
                        click_action: move |_| -> Pin<Box<dyn Future<Output = ()>>> {
                            Box::pin(async move {
                                let _ = document::eval("await new Promise(resolve => setTimeout(resolve, 800)); return true;").await;
                                completed_tasks += 1;
                            })
                        },
                    }
                }
            }
            div { id: "touch-scroller", style: "overflow:auto; height:80px; margin-block: 1rem;",
                div { style: "height:220px; padding:16px;",
                    Button { id: "scroll-touch-button", label: "Scroll cancels press", onclick: move |_| touch_count += 1 }
                }
            }
            p { id: "touch-count", "Touches: {touch_count}" }
            p { id: "completed-tasks", "Completed tasks: {completed_tasks}" }
            p { id: "callback-count", "Callbacks: {callback_count}" }
        }
    }
}
