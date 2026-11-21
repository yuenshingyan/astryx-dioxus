use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{Timer, TimerFormat};

#[test]
fn native_time_owns_duration_and_receives_attributes() {
    let html = render_element(rsx! { Timer {
        id: "reading", "datetime": "caller-value", class: "custom", format: TimerFormat::Clock,
        start_time: 1_000.0,
    } });
    for expected in [
        "<span",
        "astryx-timer custom",
        "<time",
        "id=\"reading\"",
        "datetime=\"PT0S\"",
        "0:00",
    ] {
        assert!(html.contains(expected), "{html}");
    }
    assert!(!html.contains("caller-value") && !html.contains("aria-live"));
    assert!(render_element(rsx! { Timer {} }).contains("0s"));
}
