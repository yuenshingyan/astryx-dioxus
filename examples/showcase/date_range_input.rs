use dioxus::prelude::*;

use astryx_dioxus::{DateRange, DateRangeInput};

#[component]
pub(super) fn DateRangeInputExamples() -> Element {
    let mut range = use_signal(|| None::<DateRange>);
    let display = range().map_or_else(
        || "No range selected".to_owned(),
        |range| format!("{} to {}", range.start, range.end),
    );
    rsx! {
        section { id: "date-range-input-examples",
            h2 { "Date Range Input" }
            DateRangeInput { label: "Date range", value: range(), min: "2026-01-01", max: "2026-12-31",
                max_range_span: 7, invalid_span_message: "Choose no more than seven days.",
                width: Some(400.into()), on_change: move |next| range.set(next),
            }
            output { id: "date-range-value", "{display}" }
        }
    }
}
