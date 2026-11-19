use std::sync::{Arc, Mutex};

use astryx_dioxus::{Kbd, KbdText};
use dioxus::core::{ScopeId, consume_context_from_scope};
use dioxus::document::{Document, DocumentContext, Eval, NoOpDocument};
use dioxus::events::onclick;
use dioxus::prelude::*;

fn render(component: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(component);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

struct RecordingDocument {
    calls: Arc<Mutex<Vec<String>>>,
}
impl Document for RecordingDocument {
    fn eval(&self, js: String) -> Eval {
        self.calls.lock().expect("document log").push(js.clone());
        NoOpDocument.eval(js)
    }
}

#[test]
fn server_render_never_reads_browser_platform() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let context: DocumentContext = Arc::new(RecordingDocument {
        calls: calls.clone(),
    });
    let mut dom = VirtualDom::new(|| rsx! {Kbd{keys:"mod+meta"}});
    dom.provide_root_context(context);
    dom.rebuild_in_place();
    assert!(dioxus_ssr::render(&dom).contains("Control + META"));
    assert!(calls.lock().expect("document log").is_empty());
}

#[test]
fn server_output_is_a_single_named_image_with_non_apple_modifier_text() {
    let html = render(|| rsx! {Kbd{keys:"mod+ctrl+alt+shift+meta+k"}});
    assert_eq!(html.matches("role=\"img\"").count(), 1, "{html}");
    assert!(
        html.contains("aria-label=\"Control + Control + Alt + Shift + META + K\""),
        "{html}"
    );
    for key in ["Ctrl", "Alt", "Shift", "Win", "K"] {
        assert!(html.contains(&format!(">{key}</kbd>")), "{html}");
    }
    assert!(!html.contains('⌘'), "{html}");
    assert!(!html.contains('⌃'), "{html}");
    assert_eq!(html.matches("<kbd").count(), 6, "{html}");
    assert_eq!(html.matches("aria-hidden=\"true\"").count(), 6, "{html}");
    assert_eq!(html.matches("astryx-kbd").count(), 1, "{html}");
    assert!(!html.contains("tabindex="), "{html}");
    assert!(!html.contains("aria-live="), "{html}");
}

#[test]
fn aliases_symbols_unknown_text_and_duplicate_occurrences_render_without_loss() {
    let html = render(
        || rsx! {Kbd{keys:" escape +esc+ESC+enter+return+return+backspace+tab+up+down+left+right+plus+constructor+__proto__"}},
    );
    assert_eq!(html.matches("<kbd").count(), 15, "{html}");
    assert_eq!(html.matches(">Esc</kbd>").count(), 3, "{html}");
    assert_eq!(html.matches(">↵</kbd>").count(), 3, "{html}");
    for key in [
        "⌫",
        "⇥",
        "↑",
        "↓",
        "←",
        "→",
        "+",
        "CONSTRUCTOR",
        "__PROTO__",
    ] {
        assert!(html.contains(&format!(">{key}</kbd>")), "{html}");
    }
    assert!(
        html.contains("Escape + Escape + Escape + Enter + Enter + Enter"),
        "{html}"
    );
}

#[test]
fn literal_plus_and_empty_internal_segments_keep_the_source_grammar() {
    let html = render(|| rsx! {Kbd{keys:"shift+plus"}Kbd{keys:"ctrl++k"}Kbd{keys:"f1+ß"}});
    assert_eq!(html.matches("<kbd").count(), 7, "{html}");
    assert!(html.contains("aria-label=\"Shift + Plus\""), "{html}");
    assert!(html.contains("aria-label=\"Control +  + K\""), "{html}");
    assert!(html.contains("aria-label=\"F1 + SS\""), "{html}");
    assert!(html.contains(">+</kbd>"), "{html}");
}

#[test]
fn computed_semantics_are_owned_but_other_native_aria_style_ref_and_events_reach_group() {
    let html = render(
        || rsx! {Kbd{keys:"mod+k",id:"shortcut",class:"caller",style:"margin-inline:7px",lang:"de",role:"presentation","aria-label":"wrong","aria-labelledby":"external","aria-hidden":"true","data-testid":"shortcut-test",onmounted:|_|{},attributes:vec![onclick(|_|{})]}},
    );
    for value in [
        "id=\"shortcut\"",
        "caller",
        "margin-inline:7px",
        "lang=\"de\"",
        "data-testid=\"shortcut-test\"",
        "role=\"img\"",
        "aria-label=\"Control + K\"",
        "aria-labelledby=\"external\"",
    ] {
        assert!(html.contains(value), "{html}");
    }
    assert!(!html.contains("role=\"presentation\""), "{html}");
    assert!(!html.contains("aria-label=\"wrong\""), "{html}");
}

fn german() -> KbdText {
    KbdText {
        control_short: "Strg".into(),
        control: "Steuerung".into(),
        shift_short: "Umschalt".into(),
        shift: "Umschalttaste".into(),
        alt_short: "Alt Gr".into(),
        windows_short: "Windows".into(),
        escape_short: "Abbr".into(),
        escape: "Abbrechen".into(),
        shortcut_template: "{key} mit {shortcut}".into(),
        ..KbdText::default()
    }
}

#[test]
fn localized_short_words_and_pairwise_reordering_stay_explicit_application_props() {
    let html = render(
        || rsx! {Kbd{keys:"mod+esc",text:german()}Kbd{keys:"ctrl+alt+shift+meta",text:german()}},
    );
    assert!(
        html.contains("aria-label=\"Abbrechen mit Steuerung\""),
        "{html}"
    );
    for glyph in ["Strg", "Abbr", "Alt Gr", "Umschalt", "Windows"] {
        assert!(html.contains(&format!(">{glyph}</kbd>")), "{html}");
    }
}

#[component]
fn ChangingText() -> Element {
    let is_german = use_signal(|| false);
    use_context_provider(|| is_german);
    rsx! {Kbd{keys:"mod+esc",text:if is_german(){german()}else{KbdText::default()}}}
}

#[test]
fn locale_prop_changes_update_names_and_glyphs_on_existing_consumers() {
    let mut dom = VirtualDom::new(ChangingText);
    dom.rebuild_in_place();
    assert!(dioxus_ssr::render(&dom).contains("Control + Escape"));
    let mut is_german =
        dom.in_runtime(|| consume_context_from_scope::<Signal<bool>>(ScopeId::APP).expect("state"));
    dom.in_runtime(|| is_german.set(true));
    dom.render_immediate_to_vec();
    dom.render_immediate_to_vec();
    let html = dioxus_ssr::render(&dom);
    assert!(html.contains("Abbrechen mit Steuerung"), "{html}");
    assert!(html.contains(">Strg</kbd>"), "{html}");
}

#[test]
fn empty_shortcuts_and_invalid_translation_templates_report_recoverable_errors() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            ErrorBoundary{handle_error:|_|rsx!{output{"Empty rejected"}},Kbd{keys:" + "}}
            ErrorBoundary{handle_error:|_|rsx!{output{"Pattern rejected"}},Kbd{keys:"mod+k",text:KbdText{shortcut_template:"{shortcut} then nothing".into(),..KbdText::default()}}}
            ErrorBoundary{handle_error:|_|rsx!{output{"Name rejected"}},Kbd{keys:"mod+k",text:KbdText{control:"".into(),..KbdText::default()}}}
        }
    });
    dom.rebuild_in_place();
    dom.render_immediate_to_vec();
    let html = dioxus_ssr::render(&dom);
    for value in ["Empty rejected", "Pattern rejected", "Name rejected"] {
        assert!(html.contains(value), "{html}");
    }
    assert!(!html.contains("role=\"img\""), "{html}");
}
