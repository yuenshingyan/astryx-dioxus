// Adapted from Meta Astryx, baseline 476a3153417dbf75c565f7df05b0784e94ddd59e.
// See THIRD_PARTY_NOTICES.md.

use dioxus::prelude::*;

use super::icon_default_size_context::use_icon_size;
use super::icon_size::IconSize;

/// Caller-supplied glyph with contextual sizing and accessible wrapper semantics.
///
/// The glyph should be noninteractive, inherit currentColor, and use a viewBox
/// or 1em dimensions. CSS must size direct SVG children to the wrapper. `color`
/// is an open CSS color value (including var()); it is not a semantic registry.
/// A missing, empty, or whitespace-only label makes the wrapper decorative.
#[component]
pub fn Icon(
    icon: Element,
    #[props(default)] size: Option<IconSize>,
    #[props(default)] color: Option<String>,
    #[props(default)] label: Option<String>,
    #[props(default)] class: String,
    #[props(default)] style: String,
) -> Element {
    let size = use_icon_size(size);
    let dimension = size.dimension();
    let color = color
        .as_deref()
        .filter(|color| !color.trim().is_empty())
        .unwrap_or("inherit");
    let label = label
        .as_deref()
        .map(str::trim)
        .filter(|label| !label.is_empty());
    let styles = format!(
        "display:inline-flex;flex-shrink:0;align-items:center;justify-content:center;width:var(--_icon-width, {dimension});height:var(--_icon-height, {dimension});font-size:{dimension};color:{color};{style}"
    );
    rsx! {
        span {
            class: "astryx-icon {class}",
            style: styles,
            "data-size": size.as_str(),
            role: label.map(|_| "img"),
            "aria-label": label,
            "aria-hidden": label.is_none().then_some("true"),
            span { class: "astryx-icon__glyph", "aria-hidden": "true", {icon} }
        }
    }
}
