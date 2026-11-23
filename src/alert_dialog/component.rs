use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::{Button, ButtonVariant, Dialog, DialogPurpose, Heading, HeadingLevel, SizeValue};

/// A confirmation dialog that initially focuses its least destructive action.
#[component]
pub fn AlertDialog(
    is_open: bool,
    on_open_change: EventHandler<bool>,
    title: String,
    description: String,
    action_label: String,
    /// Performs the action without implicitly dismissing the dialog.
    on_action: EventHandler<()>,
    #[props(default = "Cancel".to_owned())] cancel_label: String,
    #[props(default = ButtonVariant::Destructive)] action_variant: ButtonVariant,
    #[props(default)] is_action_loading: bool,
    #[props(default)] is_inline: bool,
    #[props(default)] width: Option<SizeValue>,
) -> Element {
    let id = use_hook(|| format!("astryx-alert-dialog-{}", current_scope_id().0));
    let title_id = format!("{id}-title");
    let description_id = format!("{id}-description");
    rsx! {
        Dialog { class: "astryx-alert-dialog", is_open, on_open_change, is_inline, width,
            purpose: DialogPurpose::Form, role: if is_inline { "group" } else { "alertdialog" },
            labelled_by: title_id.clone(), "aria-describedby": description_id.clone(),
            Heading { level: HeadingLevel::Two, id: title_id, "{title}" }
            p { class: "astryx-alert-dialog__description", id: description_id, "{description}" }
            div { class: "astryx-alert-dialog__actions",
                Button { label: cancel_label, variant: ButtonVariant::Ghost,
                    "data-autofocus": (!is_inline).then_some("true"), onclick: move |_| on_open_change.call(false),
                }
                Button { label: action_label, variant: action_variant, is_loading: is_action_loading,
                    onclick: move |_| on_action.call(()),
                }
            }
        }
    }
}
