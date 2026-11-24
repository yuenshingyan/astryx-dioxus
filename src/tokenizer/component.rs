use std::collections::HashSet;

use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::{BaseTypeahead, BaseTypeaheadOption, Field, SizeValue, Token};

/// The item affected by a tokenizer change.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenizerChange {
    /// A result was added.
    Add(BaseTypeaheadOption),
    /// A token was removed.
    Remove(BaseTypeaheadOption),
}

/// A controlled tag field with local search and individually removable tokens.
#[component]
pub fn Tokenizer(
    label: String,
    options: Vec<BaseTypeaheadOption>,
    value: Vec<BaseTypeaheadOption>,
    on_change: EventHandler<(Vec<BaseTypeaheadOption>, TokenizerChange)>,
    #[props(default)] placeholder: Option<String>,
    #[props(default)] html_name: Option<String>,
    #[props(default)] width: Option<SizeValue>,
    #[props(default)] has_entries_on_focus: bool,
    #[props(default)] is_disabled: bool,
) -> Element {
    let id = use_hook(|| format!("astryx-tokenizer-{}", current_scope_id().0));
    let mut ids = HashSet::new();
    let value: Vec<_> = value
        .into_iter()
        .filter(|item| !item.id.is_empty() && ids.insert(item.id.clone()))
        .collect();
    let options = options
        .into_iter()
        .filter(|item| !item.id.is_empty() && ids.insert(item.id.clone()))
        .collect();
    let selected = value.clone();
    let add = move |next: Option<BaseTypeaheadOption>| {
        if is_disabled {
            return;
        }
        if let Some(item) =
            next.filter(|item| !selected.iter().any(|previous| previous.id == item.id))
        {
            let mut next = selected.clone();
            next.push(item.clone());
            on_change.call((next, TokenizerChange::Add(item)));
        }
    };
    rsx! {
        Field { label: label.clone(), input_id: id.clone(), width, is_disabled,
            div { class: "astryx-tokenizer",
                if !value.is_empty() {
                    ul { class: "astryx-tokenizer__tokens", role: "list",
                        for item in &value {
                            {
                                let removed = item.clone();
                                let focus_id = id.clone();
                                let remaining: Vec<_> = value.iter().filter(|candidate| candidate.id != item.id).cloned().collect();
                                rsx! {
                                    li { key: "{item.id}",
                                        Token { label: item.label.clone(), is_disabled,
                                            on_remove: move |_: MouseEvent| {
                                                if is_disabled { return; }
                                                let eval = document::eval("document.getElementById(await dioxus.recv())?.focus({preventScroll: true});");
                                                let _ = eval.send(focus_id.clone());
                                                on_change.call((remaining.clone(), TokenizerChange::Remove(removed.clone())));
                                            },
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                BaseTypeahead { label, input_id: id, placeholder, options, value: None,
                    on_change: add, has_entries_on_focus, is_disabled,
                }
                if !is_disabled && let Some(name) = html_name {
                    for item in value {
                        input { key: "{item.id}", r#type: "hidden", name: name.clone(), value: item.id }
                    }
                }
            }
        }
    }
}
