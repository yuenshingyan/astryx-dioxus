use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{
    BaseTypeaheadOption, FieldStatusPlacement, Tokenizer, TokenizerProps, TokenizerWithClear,
};

#[test]
fn clear_is_opt_in_and_requires_enabled_selection() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            div {
                Tokenizer { label: "Default", options: vec![], value: vec![tag()], on_change: |_| {} }
                TokenizerWithClear { label: "Empty", options: vec![], value: vec![], on_change: |_| {} }
                TokenizerWithClear { label: "Disabled", options: vec![], value: vec![tag()], is_disabled: true, on_change: |_| {} }
                EnabledClear {}
            }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    assert_eq!(
        html.matches("aria-label=\"Clear all\"").count(),
        1,
        "{html}"
    );
    assert_eq!(html.matches("role=\"combobox\"").count(), 4, "{html}");
    assert!(html.contains("aria-label=\"Remove Design\""), "{html}");
}

#[component]
fn EnabledClear() -> Element {
    // Keep direct construction of the released public props source-compatible.
    TokenizerWithClear(TokenizerProps {
        label: "Enabled".into(),
        options: vec![],
        value: vec![tag()],
        on_change: EventHandler::new(|_| {}),
        placeholder: None,
        description: None,
        status: None,
        status_variant: FieldStatusPlacement::Detached,
        html_name: None,
        width: None,
        has_entries_on_focus: false,
        is_disabled: false,
    })
}

fn tag() -> BaseTypeaheadOption {
    BaseTypeaheadOption {
        id: "design".into(),
        label: "Design".into(),
        description: None,
    }
}
