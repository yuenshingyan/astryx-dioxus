use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::date_input::is_valid_date;

const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];
const WEEKDAYS: [(&str, &str); 7] = [
    ("Su", "Sunday"),
    ("Mo", "Monday"),
    ("Tu", "Tuesday"),
    ("We", "Wednesday"),
    ("Th", "Thursday"),
    ("Fr", "Friday"),
    ("Sa", "Saturday"),
];

/// Single-date Gregorian calendar with one visible month and a controlled selection.
#[component]
pub fn Calendar(
    /// Initial visible month, given as a valid `YYYY-MM-DD` date.
    default_focused_date: String,
    #[props(default)] value: Option<String>,
    #[props(default)] on_change: Option<EventHandler<String>>,
    /// Earliest selectable date, inclusive (`YYYY-MM-DD`).
    #[props(default)]
    min: Option<String>,
    /// Latest selectable date, inclusive (`YYYY-MM-DD`).
    #[props(default)]
    max: Option<String>,
    #[props(default = "Previous month".to_owned())] previous_month_label: String,
    #[props(default = "Next month".to_owned())] next_month_label: String,
) -> Element {
    let grid_id = use_hook(|| format!("astryx-calendar-{}", current_scope_id().0));
    let initial = parse_date(&default_focused_date);
    let mut view = use_signal(|| initial.map_or(0, |(year, month, _)| (year - 1) * 12 + month - 1));
    let mut focused = use_signal(|| initial.map_or(1, |(_, _, day)| day));
    let mut controller = use_signal(|| None::<document::Eval>);
    use_drop(move || {
        if let Some(eval) = *controller.peek() {
            let _ = eval.send(());
        }
    });
    if initial.is_none()
        || [&value, &min, &max]
            .into_iter()
            .flatten()
            .any(|date| !is_valid_date(date))
        || min
            .as_ref()
            .zip(max.as_ref())
            .is_some_and(|(min, max)| min > max)
    {
        return VNode::empty();
    }
    let lower = min.as_deref().and_then(parse_date);
    let upper = max.as_deref().and_then(parse_date);
    let first_month = lower.map_or(0, month_index);
    let last_month = upper.map_or(9999 * 12 - 1, month_index);
    let index = view().clamp(first_month, last_month);
    let year = index / 12 + 1;
    let month = index % 12 + 1;
    let month_name = MONTHS[month - 1];
    let title = format!("{month_name} {year}");
    let month_start = first_weekday(year, month);
    let day_count = (28..=31)
        .rev()
        .find(|day| is_valid_date(&format!("{year:04}-{month:02}-{day:02}")))
        .unwrap_or(28);
    let first_day = lower
        .filter(|&date| month_index(date) == index)
        .map_or(1, |(_, _, day)| day);
    let last_day = upper
        .filter(|&date| month_index(date) == index)
        .map_or(day_count, |(_, _, day)| day);
    let active = focused().clamp(first_day, last_day);
    let title_id = format!("{grid_id}-title");
    rsx! {
        div { class: "astryx-calendar",
            div { class: "astryx-calendar__header",
                button { r#type: "button", "aria-label": previous_month_label, "aria-disabled": (index == first_month).then_some("true"),
                    onclick: move |_| { if index > first_month { view.set(index - 1); } }, span { "aria-hidden": "true", "‹" } }
                div { id: title_id.clone(), "aria-live": "polite", "{title}" }
                button { r#type: "button", "aria-label": next_month_label, "aria-disabled": (index == last_month).then_some("true"),
                    onclick: move |_| { if index < last_month { view.set(index + 1); } }, span { "aria-hidden": "true", "›" } }
            }
            table { id: grid_id.clone(), role: "grid", "aria-labelledby": title_id,
                "data-min": min, "data-max": max, "data-month": index.to_string(),
                onmounted: move |_| {
                    if let Some(eval) = *controller.peek() { let _ = eval.send(()); }
                    let mut eval = document::eval(include_str!("calendar_focus.js"));
                    let _ = eval.send(grid_id.clone());
                    controller.set(Some(eval));
                    spawn(async move {
                        while let Ok(date) = eval.recv::<String>().await {
                            if let Some(target @ (_, _, day)) = parse_date(&date) {
                                view.set(month_index(target));
                                focused.set(day);
                            }
                        }
                    });
                },
                thead { tr {
                    for (short, long) in WEEKDAYS { th { scope: "col", abbr: long, "{short}" } }
                } }
                tbody {
                    for week in 0_usize..6 {
                        tr { key: "{week}",
                            for weekday in 0..7 {
                                {
                                    let slot = week * 7 + weekday;
                                    let day = (slot + 1).checked_sub(month_start).filter(|day| (1..=day_count).contains(day));
                                    if let Some(day) = day {
                                        let iso = format!("{year:04}-{month:02}-{day:02}");
                                        let is_selected = value.as_deref() == Some(&iso);
                                        let is_disabled = !(first_day..=last_day).contains(&day);
                                        // Focus lands on the button, not the cell carrying aria-selected.
                                        let selection_suffix = if is_selected { ", selected" } else { "" };
                                        let accessible_name = format!("{}, {month_name} {day}, {year}{selection_suffix}", WEEKDAYS[weekday].1);
                                        rsx! {
                                            td { key: "{weekday}", role: "gridcell", "aria-selected": is_selected.to_string(),
                                                button { r#type: "button", "aria-label": accessible_name, disabled: is_disabled,
                                                    "data-day": day.to_string(), "data-date": iso.clone(), tabindex: if day == active { 0 } else { -1 },
                                                    onfocus: move |_| focused.set(day),
                                                    onclick: move |_| { if let Some(change) = on_change { change.call(iso.clone()); } },
                                                    "{day}"
                                                }
                                            }
                                        }
                                    } else { rsx! { td { key: "{weekday}", role: "gridcell" } } }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn parse_date(date: &str) -> Option<(usize, usize, usize)> {
    if !is_valid_date(date) {
        return None;
    }
    Some((
        date[..4].parse().ok()?,
        date[5..7].parse().ok()?,
        date[8..].parse().ok()?,
    ))
}

fn month_index((year, month, _): (usize, usize, usize)) -> usize {
    (year - 1) * 12 + month - 1
}

fn first_weekday(year: usize, month: usize) -> usize {
    // January 1 of Gregorian year 1 was a Monday.
    let completed = year - 1;
    let before_year = 365 * completed + completed / 4 - completed / 100 + completed / 400;
    let before_month = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334][month - 1];
    let leap_day = month > 2
        && year.is_multiple_of(4)
        && (!year.is_multiple_of(100) || year.is_multiple_of(400));
    (1 + before_year + before_month + usize::from(leap_day)) % 7
}
