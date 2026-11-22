use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::form_layout::form_layout_context::{FormLayoutContext, FormOptionality};
use crate::utils::is_renderable::is_renderable;
use crate::{
    ElementSize, Field, FieldStatusInput, FieldStatusPlacement, InputStatusType, SizeValue,
    use_size,
};

/// Native single-line input types supported by TextInput.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum TextInputType {
    #[default]
    Text,
    Password,
    Email,
    Search,
    Tel,
    Url,
}

impl TextInputType {
    fn as_str(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Password => "password",
            Self::Email => "email",
            Self::Search => "search",
            Self::Tel => "tel",
            Self::Url => "url",
        }
    }
}

/// A controlled native single-line field with a visible or visually hidden label.
#[component]
pub fn TextInput(
    label: String,
    value: String,
    /// Receives the value entered by the user.
    #[props(default)]
    on_change: Option<EventHandler<String>>,
    #[props(default)] input_type: TextInputType,
    #[props(default)] id: Option<String>,
    #[props(default)] html_name: Option<String>,
    #[props(default)] placeholder: Option<String>,
    #[props(default)] autocomplete: Option<String>,
    #[props(default)] description: Option<String>,
    #[props(default)] label_tooltip: Option<String>,
    #[props(default)] status: Option<FieldStatusInput>,
    #[props(default)] status_variant: FieldStatusPlacement,
    #[props(default)] start_icon: Option<Element>,
    #[props(default)] is_label_hidden: bool,
    #[props(default)] is_required: bool,
    #[props(default)] is_optional: bool,
    #[props(default)] is_disabled: bool,
    #[props(default)] is_read_only: bool,
    #[props(default)] has_spell_check: Option<bool>,
    #[props(default)] has_auto_focus: bool,
    #[props(default)] size: Option<ElementSize>,
    #[props(default)] width: Option<SizeValue>,
    #[props(default)] class: String,
    #[props(default)] style: String,
) -> Element {
    let generated_id = use_hook(|| format!("astryx-text-input-{}", current_scope_id().0));
    let id = id.unwrap_or(generated_id);
    let input_id = id.clone();
    let size = use_size(size).as_str();
    let description = description.filter(|text| !text.is_empty());
    let description_id = description.as_ref().map(|_| format!("{id}-desc"));
    let severity = status.as_ref().map(|status| status.status_type);
    let status_id = status
        .as_ref()
        .filter(|status| status.message.as_ref().is_some_and(|text| !text.is_empty()))
        .map(|status| {
            status
                .message_id
                .clone()
                .unwrap_or_else(|| format!("{id}-status"))
        });
    let descriptions = [description_id.as_deref(), status_id.as_deref()]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join(" ");
    let is_effectively_required = !is_optional
        && (is_required
            || try_consume_context::<FormLayoutContext>().is_some_and(|context| {
                context().default_optionality == Some(FormOptionality::Required)
            }));
    let start_icon = start_icon.filter(is_renderable);
    let name = if is_disabled { None } else { html_name };
    rsx! {
        Field { class: format!("astryx-text-input-field {class}"), style, width,
            label, input_id: id, description, description_id, label_tooltip, status, status_variant,
            is_label_hidden, is_required, is_optional, is_disabled,
            div { class: "astryx-text-input", "data-size": size, "data-status": severity.map(InputStatusType::as_str),
                "data-disabled": is_disabled.then_some("true"),
                if let Some(icon) = start_icon {
                    span { class: "astryx-text-input__icon", "aria-hidden": "true", {icon} }
                }
                input { class: "astryx-text-input-control", id: input_id, name, placeholder, autocomplete,
                    r#type: input_type.as_str(), readonly: is_disabled || is_read_only,
                    spellcheck: has_spell_check.map(|enabled| enabled.to_string()), autofocus: has_auto_focus,
                    "data-autofocus": has_auto_focus.then_some("true"),
                    "aria-required": is_effectively_required.then_some("true"), "aria-disabled": is_disabled.then_some("true"),
                    "aria-invalid": (severity == Some(InputStatusType::Error)).then_some("true"),
                    "aria-describedby": (!descriptions.is_empty()).then_some(descriptions),
                    value,
                    onkeydown: move |event| { if is_disabled && event.key() == Key::Enter { event.prevent_default(); } },
                    oninput: move |event| {
                        if !is_disabled && !is_read_only && let Some(handler) = on_change { handler.call(event.value()); }
                    },
                }
            }
        }
    }
}
