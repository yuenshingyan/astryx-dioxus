use dioxus::prelude::*;
use dioxus_ssr::render_element;

#[path = "../examples/showcase/stack.rs"]
mod patterns;

use patterns::StackExamples;

#[test]
fn original_patterns_have_real_named_avatar_and_static_token_semantics() {
    let html = render_element(rsx! { StackExamples {} });
    assert_eq!(html.matches("class=\"astryx-token\"").count(), 6);
    assert_eq!(html.matches("class=\"astryx-stack-item\"").count(), 9);
    assert_eq!(html.matches("class=\"astryx-avatar\"").count(), 3);
    for name in ["Olivia Chen", "Marcus Rivera", "Aisha Patel"] {
        assert!(html.contains(&format!("aria-label=\"{name}\"")));
    }
    assert_eq!(html.matches("role=\"img\"").count(), 3);
    assert_eq!(html.matches("tabindex=\"0\"").count(), 3);
    assert_eq!(html.matches("role=\"tooltip\"").count(), 3);
    assert!(!html.contains("<img"));
    assert!(!html.contains("aria-hidden=\"true\" class=\"astryx-avatar"));
    assert!(!html.contains("aria-describedby"));
    assert!(html.contains("role=\"status\""));
    assert!(!html.contains("stack-demo-token"));
}
