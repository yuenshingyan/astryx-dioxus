use dioxus::prelude::*;

use astryx_dioxus::{Timestamp, TimestampFormat};

#[component]
pub(super) fn TimestampExamples() -> Element {
    let value = 1_774_440_000_i64;
    rsx! {
        section { id: "timestamp-examples",
            h2 { "Timestamp" }
            p { "Date and time (UTC): " Timestamp { value, format: TimestampFormat::SystemDateTime } }
            p { "Date (UTC): " Timestamp { value, format: TimestampFormat::SystemDate } }
            p { "Time (UTC): " Timestamp { value, format: TimestampFormat::SystemTime } }
            p { "Unix seconds: " Timestamp { value, format: TimestampFormat::UnixSeconds } }
        }
    }
}
