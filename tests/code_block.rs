use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::CodeBlock;

#[test]
fn copy_mounts_separate_polite_and_assertive_regions_before_feedback() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            CodeBlock {
                code: "let value = 1;",
                has_copy_button: true,
                copied_message: "Source copied",
                copy_error_message: "Clipboard unavailable",
            }
            CodeBlock { code: "No copy action" }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);

    for (marker, role) in [
        ("data-code-block-status", "status"),
        ("data-code-block-error", "alert"),
    ] {
        let mut regions = html.split('<').filter(|tag| tag.contains(marker));
        let region = regions.next().expect("copy live region");
        assert!(region.contains(&format!("role=\"{role}\"")), "{region}");
        assert_eq!(region.split_once('>').map(|(_, text)| text), Some(""));
        assert!(
            regions.next().is_none(),
            "copy-disabled block should not add a live region"
        );
    }
    assert!(!html.contains("Source copied") && !html.contains("Clipboard unavailable"));
}
