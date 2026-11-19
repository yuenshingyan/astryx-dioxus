use astryx_dioxus::{
    Button, Spinner, SpinnerShade, SpinnerSize, Text, TextColor, TextType, TextWeight,
};
use dioxus::prelude::*;

#[component]
pub fn SpinnerExamples() -> Element {
    let mut is_dark = use_signal(|| false);
    let mut is_shown = use_signal(|| false);
    let mut is_labelled = use_signal(|| false);
    let mut has_extra = use_signal(|| false);
    let mut message = use_signal(|| "Fetching records".to_string());
    let mut mounts = use_signal(|| 0);
    rsx! {
        section {
            id: "spinner-examples", "data-astryx-theme": "neutral",
            style: if is_dark() { "color-scheme:dark;background:#1b1b1b;color:var(--color-text-primary);padding:1.5rem" } else { "color-scheme:light;background:#fff;color:var(--color-text-primary);padding:1.5rem" },
            h2 { "Spinner — original examples" }
            Button { id: "sp-theme", label: "Switch Spinner light/dark", onclick: move |_| is_dark.toggle() }
            h3 { "Spinner" }
            div { style: "display:flex", Spinner { id: "sp-showcase", "data-spinner-named": "", size: SpinnerSize::Large } }
            h3 { "Spinner — On Media Shade" }
            div { style: "display:flex;align-items:center;gap:var(--spacing-4)",
                Spinner { id: "sp-default", "data-spinner-named": "" }
                div { style: "background:#1a1a2e;padding:16px;border-radius:8px",
                    Spinner { id: "sp-media", "data-spinner-named": "", shade: SpinnerShade::OnMedia }
                }
            }
            h3 { "Spinner — Sizes" }
            div { style: "display:flex;align-items:center;gap:var(--spacing-4)",
                for (id, size) in [("sp-sm", SpinnerSize::Small), ("sp-md", SpinnerSize::Medium), ("sp-lg", SpinnerSize::Large), ("sp-xl", SpinnerSize::ExtraLarge)] {
                    Spinner { key: "{id}", id, "data-spinner-named": "", size }
                }
            }
            h3 { "Spinner — With Label" }
            div { style: "display:flex;align-items:flex-start;gap:var(--spacing-8)",
                Spinner { id: "sp-label", "data-spinner-named": "", size: SpinnerSize::Large, label: "Loading..." }
                Spinner {
                    id: "sp-rich", "data-spinner-named": "", size: SpinnerSize::Large, accessible_label: "Fetching data",
                    label_content: rsx! {
                        div { style: "display:flex;flex-direction:column;align-items:center;gap:0",
                            Text { weight: TextWeight::Bold, "Fetching data" }
                            Text { text_type: TextType::SUPPORTING, color: TextColor::SECONDARY, "This may take a moment" }
                        }
                    },
                }
            }
            h3 { "All sizes and shades" }
            div { id: "sp-matrix", style: "display:grid;grid-template-columns:repeat(4,minmax(0,1fr));gap:1rem",
                for (size_name, size) in [("sm", SpinnerSize::Small), ("md", SpinnerSize::Medium), ("lg", SpinnerSize::Large), ("xl", SpinnerSize::ExtraLarge)] {
                    for (shade_name, shade) in [("default", SpinnerShade::Default), ("subtle", SpinnerShade::Subtle), ("onMedia", SpinnerShade::OnMedia), ("inherit", SpinnerShade::Inherit)] {
                        div { key: "{size_name}-{shade_name}", style: if shade == SpinnerShade::OnMedia { "background:#1a1a2e;color:white;padding:8px" } else { "color:var(--color-text-primary);padding:8px" },
                            Spinner { id: "sp-{size_name}-{shade_name}", size, shade }
                            span { " {size_name}/{shade_name}" }
                        }
                    }
                }
            }
            h3 { "Geometry, styling, native attributes, and flex hosts" }
            div { id: "sp-narrow", style: "display:flex;width:40px;gap:4px;border:1px solid var(--color-border)",
                Spinner { id: "sp-no-shrink", size: SpinnerSize::ExtraLarge, style: "min-width:0;width:999px;height:999px;opacity:0.8" }
                span { style: "flex:1 0 100px", "A fixed-width sibling" }
            }
            Spinner { id: "sp-themed", class: "spinner-themed", size: SpinnerSize::ExtraLarge, label: "Themed geometry" }
            Spinner { id: "sp-localized", loading_label: "Chargement" }
            span { id: "sp-external-name", "Imported task" }
            Spinner { id: "sp-labelledby", label: "Visible caption", "aria-labelledby": "sp-external-name" }
            Spinner { id: "sp-legacy", role: "presentation" }
            Spinner { id: "sp-hidden", "aria-hidden": "true" }
            Spinner { id: "sp-decorative", is_decorative: true }
            h3 { "Dynamic appearance, labels, phase, and cleanup" }
            div { class: "showcase-row",
                Button { id: "sp-toggle", label: "Toggle late Spinner", onclick: move |_| is_shown.toggle() }
                Button { id: "sp-toggle-label", label: "Toggle label wrapper", onclick: move |_| is_labelled.toggle() }
                Button { id: "sp-update", label: "Change task text", onclick: move |_| message.set("Refreshing records".to_string()) }
                Button { id: "sp-extra-toggle", label: "Toggle second late Spinner", onclick: move |_| has_extra.toggle() }
            }
            if is_shown() {
                Spinner {
                    id: "sp-late", label: if is_labelled() { Some(message()) } else { None }, loading_label: message(),
                    onmounted: move |_| mounts += 1,
                }
            }
            if has_extra() {
                Spinner { id: "sp-late-extra", label: "Uploading files" }
            }
            p { id: "sp-mounts", "Root mounts: {mounts}" }
        }
    }
}
