use std::cell::Cell;
use std::num::NonZeroU32;
use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{
    MetadataList, MetadataListColumns, MetadataListItem, MetadataListLabelConfig,
    MetadataListLabelPosition, MetadataListOrientation, SizeValue,
};

#[test]
fn collapse_counts_items_across_rsx_roots_and_loops() {
    let html = render_element(rsx! { MetadataList {
        max_num_of_items: 2, show_more_label: "Expand metadata", "data-columns": "caller-value",
        MetadataListItem { label: "First", "One" }
        if false { MetadataListItem { label: "Absent", "Absent value" } }
        for label in ["Second", "Third"] {
            MetadataListItem { key: "{label}", label, "Value" }
        }
    } });
    assert_eq!(html.matches("<dt").count(), 2, "{html}");
    assert!(html.contains("Second") && !html.contains("Third") && !html.contains("caller-value"));
    assert!(html.contains("aria-expanded=\"false\"") && html.contains("Expand metadata"));
}

#[component]
fn MountProbe(mounts: Rc<Cell<usize>>) -> Element {
    use_hook(move || mounts.set(mounts.get() + 1));
    rsx! { a { href: "#detail", "Details" } }
}

#[test]
fn changing_the_vertical_limit_keeps_visible_items_mounted() {
    let limit = Rc::new(Cell::new(None));
    let mounts = Rc::new(Cell::new(0));
    let mut dom = VirtualDom::new_with_props(
        |(limit, mounts): (Rc<Cell<Option<usize>>>, Rc<Cell<usize>>)| {
            rsx! { MetadataList {
                max_num_of_items: limit.get(),
                for label in ["First", "Second"] {
                    MetadataListItem { key: "{label}", label,
                        MountProbe { mounts: mounts.clone() }
                    }
                }
            } }
        },
        (limit.clone(), mounts.clone()),
    );
    dom.rebuild_to_vec();
    assert_eq!(mounts.get(), 2);
    for (next_limit, visible_count, expected_mounts) in
        [(Some(1), 1, 2), (None, 2, 3), (Some(2), 2, 3)]
    {
        limit.set(next_limit);
        dom.mark_dirty(ScopeId::APP);
        dom.render_immediate_to_vec();

        let html = dioxus_ssr::render(&dom);
        assert_eq!(html.matches("<dt").count(), visible_count, "{html}");
        assert!(html.contains(">First</dt>"), "{html}");
        assert_eq!(html.contains(">Second</dt>"), visible_count == 2, "{html}");
        assert_eq!(
            html.contains("aria-expanded=\"false\""),
            visible_count == 1,
            "{html}"
        );
        assert_eq!(mounts.get(), expected_mounts, "limit: {next_limit:?}");
    }
}

#[test]
fn fixed_and_horizontal_modes_resolve_tracks_and_item_context() {
    let fixed = MetadataListColumns::Fixed(NonZeroU32::new(2).unwrap());
    let html = render_element(rsx! { MetadataList {
        columns: fixed, label: MetadataListLabelConfig { position: Some(MetadataListLabelPosition::Start), width: Some(SizeValue::from(100)) },
        MetadataListItem { label: "Name", "Value" }
    } });
    assert!(html.contains("repeat(2, auto minmax(0, 1fr))"));
    assert!(!html.contains("100px") && !html.contains("astryx-metadata-list-item--stacked"));
    let html = render_element(rsx! { MetadataList {
        columns: fixed, max_num_of_items: 0, orientation: MetadataListOrientation::Horizontal,
        MetadataListItem { label: "Name", "Value" }
    } });
    assert!(html.contains("astryx-metadata-list-item--stacked"));
    assert!(!html.contains("grid-template-columns") && !html.contains("aria-expanded"));
}
