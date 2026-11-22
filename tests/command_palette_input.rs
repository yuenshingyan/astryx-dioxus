use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::CommandPaletteInput;

#[test]
fn standalone_input_has_an_independent_name_and_no_popup_semantics() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            CommandPaletteInput { placeholder: "Filter actions", value: "open", end_content: rsx! { "Shortcut" } }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    for fragment in [
        "aria-label=\"Search commands\"",
        "placeholder=\"Filter actions\"",
        "value=\"open\"",
        "autofocus",
        "Shortcut",
    ] {
        assert!(html.contains(fragment), "{html}");
    }
    assert!(
        !html.contains("combobox") && !html.contains("aria-expanded"),
        "{html}"
    );
}
