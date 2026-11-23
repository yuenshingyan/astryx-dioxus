use dioxus::prelude::*;

use crate::{Button, ButtonVariant, Dialog, DialogVariant};

/// An image and its accessible description for the lightbox.
#[derive(Clone, PartialEq)]
pub struct LightboxMedia {
    /// Image resource URL supplied by the caller.
    pub src: String,
    /// Text alternative for the image.
    pub alt: String,
    /// Optional visible caption.
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
    rsx! {
        Dialog { is_open, on_open_change, label, variant: DialogVariant::Fullscreen,
            class: "astryx-lightbox",
            div { class: "astryx-lightbox__toolbar",
                Button { label: close_label, variant: ButtonVariant::Ghost, "data-autofocus": "lightbox-close",
                    onclick: move |_| on_open_change.call(false),
                }
            }
            figure { class: "astryx-lightbox__figure",
                if is_open {
                    img { class: "astryx-lightbox__image", src: media.src, alt: media.alt }
                    if let Some(caption) = media.caption {
                        figcaption { class: "astryx-lightbox__caption", {caption} }
                    }
                }
            }
        }
    }
}
