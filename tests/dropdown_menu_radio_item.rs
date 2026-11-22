use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{DropdownMenuRadioGroup, DropdownMenuRadioItem};

#[test]
fn rows_reflect_controlled_value_and_disabled_state() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            DropdownMenuRadioGroup { label: "Sort by", value: "newest", on_change: |_| {},
                DropdownMenuRadioItem { value: "newest", label: "Newest first" }
                DropdownMenuRadioItem { value: "oldest", label: "Oldest first", is_disabled: true }
            }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    for attribute in [
        "aria-checked=\"true\"",
        "aria-checked=\"false\"",
        "aria-disabled=\"true\"",
    ] {
        assert!(html.contains(attribute), "missing {attribute}: {html}");
    }
    assert_eq!(html.matches("role=\"menuitemradio\"").count(), 2);
    assert_eq!(html.matches("tabindex=\"-1\"").count(), 2);
}
