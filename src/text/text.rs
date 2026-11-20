// Adapted from Meta Astryx Text/Text.tsx, baseline
// 476a3153417dbf75c565f7df05b0784e94ddd59e. See THIRD_PARTY_NOTICES.md.

use dioxus::core::{AttributeValue, current_scope_id};
use dioxus::html::events::{
    onanimationend as onanimationend_attribute,
    onanimationiteration as onanimationiteration_attribute,
    onanimationstart as onanimationstart_attribute,
};
use dioxus::prelude::*;

use super::resolve_style_color;
use super::text_clamp_context::{TextClampProvider, use_text_clamp_context};
use super::use_truncation::use_truncation_measurement;
use crate::layer::LayerPlacement;
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
/// layout measurement is installed. Otherwise Text measures overflow after layout;
/// the shared Tooltip bridge reveals it on hover, tap or keyboard interest without
/// adding a tab stop or duplicate description. That bridge has known pin-fidelity
/// gaps documented in `docs/text-port.md`; this is not a complete Tooltip port.
/// Animation callbacks are opt-in: omitted callbacks install no native listener.
/// `onmounted` supplies native MountedData instead of ref.
/// `truncate_tooltip_placement` supplies the source placement alternative without
/// changing existing boolean `has_truncate_tooltip` callers; false always wins.
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
    // Additive adapter for the source boolean-or-placement union.
    #[props(default)] truncate_tooltip_placement: Option<LayerPlacement>,
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
    #[props(default)] oncopy: EventHandler<ClipboardEvent>,
    #[props(default)] oncut: EventHandler<ClipboardEvent>,
    #[props(default)] onpaste: EventHandler<ClipboardEvent>,
    #[props(default)] oncompositionstart: EventHandler<CompositionEvent>,
    #[props(default)] oncompositionupdate: EventHandler<CompositionEvent>,
    #[props(default)] oncompositionend: EventHandler<CompositionEvent>,
    #[props(default)] ondrag: EventHandler<DragEvent>,
    #[props(default)] ondragstart: EventHandler<DragEvent>,
    #[props(default)] ondragend: EventHandler<DragEvent>,
    #[props(default)] ondragenter: EventHandler<DragEvent>,
    #[props(default)] ondragleave: EventHandler<DragEvent>,
    #[props(default)] ondragover: EventHandler<DragEvent>,
    #[props(default)] ondrop: EventHandler<DragEvent>,
    #[props(default)] oninput: EventHandler<FormEvent>,
    #[props(default)] onchange: EventHandler<FormEvent>,
    #[props(default)] oninvalid: EventHandler<FormEvent>,
    #[props(default)] onreset: EventHandler<FormEvent>,
    #[props(default)] onsubmit: EventHandler<FormEvent>,
    #[props(default)] onselect: EventHandler<SelectionEvent>,
    #[props(default)] onscroll: EventHandler<ScrollEvent>,
    #[props(default)] onwheel: EventHandler<WheelEvent>,
    #[props(default)] onanimationstart: Option<EventHandler<AnimationEvent>>,
    #[props(default)] onanimationiteration: Option<EventHandler<AnimationEvent>>,
    #[props(default)] onanimationend: Option<EventHandler<AnimationEvent>>,
    #[props(default)] ontransitionend: EventHandler<TransitionEvent>,
    #[props(default)] ontoggle: EventHandler<ToggleEvent>,
    #[props(default)] onload: EventHandler<ImageEvent>,
    #[props(default)] onerror: EventHandler<ImageEvent>,
    #[props(default)] onabort: EventHandler<MediaEvent>,
    #[props(default)] oncanplay: EventHandler<MediaEvent>,
    #[props(default)] oncanplaythrough: EventHandler<MediaEvent>,
    #[props(default)] ondurationchange: EventHandler<MediaEvent>,
    #[props(default)] onemptied: EventHandler<MediaEvent>,
    #[props(default)] onencrypted: EventHandler<MediaEvent>,
    #[props(default)] onended: EventHandler<MediaEvent>,
    #[props(default)] onloadeddata: EventHandler<MediaEvent>,
    #[props(default)] onloadedmetadata: EventHandler<MediaEvent>,
    #[props(default)] onloadstart: EventHandler<MediaEvent>,
    #[props(default)] onpause: EventHandler<MediaEvent>,
    #[props(default)] onplay: EventHandler<MediaEvent>,
    #[props(default)] onplaying: EventHandler<MediaEvent>,
    #[props(default)] onprogress: EventHandler<MediaEvent>,
    #[props(default)] onratechange: EventHandler<MediaEvent>,
    #[props(default)] onseeked: EventHandler<MediaEvent>,
    #[props(default)] onseeking: EventHandler<MediaEvent>,
    #[props(default)] onstalled: EventHandler<MediaEvent>,
    #[props(default)] onsuspend: EventHandler<MediaEvent>,
    #[props(default)] ontimeupdate: EventHandler<MediaEvent>,
    #[props(default)] onvolumechange: EventHandler<MediaEvent>,
    #[props(default)] onwaiting: EventHandler<MediaEvent>,
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
    let (truncation, is_plain_overflow) = use_truncation_measurement(
        tooltip_id.clone(),
        if has_truncate_tooltip { max_lines } else { 0 },
        false,
    );
    let handle_mounted = move |event: MountedEvent| {
        truncation.onmounted.call(event.clone());
        mounted.call(event.clone());
        onmounted.call(event);
    };
    // Keep each optional listener in its own dynamic slot: changing one must not
    // shift the caller's raw attributes or another event's diff position. Absent
    // callbacks must not register delegated animation listeners on first render.
    let animation_start_attributes: Vec<_> = onanimationstart
        .map(|handler| onanimationstart_attribute(move |event| handler.call(event)))
        .into_iter()
        .collect();
    let animation_iteration_attributes: Vec<_> = onanimationiteration
        .map(|handler| onanimationiteration_attribute(move |event| handler.call(event)))
        .into_iter()
        .collect();
    let animation_end_attributes: Vec<_> = onanimationend
        .map(|handler| onanimationend_attribute(move |event| handler.call(event)))
        .into_iter()
        .collect();
    let mut attributes: Vec<_> = attributes
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
                    | "data-astryx-truncation-placement"
            )
        })
        .collect();
    // Keep caller tabindex (including -1) authoritative; the generated stop
    // belongs to the same spread so ownership can change without duplicate attrs.
    if has_truncate_tooltip
        && is_clamped
        && is_plain_overflow()
        && !attributes.iter().any(|attribute| {
            attribute.name == "tabindex" && !matches!(attribute.value, AttributeValue::None)
        })
    {
        attributes.retain(|attribute| attribute.name != "tabindex");
        attributes.push(Attribute::new("tabindex", 0, None, false));
    }
    // Dioxus diffs each spread as a name-sorted attribute list.
    attributes.sort_by_key(|attribute| attribute.name);
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
                    "data-astryx-truncation-placement": truncate_tooltip_placement.unwrap_or_default().as_str(),
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
                    ontouchend: ontouchend, ontouchcancel: ontouchcancel,
                    oncopy: oncopy, oncut: oncut, onpaste: onpaste,
                    oncompositionstart: oncompositionstart, oncompositionupdate: oncompositionupdate,
                    oncompositionend: oncompositionend,
                    ondrag: ondrag, ondragstart: ondragstart, ondragend: ondragend,
                    ondragenter: ondragenter, ondragleave: ondragleave,
                    ondragover: ondragover, ondrop: ondrop,
                    oninput: oninput, onchange: onchange, oninvalid: oninvalid,
                    onreset: onreset, onsubmit: onsubmit, onselect: onselect,
                    onscroll: onscroll, onwheel: onwheel,
                    ontransitionend: ontransitionend,
                    ontoggle: ontoggle, onload: onload, onerror: onerror,
                    onabort: onabort, oncanplay: oncanplay, oncanplaythrough: oncanplaythrough,
                    ondurationchange: ondurationchange, onemptied: onemptied, onencrypted: onencrypted,
                    onended: onended, onloadeddata: onloadeddata, onloadedmetadata: onloadedmetadata,
                    onloadstart: onloadstart, onpause: onpause, onplay: onplay, onplaying: onplaying,
                    onprogress: onprogress, onratechange: onratechange, onseeked: onseeked,
                    onseeking: onseeking, onstalled: onstalled, onsuspend: onsuspend,
                    ontimeupdate: ontimeupdate, onvolumechange: onvolumechange, onwaiting: onwaiting,
                    onmounted: handle_mounted,
                    ..animation_start_attributes,
                    ..animation_iteration_attributes,
                    ..animation_end_attributes,
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
