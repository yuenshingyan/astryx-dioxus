use astryx_dioxus::{ElementSize, SizeContext, SizeProvider, use_size};
use dioxus::core::{ScopeId, consume_context_from_scope};
use dioxus::prelude::*;

#[component]
fn SizeProbe(#[props(default)] size: Option<ElementSize>) -> Element {
    let resolved = use_size(size);
    let inherited = try_consume_context::<SizeContext>().map(|context| (context.0)());
    rsx! { output { "data-size": resolved.as_str(), "data-inherited": inherited.map(ElementSize::as_str), "Size" } }
}

#[test]
fn size_resolution_is_explicit_then_nearest_provider_then_medium() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            SizeProbe {}
            SizeProvider { size: ElementSize::Large,
                SizeProbe {}
                SizeProbe { size: ElementSize::Medium }
                SizeProvider { size: ElementSize::Small, SizeProbe {} }
            }
        }
    });
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);
    assert_eq!(html.matches("data-size=\"md\"").count(), 2, "{html}");
    assert_eq!(html.matches("data-size=\"lg\"").count(), 1, "{html}");
    assert_eq!(html.matches("data-size=\"sm\"").count(), 1, "{html}");
    assert!(
        html.contains("data-size=\"md\" data-inherited=\"lg\""),
        "{html}"
    );
    assert!(!html.contains("<div"), "provider must be headless: {html}");
}

#[component]
fn ReactiveSize() -> Element {
    let size = use_context_provider(|| Signal::new(ElementSize::Small));
    rsx! { SizeProvider { size: size(), SizeProbe {} } }
}

#[test]
fn size_provider_updates_existing_consumers() {
    let mut dom = VirtualDom::new(ReactiveSize);
    dom.rebuild_in_place();
    let mut size = dom.in_runtime(|| {
        consume_context_from_scope::<Signal<ElementSize>>(ScopeId::APP).expect("test size")
    });
    dom.in_runtime(|| size.set(ElementSize::Large));
    dom.render_immediate_to_vec();
    dom.render_immediate_to_vec();
    assert!(dioxus_ssr::render(&dom).contains("data-size=\"lg\""));
}
