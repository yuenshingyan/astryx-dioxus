use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{Timestamp, TimestampFormat};

#[test]
fn absolute_timestamp_uses_supporting_secondary_text_around_the_native_time() {
    let render_timestamp = || {
        let mut dom = VirtualDom::new(|| {
            rsx! { Timestamp { value: 1_774_440_000, format: TimestampFormat::SystemDateTime } }
        });
        dom.rebuild_in_place();
        render(&dom)
    };
    let html = render_timestamp();
    let (wrapper, time) = html.split_once("<time ").expect("native time element");
    assert!(wrapper.starts_with("<span "), "{html}");
    for attribute in [r#"data-type="supporting""#, r#"data-color="secondary""#] {
        assert!(wrapper.contains(attribute), "{html}");
    }
    assert!(time.contains(r#"class="astryx-timestamp""#), "{html}");
    assert!(time.contains(r#"data-format="system_date_time""#), "{html}");
    assert!(
        time.ends_with(r#"datetime="2026-03-25T12:00:00Z">2026-03-25 12:00:00</time></span>"#),
        "{html}"
    );
    assert!(!html.contains("tabindex"), "{html}");
    assert!(!html.contains("aria-label"), "{html}");
    assert_eq!(html, render_timestamp());
}

#[test]
fn utc_timestamps_handle_pre_epoch_leap_days_and_year_bounds() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            Timestamp { value: -1, format: TimestampFormat::UnixSeconds }
            Timestamp { value: 951_782_400, format: TimestampFormat::SystemDate }
            Timestamp { value: -62_135_596_800_i64, format: TimestampFormat::SystemTime }
            Timestamp { value: 253_402_300_799_i64, format: TimestampFormat::SystemDateTime }
            Timestamp { value: 253_402_300_800_i64, format: TimestampFormat::SystemDate }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    for (instant, text) in [
        ("1969-12-31T23:59:59Z", "-1"),
        ("2000-02-29T00:00:00Z", "2000-02-29"),
        ("0001-01-01T00:00:00Z", "00:00:00"),
        ("9999-12-31T23:59:59Z", "9999-12-31 23:59:59"),
    ] {
        assert!(
            html.contains(&format!("datetime=\"{instant}\">{text}</time>")),
            "{html}"
        );
    }
    assert_eq!(html.matches("<time ").count(), 4);
    assert!(!html.contains("tabindex"), "{html}");
}
