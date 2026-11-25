use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::SpacingStep;
use crate::hooks::use_overflow::use_overflow;
use crate::layout::padding::spacing_step_css;

/// Item passed to the overflow renderer, with its position in the input list.
#[derive(Clone, PartialEq)]
pub struct OverflowItem {
    pub index: usize,
    pub child: Element,
}

/// Logical edge at which items collapse.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OverflowCollapseFrom {
    Start,
    #[default]
    End,
}

/// Single-row list that collapses items as available width changes.
/// Items and the indicator are also rendered in an inert measurement row.
#[component]
pub fn OverflowList(
    items: Vec<Element>,
    #[props(default = SpacingStep::Two)] gap: SpacingStep,
    #[props(default)] min_visible_items: usize,
    #[props(default)] max_visible_items: Option<usize>,
    #[props(default)] collapse_from: OverflowCollapseFrom,
    #[props(default)] overflow_renderer: Option<Callback<Vec<OverflowItem>, Element>>,
) -> Element {
    let id = use_hook(|| format!("astryx-overflow-list-{}", current_scope_id().0));
    let (measured, onmounted) = use_overflow(id.clone());
    let len = items.len();
    let floor = min_visible_items.min(len);
    let cap = max_visible_items.unwrap_or(len).clamp(floor, len);
    let count = measured().unwrap_or(cap).clamp(floor, cap);
    let (visible, overflow, collapse) = match collapse_from {
        OverflowCollapseFrom::Start => (len - count..len, 0..len - count, "start"),
        OverflowCollapseFrom::End => (0..count, count..len, "end"),
    };
    let render = |indices: std::ops::Range<usize>| {
        overflow_renderer.map(|renderer| {
            renderer.call(
                indices
                    .map(|index| OverflowItem {
                        index,
                        child: items[index].clone(),
                    })
                    .collect(),
            )
        })
    };
    let indicator = (!overflow.is_empty()).then(|| render(overflow)).flatten();
    let measure_indicator = (len > floor).then(|| render(0..len)).flatten();
    let gap_css = spacing_step_css(gap);
    rsx! {
        div { id, class: "astryx-overflow-list", style: "--_overflow-gap:{gap_css}",
            "data-min": floor.to_string(), "data-max": cap.to_string(), "data-collapse": collapse,
            onmounted,
            div { class: "astryx-overflow-list__measure", inert: true, "aria-hidden": "true",
                for (index, child) in items.iter().enumerate() {
                    div { key: "{index}", {child.clone()} }
                }
                if let Some(indicator) = measure_indicator {
                    div { "data-overflow-indicator": "", {indicator} }
                }
            }
            div { class: "astryx-overflow-list__row",
                if collapse_from == OverflowCollapseFrom::Start { {indicator.clone()} }
                for index in visible {
                    div { key: "{index}", {items[index].clone()} }
                }
                if collapse_from == OverflowCollapseFrom::End { {indicator} }
            }
        }
    }
}
