use dioxus::core::{AttributeValue, current_scope_id};
use dioxus::html::PlatformEventData;
use dioxus::prelude::*;

use crate::layout::padding::spacing_step_css;
use crate::{ElementSize, Section, SectionDivider, SectionVariant, SizeProvider, SpacingStep};

/// Arrow-key axis for the toolbar.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ToolbarOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/// Named action group with two or three content lanes.
#[component]
pub fn Toolbar(
    label: String,
    #[props(default)] start_content: Option<Element>,
    #[props(default)] center_content: Option<Element>,
    #[props(default)] end_content: Option<Element>,
    #[props(default = ElementSize::Medium)] size: ElementSize,
    #[props(default = SpacingStep::One)] gap: SpacingStep,
    #[props(default)] orientation: ToolbarOrientation,
    #[props(default)] variant: SectionVariant,
    #[props(default)] dividers: Vec<SectionDivider>,
    #[props(default)] class: String,
    #[props(default)] style: String,
    /// Mounted handle for the outer Section.
    #[props(default)]
    onmounted: Option<EventHandler<MountedEvent>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let id = use_hook(|| current_scope_id().0.to_string());
    let mount_id = id.clone();
    let mut controller = use_signal(|| None::<document::Eval>);
    use_drop(move || {
        if let Some(eval) = *controller.peek() {
            let _ = eval.send(());
        }
    });
    let orientation = match orientation {
        ToolbarOrientation::Horizontal => "horizontal",
        ToolbarOrientation::Vertical => "vertical",
    };
    let has_center = center_content.is_some();
    let row_style = format!("--_toolbar-gap:{};", spacing_step_css(gap));
    let attributes = attributes
        .into_iter()
        .filter(|attr| {
            !matches!(
                attr.name,
                "class"
                    | "style"
                    | "role"
                    | "aria-label"
                    | "aria-orientation"
                    | "data-size"
                    | "data-center"
                    | "data-astryx-toolbar-owner"
            )
        })
        .collect::<Vec<_>>();
    let outer_attributes = onmounted.map(|handler| {
        Attribute::new(
            "onmounted",
            AttributeValue::listener(move |event: Event<PlatformEventData>| {
                handler.call(event.map(|data| MountedData::from(data)));
            }),
            None,
            false,
        )
    });
    rsx! {
        SizeProvider { size,
            Section { variant, dividers, class, style, padding_block: SpacingStep::Two,
                attributes: outer_attributes.into_iter().collect(),
                div { class: "astryx-toolbar", role: "toolbar", "aria-label": label,
                    "aria-orientation": orientation, "data-size": size.as_str(),
                    "data-center": has_center.then_some("true"), "data-astryx-toolbar-owner": id, style: row_style,
                    onmounted: move |_| {
                        if let Some(eval) = *controller.peek() { let _ = eval.send(()); }
                        let eval = document::eval(include_str!("../hooks/use_list_focus.js"));
                        let _ = eval.send(mount_id.clone());
                        controller.set(Some(eval));
                    },
                    ..attributes,
                    if has_center || start_content.is_some() { div { class: "astryx-toolbar__start", {start_content} } }
                    if has_center { div { class: "astryx-toolbar__center", {center_content} } }
                    if has_center || end_content.is_some() { div { class: "astryx-toolbar__end", {end_content} } }
                }
            }
        }
    }
}
