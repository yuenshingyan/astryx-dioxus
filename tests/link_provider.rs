use astryx_dioxus::{Button, LinkComponentType, LinkProvider, LinkRenderProps, use_link_component};
use dioxus::prelude::*;

fn render_link(props: LinkRenderProps, marker: &'static str) -> Element {
    rsx! {
        a {
            href: props.href, target: props.target, rel: props.rel,
            onclick: props.onclick, onkeydown: props.onkeydown, onmounted: props.onmounted,
            "data-adapter": marker,
            ..props.attributes,
            {props.children}
        }
    }
}

fn outer(props: LinkRenderProps) -> Element {
    render_link(props, "outer")
}
fn inner(props: LinkRenderProps) -> Element {
    render_link(props, "inner")
}

#[component]
fn RendererProbe(#[props(default)] component: Option<LinkComponentType>) -> Element {
    let renderer = use_link_component(component);
    rsx! { output { "data-has-renderer": renderer.is_some().to_string() } }
}

#[test]
fn shared_link_provider_supports_native_fallback_nearest_provider_and_explicit_override() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            RendererProbe {}
            LinkProvider { component: outer,
                Button { label: "Outer", href: "/outer" }
                LinkProvider { component: inner,
                    Button { label: "Inner", href: "/inner" }
                    Button { label: "Override", href: "/override", render_link: outer }
                    Button { label: "Disabled", href: "/disabled", is_disabled: true }
                    RendererProbe {}
                }
            }
        }
    });
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);
    assert_eq!(html.matches("data-adapter=\"outer\"").count(), 2, "{html}");
    assert_eq!(html.matches("data-adapter=\"inner\"").count(), 1, "{html}");
    assert!(!html.contains("href=\"/disabled\""), "{html}");
    assert!(html.contains("data-has-renderer=\"false\""), "{html}");
    assert!(html.contains("data-has-renderer=\"true\""), "{html}");
}
