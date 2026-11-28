use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::i18n::use_message;
use crate::{
    Button, ButtonVariant, Dialog, DialogVariant, Link, MediaTheme, MediaThemeMode, Text,
    TextColor, TextTag, TextType, use_truncation,
};

const CAPTION_LINES: u32 = 2;

/// An image and its accessible description for the lightbox.
#[derive(Clone, PartialEq)]
pub struct LightboxMedia {
    /// Image resource URL supplied by the caller.
    pub src: String,
    /// Text alternative for the image.
    pub alt: String,
    /// Optional caption. Captions that exceed two lines can be expanded.
    pub caption: Option<Element>,
}

/// A controlled fullscreen viewer for a single image.
#[component]
pub fn Lightbox(
    is_open: bool,
    on_open_change: EventHandler<bool>,
    media: LightboxMedia,
    #[props(default = "Media viewer".to_owned())] label: String,
    #[props(default = "Close".to_owned())] close_label: String,
) -> Element {
    let caption_id = use_hook(|| format!("astryx-lightbox-caption-{}", current_scope_id().0));
    let described_by = (is_open && media.caption.is_some()).then(|| caption_id.clone());
    rsx! {
        Dialog { is_open, on_open_change, label, variant: DialogVariant::Fullscreen,
            class: "astryx-lightbox", "aria-describedby": described_by,
            div { class: "astryx-lightbox__toolbar",
                Button { label: close_label, variant: ButtonVariant::Ghost, "data-autofocus": "lightbox-close",
                    onclick: move |_| on_open_change.call(false),
                }
            }
            figure { class: "astryx-lightbox__figure",
                if is_open {
                    img { class: "astryx-lightbox__image", src: media.src, alt: media.alt }
                    if let Some(caption) = media.caption {
                        LightboxCaption { key: "{media.src}", id: caption_id, caption }
                    }
                }
            }
        }
    }
}

#[component]
fn LightboxCaption(id: String, caption: Element) -> Element {
    let mut expanded = use_signal(|| false);
    let is_expanded = expanded();
    // Measure only the caption, not the action, so showing More cannot cause overflow.
    let truncation = use_truncation(id.clone(), CAPTION_LINES, is_expanded);
    let has_toggle = is_expanded || (truncation.is_truncated)();
    let more = use_message("@astryx.lightbox.captionMore", "more");
    let less = use_message("@astryx.lightbox.captionLess", "less");
    let toggle_label = if is_expanded { less } else { more };

    rsx! {
        figcaption {
            class: "astryx-lightbox__caption",
            "data-expanded": is_expanded.to_string(),
            "data-disclosure": has_toggle.to_string(),
            MediaTheme { mode: MediaThemeMode::Dark,
                div { class: "astryx-lightbox__caption-panel",
                    div {
                        id: id.clone(),
                        class: "astryx-lightbox__caption-text",
                        "data-astryx-truncation-id": id.clone(),
                        tabindex: is_expanded.then_some("0"),
                        onmounted: truncation.onmounted,
                        Text {
                            tag: TextTag::Div,
                            text_type: TextType::BODY,
                            max_lines: if is_expanded { 0 } else { CAPTION_LINES },
                            // Dioxus can retain the old inline declaration when style becomes empty.
                            style: if is_expanded { "-webkit-line-clamp:none" } else { "" },
                            has_truncate_tooltip: false,
                            {caption}
                        }
                    }
                    div { class: "astryx-lightbox__caption-action", hidden: !has_toggle,
                        Link {
                            text_type: TextType::INHERIT,
                            color: TextColor::INHERIT,
                            has_underline: true,
                            "aria-controls": id.clone(),
                            "aria-expanded": is_expanded.to_string(),
                            onclick: move |_| expanded.toggle(),
                            "{toggle_label}"
                        }
                    }
                }
            }
        }
    }
}
