use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::Element as WebElement;

use astryx_dioxus::{
    Badge, BadgeVariant, Button, ButtonVariant, Card, CardVariant, HStack, Icon, IconButton,
    IconSize, SpacingStep, StackCrossAlignment, StackMainAlignment, StackWrap, Text, TextColor,
    TextType, TextWeight, VStack, VisuallyHidden, VisuallyHiddenRenderProps,
};

// Heroicons v2.2.0, MIT; see THIRD_PARTY_NOTICES.md.
const DOWNLOAD: &str = "M3 16.5v2.25A2.25 2.25 0 0 0 5.25 21h13.5A2.25 2.25 0 0 0 21 18.75V16.5M16.5 12 12 16.5m0 0L7.5 12m4.5 4.5V3";
const SHARE: &str = "M7.217 10.907a2.25 2.25 0 1 0 0 2.186m0-2.186c.18.324.283.696.283 1.093s-.103.77-.283 1.093m0-2.186 9.566-5.314m-9.566 7.5 9.566 5.314m0 0a2.25 2.25 0 1 0 3.935 2.186 2.25 2.25 0 0 0-3.935-2.186Zm0-12.814a2.25 2.25 0 1 0 3.933-2.185 2.25 2.25 0 0 0-3.933 2.185Z";
const DELETE: &str = "m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0";
const SPEAKER: &str = "M13.5 4.06c0-1.336-1.616-2.005-2.56-1.06l-4.5 4.5H4.508c-1.141 0-2.318.664-2.66 1.905A9.76 9.76 0 0 0 1.5 12c0 .898.121 1.768.35 2.595.341 1.24 1.518 1.905 2.659 1.905h1.93l4.5 4.5c.945.945 2.561.276 2.561-1.06V4.06ZM18.584 5.106a.75.75 0 0 1 1.06 0c3.808 3.807 3.808 9.98 0 13.788a.75.75 0 0 1-1.06-1.06 8.25 8.25 0 0 0 0-11.668.75.75 0 0 1 0-1.06Z M15.932 7.757a.75.75 0 0 1 1.061 0 6 6 0 0 1 0 8.486.75.75 0 0 1-1.06-1.061 4.5 4.5 0 0 0 0-6.364.75.75 0 0 1 0-1.06Z";
const UP: &str = "M11.47 2.47a.75.75 0 0 1 1.06 0l7.5 7.5a.75.75 0 1 1-1.06 1.06l-6.22-6.22V21a.75.75 0 0 1-1.5 0V4.81l-6.22 6.22a.75.75 0 1 1-1.06-1.06l7.5-7.5Z";
const DOWN: &str = "M12 2.25a.75.75 0 0 1 .75.75v16.19l6.22-6.22a.75.75 0 1 1 1.06 1.06l-7.5 7.5a.75.75 0 0 1-1.06 0l-7.5-7.5a.75.75 0 1 1 1.06-1.06l6.22 6.22V3a.75.75 0 0 1 .75-.75Z";

fn glyph(path_data: &'static str, is_solid: bool) -> Element {
    rsx! {
        svg { view_box: "0 0 24 24", width: "1em", height: "1em",
            fill: if is_solid { "currentColor" } else { "none" },
            stroke: (!is_solid).then_some("currentColor"), stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round",
            path { d: path_data, fill_rule: is_solid.then_some("evenodd"), clip_rule: is_solid.then_some("evenodd") }
        }
    }
}

fn live_root(props: VisuallyHiddenRenderProps) -> Element {
    rsx! { div { ..props.attributes, {props.children} } }
}

fn is_mounted_result_root(event: &MountedEvent) -> bool {
    #[cfg(target_arch = "wasm32")]
    if let Some(element) = event.data().downcast::<WebElement>() {
        return element.tag_name() == "DIV" && element.id() == "vh-results";
    }
    let _ = event;
    false
}

#[component]
fn BuildFilter() -> Element {
    let mut is_filtered = use_signal(|| false);
    let mut mounts = use_signal(|| 0usize);
    let mut clicks = use_signal(|| 0usize);
    let count = if is_filtered() { 1 } else { 2 };
    rsx! {
        button { id: "vh-filter", r#type: "button", aria_pressed: is_filtered(),
            onclick: move |_| { let next = !is_filtered(); is_filtered.set(next); },
            span { aria_hidden: "true", "✓" }
            VisuallyHidden { "Show passing builds only" }
        }
        ul {
            li { "astryx-core — Passing" }
            if !is_filtered() {
                li { "astryx-charts — Failing" }
            }
        }
        VisuallyHidden { id: "vh-results", as_element: Callback::new(live_root), aria_live: "polite", aria_atomic: "true",
            "data-mounts": mounts(), "data-clicks": clicks(),
            attributes: vec![
                onmounted(move |event| { if is_mounted_result_root(&event) { mounts += 1; } }),
                onclick(move |_| clicks += 1),
            ],
            if count == 1 { "1 build result" } else { "{count} build results" }
        }
    }
}

#[component]
pub(super) fn VisuallyHiddenExamples() -> Element {
    let mut column = use_signal(|| 0usize);
    let current = ["Backlog", "In progress", "Done"][column()];
    rsx! {
        section { id: "visually-hidden-examples", "data-astryx-theme": "neutral",
            h2 { "VisuallyHidden — original patterns" }
            h3 { "Structural heading" }
            VStack { gap: SpacingStep::Three, h_align: StackCrossAlignment::Start,
                Text { text_type: TextType::SUPPORTING, color: TextColor::SECONDARY,
                    "The layout makes this group obvious to sighted users. A hidden heading gives screen-reader users the same landmark to jump to."
                }
                VisuallyHidden { id: "vh-heading", as_element: Callback::new(|props: VisuallyHiddenRenderProps| rsx! { h2 { ..props.attributes, {props.children} } }), "Build status" }
                VStack { gap: SpacingStep::Two,
                    for (name, status, variant) in [("astryx-core", "Passing", BadgeVariant::SUCCESS), ("astryx-charts", "Failing", BadgeVariant::ERROR), ("astryx-cli", "Passing", BadgeVariant::SUCCESS)] {
                        Card { key: "{name}", variant: CardVariant::MUTED, padding: SpacingStep::Three,
                            HStack { gap: SpacingStep::Three, v_align: StackCrossAlignment::Center,
                                Text { "{name}" }
                                Badge { label: status, variant }
                            }
                        }
                    }
                }
            }
            h3 { "Live region" }
            VStack { gap: SpacingStep::Four, h_align: StackCrossAlignment::Start,
                Text { text_type: TextType::SUPPORTING, color: TextColor::SECONDARY, "Drag-and-drop and other visual-only changes are silent to screen readers. A live region narrates them." }
                HStack { gap: SpacingStep::Three, v_align: StackCrossAlignment::Center,
                    Button { id: "vh-move", label: "Move task", variant: ButtonVariant::Secondary, onclick: move |_| { let next = (column() + 1) % 3; column.set(next); } }
                    Text { "Task is in " Text { weight: TextWeight::Bold, "{current}" } }
                }
                VisuallyHidden { id: "vh-live", as_element: Callback::new(live_root), "aria-live": "polite", "Task moved to {current}" }
            }
            h3 { "Supplementary context" }
            HStack { gap: SpacingStep::Four, wrap: StackWrap::Wrap,
                for (label, value, delta, is_up) in [("Revenue", "$48.2k", "+12%", true), ("Churn", "2.1%", "-4%", false)] {
                    Card { key: "{label}", variant: CardVariant::MUTED,
                        VStack { gap: SpacingStep::One,
                            Text { text_type: TextType::SUPPORTING, color: TextColor::SECONDARY, "{label}" }
                            Text { text_type: TextType::DISPLAY_3, "{value}" }
                            HStack { gap: SpacingStep::One, v_align: StackCrossAlignment::Center,
                                Icon { size: IconSize::Small, icon: glyph(if is_up { UP } else { DOWN }, true), color: if is_up { "var(--color-accent)".to_owned() } else { "var(--color-icon-secondary)".to_owned() } }
                                Text { "{delta}" VisuallyHidden { if is_up { " increase from last month" } else { " decrease from last month" } } }
                            }
                        }
                    }
                }
            }
            h3 { "Showcase" }
            VStack { gap: SpacingStep::Five, h_align: StackCrossAlignment::Center,
                HStack { gap: SpacingStep::Six, v_align: StackCrossAlignment::Stretch, h_align: StackMainAlignment::Center, wrap: StackWrap::Wrap,
                    Card { variant: CardVariant::MUTED,
                        VStack { gap: SpacingStep::Four, h_align: StackCrossAlignment::Center,
                            Text { text_type: TextType::SUPPORTING, color: TextColor::SECONDARY, "What you see" }
                            HStack { gap: SpacingStep::Two,
                                for (label, path_data) in [("Download", DOWNLOAD), ("Share", SHARE), ("Delete", DELETE)] {
                                    IconButton { key: "{label}", label, variant: ButtonVariant::Ghost, icon: rsx! { Icon { icon: glyph(path_data, false) } } }
                                }
                            }
                        }
                    }
                    Card { variant: CardVariant::MUTED,
                        VStack { gap: SpacingStep::Four, h_align: StackCrossAlignment::Start,
                            HStack { gap: SpacingStep::Two, v_align: StackCrossAlignment::Center,
                                Icon { size: IconSize::Small, icon: glyph(SPEAKER, true) }
                                Text { text_type: TextType::SUPPORTING, color: TextColor::SECONDARY, "What a screen reader hears" }
                            }
                            VStack { gap: SpacingStep::Two,
                                for label in ["Download", "Share", "Delete"] { Text { key: "{label}", "{label}, button" } }
                            }
                        }
                    }
                }
                VisuallyHidden { id: "vh-actions", as_element: Callback::new(live_root), "aria-live": "polite", "Actions available: Download, Share, Delete." }
            }
            h3 { "Icon-only filter and result count" }
            BuildFilter {}
        }
    }
}
