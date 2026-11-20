use dioxus::prelude::*;

use super::{
    SelectableCardInput, SelectableCardInputContext, SelectableCardInputType, requested_selection,
};
use crate::selectable_card::SelectableCard;

#[component]
fn InputHarness(is_disabled: bool, input_type: SelectableCardInputType) -> Element {
    let input = use_signal(move || {
        Some(SelectableCardInput {
            input_type,
            name: "plan".to_owned(),
            value: "pro".to_owned(),
        })
    });
    use_context_provider(|| SelectableCardInputContext(input));
    rsx! { SelectableCard { label: "Pro", is_selected: true, is_disabled, onchange: |_| {} } }
}

fn render(component: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(component);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[test]
fn checkbox_toggles_and_radio_only_selects() {
    assert_eq!(requested_selection(false, false), Some(true));
    assert_eq!(requested_selection(false, true), Some(false));
    assert_eq!(requested_selection(true, false), Some(true));
    assert_eq!(requested_selection(true, true), None);
}

#[test]
fn radio_context_has_native_group_name_value_and_detached_form_owner() {
    let html = render(
        || rsx! {InputHarness {is_disabled: false, input_type: SelectableCardInputType::Radio}},
    );
    for value in [
        "type=\"radio\"",
        "name=\"plan\"",
        "value=\"pro\"",
        "form=\"\"",
        "checked",
    ] {
        assert!(html.contains(value), "{value}: {html}");
    }
    assert!(!html.contains("type=\"checkbox\""));
}

#[test]
fn disabled_radio_is_still_native_and_focusable() {
    let html = render(
        || rsx! {InputHarness {is_disabled: true, input_type: SelectableCardInputType::Radio}},
    );
    assert!(html.contains("aria-disabled=\"true\""));
    assert!(html.contains("form=\"\""));
    assert!(!html.contains(" disabled="));
    assert!(!html.contains("tabindex=\"-1\""));
}

#[test]
fn group_checkbox_preserves_form_ownership_until_disabled() {
    let html = render(
        || rsx! {InputHarness {is_disabled: false, input_type: SelectableCardInputType::Checkbox}},
    );
    assert!(html.contains("type=\"checkbox\""));
    assert!(html.contains("name=\"plan\""));
    assert!(!html.contains("form="));
    let disabled = render(
        || rsx! {InputHarness {is_disabled: true, input_type: SelectableCardInputType::Checkbox}},
    );
    assert!(disabled.contains("form=\"\""));
}
