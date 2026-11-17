use dioxus::prelude::*;

/// Visual treatment for a button.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
}

impl ButtonVariant {
    fn class(self) -> &'static str {
        match self {
            Self::Primary => "astryx-button--primary",
            Self::Secondary => "astryx-button--secondary",
        }
    }
}

/// A native button. Defaults to `type="button"` to avoid accidental form submission.
///
/// This starter API and styling are provisional until the original Astryx button is ported.
#[component]
pub fn Button(
    children: Element,
    #[props(default)] variant: ButtonVariant,
    #[props(default)] disabled: bool,
    #[props(default)] class: String,
    #[props(default)] onclick: EventHandler<MouseEvent>,
    #[props(extends = GlobalAttributes)]
    #[props(extends = button)]
    attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        button {
            r#type: "button",
            class: "astryx-button {variant.class()} {class}",
            disabled: disabled,
            onclick: move |event| {
                if !disabled {
                    onclick.call(event);
                }
            },
            ..attributes,
            {children}
        }
    }
}
