use astryx_dioxus::{Lightbox, LightboxMedia};
use dioxus::prelude::*;
use dioxus_ssr::render;

#[test]
fn caption_starts_clamped_and_describes_the_viewer_before_measurement() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            Lightbox { is_open: true, on_open_change: |_| {},
                media: LightboxMedia {
                    src: "/coast.png".into(),
                    alt: "Coastal shoreline".into(),
                    caption: Some(rsx! { "Waves roll past the " em { "headland" } "." }),
                },
            }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    let caption_id = html
        .split_once("aria-describedby=\"")
        .and_then(|(_, rest)| rest.split('"').next())
        .expect("caption description ID");
    for expected in [
        format!("id=\"{caption_id}\""),
        format!("aria-controls=\"{caption_id}\""),
        "aria-expanded=\"false\"".into(),
        "type=\"button\"".into(),
        "astryx-lightbox__caption-action\" hidden".into(),
        "-webkit-line-clamp:2".into(),
        "<em>headland</em>".into(),
        "alt=\"Coastal shoreline\"".into(),
    ] {
        assert!(html.contains(&expected), "missing {expected}: {html}");
    }
}

#[test]
fn viewer_without_caption_has_no_description_or_disclosure() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            Lightbox { is_open: true, on_open_change: |_| {},
                media: LightboxMedia { src: "/coast.png".into(), alt: "Coast".into(), caption: None },
            }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    assert!(!html.contains("aria-describedby"), "{html}");
    assert!(!html.contains("figcaption"), "{html}");
    assert!(!html.contains("aria-expanded"), "{html}");
}
