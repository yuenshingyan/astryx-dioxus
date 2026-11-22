use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::DropdownMenuCheckboxItem;

#[test]
fn checked_and_disabled_are_independent_and_rows_are_not_tab_stops() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            DropdownMenuCheckboxItem { label: "Grid", value: true }
            DropdownMenuCheckboxItem { label: "Guides", value: false, is_disabled: true }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    assert_eq!(html.matches("role=\"menuitemcheckbox\"").count(), 2);
    assert_eq!(html.matches("tabindex=\"-1\"").count(), 2);
    for attribute in [
        "aria-checked=\"true\"",
        "aria-checked=\"false\"",
        "aria-disabled=\"true\"",
    ] {
        assert!(html.contains(attribute), "missing {attribute}: {html}");
    }
}
