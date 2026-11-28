use dioxus::prelude::*;

use astryx_dioxus::{ComplexSelector, ComplexSelectorContent, FieldStatusInput, InputStatusType};

type Blend = (&'static str, &'static str);

#[component]
pub(super) fn ComplexSelectorExamples() -> Element {
    let mut value = use_signal(|| ("Apple", "Juicy"));
    let mut is_loading = use_signal(|| false);
    let (fruit, ripeness) = value();
    rsx! {
        section { id: "complex-selector-examples",
            h2 { "Complex Selector" }
            button { id: "complex-selector-toggle-loading", r#type: "button", onclick: move |_| is_loading.toggle(), "Toggle simulated loading" }
            ComplexSelector { label: "Fruit blend", width: Some(280.into()), value: value(), is_loading: is_loading(),
                description: "Choose a fruit and ripeness in one control.",
                trigger_label: rsx! { "{fruit} · {ripeness}" },
                status: (ripeness == "Peak").then(|| FieldStatusInput {
                    status_type: InputStatusType::Warning,
                    message: Some("Use peak fruit today.".into()),
                    message_id: Some("fruit-blend-status".into()),
                }),
                on_change: move |next| value.set(next),
                content: |props: ComplexSelectorContent<Blend>| rsx! {
                    table { "aria-label": "Fruit ripeness choices",
                        thead { tr { th { scope: "col", "Fruit" }
                            for level in ["Crisp", "Tender", "Juicy", "Peak"] { th { scope: "col", "{level}" } }
                        } }
                        tbody {
                            for fruit in ["Apple", "Pear", "Peach", "Plum"] {
                                tr { key: "{fruit}", th { scope: "row", "{fruit}" }
                                    for level in ["Crisp", "Tender", "Juicy", "Peak"] {
                                        td { key: "{level}",
                                            button { r#type: "button", "aria-label": "{fruit}, {level}",
                                                "aria-pressed": (props.value == (fruit, level)).to_string(),
                                                "data-autofocus": (props.value == (fruit, level)).then_some("true"),
                                                onclick: move |_| { props.on_change.call((fruit, level)); props.close.call(()); },
                                                "{level}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
            }
        }
    }
}
