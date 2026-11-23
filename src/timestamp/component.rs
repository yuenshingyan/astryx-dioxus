use dioxus::prelude::*;

/// Absolute timestamp formats supported by the UTC renderer.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TimestampFormat {
    /// Gregorian date in UTC.
    SystemDate,
    /// Gregorian date and 24-hour time in UTC.
    SystemDateTime,
    /// 24-hour time in UTC, including seconds.
    SystemTime,
    /// Whole Unix seconds, independent of time zone.
    UnixSeconds,
}

/// A semantic timestamp for a whole-second instant, rendered without a clock.
/// Values outside Gregorian years 1–9999 render nothing.
#[component]
pub fn Timestamp(value: i64, format: TimestampFormat) -> Element {
    if !(-62_135_596_800..=253_402_300_799).contains(&value) {
        return VNode::empty();
    }
    let (year, month, day) = civil_date(value.div_euclid(86_400));
    let seconds = value.rem_euclid(86_400);
    let date = format!("{year:04}-{month:02}-{day:02}");
    let time = format!(
        "{:02}:{:02}:{:02}",
        seconds / 3600,
        seconds / 60 % 60,
        seconds % 60
    );
    let datetime = format!("{date}T{time}Z");
    let (name, text) = match format {
        TimestampFormat::SystemDate => ("system_date", date),
        TimestampFormat::SystemDateTime => ("system_date_time", format!("{date} {time}")),
        TimestampFormat::SystemTime => ("system_time", time),
        TimestampFormat::UnixSeconds => ("unix_seconds", value.to_string()),
    };
    rsx! { time { class: "astryx-timestamp", "data-format": name, datetime, "{text}" } }
}

// March-based Gregorian conversion, adapted from Howard Hinnant's public-domain
// civil_from_days algorithm: https://howardhinnant.github.io/date_algorithms.html.
fn civil_date(days: i64) -> (i64, i64, i64) {
    let shifted = days + 719_468;
    let era = shifted.div_euclid(146_097);
    let day_of_era = shifted.rem_euclid(146_097);
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let year = era * 400 + year_of_era;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let march_month = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * march_month + 2) / 5 + 1;
    let month = if march_month < 10 {
        march_month + 3
    } else {
        march_month - 9
    };
    (year + i64::from(month <= 2), month, day)
}
