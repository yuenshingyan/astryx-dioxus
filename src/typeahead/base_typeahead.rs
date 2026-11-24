use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use super::{TypeaheadItem, TypeaheadItemContent};

/// One result in a local search source. IDs must be unique within the source.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BaseTypeaheadOption {
    pub id: String,
    pub label: String,
    pub description: Option<String>,
}

/// A single-selection combobox with local search and caller-owned selection.
#[component]
pub fn BaseTypeahead(
    label: String,
    options: Vec<BaseTypeaheadOption>,
    value: Option<BaseTypeaheadOption>,
    on_change: EventHandler<Option<BaseTypeaheadOption>>,
    #[props(default)] placeholder: Option<String>,
    /// ID used by an external label.
    #[props(default)]
    input_id: Option<String>,
    /// IDs of descriptions associated with the input.
    #[props(default)]
    described_by: Option<String>,
    /// Reflects a caller-provided validation error.
    #[props(default)]
    is_invalid: bool,
    #[props(default)] has_entries_on_focus: bool,
    #[props(default)] is_disabled: bool,
    #[props(default = 10)] max_menu_items: usize,
    #[props(default = "No results".to_owned())] empty_search_text: String,
) -> Element {
    let id = use_hook(|| format!("astryx-base-typeahead-{}", current_scope_id().0));
    let mut draft = use_signal(|| None::<String>);
    let mut is_open = use_signal(|| false);
    let mut active = use_signal(|| None::<usize>);
    let text = draft().unwrap_or_else(|| {
        value
            .as_ref()
            .map_or_else(String::new, |item| item.label.clone())
    });
    let needle = text.trim().to_lowercase();
    let is_browsing = draft.read().is_none();
    let results: Vec<_> = options
        .into_iter()
        .enumerate()
        .filter(|(_, item)| is_browsing || item.label.to_lowercase().contains(&needle))
        .take(max_menu_items)
        .collect();
    let count = results.len();
    let index = active().filter(|&index| index < count);
    let active_result = index.and_then(|index| results.get(index));
    let shown = !is_disabled && is_open();
    let active_id = active_result
        .filter(|_| shown)
        .map(|(source, _)| format!("{id}-{source}"));
    let active_item = active_result.map(|(_, item)| item.clone());
    let select = use_callback(move |item: BaseTypeaheadOption| {
        is_open.set(false);
        active.set(None);
        draft.set(None);
        on_change.call(Some(item));
    });
    rsx! {
        div { class: "astryx-base-typeahead",
            input { id: input_id, r#type: "text", role: "combobox", "aria-label": label.clone(), "aria-autocomplete": "list",
                "aria-describedby": described_by, "aria-invalid": is_invalid.then_some("true"),
                "aria-expanded": shown.to_string(), "aria-controls": id.clone(), "aria-activedescendant": active_id,
                autocomplete: "off", placeholder, value: text, disabled: is_disabled,
                onfocus: move |_| { if has_entries_on_focus { is_open.set(true); } },
                onblur: move |_| { is_open.set(false); active.set(None); draft.set(None); },
                oninput: move |event| {
                    draft.set(Some(event.value()));
                    active.set(None);
                    is_open.set(true);
                },
                onkeydown: move |event: KeyboardEvent| {
                    if event.is_composing() || !event.default_action_enabled() { return; }
                    match event.key() {
                        Key::Escape if shown => { is_open.set(false); active.set(None); }
                        Key::ArrowDown | Key::ArrowUp if count > 0 => {
                            let next = if event.key() == Key::ArrowDown {
                                index.map_or(0, |index| (index + 1) % count)
                            } else {
                                index.map_or(count - 1, |index| (index + count - 1) % count)
                            };
                            is_open.set(true);
                            active.set(Some(next));
                        }
                        Key::Enter if shown => {
                            if let Some(item) = &active_item { select.call(item.clone()); } else { return; }
                        }
                        _ => return,
                    }
                    event.prevent_default();
                },
            }
            div { class: "astryx-base-typeahead__popup", hidden: !shown,
                div { id: id.clone(), role: "listbox", "aria-label": "{label}",
                    for (position, (source, item)) in results.into_iter().enumerate() {
                        {
                            let selection = item.clone();
                            rsx! {
                                div { key: "{item.id}", id: "{id}-{source}", role: "option", "aria-selected": (index == Some(position)).to_string(),
                                    onmousedown: move |event| event.prevent_default(),
                                    onclick: move |_| select.call(selection.clone()),
                                    TypeaheadItem { item: TypeaheadItemContent { label: item.label, element: None }, description: item.description }
                                }
                            }
                        }
                    }
                }
                if count == 0 { div { role: "status", class: "astryx-base-typeahead__empty", "{empty_search_text}" } }
            }
        }
    }
}
