use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::date_input::is_valid_date;
use crate::time_input::is_valid_time;
use crate::{FieldLabel, SizeValue};

/// A local Gregorian date and time, entered through separate native controls.
/// The value uses `YYYY-MM-DDTHH:MM`, or includes seconds when enabled.
#[component]
pub fn DateTimeInput(
    label: String,
    #[props(default)] value: Option<String>,
    #[props(default)] on_change: Option<EventHandler<Option<String>>>,
    #[props(default = "Time".to_owned())] time_label: String,
    #[props(default)] html_name: Option<String>,
    #[props(default)] has_seconds: bool,
    #[props(default)] is_required: bool,
    #[props(default)] is_disabled: bool,
    #[props(default)] width: Option<SizeValue>,
) -> Element {
    let id = use_hook(|| format!("astryx-date-time-input-{}", current_scope_id().0));
    let mut time_draft = use_signal(|| None::<String>);
    let mut pending_date = use_signal(|| None::<String>);
    let (date, time) = if let Some(value) = value.as_deref() {
        let Some((date, time)) = value.split_once('T') else {
            return VNode::empty();
        };
        if !is_valid_date(date) || !is_valid_time(time, has_seconds) {
            return VNode::empty();
        }
        (
            date.to_owned(),
            Some(normalize_time(time.to_owned(), has_seconds)),
        )
    } else {
        (String::new(), time_draft.read().clone())
    };
    let style = width
        .map(|size| size.as_css().map(|width| format!("inline-size:{width}")))
        .transpose()?;
    let date_id = format!("{id}-date");
    let time_id = format!("{id}-time");
    let time_value = time.clone().unwrap_or_default();
    rsx! {
        div { class: "astryx-date-time-input", style,
            FieldLabel { label, input_id: date_id.clone(), is_required, is_disabled }
            div { class: "astryx-date-time-input__segments",
                input { id: date_id, class: "astryx-date-input__control", r#type: "date", value: date.clone(),
                    required: is_required, disabled: is_disabled,
                    onchange: move |event| {
                        pending_date.set(None);
                        if is_disabled { return; }
                        let Some(change) = on_change else { return };
                        let next = event.value();
                        if next.is_empty() {
                            time_draft.set(None);
                            change.call(None);
                        } else if is_valid_date(&next) {
                            if let Some(time) = &time {
                                time_draft.set(None);
                                change.call(Some(format!("{next}T{time}")));
                            } else {
                                pending_date.set(Some(next.clone()));
                                spawn(async move {
                                    let result = document::eval(include_str!("local_time.js")).join::<String>().await;
                                    if let Ok(time) = result && pending_date.peek().as_deref() == Some(&next) {
                                        pending_date.set(None);
                                        let time = normalize_time(time, has_seconds);
                                        change.call(Some(format!("{next}T{time}")));
                                    }
                                });
                            }
                        }
                    },
                }
                input { id: time_id, class: "astryx-time-input__control", r#type: "time", value: time_value,
                    "aria-label": time_label, step: if has_seconds { "1" } else { "60" },
                    required: is_required, disabled: is_disabled,
                    onchange: move |event| {
                        if is_disabled { return; }
                        let next = event.value();
                        if next.is_empty() {
                            pending_date.set(None);
                            time_draft.set(None);
                            if !date.is_empty() && let Some(change) = on_change { change.call(None); }
                        } else if is_valid_time(&next, has_seconds) {
                            let next = normalize_time(next, has_seconds);
                            let selected_date = pending_date.take();
                            if let Some(date) = selected_date.as_deref().or_else(|| (!date.is_empty()).then_some(date.as_str())) {
                                time_draft.set(None);
                                if let Some(change) = on_change { change.call(Some(format!("{date}T{next}"))); }
                            } else {
                                time_draft.set(Some(next));
                            }
                        }
                    },
                }
            }
            if let Some(name) = html_name {
                input { r#type: "hidden", name, value: value.unwrap_or_default(), disabled: is_disabled }
            }
        }
    }
}

fn normalize_time(mut time: String, has_seconds: bool) -> String {
    if has_seconds && time.len() == 5 {
        time.push_str(":00");
    }
    if !has_seconds {
        time.truncate(5);
    }
    time
}
