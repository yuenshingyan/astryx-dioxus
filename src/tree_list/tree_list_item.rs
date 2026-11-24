use dioxus::prelude::*;

use super::component::TreeListItemData;

#[component]
pub(super) fn TreeListItem(item: TreeListItemData, is_initial_stop: bool) -> Element {
    let mut expanded = use_signal(|| item.is_expanded);
    let has_children = !item.children.is_empty();
    let is_open = has_children && expanded();
    rsx! {
        div { class: "astryx-tree-list__item", role: "treeitem", "aria-label": item.label.clone(),
            "aria-selected": item.is_selected.to_string(), "aria-disabled": item.is_disabled.then_some("true"),
            "aria-expanded": has_children.then(|| is_open.to_string()), tabindex: if is_initial_stop { 0 } else { -1 },
            div { class: "astryx-tree-list__row", onclick: move |event: MouseEvent| {
                if item.is_disabled { return; }
                if let Some(handler) = item.onclick { handler.call(event.clone()); }
                if has_children && event.default_action_enabled() { expanded.toggle(); }
            },
                span { class: "astryx-tree-list__marker", "aria-hidden": "true",
                    if is_open { "⌄" } else if has_children { "›" }
                }
                span { "{item.label}" }
            }
            if has_children {
                div { class: "astryx-tree-list__group", role: "group", hidden: !is_open,
                    for child in item.children {
                        TreeListItem { key: "{child.id}", item: child, is_initial_stop: false }
                    }
                }
            }
        }
    }
}
