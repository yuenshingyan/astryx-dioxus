use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{Collapsible, CollapsibleGroup, CollapsibleGroupValue};

#[test]
fn nearest_group_owns_named_items_and_nested_panels_reset_presentation() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            CollapsibleGroup { has_dividers: true, value: CollapsibleGroupValue::Single(Some("outer".into())),
                Collapsible { value: "outer", is_open: false, trigger: rsx! { "Outer" },
                    CollapsibleGroup { class: "no-wrapper-chrome", value: CollapsibleGroupValue::Multiple(vec!["inner".into()]),
                        Collapsible { value: "inner", trigger: rsx! { "Inner" }, "Inner panel" }
                        Collapsible { value: "outer", trigger: rsx! { "Not selected" }, "Closed panel" }
                    }
                }
            }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    assert_eq!(html.matches("aria-expanded=\"true\"").count(), 2, "{html}");
    assert_eq!(html.matches("aria-expanded=\"false\"").count(), 1, "{html}");
    assert_eq!(html.matches("data-divided").count(), 1, "{html}");
    assert!(!html.contains("no-wrapper-chrome"), "{html}");
}
