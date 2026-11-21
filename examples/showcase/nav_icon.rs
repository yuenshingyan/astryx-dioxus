use dioxus::prelude::*;

use astryx_dioxus::{HStack, Icon, NavIcon, SpacingStep, StackCrossAlignment};

#[component]
fn Glyph(children: Element) -> Element {
    rsx! {
        svg { view_box: "0 0 24 24", width: "1em", height: "1em", fill: "none", stroke: "currentColor",
            stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round", "aria-hidden": "true", {children}
        }
    }
}

#[component]
fn Marks(#[props(default)] has_wrench: bool) -> Element {
    rsx! {
        HStack { gap: SpacingStep::Four, v_align: StackCrossAlignment::Center,
            NavIcon { icon: rsx! { Icon { icon: rsx! { Glyph {
                circle { cx: "11", cy: "11", r: "8" }
                path { d: "M21 21l-4.35-4.35" }
            } } } } }
            NavIcon { icon: rsx! { Icon { icon: rsx! { Glyph {
                rect { x: "3", y: "4", width: "18", height: "18", rx: "2" }
                path { d: "M16 2v4M8 2v4M3 10h18" }
            } } } } }
            if has_wrench {
                NavIcon { icon: rsx! { Icon { icon: rsx! { Glyph {
                    path { d: "M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z" }
                } } } } }
            }
        }
    }
}

#[component]
pub(super) fn NavIconExamples() -> Element {
    rsx! {
        section { id: "nav-icon-examples", "data-astryx-theme": "neutral",
            h2 { "NavIcon" }
            h3 { "Basic" }
            Marks {}
            h3 { "Showcase" }
            Marks { has_wrench: true }
        }
    }
}
