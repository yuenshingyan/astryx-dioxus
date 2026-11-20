use std::borrow::Cow;
use std::error::Error;
use std::fmt::{Display, Formatter};

use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use super::use_field_status_announcement::use_field_status_announcement;
use crate::icon::global_icon_registry::{IconRequest, resolve_icon};
use crate::icon::{IconSize, render_icon};

/// Status shared by field controls and their validation messages.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputStatusType {
    /// A blocking validation error.
    Error,
    /// A warning that does not prevent completion.
    Warning,
    /// A successful validation result.
    Success,
}

impl InputStatusType {
    fn as_str(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Success => "success",
        }
    }
}

/// Theme-extensible message presentation identifier.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldStatusVariant(Cow<'static, str>);

impl FieldStatusVariant {
    /// Message overlaps a bordered control; the control owns the glyph.
    pub const ATTACHED: Self = Self(Cow::Borrowed("attached"));
    /// Separate message with a leading decorative glyph.
    pub const DETACHED: Self = Self(Cow::Borrowed("detached"));
    /// Legacy plain message box. Tooltip presentation belongs to the field control.
    #[deprecated(
        note = "FieldStatus does not render tooltips; use the field control's tooltip presentation"
    )]
    pub const TOOLTIP: Self = Self(Cow::Borrowed("tooltip"));

    /// Validates a custom theme identifier without altering its spelling.
    pub fn new(name: impl Into<String>) -> Result<Self, InvalidFieldStatusVariant> {
        let name = name.into();
        if name.is_empty()
            || !name
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
        {
            return Err(InvalidFieldStatusVariant);
        }
        Ok(Self(Cow::Owned(name)))
    }

    /// Returns the theme selector value.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for FieldStatusVariant {
    fn default() -> Self {
        Self::ATTACHED
    }
}

/// Invalid FieldStatus presentation identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvalidFieldStatusVariant;

impl Display for InvalidFieldStatusVariant {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("FieldStatus variant must be a nonempty identifier containing only ASCII letters, digits, hyphens, or underscores")
    }
}

impl Error for InvalidFieldStatusVariant {}

fn status_icon(status: InputStatusType) -> Element {
    let path = match status {
        InputStatusType::Success => {
            "M12 3a9 9 0 100 18 9 9 0 000-18zm4.06 6.56a.75.75 0 00-1.12-1l-3.94 4.4-1.94-1.94a.75.75 0 00-1.06 1.06l2.5 2.5a.75.75 0 001.09-.03l4.47-5z"
        }
        InputStatusType::Warning => {
            "M10.29 3.86L2.07 19.05A2 2 0 003.78 22h16.44a2 2 0 001.71-2.95L13.71 3.86a2 2 0 00-3.42 0zM12 9a.75.75 0 01.75.75v4.5a.75.75 0 01-1.5 0v-4.5A.75.75 0 0112 9zm0 9a1 1 0 100-2 1 1 0 000 2z"
        }
        InputStatusType::Error => {
            "M12 3a9 9 0 100 18 9 9 0 000-18zm-2.47 5.47a.75.75 0 00-1.06 1.06L10.94 12l-2.47 2.47a.75.75 0 101.06 1.06L12 13.06l2.47 2.47a.75.75 0 101.06-1.06L13.06 12l2.47-2.47a.75.75 0 00-1.06-1.06L12 10.94l-2.47-2.47z"
        }
    };
    let fallback = rsx! {
        svg { class: "astryx-field-status__core-glyph", view_box: "0 0 24 24", fill: "currentColor",
            path { fill_rule: "evenodd", clip_rule: "evenodd", d: path }
        }
        svg { class: "astryx-field-status__neutral-glyph", view_box: "0 0 24 24", fill: "none",
            stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
            match status {
                InputStatusType::Error => rsx! { circle { cx: "12", cy: "12", r: "10" } path { d: "m15 9-6 6m0-6 6 6" } },
                InputStatusType::Warning => rsx! { path { d: "m21.73 18-8-14a2 2 0 0 0-3.46 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3ZM12 9v4m0 4h.01" } },
                InputStatusType::Success => rsx! { circle { cx: "12", cy: "12", r: "10" } path { d: "m9 12 2 2 4-4" } },
            }
        }
    };
    resolve_icon(
        status.as_str(),
        IconRequest {
            size: IconSize::Small,
            variant: None,
        },
        fallback,
    )
}

/// A visible validation message with a separate persistent announcement channel.
#[component]
pub fn FieldStatus(
    /// Validation severity.
    r#type: InputStatusType,
    /// Localized message that describes the status in words.
    message: String,
    /// Message presentation, attached by default.
    #[props(default)]
    variant: FieldStatusVariant,
    /// Caller classes on the message box.
    #[props(default)]
    class: String,
    /// Caller inline declarations on the message box.
    #[props(default)]
    style: String,
    /// Native root attributes, including `id` for description associations.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    const MARKER: &str = "data-astryx-field-status";
    let marker = format!("field-status-{}", current_scope_id().0);
    let status = r#type;
    use_field_status_announcement(marker.clone(), message.clone(), status);
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attr| {
            !matches!(
                attr.name,
                "class" | "style" | MARKER | "data-type" | "data-variant"
            )
        })
        .collect();
    rsx! {
        div {
            class: "astryx-field-status {class}", style: (!style.is_empty()).then_some(style),
            "data-type": status.as_str(), "data-variant": variant.as_str(),
            "data-astryx-field-status": marker, ..attributes,
            if variant == FieldStatusVariant::DETACHED {
                span { class: "astryx-field-status__content",
                    span { class: "astryx-field-status__lead",
                        {render_icon(status_icon(status), IconSize::Small, None, None, "astryx-field-status-icon".into(), String::new(), vec![Attribute::new("data-type", status.as_str(), None, false)])}
                    }
                    span { {message} }
                }
            } else { {message} }
        }
    }
}
