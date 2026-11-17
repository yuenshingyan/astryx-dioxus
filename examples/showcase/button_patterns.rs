use astryx_dioxus::{Button, ButtonElevation, ButtonSize, ButtonVariant, Icon};
use dioxus::prelude::*;

#[derive(Clone, Copy)]
enum Glyph {
    Add,
    Edit,
    Download,
    Delete,
}

fn glyph(kind: Glyph) -> Element {
    let path = match kind {
        Glyph::Add => "M12 5v14M5 12h14",
        Glyph::Edit => "M15 5l4 4M4 20l1-5L16 4l4 4L9 19zM4 20h16",
        Glyph::Download => "M12 3v12m-5-5l5 5 5-5M4 16v5h16v-5",
        Glyph::Delete => "M4 7h16M9 7V3h6v4M6 7l1 14h10l1-14M10 11v6m4-6v6",
    };
    rsx! { svg { view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round", path { d: path } } }
}

#[component]
pub fn DocumentedButtonPatterns() -> Element {
    let variants = [
        (ButtonVariant::Primary, "Primary", "primary"),
        (ButtonVariant::Secondary, "Secondary", "secondary"),
        (ButtonVariant::Ghost, "Ghost", "ghost"),
        (ButtonVariant::Destructive, "Destructive", "destructive"),
    ];
    let sizes = [
        (ButtonSize::Small, "Small"),
        (ButtonSize::Medium, "Medium"),
        (ButtonSize::Large, "Large"),
    ];
    let mut is_dark = use_signal(|| false);
    rsx! {
        section { id: "documented-patterns", "data-astryx-theme": "neutral", style: if is_dark() { "color-scheme:dark;background:#1b1b1b;color:white;padding:24px;border-radius:12px" } else { "color-scheme:light;background:white;color:black;padding:24px;border-radius:12px" },
            h2 { "Original Button examples — Neutral theme" }
            Button { id: "preview-mode", label: "Switch preview light/dark", onclick: move |_| is_dark.toggle() }
            h3 { "Button — Variants" }
            div { id: "neutral-samples", class: "showcase-row",
                {variants.iter().map(|(variant, label, id)| rsx! {
                    Button { id: "neutral-{id}", label: *label, variant: *variant }
                })}
            }
            h3 { "Disabled" }
            div { class: "showcase-row",
                {variants.iter().map(|(variant, label, _)| rsx! { Button { label: *label, variant: *variant, is_disabled: true } })}
            }
            h3 { "Loading" }
            div { class: "showcase-row",
                {variants.iter().map(|(variant, label, _)| rsx! { Button { label: *label, variant: *variant, is_loading: true } })}
            }
            h3 { "Button — Sizes" }
            {variants[..2].iter().map(|(variant, _, _)| rsx! {
                div { class: "showcase-row", style: "margin-block:8px;",
                    {sizes.iter().map(|(size, label)| rsx! { Button { label: *label, size: *size, variant: *variant } })}
                }
            })}
            h3 { "Button — Floating" }
            div { class: "showcase-row",
                {variants.iter().map(|(variant, label, _)| rsx! {
                    Button { label: *label, variant: *variant, elevation: ButtonElevation::Medium, icon: rsx! { Icon { icon: glyph(Glyph::Add) } } }
                })}
            }
            h3 { "Button — Icon" }
            div { class: "showcase-row",
                Button { label: "New item", variant: ButtonVariant::Primary, icon: rsx! { Icon { icon: glyph(Glyph::Add) } } }
                Button { label: "Edit", variant: ButtonVariant::Secondary, icon: rsx! { Icon { icon: glyph(Glyph::Edit) } } }
                Button { label: "Download", variant: ButtonVariant::Ghost, icon: rsx! { Icon { icon: glyph(Glyph::Download) } } }
                Button { label: "Delete", variant: ButtonVariant::Destructive, icon: rsx! { Icon { icon: glyph(Glyph::Delete) } } }
            }
            h3 { "Button — End Slot" }
            p { "Caller-supplied count/status content; not a completed standalone Badge port." }
            div { class: "showcase-row",
                Button { label: "Messages", variant: ButtonVariant::Primary, end_content: rsx! { span { class: "demo-count demo-count--info", "3" } } }
                Button { label: "Notifications", variant: ButtonVariant::Secondary, end_content: rsx! { span { class: "demo-count demo-count--warning", "12" } } }
                Button { label: "Updates", variant: ButtonVariant::Ghost, end_content: rsx! { span { class: "demo-count", "New" } } }
            }
            h3 { "Actions in narrow rows" }
            div { style: "width:min(320px,100%);border:1px solid #919191;border-radius:12px;padding:12px;box-sizing:border-box;",
                h4 { "Design review" }
                div { class: "showcase-row", style: "flex-wrap:nowrap;",
                    Button { label: "Request review from the design team", size: ButtonSize::Small }
                    Button { label: "More review options", is_icon_only: true, size: ButtonSize::Small, icon: rsx! { span { "aria-hidden": "true", "…" } } }
                }
                p { "Reviewer emails" }
                input { r#type: "email", "aria-label": "Reviewer emails", placeholder: "name@example.com", style: "max-width:100%;box-sizing:border-box;" }
                div { class: "showcase-row", style: "flex-wrap:nowrap;margin-top:12px;",
                    Button { label: "Cancel", style: "flex-shrink:0" }
                    Button { label: "Send invitations to all 12 reviewers", variant: ButtonVariant::Primary }
                }
            }
        }
    }
}
