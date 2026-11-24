use dioxus::prelude::*;

use astryx_dioxus::{TreeList, TreeListItemData};

#[component]
pub(super) fn TreeListExamples() -> Element {
    let mut selected = use_signal(|| None::<&'static str>);
    let current = selected();
    let leaf = |id: &'static str, label| {
        let mut item = TreeListItemData::new(id, label);
        item.is_selected = current == Some(id);
        item.onclick = Some(Callback::new(move |_| selected.set(Some(id))));
        item
    };
    let mut components = TreeListItemData::new("components", "components");
    components.children = vec![
        leaf("button", "Button.tsx"),
        leaf("card", "Card.tsx"),
        leaf("list", "List.tsx"),
    ];
    let mut src = TreeListItemData::new("src", "src");
    src.is_expanded = true;
    src.children = vec![
        components,
        leaf("app", "App.tsx"),
        leaf("index", "index.tsx"),
    ];
    let mut public = TreeListItemData::new("public", "public");
    public.children = vec![
        leaf("favicon", "favicon.ico"),
        leaf("index-html", "index.html"),
    ];
    let items = vec![
        src,
        public,
        leaf("pkg", "package.json"),
        leaf("readme", "README.md"),
    ];
    rsx! {
        section { id: "tree-list-examples",
            h2 { "Tree List" }
            TreeList { label: "Project files", items }
        }
    }
}
