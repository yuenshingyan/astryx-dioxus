use dioxus::core::consume_context_from_scope;
use dioxus::prelude::*;

use astryx_dioxus::{LayerInset, LayerProvider, Popover, PopoverTriggerRenderProps};

#[component]
fn InsetProbe(label: String) -> Element {
    rsx! {
        Popover { label,
            trigger: move |props: PopoverTriggerRenderProps| rsx! {
                button { id: props.anchor_id, "Open" }
            },
            content: rsx! { p { "Layer content" } },
        }
    }
}

fn panel_tag<'a>(html: &'a str, label: &str) -> &'a str {
    let name = format!("aria-label=\"{label}\"");
    html.split('>')
        .find(|tag| tag.contains(&name) && tag.contains("role=\"dialog\""))
        .unwrap_or_else(|| panic!("missing panel {label}: {html}"))
}

fn assert_unreserved(panel: &str) {
    assert!(!panel.contains("data-layer-inset"), "{panel}");
    assert!(!panel.contains("--_astryx-layer-inset-"), "{panel}");
}

#[test]
fn providers_with_no_usable_edges_leave_the_consumer_unreserved() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            InsetProbe { label: "No provider" }
            LayerProvider { InsetProbe { label: "Default" } }
            LayerProvider { inset: LayerInset::default(), InsetProbe { label: "Empty" } }
            LayerProvider { inset: LayerInset {
                block_start: Some((-1).into()),
                inline_end: Some("".into()),
                ..LayerInset::default()
            }, InsetProbe { label: "Invalid" } }
            LayerProvider { inset: LayerInset {
                block_start: Some((-1).into()),
                block_end: Some(0.into()),
                inline_start: Some("2rem".into()),
                ..LayerInset::default()
            }, InsetProbe { label: "Declared" } }
        }
    });
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);
    for label in ["No provider", "Default", "Empty", "Invalid"] {
        assert_unreserved(panel_tag(&html, label));
    }
    let declared = panel_tag(&html, "Declared");
    assert!(declared.contains("data-layer-inset=\"true\""), "{declared}");
    assert!(declared.contains("--_astryx-layer-inset-block-end:0px;"));
    assert!(declared.contains("--_astryx-layer-inset-inline-start:2rem;"));
    assert!(!declared.contains("--_astryx-layer-inset-block-start:"));
}

#[component]
fn NestedReservation() -> Element {
    let local = use_context_provider(|| Signal::new(None::<LayerInset>));
    rsx! {
        LayerProvider { inset: LayerInset { block_end: Some(56.into()), ..LayerInset::default() },
            LayerProvider { inset: local(),
                LayerProvider { InsetProbe { label: "Nested" } }
            }
            InsetProbe { label: "Sibling" }
        }
    }
}

fn flush_inset_update(dom: &mut VirtualDom) {
    // A render turn picks up notifications queued by the previous turn:
    // changed props, then the inheriting provider, then its consumer.
    for _ in 0..3 {
        dom.render_immediate_to_vec();
    }
}

#[test]
fn empty_override_reactively_clears_only_its_subtree_and_can_resume_inheritance() {
    let mut dom = VirtualDom::new(NestedReservation);
    dom.rebuild_in_place();
    let mut local = dom.in_runtime(|| {
        consume_context_from_scope::<Signal<Option<LayerInset>>>(ScopeId::APP).expect("test state")
    });
    let outer_edge = "--_astryx-layer-inset-block-end:56px;";
    let html = dioxus_ssr::render(&dom);
    assert!(panel_tag(&html, "Nested").contains(outer_edge));

    dom.in_runtime(|| local.set(Some(LayerInset::default())));
    flush_inset_update(&mut dom);
    let html = dioxus_ssr::render(&dom);
    assert_unreserved(panel_tag(&html, "Nested"));
    assert!(panel_tag(&html, "Sibling").contains(outer_edge));

    dom.in_runtime(|| local.set(None));
    flush_inset_update(&mut dom);
    let html = dioxus_ssr::render(&dom);
    assert!(panel_tag(&html, "Nested").contains(outer_edge));
}
