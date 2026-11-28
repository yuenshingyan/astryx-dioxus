use std::collections::HashSet;
use std::rc::Rc;

use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::field::input_clear_button::InputClearButton;
use crate::hooks::use_announce::ANNOUNCE_CONTROLLER;
use crate::i18n::use_message;
use crate::{
    BaseTypeahead, BaseTypeaheadOption, Field, FieldStatusInput, FieldStatusPlacement,
    InputStatusType, SizeValue, Token,
};

/// The item affected by a tokenizer change.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenizerChange {
    /// A result was added.
    Add(BaseTypeaheadOption),
    /// A token was removed. Clear all reports the last removable token in selection order.
    Remove(BaseTypeaheadOption),
}

type CanRemoveItem = dyn Fn(&BaseTypeaheadOption) -> bool;

// Kept internal for Tokenizer compositions that lock some selections.
#[derive(Clone)]
struct TokenizerItemRemovable(Rc<CanRemoveItem>);

fn clear_all_change(
    value: &[BaseTypeaheadOption],
    can_remove_item: &CanRemoveItem,
) -> Option<(Vec<BaseTypeaheadOption>, TokenizerChange)> {
    let last_removed = value.iter().rfind(|item| can_remove_item(item))?;
    let remaining = value
        .iter()
        .filter(|item| !can_remove_item(item))
        .cloned()
        .collect();
    Some((remaining, TokenizerChange::Remove(last_removed.clone())))
}

fn focus_search(id: &str) {
    let eval = document::eval(
        "document.getElementById(await dioxus.recv())?.focus({preventScroll: true});",
    );
    let _ = eval.send(id);
}

fn announce_clear_all(id: &str, message: &str) {
    let eval = document::eval(&format!(
        "{ANNOUNCE_CONTROLLER}\n{}",
        include_str!("announce_clear_all.js"),
    ));
    let _ = eval.send((id, message));
}

/// A controlled tag field with local search and individually removable tokens.
#[component]
pub fn Tokenizer(
    label: String,
    options: Vec<BaseTypeaheadOption>,
    value: Vec<BaseTypeaheadOption>,
    on_change: EventHandler<(Vec<BaseTypeaheadOption>, TokenizerChange)>,
    #[props(default)] placeholder: Option<String>,
    /// Help text associated with the search input.
    #[props(default)]
    description: Option<String>,
    /// Caller-owned validation feedback for the token selection.
    #[props(default)]
    status: Option<FieldStatusInput>,
    /// Placement of the validation message.
    #[props(default)]
    status_variant: FieldStatusPlacement,
    #[props(default)] html_name: Option<String>,
    #[props(default)] width: Option<SizeValue>,
    #[props(default)] has_entries_on_focus: bool,
    #[props(default)] is_disabled: bool,
) -> Element {
    render_tokenizer(
        TokenizerProps {
            label,
            options,
            value,
            on_change,
            placeholder,
            description,
            status,
            status_variant,
            html_name,
            width,
            has_entries_on_focus,
            is_disabled,
        },
        false,
    )
}

/// A [`Tokenizer`] with a Clear all action while enabled and holding removable tokens.
#[component]
pub fn TokenizerWithClear(props: TokenizerProps) -> Element {
    render_tokenizer(props, true)
}

fn render_tokenizer(
    TokenizerProps {
        label,
        options,
        value,
        on_change,
        placeholder,
        description,
        status,
        status_variant,
        html_name,
        width,
        has_entries_on_focus,
        is_disabled,
    }: TokenizerProps,
    has_clear: bool,
) -> Element {
    let id = use_hook(|| format!("astryx-tokenizer-{}", current_scope_id().0));
    let description_id = description
        .as_ref()
        .filter(|text| !text.is_empty())
        .map(|_| format!("{id}-desc"));
    let is_invalid = status
        .as_ref()
        .is_some_and(|status| status.status_type == InputStatusType::Error);
    let status_id = status
        .as_ref()
        .filter(|status| status.message.as_ref().is_some_and(|text| !text.is_empty()))
        .map(|status| {
            status
                .message_id
                .clone()
                .unwrap_or_else(|| format!("{id}-status"))
        });
    let described_by = [description_id.as_deref(), status_id.as_deref()]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join(" ");
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
    let removal_policy = try_consume_context::<TokenizerItemRemovable>();
    let can_remove_item = move |item: &BaseTypeaheadOption| {
        removal_policy
            .as_ref()
            .is_none_or(|policy| (policy.0)(item))
    };
    let clear_change = (has_clear && !is_disabled)
        .then(|| clear_all_change(&value, &can_remove_item))
        .flatten();
    let has_clear_button = clear_change.is_some();
    let clear_label = use_message("@astryx.tokenizer.clearAll", "Clear all");
    let clear_message = use_message("@astryx.tokenizer.allRemoved", "All items removed");
    let focus_id = id.clone();
    rsx! {
        Field { label: label.clone(), input_id: id.clone(), width, is_disabled, description, description_id, status, status_variant,
            div { class: "astryx-tokenizer", "data-has-clear": has_clear_button.then_some("true"),
                if !value.is_empty() {
                    ul { class: "astryx-tokenizer__tokens", role: "list",
                        for item in &value {
                            {
                                let on_remove = can_remove_item(item).then(|| {
                                    let removed = item.clone();
                                    let focus_id = id.clone();
                                    let remaining: Vec<_> = value.iter().filter(|candidate| candidate.id != item.id).cloned().collect();
                                    EventHandler::new(move |_: MouseEvent| {
                                        if is_disabled { return; }
                                        focus_search(&focus_id);
                                        on_change.call((remaining.clone(), TokenizerChange::Remove(removed.clone())));
                                    })
                                });
                                rsx! {
                                    li { key: "{item.id}",
                                        Token { label: item.label.clone(), is_disabled, on_remove }
                                    }
                                }
                            }
                        }
                    }
                }
                BaseTypeahead { label, input_id: id, placeholder, options, value: None,
                    on_change: add, has_entries_on_focus, is_disabled, is_invalid,
                    described_by: (!described_by.is_empty()).then_some(described_by),
                }
                if let Some(change) = clear_change {
                    InputClearButton { label: clear_label, onclick: move |event: MouseEvent| {
                        event.stop_propagation();
                        on_change.call(change.clone());
                        focus_search(&focus_id);
                        announce_clear_all(&focus_id, &clear_message);
                    } }
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

#[cfg(test)]
mod tests {
    use super::{BaseTypeaheadOption, TokenizerChange, clear_all_change};

    #[test]
    fn clear_all_retains_locked_tokens_in_order_and_reports_last_removal() {
        let selection = ["a", "locked-1", "b", "locked-2"].map(|id| BaseTypeaheadOption {
            id: id.into(),
            label: id.into(),
            description: None,
        });
        assert_eq!(
            clear_all_change(&selection, &|item| !item.id.starts_with("locked")),
            Some((
                vec![selection[1].clone(), selection[3].clone()],
                TokenizerChange::Remove(selection[2].clone()),
            )),
        );
        assert_eq!(clear_all_change(&selection, &|_| false), None);
    }
}
