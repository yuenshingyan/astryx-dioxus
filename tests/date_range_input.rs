use astryx_dioxus::{DateRange, DateRangeInput};
use dioxus::prelude::*;

#[test]
fn span_limits_do_not_mark_committed_ranges_invalid() {
    fn reject_change(_: Option<DateRange>) {
        panic!("render must not change the committed range");
    }

    let mut dom = VirtualDom::new(|| {
        rsx! {
            DateRangeInput {
                label: "Short committed range",
                value: DateRange { start: "2026-04-10".into(), end: "2026-04-11".into() },
                min_range_span: 3,
                on_change: reject_change,
            }
            DateRangeInput {
                label: "Long committed range",
                value: DateRange { start: "2026-05-10".into(), end: "2026-05-18".into() },
                max_range_span: 7,
                on_change: reject_change,
            }
        }
    });
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);

    let inputs = html
        .match_indices("<input ")
        .map(|(start, _)| html[start..].split('>').next().expect("input tag"));
    let dates = ["2026-04-10", "2026-04-11", "2026-05-10", "2026-05-18"];
    assert_eq!(inputs.clone().count(), dates.len(), "{html}");
    for (input, date) in inputs.zip(dates) {
        assert!(input.contains(&format!("value=\"{date}\"")), "{input}");
        assert!(!input.contains("aria-invalid"), "{input}");
        assert!(!input.contains("aria-describedby"), "{input}");
    }
    assert!(!html.contains("Choose a range within"), "{html}");
}
