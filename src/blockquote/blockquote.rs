// Adapted from Meta Astryx, baseline 476a3153417dbf75c565f7df05b0784e94ddd59e.
// See THIRD_PARTY_NOTICES.md.

use dioxus::events::onmounted as mounted_attribute;
use dioxus::prelude::*;

/// Shallow attribution slot: empty scalars disappear, but rich nodes stay opaque.
///
/// Booleans map to `Empty`; whitespace and zero are renderable. `Rich` keeps its
/// cite wrapper even when its descendants render nothing, matching isRenderable.
#[derive(Clone, Default, PartialEq)]
pub enum BlockquoteCite {
    /// Absent, null-like or boolean content.
    #[default]
    Empty,
    /// Empty text is omitted; other text is escaped normally.
    Text(String),
    /// Floating-point content, using JavaScript's scalar display thresholds.
    Number(f64),
    /// Arbitrary caller-owned attribution markup, including links or fragments.
    Rich(Element),
}

impl From<&str> for BlockquoteCite {
    fn from(value: &str) -> Self {
        Self::Text(value.to_string())
    }
}
impl From<String> for BlockquoteCite {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}
impl From<bool> for BlockquoteCite {
    fn from(_: bool) -> Self {
        Self::Empty
    }
}
impl From<Element> for BlockquoteCite {
    fn from(value: Element) -> Self {
        Self::Rich(value)
    }
}
impl From<f64> for BlockquoteCite {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
impl From<f32> for BlockquoteCite {
    fn from(value: f32) -> Self {
        Self::Number(f64::from(value))
    }
}
macro_rules! integer_citations {
    ($($number:ty),*) => { $(impl From<$number> for BlockquoteCite {
        fn from(value: $number) -> Self { Self::Text(value.to_string()) }
    })* };
}
integer_citations!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);

fn number_text(value: f64) -> String {
    if value.is_nan() {
        return "NaN".to_string();
    }
    if value == 0.0 {
        return "0".to_string();
    }
    if value.is_infinite() {
        return if value.is_sign_negative() {
            "-Infinity"
        } else {
            "Infinity"
        }
        .to_string();
    }
    if (1e-6..1e21).contains(&value.abs()) {
        return value.to_string();
    }
    let scientific = format!("{value:e}");
    if value.abs() >= 1e21 {
        scientific.replace('e', "e+")
    } else {
        scientific
    }
}

/// A semantic quotation with a logical inline-start rule and optional attribution.
///
/// Content and quotation marks remain caller-owned. `cite` is attribution, not
/// the native HTML citation-URL attribute. It renders in a bare `cite`, never a
/// landmark footer. Native attributes/events reach the root; `onmounted` is the
/// Dioxus root-ref bridge. Caller `class` and inline `style` override base CSS
/// without adding a wrapper (the existing Rust alternative to StyleX xstyle).
#[component]
pub fn Blockquote(
    children: Element,
    #[props(default, into)] cite: BlockquoteCite,
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(default)] onmounted: Option<EventHandler<MountedEvent>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let cite = match cite {
        BlockquoteCite::Empty => None,
        BlockquoteCite::Text(text) if text.is_empty() => None,
        BlockquoteCite::Text(text) => Some(rsx! { "{text}" }),
        BlockquoteCite::Number(number) => Some(rsx! { "{number_text(number)}" }),
        BlockquoteCite::Rich(content) => Some(content),
    };
    let mut attributes = attributes;
    if let Some(onmounted) = onmounted {
        attributes.push(mounted_attribute(onmounted));
    }
    rsx! {
        blockquote {
            class: "astryx-blockquote {class}",
            style,
            ..attributes,
            {children}
            if let Some(cite) = cite {
                cite { class: "astryx-blockquote__cite", {cite} }
            }
        }
    }
}
