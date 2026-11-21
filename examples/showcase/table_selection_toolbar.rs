use dioxus::prelude::*;

use astryx_dioxus::{
    Button, Table, TableBody, TableCell, TableRow, TableSelectionSummary, TableSelectionToolbar,
};

#[component]
pub(super) fn TableSelectionToolbarExamples() -> Element {
    let mut selected = use_signal(|| [false; 3]);
    let count = selected
        .read()
        .iter()
        .filter(|&&is_selected| is_selected)
        .count();
    let clear_selection = EventHandler::new(move |()| selected.set([false; 3]));
    rsx! {
        section { id: "table-selection-toolbar-examples",
            h2 { "TableSelectionToolbar" }
            p { "This example keeps selection in the caller, outside the table." }
            TableSelectionToolbar {
                id: "table-bulk-actions",
                selection: TableSelectionSummary { selected_count: count, clear_selection },
                start_content: rsx! { Button { label: "Archive", onclick: move |_| clear_selection.call(()) } },
            }
            Table { label: "Reports",
                TableBody {
                    for (index, name) in ["Sales report", "Usage report", "Inventory report"].into_iter().enumerate() {
                        TableRow { key: "{name}",
                            TableCell { label { input { r#type: "checkbox", checked: selected.read()[index],
                                onchange: move |event| selected.write()[index] = event.checked(),
                            } " {name}" } }
                        }
                    }
                }
            }
        }
    }
}
