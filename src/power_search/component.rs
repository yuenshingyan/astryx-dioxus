use dioxus::prelude::*;

use crate::Token;

/// Value accepted by a filter operator.
#[derive(Clone, PartialEq, Eq)]
pub enum PowerSearchValueKind {
    /// Nonblank free text.
    Text,
    /// One of the listed values.
    Enum(Vec<PowerSearchChoice>),
}

/// A configured enumeration choice.
#[derive(Clone, PartialEq, Eq)]
pub struct PowerSearchChoice {
    pub value: String,
    pub label: String,
}

/// An operator and its expected value.
#[derive(Clone, PartialEq, Eq)]
pub struct PowerSearchOperator {
    pub key: String,
    pub label: String,
    pub kind: PowerSearchValueKind,
}

/// A searchable field with a list of supported operators in default order.
#[derive(Clone, PartialEq, Eq)]
pub struct PowerSearchField {
    pub key: String,
    pub label: String,
    pub operators: Vec<PowerSearchOperator>,
}

/// A typed filter value.
#[derive(Clone, PartialEq, Eq)]
pub enum PowerSearchValue {
    Text(String),
    Enum(String),
}

/// A committed filter clause. The caller owns how clauses are interpreted.
#[derive(Clone, PartialEq, Eq)]
pub struct PowerSearchFilter {
    pub field: String,
    pub operator: String,
    pub value: PowerSearchValue,
}

/// A controlled filter builder for local text and enumeration fields.
#[component]
pub fn PowerSearch(
    label: String,
    fields: Vec<PowerSearchField>,
    filters: Vec<PowerSearchFilter>,
    on_change: EventHandler<Vec<PowerSearchFilter>>,
    #[props(default = "Field".to_owned())] field_label: String,
    #[props(default = "Operator".to_owned())] operator_label: String,
    #[props(default = "Value".to_owned())] value_label: String,
    #[props(default = "Add filter".to_owned())] add_label: String,
    #[props(default = "Enter a value".to_owned())] placeholder: String,
    #[props(default)] is_disabled: bool,
) -> Element {
    let mut field_key = use_signal(String::new);
    let mut operator_key = use_signal(String::new);
    let mut draft = use_signal(String::new);
    let field = fields
        .iter()
        .find(|field| field.key == *field_key.read())
        .or_else(|| fields.first());
    let operators = field.map_or(&[][..], |field| field.operators.as_slice());
    let operator = operators
        .iter()
        .find(|operator| operator.key == *operator_key.read())
        .or_else(|| operators.first());
    let next_filter = field
        .zip(operator)
        .and_then(|(field, operator)| {
            let text = draft.read();
            let value = match &operator.kind {
                PowerSearchValueKind::Text if !text.trim().is_empty() => {
                    PowerSearchValue::Text(text.trim().to_owned())
                }
                PowerSearchValueKind::Enum(choices)
                    if !text.is_empty() && choices.iter().any(|choice| choice.value == *text) =>
                {
                    PowerSearchValue::Enum(text.clone())
                }
                _ => return None,
            };
            Some(PowerSearchFilter {
                field: field.key.clone(),
                operator: operator.key.clone(),
                value,
            })
        })
        .filter(|next| !filters.contains(next));
    let can_add = !is_disabled && next_filter.is_some();
    let previous = filters.clone();
    let operator_key_dom = field.map_or("", |field| field.key.as_str());
    let value_key = format!(
        "{}:{}",
        operator_key_dom,
        operator.map_or("", |operator| operator.key.as_str())
    );
    rsx! {
        fieldset { class: "astryx-power-search", disabled: is_disabled,
            legend { "{label}" }
            if !filters.is_empty() {
                ul { class: "astryx-power-search__filters", role: "list",
                    for (index, filter) in filters.iter().enumerate() {
                        {
                            let summary = describe_filter(filter, &fields);
                            let mut remaining = filters.clone();
                            remaining.remove(index);
                            rsx! {
                                li { key: "{index}",
                                    Token { label: summary, is_disabled,
                                        on_remove: move |_| { if !is_disabled { on_change.call(remaining.clone()); } }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div { class: "astryx-power-search__controls",
                select { "aria-label": field_label, value: field.map(|field| field.key.clone()),
                    onchange: move |event| { field_key.set(event.value()); operator_key.set(String::new()); draft.set(String::new()); },
                    for entry in &fields { option { key: "{entry.key}", value: entry.key.clone(), "{entry.label}" } }
                }
                select { key: "{operator_key_dom}", "aria-label": operator_label, value: operator.map(|operator| operator.key.clone()),
                    onchange: move |event| { operator_key.set(event.value()); draft.set(String::new()); },
                    for entry in operators { option { key: "{entry.key}", value: entry.key.clone(), "{entry.label}" } }
                }
                if let Some(operator) = operator {
                    match &operator.kind {
                        PowerSearchValueKind::Text => rsx! {
                            input { r#type: "text", "aria-label": value_label.clone(), placeholder, value: draft(), oninput: move |event| draft.set(event.value()) }
                        },
                        PowerSearchValueKind::Enum(choices) => rsx! {
                            select { key: "{value_key}", "aria-label": value_label.clone(), value: draft(), onchange: move |event| draft.set(event.value()),
                                option { value: "", disabled: true, selected: draft.read().is_empty(), "{placeholder}" }
                                for choice in choices { option { key: "{choice.value}", value: choice.value.clone(), "{choice.label}" } }
                            }
                        },
                    }
                }
                button { r#type: "button", disabled: !can_add, onclick: move |_| {
                    if !is_disabled && let Some(filter) = &next_filter {
                        let mut next = previous.clone();
                        next.push(filter.clone());
                        on_change.call(next);
                        draft.set(String::new());
                    }
                }, "{add_label}" }
            }
        }
    }
}

fn describe_filter(filter: &PowerSearchFilter, fields: &[PowerSearchField]) -> String {
    let field = fields.iter().find(|field| field.key == filter.field);
    let operator = field.and_then(|field| {
        field
            .operators
            .iter()
            .find(|operator| operator.key == filter.operator)
    });
    let value = match (&filter.value, operator.map(|operator| &operator.kind)) {
        (PowerSearchValue::Enum(value), Some(PowerSearchValueKind::Enum(choices))) => choices
            .iter()
            .find(|choice| choice.value == *value)
            .map_or(value.as_str(), |choice| choice.label.as_str()),
        (PowerSearchValue::Text(value) | PowerSearchValue::Enum(value), _) => value,
    };
    format!(
        "{} {} {value}",
        field.map_or(filter.field.as_str(), |field| field.label.as_str()),
        operator.map_or(filter.operator.as_str(), |operator| operator.label.as_str())
    )
}
