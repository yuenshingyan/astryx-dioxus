//! Form field arrangement adapted from Astryx FormLayout.tsx.

use dioxus::prelude::*;

use super::form_layout_context::{
    EntryErrorBehavior, FormLayoutContext, FormLayoutDirection, FormLayoutSettings, FormOptionality,
};

/// Arranges caller-owned fields without creating a form or changing validation.
#[component]
pub fn FormLayout(
    /// Fields and nested layouts.
    #[props(default = rsx! {})]
    children: Element,
    /// Outer field arrangement.
    #[props(default)]
    direction: FormLayoutDirection,
    /// Optionality default for fields that consume FormLayout settings.
    #[props(default)]
    default_optionality: Option<FormOptionality>,
    /// Entry-error presentation default for compatible inputs.
    #[props(default)]
    default_entry_error_behavior: Option<EntryErrorBehavior>,
    /// Classes on the layout root.
    #[props(default)]
    class: String,
    /// Inline CSS on the layout root.
    #[props(default)]
    style: String,
    /// Native root attributes, events, and mounted callback.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let settings = FormLayoutSettings {
        direction,
        default_optionality,
        default_entry_error_behavior,
    };
    let mut context = use_context_provider(|| FormLayoutContext::new(settings));
    if *context.peek() != settings {
        context.set(settings);
    }
    let classes = if class.is_empty() {
        "astryx-form-layout".into()
    } else {
        format!("astryx-form-layout {class}")
    };
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attr| !matches!(attr.name, "class" | "style" | "data-direction"))
        .collect();
    rsx! {
        div { class: classes, "data-direction": direction.as_str(), style: (!style.is_empty()).then_some(style), ..attributes,
            {children}
        }
    }
}

#[cfg(test)]
mod tests {
    use dioxus::prelude::*;
    use dioxus_ssr::render_element;

    use super::FormLayout;
    use crate::form_layout::form_layout_context::{
        EntryErrorBehavior, FormLayoutContext, FormLayoutDirection, FormOptionality,
    };
    use crate::layer::LayerContentBoundary;

    #[component]
    fn SettingsProbe() -> Element {
        let settings = use_context::<FormLayoutContext>()();
        let direction = format!("{:?}", settings.direction);
        let optionality = format!("{:?}", settings.default_optionality);
        let errors = format!("{:?}", settings.default_entry_error_behavior);
        rsx! { output { "{direction};{optionality};{errors}" } }
    }

    #[test]
    fn nested_layout_and_layer_reset_all_defaults() {
        let html = render_element(rsx! { FormLayout {
            direction: FormLayoutDirection::HorizontalLabels, default_optionality: FormOptionality::Required,
            default_entry_error_behavior: EntryErrorBehavior::BlurOrEnter,
            SettingsProbe {}
            FormLayout { direction: FormLayoutDirection::Horizontal, default_optionality: FormOptionality::Optional, SettingsProbe {} }
            LayerContentBoundary { SettingsProbe {} }
            SettingsProbe {}
        } });
        assert_eq!(
            html.matches("HorizontalLabels;Some(Required);Some(BlurOrEnter)")
                .count(),
            2
        );
        assert!(html.contains("Horizontal;Some(Optional);None"));
        assert!(html.contains("Vertical;None;None"));
    }
}
