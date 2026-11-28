use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::AlertDialog;

#[test]
fn modal_name_and_fallback_use_the_visible_title() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            AlertDialog {
                is_open: true,
                on_open_change: |_| {},
                title: "Delete draft?",
                description: "This cannot be undone.",
                action_label: "Delete",
                on_action: |_| {},
            }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    let (surface, content) = html.split_once('>').expect("dialog opening tag");
    assert!(surface.starts_with("<dialog "), "{html}");
    for attribute in [
        "role=\"alertdialog\"",
        "data-purpose=\"form\"",
        "aria-label=\"Delete draft?\"",
    ] {
        assert!(surface.contains(attribute), "{html}");
    }

    for (attribute, tag, text) in [
        ("aria-labelledby", "h2", "Delete draft?"),
        ("aria-describedby", "p", "This cannot be undone."),
    ] {
        let id = surface
            .split_once(&format!(" {attribute}=\""))
            .and_then(|(_, value)| value.split_once('"'))
            .expect("dialog ID reference")
            .0;
        let (opening, body) = content
            .split_once(&format!("<{tag} "))
            .and_then(|(_, element)| element.split_once('>'))
            .expect("referenced element");
        assert!(opening.contains(&format!("id=\"{id}\"")), "{html}");
        let (body, _) = body
            .split_once(&format!("</{tag}>"))
            .expect("referenced element closing tag");
        assert!(body.contains(text), "{html}");
    }
}
