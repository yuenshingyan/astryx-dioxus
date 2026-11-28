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
    use dioxus::core::{Runtime, ScopeId};
    use dioxus::prelude::*;
    use dioxus_ssr::render;

    use super::FormLayout;
    use crate::form_layout::form_layout_context::{
        EntryErrorBehavior, FormLayoutContext, FormLayoutDirection, FormLayoutSettings,
        FormOptionality,
    };
    use crate::layer::LayerContentBoundary;

    #[component]
    fn SettingsProbe(id: &'static str) -> Element {
        let settings = use_context::<FormLayoutContext>()();
        let direction = settings.direction.as_str();
        let optionality = format!("{:?}", settings.default_optionality);
        let errors = format!("{:?}", settings.default_entry_error_behavior);
        rsx! { output { id, "{direction};{optionality};{errors}" } }
    }

    #[derive(Clone, Copy)]
    struct OuterProps(Signal<FormLayoutSettings>);

    fn scoped_layouts() -> Element {
        let outer = use_signal(|| FormLayoutSettings {
            direction: FormLayoutDirection::HorizontalLabels,
            default_optionality: Some(FormOptionality::Required),
            default_entry_error_behavior: Some(EntryErrorBehavior::BlurOrEnter),
        });
        use_context_provider(|| OuterProps(outer));
        let settings = outer();
        rsx! {
            FormLayout {
                direction: settings.direction,
                default_optionality: settings.default_optionality,
                default_entry_error_behavior: settings.default_entry_error_behavior,
                SettingsProbe { id: "outer-before" }
                FormLayout { SettingsProbe { id: "nested-unset" } }
                FormLayout {
                    direction: FormLayoutDirection::Horizontal,
                    default_optionality: FormOptionality::Optional,
                    default_entry_error_behavior: EntryErrorBehavior::Complete,
                    SettingsProbe { id: "nested-explicit" }
                }
                LayerContentBoundary {
                    SettingsProbe { id: "layer-reset" }
                    FormLayout {
                        default_entry_error_behavior: EntryErrorBehavior::BlurOrEnter,
                        SettingsProbe { id: "layer-local" }
                    }
                }
                SettingsProbe { id: "outer-after" }
            }
        }
    }

    fn assert_probe(html: &str, id: &str, settings: &str) {
        assert!(
            html.contains(&format!("<output id=\"{id}\">{settings}</output>")),
            "wrong settings at {id}: {html}"
        );
    }

    #[test]
    fn form_layout_updates_the_nearest_provider_without_leaking_across_scopes() {
        let mut dom = VirtualDom::new(scoped_layouts);
        dom.rebuild_in_place();
        let mut outer = dom.in_runtime(|| {
            Runtime::current()
                .consume_context::<OuterProps>(ScopeId::APP)
                .unwrap()
                .0
        });
        for (settings, expected) in [
            (
                dom.in_runtime(|| *outer.peek()),
                "horizontal-labels;Some(Required);Some(BlurOrEnter)",
            ),
            (
                FormLayoutSettings {
                    direction: FormLayoutDirection::Horizontal,
                    default_optionality: Some(FormOptionality::Optional),
                    default_entry_error_behavior: Some(EntryErrorBehavior::Complete),
                },
                "horizontal;Some(Optional);Some(Complete)",
            ),
            (FormLayoutSettings::default(), "vertical;None;None"),
        ] {
            dom.in_runtime(|| outer.set(settings));
            dom.render_immediate_to_vec();
            // The provider update schedules its consumers after the parent render.
            dom.render_immediate_to_vec();
            let html = render(&dom);
            assert_probe(&html, "outer-before", expected);
            assert_probe(&html, "outer-after", expected);
            assert_probe(&html, "nested-unset", "vertical;None;None");
            assert_probe(
                &html,
                "nested-explicit",
                "horizontal;Some(Optional);Some(Complete)",
            );
            assert_probe(&html, "layer-reset", "vertical;None;None");
            assert_probe(&html, "layer-local", "vertical;None;Some(BlurOrEnter)");
        }
    }
}
