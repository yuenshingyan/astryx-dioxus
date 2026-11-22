use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use super::radio_list_context::RadioListContext;
use crate::utils::is_renderable::is_renderable;
use crate::{IndicatorSize, Item, ItemAlign, ItemLabel, RadioIndicator, RadioIndicatorState};

/// A native radio option for a controlled RadioList.
#[component]
pub fn RadioListItem(
    /// Value submitted and reported to the enclosing group.
    value: String,
    #[props(into)] label: ItemLabel,
    #[props(default)] accessible_label: Option<String>,
    #[props(default)] description: Option<ItemLabel>,
    #[props(default)] start_content: Option<Element>,
    #[props(default)] end_content: Option<Element>,
    #[props(default)] align: ItemAlign,
    #[props(default)] is_disabled: bool,
    #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let context = try_consume_context::<RadioListContext>()
        .ok_or_else(|| std::io::Error::other("RadioListItem requires a RadioList ancestor"))?;
    let settings = (context.0)();
    let id = use_hook(|| format!("astryx-radio-list-item-{}", current_scope_id().0));
    let label_id = format!("{id}-label");
    let description_id = format!("{id}-desc");
    let labelled_by = accessible_label.is_none().then(|| label_id.clone());
    let label = match label {
        ItemLabel::Text(text) => rsx! { "{text}" },
        ItemLabel::Rich(content) => content,
    };
    let description = description.and_then(|description| match description {
        ItemLabel::Text(text) if !text.is_empty() => Some(rsx! { "{text}" }),
        ItemLabel::Rich(content) if is_renderable(&content) => Some(content),
        _ => None,
    });
    let described_by = description.as_ref().map(|_| description_id.clone());
    let description =
        description.map(|content| ItemLabel::Rich(rsx! { span { id: description_id, {content} } }));
    let is_checked = settings.value.as_deref() == Some(value.as_str());
    let is_disabled = is_disabled || settings.is_disabled;
    let state = if is_checked {
        RadioIndicatorState::Checked
    } else {
        RadioIndicatorState::Unchecked
    };
    let size = settings.size;
    let requested = value.clone();
    rsx! {
        Item { class: format!("astryx-radio-list-item {class}"), style, align, attributes, description, end_content,
            label: rsx! { span { id: label_id, {label} } },
            start_content: rsx! {
                span { class: "astryx-radio-list-item__control", "data-size": if size == IndicatorSize::Small { "sm" } else { "md" },
                    input { id, r#type: "radio", name: settings.name, value, required: settings.is_required, disabled: is_disabled,
                        checked: is_checked, "data-radio-checked": is_checked.to_string(),
                        "aria-label": accessible_label, "aria-labelledby": labelled_by, "aria-describedby": described_by,
                        onclick: move |event| { if !is_disabled && let Some(handler) = onclick { handler.call(event); } },
                        onchange: move |_| {
                            // Native radios change only on the newly selected option.
                            if !is_disabled && let Some(handler) = settings.on_change { handler.call(requested.clone()); }
                        },
                    }
                    RadioIndicator { state, size, is_disabled }
                }
                {start_content}
            },
        }
    }
}
