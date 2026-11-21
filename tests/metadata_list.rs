use std::num::NonZeroU32;

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
