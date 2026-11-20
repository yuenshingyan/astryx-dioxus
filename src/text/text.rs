// Adapted from Meta Astryx Text/Text.tsx, baseline
// 476a3153417dbf75c565f7df05b0784e94ddd59e. See THIRD_PARTY_NOTICES.md.

use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use super::text_clamp_context::{TextClampProvider, use_text_clamp_context};
use super::{resolve_style_color, use_truncation};
use crate::theme::{
    TextColor, TextDisplay, TextJustify, TextSize, TextType, TextWeight, TextWrap, WordBreak,
};
use crate::tooltip::use_tooltip::use_truncation_tooltip;

/// Upstream native root elements. Typography does not imply heading semantics.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextTag {
    /// Default inline semantic element.
    #[default]
    Span,
    /// Paragraph.
    P,
    /// Generic block.
    Div,
    /// Native form label; use `html_for` to associate a control.
    Label,
    /// Level-one heading.
    H1,
    /// Level-two heading.
    H2,
    /// Level-three heading.
    H3,
}

/// Semantic typography with open theme types/colors and native HTML behavior.
///
/// `size` changes only font size; `text_type` retains its leading, family and
/// default weight. Custom names use body/primary baselines and remain reflected
/// in `data-type`/`data-color` for consumer CSS overrides. `style` is the native
/// inline CSS escape hatch replacing StyleX `xstyle`.
///
/// Clamping is CSS-only when `has_truncate_tooltip` is false: no controller or
/// layout measurement is installed. Otherwise the shared Tooltip owner measures
/// overflow after layout and reveals it on hover, tap or keyboard interest, without adding a tab stop or a
/// duplicate description. `onmounted` supplies native MountedData instead of ref.
#[component]
pub fn Text(
    children: Element,
    #[props(default)] text_type: TextType,
    #[props(default)] size: Option<TextSize>,
    #[props(default)] color: Option<TextColor>,
    #[props(default)] weight: Option<TextWeight>,
    #[props(default)] display: TextDisplay,
    #[props(default)] max_lines: u32,
    #[props(default = true)] has_truncate_tooltip: bool,
    #[props(default)] word_break: Option<WordBreak>,
    #[props(default)] text_wrap: Option<TextWrap>,
    #[props(default)] justify: TextJustify,
    #[props(default)] has_capsize: bool,
    #[props(default)] has_strikethrough: bool,
    #[props(default)] has_tabular_numbers: bool,
    #[props(default)] tag: TextTag,
    #[props(default)] html_for: Option<String>,
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(default)] onclick: EventHandler<MouseEvent>,
    #[props(default)] ondoubleclick: EventHandler<MouseEvent>,
    #[props(default)] oncontextmenu: EventHandler<MouseEvent>,
    #[props(default)] onmousedown: EventHandler<MouseEvent>,
    #[props(default)] onmouseup: EventHandler<MouseEvent>,
    #[props(default)] onmousemove: EventHandler<MouseEvent>,
    #[props(default)] onmouseenter: EventHandler<MouseEvent>,
    #[props(default)] onmouseleave: EventHandler<MouseEvent>,
    #[props(default)] onmouseover: EventHandler<MouseEvent>,
    #[props(default)] onmouseout: EventHandler<MouseEvent>,
    #[props(default)] onpointerdown: EventHandler<PointerEvent>,
    #[props(default)] onpointerup: EventHandler<PointerEvent>,
    #[props(default)] onpointermove: EventHandler<PointerEvent>,
    #[props(default)] onpointerenter: EventHandler<PointerEvent>,
    #[props(default)] onpointerleave: EventHandler<PointerEvent>,
    #[props(default)] onpointerover: EventHandler<PointerEvent>,
    #[props(default)] onpointerout: EventHandler<PointerEvent>,
    #[props(default)] onpointercancel: EventHandler<PointerEvent>,
    #[props(default)] ongotpointercapture: EventHandler<PointerEvent>,
    #[props(default)] onlostpointercapture: EventHandler<PointerEvent>,
    #[props(default)] onfocus: EventHandler<FocusEvent>,
    #[props(default)] onblur: EventHandler<FocusEvent>,
    #[props(default)] onkeydown: EventHandler<KeyboardEvent>,
    #[props(default)] onkeyup: EventHandler<KeyboardEvent>,
    #[props(default)] onkeypress: EventHandler<KeyboardEvent>,
    #[props(default)] ontouchstart: EventHandler<TouchEvent>,
    #[props(default)] ontouchmove: EventHandler<TouchEvent>,
    #[props(default)] ontouchend: EventHandler<TouchEvent>,
    #[props(default)] ontouchcancel: EventHandler<TouchEvent>,
    #[props(default)] onmounted: EventHandler<MountedEvent>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let is_in_clamped_text = use_text_clamp_context();
    let is_clamped = max_lines == 1 || (max_lines > 1 && !has_capsize);
    let tooltip_id = use_hook(|| format!("astryx-text-{}", current_scope_id().0));
    let mounted = use_truncation_tooltip(
        tooltip_id.clone(),
        String::new(),
        has_truncate_tooltip && max_lines > 0,
    );
    let truncation = use_truncation(
        tooltip_id.clone(),
        if has_truncate_tooltip { max_lines } else { 0 },
        false,
    );
    let handle_mounted = move |event: MountedEvent| {
        truncation.onmounted.call(event.clone());
        mounted.call(event.clone());
        onmounted.call(event);
    };
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attribute| {
            !matches!(
                attribute.name,
                "class"
                    | "style"
                    | "data-type"
                    | "data-size"
                    | "data-color"
                    | "data-astryx-truncation-id"
            )
        })
        .collect();
    let color = color.unwrap_or_else(|| text_type.default_color());
    let mut classes = vec![
        "astryx-text".to_string(),
        format!("astryx-text--type-{}", text_type.baseline()),
        format!("astryx-text--color-{}", resolve_style_color(&color)),
    ];
    if let Some(size) = size {
        classes.push(format!("astryx-text--size-{}", size.as_str()));
    }
    if let Some(weight) = weight {
        classes.push(format!("astryx-text--weight-{}", weight.as_str()));
    }
    let display_class = match max_lines {
        1 => "astryx-text--single-line",
        2.. => "astryx-text--multi-line",
        _ if has_capsize || display == TextDisplay::Block => "astryx-text--block",
        _ => "astryx-text--inline",
    };
    classes.push(display_class.to_string());
    if max_lines > 0 {
        let word_break = word_break.unwrap_or(if max_lines == 1 {
            WordBreak::BreakAll
        } else {
            WordBreak::BreakWord
        });
        classes.push(format!("astryx-text--{}", word_break.as_str()));
    }
    if let Some(text_wrap) = text_wrap {
        classes.push(format!("astryx-text--wrap-{}", text_wrap.as_str()));
    }
    if justify != TextJustify::Start {
        classes.push(format!("astryx-text--justify-{}", justify.as_str()));
    }
    if has_capsize {
        classes.push("astryx-text--capsize".to_string());
    }
    if has_strikethrough {
        classes.push("astryx-text--strikethrough".to_string());
    }
    if has_tabular_numbers {
        classes.push("astryx-text--tabular-numbers".to_string());
    }
    classes.push(class);
    let class = classes.join(" ");
    // The dynamic clamp wins over a caller's inline style, as in upstream.
    let style = if max_lines > 1 {
        format!("{style};-webkit-line-clamp:{max_lines}")
    } else {
        style
    };
    let children = rsx! {
        TextClampProvider { is_clamped: is_clamped || is_in_clamped_text, {children} }
    };
    macro_rules! render_tag {
        ($tag:ident) => {
            rsx! {
                $tag {
                    class: class,
                    style: style,
                    "data-astryx-truncation-id": tooltip_id.clone(),
                    "data-type": text_type.as_str(),
                    "data-size": size.map(TextSize::as_str),
                    "data-color": color.as_str(),
                    "for": (tag == TextTag::Label).then_some(html_for).flatten(),
                    onclick: onclick, ondoubleclick: ondoubleclick, oncontextmenu: oncontextmenu,
                    onmousedown: onmousedown, onmouseup: onmouseup, onmousemove: onmousemove,
                    onmouseenter: onmouseenter, onmouseleave: onmouseleave,
                    onmouseover: onmouseover, onmouseout: onmouseout,
                    onpointerdown: onpointerdown, onpointerup: onpointerup,
                    onpointermove: onpointermove, onpointerenter: onpointerenter,
                    onpointerleave: onpointerleave, onpointerover: onpointerover,
                    onpointerout: onpointerout, onpointercancel: onpointercancel,
                    ongotpointercapture: ongotpointercapture, onlostpointercapture: onlostpointercapture,
                    onfocus: onfocus, onblur: onblur, onkeydown: onkeydown,
                    onkeyup: onkeyup, onkeypress: onkeypress,
                    ontouchstart: ontouchstart, ontouchmove: ontouchmove,
                    ontouchend: ontouchend, ontouchcancel: ontouchcancel, onmounted: handle_mounted,
                    ..attributes,
                    {children}
                }
            }
        };
    }
    match tag {
        TextTag::Span => render_tag!(span),
        TextTag::P => render_tag!(p),
        TextTag::Div => render_tag!(div),
        TextTag::Label => render_tag!(label),
        TextTag::H1 => render_tag!(h1),
        TextTag::H2 => render_tag!(h2),
        TextTag::H3 => render_tag!(h3),
    }
}
