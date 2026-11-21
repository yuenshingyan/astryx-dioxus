use dioxus::prelude::*;

use crate::{
    Button, ButtonVariant, ElementSize, SectionDivider, SectionVariant, SpacingStep, Toolbar,
    ToolbarOrientation,
};

/// Caller-owned selection summary. The clear action must clear the entire selection.
#[derive(Clone, PartialEq)]
pub struct TableSelectionSummary {
    pub selected_count: usize,
    pub clear_selection: EventHandler<()>,
}

/// Bulk actions displayed only while rows are selected.
#[component]
pub fn TableSelectionToolbar(
    selection: TableSelectionSummary,
    #[props(default = "Bulk actions".to_owned())] label: String,
    #[props(default = "Unselect All".to_owned())] clear_label: String,
    #[props(default)] render_selection_label: Option<Callback<usize, Element>>,
    #[props(default)] start_content: Option<Element>,
    #[props(default)] center_content: Option<Element>,
    #[props(default = ElementSize::Small)] size: ElementSize,
    #[props(default = SpacingStep::One)] gap: SpacingStep,
    #[props(default)] orientation: ToolbarOrientation,
    #[props(default = SectionVariant::MUTED)] variant: SectionVariant,
    #[props(default)] dividers: Vec<SectionDivider>,
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(default)] onmounted: Option<EventHandler<MountedEvent>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let count = selection.selected_count;
    if count == 0 {
        return rsx! {};
    }
    let count_label = match render_selection_label {
        Some(render) => render.call(count),
        None => rsx! { "{count} selected" },
    };
    rsx! {
        Toolbar { label, size, gap, orientation, variant, dividers, class, style, onmounted, attributes,
            start_content, center_content,
            end_content: rsx! {
                span { {count_label} }
                Button { label: clear_label, variant: ButtonVariant::Ghost,
                    onclick: move |_| selection.clear_selection.call(()),
                }
            },
        }
    }
}
