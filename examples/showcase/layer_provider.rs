use dioxus::prelude::*;

use astryx_dioxus::{Button, LayerInset, LayerProvider, Popover, PopoverTriggerRenderProps};

#[component]
pub(super) fn LayerProviderExamples() -> Element {
    let mut reserve = use_signal(|| true);
    let inset = reserve().then(|| LayerInset {
        block_end: Some(56.into()),
        inline_end: Some(80.into()),
        ..LayerInset::default()
    });
    rsx! {
        section { id: "layer-provider-examples",
            h2 { "Layer Provider" }
            Button { label: "Toggle viewport reservation", onclick: move |_| reserve.toggle() }
            LayerProvider { inset,
                LayerProvider {
                    InsetPreview { label: "Inherited inset" }
                }
                LayerProvider { inset: LayerInset { inline_start: Some(24.into()), ..LayerInset::default() },
                    InsetPreview { label: "Local inset" }
                }
                LayerProvider { inset: LayerInset::default(),
                    InsetPreview { label: "Cleared inset" }
                }
            }
            InsetPreview { label: "No provider" }
        }
    }
}

#[component]
fn InsetPreview(label: String) -> Element {
    let trigger_label = label.clone();
    rsx! {
        Popover { label, width: Some("100vi".into()),
            trigger: move |props: PopoverTriggerRenderProps| rsx! {
                Button { id: props.anchor_id, label: trigger_label.clone(), onclick: props.onclick,
                    "aria-haspopup": "dialog", "aria-controls": props.controls,
                    "aria-expanded": props.is_expanded.to_string(),
                }
            },
            content: rsx! { p { "This popover uses the nearest provider’s viewport budget." } },
        }
    }
}
