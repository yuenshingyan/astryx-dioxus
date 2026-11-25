use dioxus::prelude::*;

use astryx_dioxus::Calendar;

#[component]
pub(super) fn CalendarExamples() -> Element {
    let mut selected = use_signal(|| Some("2026-04-15".to_owned()));
    rsx! {
        section { id: "calendar-examples",
            h2 { "Calendar" }
            Calendar { default_focused_date: "2026-04-15", value: selected(),
                min: "2026-04-10", max: "2026-05-20",
                on_change: move |date| selected.set(Some(date)),
            }
        }
    }
}
