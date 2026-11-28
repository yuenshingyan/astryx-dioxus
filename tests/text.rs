use astryx_dioxus::{
    Link, Text, TextColor, TextDisplay, TextJustify, TextSize, TextTag, TextType, TextWeight,
    TextWrap, WordBreak,
};
use dioxus::core::consume_context_from_scope;
use dioxus::prelude::*;

fn render_element<F: FnOnce() -> Element + Clone + 'static>(render: F) -> String {
    let mut dom = VirtualDom::new_with_props(|render: F| render(), render);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}
#[component]
fn StatefulChild() -> Element {
    let mut mounts = use_context::<Signal<usize>>();
    use_hook(move || *mounts.write() += 1);
    rsx! { Link { href: "/child", "Child" } }
}

fn changing_clamp() -> Element {
    let max_lines = use_context_provider(|| Signal::new(2_u32));
    use_context_provider(|| Signal::new(0_usize));
    rsx! { Text { max_lines: max_lines(), Text { StatefulChild {} } } }
}

#[test]
fn changing_clamp_preserves_descendants_and_updates_inherited_context() {
    let mut dom = VirtualDom::new(changing_clamp);
    dom.rebuild_in_place();
    let mut max_lines =
        dom.in_runtime(|| consume_context_from_scope::<Signal<u32>>(ScopeId::APP).unwrap());
    let mounts =
        dom.in_runtime(|| consume_context_from_scope::<Signal<usize>>(ScopeId::APP).unwrap());
    for lines in [0, 2, 1, 0] {
        dom.in_runtime(|| max_lines.set(lines));
        dom.render_immediate_to_vec();
        dom.render_immediate_to_vec();
        let html = dioxus_ssr::render(&dom);
        assert_eq!(html.contains("astryx-link--in-clamp"), lines > 0, "{html}");
        assert_eq!(dom.in_runtime(&*mounts), 1);
    }
}

#[test]
fn enabled_truncation_keeps_server_content_without_eager_tooltip_layers() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            ErrorBoundary {
                handle_error: move |_| rsx! { output { "tooltip-integration-required" } },
                Text { max_lines: 1, has_truncate_tooltip: true, "Content" }
            }
        }
    });
    dom.rebuild_in_place();
    dom.render_immediate_to_vec();
    let html = dioxus_ssr::render(&dom);
    assert!(!html.contains("tooltip-integration-required"), "{html}");
    assert!(html.contains("astryx-text--single-line"), "{html}");
    assert!(
        !html.contains("role=\"tooltip\""),
        "tooltips mount only on interest: {html}"
    );
}

#[test]
fn defaults_and_theme_color_resolution() {
    let html = render_element(move || rsx! { Text { "Hello" } });
    assert!(html.starts_with("<span"), "{html}");
    assert!(html.contains("astryx-text--type-body"));
    assert!(html.contains("astryx-text--color-primary"));
    assert!(html.contains("data-type=\"body\""));
    assert!(html.contains("data-color=\"primary\""));
    assert!(!html.contains("data-size="));
    assert!(html.contains("Hello"));
    assert!(!html.contains("title="));
    assert!(!html.contains("aria-describedby"));

    for (text_type, color) in [
        (TextType::SUPPORTING, "secondary"),
        (TextType::INHERIT, "inherit"),
        (TextType::new("caption").unwrap(), "primary"),
    ] {
        let html = render_element(move || rsx! { Text { text_type, "content" } });
        assert!(html.contains(&format!("data-color=\"{color}\"")), "{html}");
    }
}

#[test]
fn custom_names_are_open_validated_and_keep_baselines() {
    let html = render_element(move || {
        rsx! {
            Text {
                text_type: TextType::new("hero-xl").unwrap(),
                color: TextColor::new("brand_2").unwrap(),
                "Custom"
            }
        }
    });
    assert!(html.contains("data-type=\"hero-xl\""));
    assert!(html.contains("data-color=\"brand_2\""));
    assert!(html.contains("astryx-text--type-body"));
    assert!(html.contains("astryx-text--color-primary"));
    for invalid in ["", " ", "two names", "3hero", "x\"onclick", "x;y", "é"] {
        assert!(TextType::new(invalid).is_err(), "{invalid}");
        assert!(TextColor::new(invalid).is_err(), "{invalid}");
    }
}

#[test]
fn every_native_tag_and_native_attributes() {
    for (tag, name) in [
        (TextTag::Span, "span"),
        (TextTag::P, "p"),
        (TextTag::Div, "div"),
        (TextTag::Label, "label"),
        (TextTag::H1, "h1"),
        (TextTag::H2, "h2"),
        (TextTag::H3, "h3"),
    ] {
        let html = render_element(move || {
            rsx! {
                Text {
                    tag, html_for: "control", id: "native", role: "note",
                    dir: "rtl", tabindex: 0, "aria-label": "Accessible",
                    "data-testid": "text", class: "caller", style: "margin:0",
                    onclick: move |_| {}, onfocus: move |_| {}, onpointerdown: move |_| {},
                    onmounted: move |_| {},
                    "Label"
                }
            }
        });
        assert!(html.starts_with(&format!("<{name}")), "{html}");
        assert!(html.contains(&format!("</{name}>")), "{html}");
        for attribute in [
            "id=\"native\"",
            "role=\"note\"",
            "dir=\"rtl\"",
            "tabindex=0",
            "aria-label=\"Accessible\"",
            "data-testid=\"text\"",
            "margin:0",
            "caller",
        ] {
            assert!(html.contains(attribute), "{attribute}: {html}");
        }
        assert_eq!(html.contains("for=\"control\""), tag == TextTag::Label);
    }
}

#[test]
fn display_typography_does_not_choose_heading_semantics() {
    for (tag, text_type, element, reflected_type) in [
        (TextTag::Span, TextType::DISPLAY_1, "span", "display-1"),
        (TextTag::H1, TextType::BODY, "h1", "body"),
        (TextTag::H2, TextType::DISPLAY_1, "h2", "display-1"),
        (TextTag::H3, TextType::DISPLAY_2, "h3", "display-2"),
    ] {
        let html = render_element(move || rsx! { Text { tag, text_type, "Summary" } });
        assert!(html.starts_with(&format!("<{element} ")), "{html}");
        assert!(html.ends_with(&format!(">Summary</{element}>")), "{html}");
        assert!(
            html.contains(&format!("data-type=\"{reflected_type}\"")),
            "{html}"
        );
        assert!(!html.contains(" role="), "{html}");
        assert!(!html.contains(" aria-level="), "{html}");
    }
}

#[test]
fn explicit_overrides_preserve_type_and_color() {
    let html = render_element(move || {
        rsx! {
            Text {
                text_type: TextType::CODE, size: TextSize::TwoExtraSmall,
                color: TextColor::ACCENT, weight: TextWeight::Bold, display: TextDisplay::Block,
                text_wrap: TextWrap::Balance, justify: TextJustify::End,
                has_capsize: true, has_strikethrough: true, has_tabular_numbers: true,
                "123"
            }
        }
    });
    for expected in [
        "data-type=\"code\"",
        "data-size=\"2xs\"",
        "data-color=\"accent\"",
        "astryx-text--type-code",
        "astryx-text--size-2xs",
        "astryx-text--weight-bold",
        "astryx-text--block",
        "astryx-text--wrap-balance",
        "astryx-text--justify-end",
        "astryx-text--capsize",
        "astryx-text--strikethrough",
        "astryx-text--tabular-numbers",
    ] {
        assert!(html.contains(expected), "{expected}: {html}");
    }
}

#[component]
fn ClampProbe() -> Element {
    rsx! { Link { href: "/probe", "Probe" } }
}

#[test]
fn clamp_scope_is_inherited_and_capsize_removes_only_multiline_clamping() {
    for (lines, capsize, expected) in [
        (0, false, false),
        (1, false, true),
        (2, false, true),
        (0, true, false),
        (1, true, true),
        (2, true, false),
    ] {
        let html = render_element(move || {
            rsx! {
                Text { max_lines: lines, has_capsize: capsize, has_truncate_tooltip: false,
                    Text { ClampProbe {} }
                }
            }
        });
        assert_eq!(html.contains("astryx-link--in-clamp"), expected, "{html}");
    }
    let html = render_element(move || rsx! { ClampProbe {} });
    assert!(!html.contains("astryx-link--in-clamp"));
}

#[test]
fn css_only_clamping_and_smart_word_break() {
    for (lines, expected_class, word_break) in [
        (0, "astryx-text--inline", None),
        (
            1,
            "astryx-text--single-line",
            Some("astryx-text--break-all"),
        ),
        (
            3,
            "astryx-text--multi-line",
            Some("astryx-text--break-word"),
        ),
    ] {
        let html = render_element(move || {
            rsx! {
                Text { max_lines: lines, has_truncate_tooltip: false, "Long content" }
            }
        });
        assert!(html.contains(expected_class), "{html}");
        if let Some(word_break) = word_break {
            assert!(html.contains(word_break));
        } else {
            assert!(!html.contains("astryx-text--break-"));
        }
        assert_eq!(html.contains("-webkit-line-clamp:3"), lines == 3);
        assert!(!html.contains("title="));
        assert!(!html.contains("tooltip"));
    }
    let html = render_element(move || {
        rsx! {
            Text { max_lines: 1, word_break: WordBreak::BreakWord, "Content" }
        }
    });
    assert!(html.contains("astryx-text--break-word"));
}

#[test]
fn all_builtin_type_size_color_and_closed_options_render() {
    for text_type in [
        TextType::BODY,
        TextType::LARGE,
        TextType::LABEL,
        TextType::CODE,
        TextType::SUPPORTING,
        TextType::DISPLAY_1,
        TextType::DISPLAY_2,
        TextType::DISPLAY_3,
        TextType::INHERIT,
    ] {
        let expected = format!("astryx-text--type-{}", text_type.as_str());
        let html = render_element(move || rsx! { Text { text_type, "Type" } });
        assert!(html.contains(&expected));
    }
    for size in [
        TextSize::FourExtraSmall,
        TextSize::ThreeExtraSmall,
        TextSize::TwoExtraSmall,
        TextSize::ExtraSmall,
        TextSize::Small,
        TextSize::Base,
        TextSize::Large,
        TextSize::ExtraLarge,
        TextSize::TwoExtraLarge,
        TextSize::ThreeExtraLarge,
        TextSize::FourExtraLarge,
    ] {
        let html = render_element(move || rsx! { Text { size, "Size" } });
        assert!(html.contains(&format!("data-size=\"{}\"", size.as_str())));
    }
    for color in [
        TextColor::PRIMARY,
        TextColor::SECONDARY,
        TextColor::DISABLED,
        TextColor::PLACEHOLDER,
        TextColor::ACCENT,
        TextColor::INHERIT,
    ] {
        let expected = format!("astryx-text--color-{}", color.as_str());
        let html = render_element(move || rsx! { Text { color, "Color" } });
        assert!(html.contains(&expected));
    }
    for weight in [
        TextWeight::Normal,
        TextWeight::Medium,
        TextWeight::Semibold,
        TextWeight::Bold,
    ] {
        let html = render_element(move || rsx! { Text { weight, "Weight" } });
        assert!(html.contains(&format!("astryx-text--weight-{}", weight.as_str())));
    }
    for text_wrap in [
        TextWrap::Wrap,
        TextWrap::NoWrap,
        TextWrap::Balance,
        TextWrap::Pretty,
    ] {
        let html = render_element(move || rsx! { Text { text_wrap, "Wrap" } });
        assert!(html.contains(&format!("astryx-text--wrap-{}", text_wrap.as_str())));
    }
}
