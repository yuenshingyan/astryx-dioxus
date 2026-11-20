// Adapted from Meta Astryx Heading/Heading.tsx, baseline
// 476a3153417dbf75c565f7df05b0784e94ddd59e. See THIRD_PARTY_NOTICES.md.

use std::error::Error;
use std::fmt::{Display, Formatter};

use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::layer::LayerPlacement;
use crate::text::{resolve_style_color, use_truncation};
use crate::theme::{
    InvalidTextName, TextColor, TextDisplay, TextJustify, TextType, TextWeight, TextWrap, WordBreak,
};
use crate::tooltip::use_tooltip::use_truncation_tooltip;

/// Native heading level, independent of the visual display type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeadingLevel {
    /// Native h1.
    One,
    /// Native h2.
    Two,
    /// Native h3.
    Three,
    /// Native h4.
    Four,
    /// Native h5.
    Five,
    /// Native h6.
    Six,
}

impl HeadingLevel {
    /// Semantic outline level (1–6).
    pub const fn number(self) -> u8 {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
            Self::Six => 6,
        }
    }
}

/// An external heading level was outside 1–6.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvalidHeadingLevel;
impl Display for InvalidHeadingLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("heading level must be between 1 and 6")
    }
}
impl Error for InvalidHeadingLevel {}
impl TryFrom<u8> for HeadingLevel {
    type Error = InvalidHeadingLevel;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            4 => Ok(Self::Four),
            5 => Ok(Self::Five),
            6 => Ok(Self::Six),
            _ => Err(InvalidHeadingLevel),
        }
    }
}

/// Open visual heading type. Custom names retain the semantic level's baseline.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HeadingType(TextType);
impl HeadingType {
    /// Largest built-in display scale.
    pub const DISPLAY_1: Self = Self(TextType::DISPLAY_1);
    /// Middle built-in display scale.
    pub const DISPLAY_2: Self = Self(TextType::DISPLAY_2);
    /// Smallest built-in display scale.
    pub const DISPLAY_3: Self = Self(TextType::DISPLAY_3);
    /// Validates a consumer-defined theme identifier using the shared theme policy.
    pub fn new(value: impl Into<String>) -> Result<Self, InvalidTextName> {
        TextType::new(value).map(Self)
    }
    /// Name reflected on the heading target.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
    fn builtin(&self) -> Option<&str> {
        match self.as_str() {
            "display-1" | "display-2" | "display-3" => Some(self.as_str()),
            _ => None,
        }
    }
}

/// Native h1–h6 with independent visual type, outline override and shared text controls.
///
/// `level` is required. Custom types/colors retain level/primary baselines until
/// consumer CSS paints them. `style` is the inline replacement for StyleX xstyle;
/// the dynamic line clamp wins over it. `onmounted` provides the native root handle
/// in place of React ref; supported attributes and events reach that same root.
/// No tab stop or native title is added for truncation.
#[component]
pub fn Heading(
    level: HeadingLevel,
    children: Element,
    #[props(default)] heading_type: Option<HeadingType>,
    #[props(default)] accessibility_level: Option<HeadingLevel>,
    #[props(default)] color: TextColor,
    #[props(default)] weight: Option<TextWeight>,
    #[props(default = TextDisplay::Block)] display: TextDisplay,
    #[props(default)] max_lines: u32,
    #[props(default = true)] has_truncate_tooltip: bool,
    #[props(default)] truncate_tooltip_placement: Option<LayerPlacement>,
    #[props(default)] word_break: Option<WordBreak>,
    #[props(default)] text_wrap: Option<TextWrap>,
    #[props(default)] justify: TextJustify,
    #[props(default)] has_capsize: bool,
    #[props(default)] has_strikethrough: bool,
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
    let tooltip_id = use_hook(|| format!("astryx-heading-{}", current_scope_id().0));
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
    let mut classes = vec![
        "astryx-heading".to_string(),
        format!("astryx-heading--level-{}", level.number()),
        format!("astryx-text--color-{}", resolve_style_color(&color)),
    ];
    if let Some(builtin) = heading_type.as_ref().and_then(HeadingType::builtin) {
        classes.push(format!("astryx-text--type-{builtin}"));
    }
    if let Some(weight) = weight {
        classes.push(format!("astryx-text--weight-{}", weight.as_str()));
    }
    classes.push(
        match max_lines {
            1 => "astryx-text--single-line",
            2.. => "astryx-text--multi-line",
            _ if has_capsize || display == TextDisplay::Block => "astryx-text--block",
            _ => "astryx-text--inline",
        }
        .to_string(),
    );
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
    classes.push(class);
    let class = classes.join(" ");
    let style = if max_lines > 1 {
        format!("{style};-webkit-line-clamp:{max_lines}")
    } else {
        style
    };
    let aria_level = accessibility_level
        .filter(|value| *value != level)
        .map(HeadingLevel::number);
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attribute| {
            !matches!(
                attribute.name,
                "class"
                    | "style"
                    | "data-level"
                    | "data-color"
                    | "data-type"
                    | "data-weight"
                    | "data-astryx-truncation-id"
                    | "data-astryx-truncation-placement"
            )
        })
        .collect();
    macro_rules! render_tag {
        ($tag:ident) => {
            rsx! {
                $tag {
                    class: class,
                    style: style,
                    "data-level": level.number(),
                    "data-color": color.as_str(),
                    "data-type": heading_type.as_ref().map(HeadingType::as_str),
                    "data-weight": weight.map(TextWeight::as_str),
                    "data-astryx-truncation-id": tooltip_id.clone(),
                    "data-astryx-truncation-placement": truncate_tooltip_placement.unwrap_or_default().as_str(),
                    "aria-level": aria_level,
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
    match level {
        HeadingLevel::One => render_tag!(h1),
        HeadingLevel::Two => render_tag!(h2),
        HeadingLevel::Three => render_tag!(h3),
        HeadingLevel::Four => render_tag!(h4),
        HeadingLevel::Five => render_tag!(h5),
        HeadingLevel::Six => render_tag!(h6),
    }
}
