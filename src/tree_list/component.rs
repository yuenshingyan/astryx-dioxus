use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use super::tree_list_item::TreeListItem;

/// A recursive entry with a stable ID among its siblings.
#[derive(Clone, PartialEq)]
pub struct TreeListItemData {
    pub id: String,
    pub label: String,
    pub children: Vec<Self>,
    pub is_expanded: bool,
    pub is_disabled: bool,
    pub is_selected: bool,
    pub onclick: Option<EventHandler<MouseEvent>>,
}

impl TreeListItemData {
    /// Creates an enabled, unselected leaf.
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            children: Vec::new(),
            is_expanded: false,
            is_disabled: false,
            is_selected: false,
            onclick: None,
        }
    }
}

/// A named tree with local branch expansion and caller-owned selection.
#[component]
pub fn TreeList(label: String, items: Vec<TreeListItemData>) -> Element {
    let id = use_hook(|| format!("astryx-tree-list-{}", current_scope_id().0));
    let mount_id = id.clone();
    let mut controller = use_signal(|| None::<document::Eval>);
    use_drop(move || {
        if let Some(eval) = *controller.peek() {
            let _ = eval.send(());
        }
    });
    rsx! {
        div { id, class: "astryx-tree-list", role: "tree", "aria-label": label,
            onmounted: move |_| {
                if let Some(eval) = *controller.peek() { let _ = eval.send(()); }
                let eval = document::eval(concat!(
                        include_str!("../utils/typeahead_match.js"),
                        include_str!("../hooks/use_tree_focus.js"),
                        include_str!("tree_list.js"),
                    ));
                let _ = eval.send(mount_id.clone());
                controller.set(Some(eval));
            },
            for (index, item) in items.into_iter().enumerate() {
                TreeListItem { key: "{item.id}", item, is_initial_stop: index == 0 }
            }
        }
    }
}
