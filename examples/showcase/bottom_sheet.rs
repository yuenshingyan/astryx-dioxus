use dioxus::prelude::*;

use astryx_dioxus::{
    BottomSheet, Button, CheckboxIndicatorState, CheckboxList, CheckboxListItem, Divider, Heading,
    HeadingLevel,
};

#[component]
pub(super) fn BottomSheetExamples() -> Element {
    let mut is_open = use_signal(|| false);
    let mut filters = use_signal(|| [false; 3]);
    rsx! {
        section { id: "bottom-sheet-examples",
            h2 { "Bottom Sheet" }
            Button { id: "bottom-sheet-trigger", label: "Open sheet", onclick: move |_| is_open.set(true) }
            BottomSheet { label: "Filters", is_open: is_open(), on_open_change: move |next| is_open.set(next),
                Heading { level: HeadingLevel::Three, "Filters" }
                Divider {}
                CheckboxList { label: "Filters", is_label_hidden: true,
                    for (index, label) in ["In stock", "On sale", "Free shipping"].into_iter().enumerate() {
                        CheckboxListItem { key: "{label}", label,
                            is_checked: if filters.read()[index] { CheckboxIndicatorState::Checked } else { CheckboxIndicatorState::Unchecked },
                            on_check: move |next| filters.write()[index] = next,
                        }
                    }
                }
                Button { label: "Apply", onclick: move |_| is_open.set(false) }
            }
        }
    }
}
