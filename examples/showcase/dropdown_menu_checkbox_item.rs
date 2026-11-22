use dioxus::prelude::*;

use astryx_dioxus::DropdownMenuCheckboxItem;

#[component]
pub(super) fn DropdownMenuCheckboxItemExamples() -> Element {
    let mut grid = use_signal(|| true);
    let mut labels = use_signal(|| false);
    rsx! {
        section { id: "dropdown-menu-checkbox-item-examples",
            h2 { "DropdownMenuCheckboxItem" }
            p { "Inline pointer preview; popup and keyboard navigation are not wired." }
            div { role: "menu", "aria-label": "Display options", style: "max-width:320px",
                DropdownMenuCheckboxItem { id: "menu-checkbox-grid", label: "Show grid", value: grid(), on_change: move |checked| grid.set(checked) }
                DropdownMenuCheckboxItem { id: "menu-checkbox-labels", label: "Show labels", value: labels(), on_change: move |checked| labels.set(checked) }
                DropdownMenuCheckboxItem { id: "menu-checkbox-guides", label: "Show guides", value: false, is_disabled: true, on_change: move |checked| labels.set(checked) }
            }
        }
    }
}
