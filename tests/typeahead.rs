use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{BaseTypeahead, BaseTypeaheadOption, Typeahead};

fn banana() -> BaseTypeaheadOption {
    BaseTypeaheadOption {
        id: "banana".into(),
        label: "Banana".into(),
        description: None,
    }
}

#[component]
fn Fixture(
    #[props(default)] has_selection: bool,
    #[props(default)] is_disabled: bool,
    #[props(default)] is_base: bool,
) -> Element {
    let value = has_selection.then(banana);
    if is_base {
        return rsx! {
            BaseTypeahead { label: "Fruit", options: vec![banana()], value, on_change: |_| {} }
        };
    }
    rsx! {
        Typeahead { label: "Fruit", options: vec![banana()], value, is_disabled, on_change: |_| {} }
    }
}

#[test]
fn only_enabled_typeahead_with_a_selection_offers_clear() {
    let selected = render_element(rsx! { Fixture { has_selection: true } });
    assert!(selected.contains("value=\"Banana\""));
    let clear = selected
        .split("<button")
        .skip(1)
        .filter_map(|part| part.split('>').next())
        .find(|attrs| attrs.contains("aria-label=\"Clear Fruit selection\""))
        .expect("an accessible clear button for the selection");
    assert!(clear.contains("type=\"button\""));

    let empty = render_element(rsx! { Fixture {} });
    let disabled = render_element(rsx! { Fixture { has_selection: true, is_disabled: true } });
    let base = render_element(rsx! { Fixture { has_selection: true, is_base: true } });
    for html in [empty, disabled, base] {
        assert!(!html.contains("astryx-input-clear-button"));
    }
}
