use std::cell::RefCell;
use std::ops::Range;
use std::rc::Rc;

use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::SpacingStep;
use crate::hooks::use_overflow::use_overflow;
use crate::layout::padding::spacing_step_css;

/// Collapsed item, with its original position in the input list.
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
    render_overflow_list(
        OverflowListProps {
            items,
            gap,
            min_visible_items,
            max_visible_items,
            collapse_from,
            overflow_renderer,
        },
        None,
    )
}

/// Single-row [`OverflowList`] with an independent overflow change callback.
///
/// Reports the collapsed items in input order after measurement. A list that
/// starts out fitting is silent; returning to fit reports an empty vector.
/// Unchanged sets are not repeated. Use stable root keys on dynamic items to
/// distinguish replacements and reorders at the same indices.
#[component]
pub fn OverflowListWithNotifications(
    list: OverflowListProps,
    on_overflow_change: Callback<Vec<OverflowItem>>,
) -> Element {
    render_overflow_list(list, Some(on_overflow_change))
}

#[derive(PartialEq)]
struct MeasurementInputs {
    keys: Vec<Option<String>>,
    gap: SpacingStep,
    floor: usize,
    cap: usize,
    collapse_from: OverflowCollapseFrom,
}

#[derive(Default)]
struct OverflowReport {
    inputs: Option<MeasurementInputs>,
    revision: usize,
    reported: Vec<(usize, Option<String>)>,
}

impl OverflowReport {
    fn revision_for(&mut self, inputs: MeasurementInputs) -> usize {
        if self.inputs.as_ref() != Some(&inputs) {
            self.inputs = Some(inputs);
            self.revision += 1;
        }
        self.revision
    }

    fn record_change(&mut self, measured_revision: usize, overflow: Range<usize>) -> bool {
        if measured_revision != self.revision {
            return false;
        }
        let Some(inputs) = &self.inputs else {
            return false;
        };
        let identities = overflow
            .map(|index| (index, inputs.keys[index].clone()))
            .collect::<Vec<_>>();
        if self.reported == identities {
            return false;
        }
        self.reported = identities;
        true
    }
}

fn render_overflow_list(
    list: OverflowListProps,
    on_overflow_change: Option<Callback<Vec<OverflowItem>>>,
) -> Element {
    let OverflowListProps {
        items,
        gap,
        min_visible_items,
        max_visible_items,
        collapse_from,
        overflow_renderer,
    } = list;
    let id = use_hook(|| format!("astryx-overflow-list-{}", current_scope_id().0));
    let (measured, onmounted) = use_overflow(id.clone());
    let len = items.len();
    let floor = min_visible_items.min(len);
    let cap = max_visible_items.unwrap_or(len).clamp(floor, len);
    let count = measured().map_or(cap, |(count, _)| count.clamp(floor, cap));
    let (visible, overflow, collapse) = match collapse_from {
        OverflowCollapseFrom::Start => (len - count..len, 0..len - count, "start"),
        OverflowCollapseFrom::End => (0..count, count..len, "end"),
    };
    let report = use_hook(|| Rc::new(RefCell::new(OverflowReport::default())));
    // An opaque revision lets the observer acknowledge the exact DOM inputs it
    // measured, even if the visible count did not change.
    let revision = on_overflow_change.map(|_| {
        report.borrow_mut().revision_for(MeasurementInputs {
            keys: items
                .iter()
                .map(|item| item.as_ref().ok().and_then(|node| node.key.clone()))
                .collect(),
            gap,
            floor,
            cap,
            collapse_from,
        })
    });
    let report_items = on_overflow_change.map(|_| collect_items(&items, overflow.clone()));
    let report_range = overflow.clone();
    use_effect(move || {
        let (Some(callback), Some(items), Some(revision)) =
            (on_overflow_change, &report_items, revision)
        else {
            return;
        };
        if measured() != Some((count, revision)) {
            return;
        }
        if report
            .borrow_mut()
            .record_change(revision, report_range.clone())
        {
            callback.call(items.clone());
        }
    });
    let render = |indices: Range<usize>| {
        overflow_renderer.map(|renderer| renderer.call(collect_items(&items, indices)))
    };
    let indicator = (!overflow.is_empty()).then(|| render(overflow)).flatten();
    let measure_indicator = (len > floor).then(|| render(0..len)).flatten();
    let gap_css = spacing_step_css(gap);
    rsx! {
        div { id, class: "astryx-overflow-list", style: "--_overflow-gap:{gap_css}",
            "data-min": floor.to_string(), "data-max": cap.to_string(), "data-collapse": collapse,
            "data-overflow-revision": revision.map(|revision| revision.to_string()),
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

fn collect_items(items: &[Element], indices: Range<usize>) -> Vec<OverflowItem> {
    indices
        .map(|index| OverflowItem {
            index,
            child: items[index].clone(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{MeasurementInputs, OverflowCollapseFrom, OverflowReport, SpacingStep};

    #[test]
    fn overflow_reports_only_changed_sets_for_current_measurement() {
        let mut report = OverflowReport::default();
        let inputs = |keys: [&str; 3], collapse_from| MeasurementInputs {
            keys: keys.map(|key| Some(key.to_owned())).into(),
            gap: SpacingStep::Two,
            floor: 0,
            cap: 3,
            collapse_from,
        };
        let end = OverflowCollapseFrom::End;
        let first = report.revision_for(inputs(["a", "b", "c"], end));
        assert!(!report.record_change(first, 3..3), "initial fit is silent");
        assert!(report.record_change(first, 1..3), "first collapse reports");
        assert_eq!(report.revision_for(inputs(["a", "b", "c"], end)), first);
        assert!(!report.record_change(first, 1..3), "same set is silent");

        let reordered = report.revision_for(inputs(["a", "c", "b"], end));
        assert!(!report.record_change(first, 1..3), "old result is stale");
        assert!(
            report.record_change(reordered, 1..3),
            "same-count reorder reports"
        );
        let from_start = report.revision_for(inputs(["a", "c", "b"], OverflowCollapseFrom::Start));
        assert!(
            report.record_change(from_start, 0..2),
            "same-count edge change reports"
        );
        assert!(
            report.record_change(from_start, 0..0),
            "grow-back reports empty"
        );
        assert!(
            !report.record_change(from_start, 0..0),
            "still fitting is silent"
        );
    }
}
