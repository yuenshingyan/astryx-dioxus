use dioxus::prelude::*;

use astryx_dioxus::{TreeList, TreeListItemData};

#[component]
pub(super) fn TreeListExamples() -> Element {
    let mut selected = use_signal(|| "app");
    let current = selected();
    let leaf = |id: &'static str, label| {
        let mut item = TreeListItemData::new(id, label);
        item.is_selected = current == id;
        item.onclick = Some(Callback::new(move |_| selected.set(id)));
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
            h3 { "Type to find a city" }
            TreeList { label: "Cities", items: vec![
                TreeListItemData { is_disabled: true, ..TreeListItemData::new("essen", "Essen") },
                TreeListItemData::new("el-paso", "El Paso"),
                TreeListItemData::new("evora", "Évora"),
                TreeListItemData::new("oslo", "Oslo"),
                TreeListItemData::new("odense", "Odense"),
            ] }
        }
    }
}
