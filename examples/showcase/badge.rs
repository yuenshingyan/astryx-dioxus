use astryx_dioxus::{
    Badge, BadgeLabel, BadgeVariant, Button, Icon, IconDefaultSizeProvider, IconSize, Text,
    TextType,
};
use dioxus::events::onclick;
use dioxus::prelude::*;

const VARIANTS: [(BadgeVariant, &str); 14] = [
    (BadgeVariant::NEUTRAL, "Neutral"),
    (BadgeVariant::INFO, "Info"),
    (BadgeVariant::SUCCESS, "Success"),
    (BadgeVariant::WARNING, "Warning"),
    (BadgeVariant::ERROR, "Error"),
    (BadgeVariant::BLUE, "Blue"),
    (BadgeVariant::CYAN, "Cyan"),
    (BadgeVariant::GREEN, "Green"),
    (BadgeVariant::ORANGE, "Orange"),
    (BadgeVariant::PINK, "Pink"),
    (BadgeVariant::PURPLE, "Purple"),
    (BadgeVariant::RED, "Red"),
    (BadgeVariant::TEAL, "Teal"),
    (BadgeVariant::YELLOW, "Yellow"),
];

#[component]
fn NamedBadge(label: String, variant: BadgeVariant) -> Element {
    rsx! { Badge { label, variant, "data-badge-named": "" } }
}

#[component]
pub fn BadgeExamples() -> Element {
    let mut is_dark = use_signal(|| false);
    let mut has_updated = use_signal(|| false);
    let mut events = use_signal(|| 0);
    let custom = BadgeVariant::new("premium")?;
    rsx! {
        section {
            id: "badge-examples", "data-astryx-theme": "neutral",
            style: if is_dark() { "color-scheme:dark;background:#1b1b1b;color:var(--color-text-primary);padding:1.5rem" } else { "color-scheme:light;background:#fff;color:var(--color-text-primary);padding:1.5rem" },
            h2 { "Badge — original examples" }
            Button { id: "bd-theme", label: "Switch Badge light/dark", onclick: move |_| is_dark.toggle() }
            h3 { "Badge — Variants" }
            div { style: "display:flex;flex-direction:column;align-items:center;gap:var(--spacing-4)",
                div { style: "display:flex;gap:var(--spacing-2)",
                    for (variant,label) in VARIANTS.iter().take(5) { NamedBadge { key: "{label}", label: *label, variant: variant.clone() } }
                }
                div { style: "display:flex;gap:var(--spacing-2)",
                    for (variant,label) in [(BadgeVariant::BLUE,"Blue"),(BadgeVariant::PURPLE,"Purple"),(BadgeVariant::PINK,"Pink"),(BadgeVariant::TEAL,"Teal"),(BadgeVariant::ORANGE,"Orange")] { NamedBadge { key: "{label}", label, variant } }
                }
            }
            h3 { "Badge — Colors" }
            div { style: "display:flex;flex-direction:column;gap:var(--spacing-6)",
                div { style: "display:flex;flex-direction:column;gap:var(--spacing-2)",
                    Text { text_type: TextType::SUPPORTING, "Teams" }
                    div { style: "display:flex;flex-wrap:wrap;gap:var(--spacing-2)",
                        for (variant,label) in [(BadgeVariant::BLUE,"Design"),(BadgeVariant::CYAN,"DevOps"),(BadgeVariant::GREEN,"Backend"),(BadgeVariant::PINK,"Marketing"),(BadgeVariant::PURPLE,"Engineering"),(BadgeVariant::TEAL,"Research")] { NamedBadge { key: "{label}", label, variant } }
                    }
                }
                div { style: "display:flex;flex-direction:column;gap:var(--spacing-2)",
                    Text { text_type: TextType::SUPPORTING, "Priority" }
                    div { style: "display:flex;gap:var(--spacing-2)",
                        for (variant,label) in [(BadgeVariant::ORANGE,"Urgent"),(BadgeVariant::RED,"Critical"),(BadgeVariant::YELLOW,"Review")] { NamedBadge { key: "{label}", label, variant } }
                    }
                }
            }
            h3 { "Badge — Counts" }
            div { style: "display:flex;justify-content:center;align-items:center;gap:var(--spacing-8)",
                for (variant,label,note) in [(BadgeVariant::INFO,"3","Messages"),(BadgeVariant::ERROR,"99+","Alerts"),(BadgeVariant::SUCCESS,"12","Completed"),(BadgeVariant::WARNING,"5","Pending")] {
                    div { key: "{note}", style: "display:flex;flex-direction:column;align-items:center;gap:var(--spacing-2)",
                        NamedBadge { label, variant }
                        Text { text_type: TextType::SUPPORTING, "{note}" }
                    }
                }
            }
            h3 { "Badge — Status" }
            div { style: "display:flex;flex-direction:column;gap:var(--spacing-6)",
                div { style: "display:flex;flex-direction:column;gap:var(--spacing-2)",
                    Text { text_type: TextType::SUPPORTING, "System status" }
                    div { style: "display:flex;align-items:center;gap:var(--spacing-2)",
                        for (variant,label) in [(BadgeVariant::SUCCESS,"Active"),(BadgeVariant::WARNING,"Pending"),(BadgeVariant::ERROR,"Failed")] { NamedBadge { key: "{label}", label, variant } }
                    }
                }
                div { style: "display:flex;flex-direction:column;gap:var(--spacing-2)",
                    Text { text_type: TextType::SUPPORTING, "Workflow" }
                    div { style: "display:flex;align-items:center;gap:var(--spacing-2)",
                        NamedBadge { variant: BadgeVariant::NEUTRAL, label: "Draft" }
                        NamedBadge { variant: BadgeVariant::INFO, label: "In Review" }
                    }
                }
            }
            h3 { "All 14 built-ins and a custom theme value" }
            div { id: "bd-all", style: "display:flex;flex-wrap:wrap;gap:var(--spacing-2)",
                for (variant,label) in VARIANTS { Badge { key: "{label}", label, variant } }
            }
            Badge { id: "bd-custom", label: "Premium", variant: custom, style: "color:rgb(10,60,100)" }
            h3 { "Readonly content, titles, native events and icons" }
            Badge { id: "bd-number", label: 42 }
            Badge { id: "bd-zero", label: 0 }
            Badge { id: "bd-rich", label: BadgeLabel::Rich(rsx! { strong { "Rich " } em { "content" } }) }
            Badge { id: "bd-empty", label: BadgeLabel::Empty }
            Badge { id: "bd-empty-string", label: "" }
            Badge { id: "bd-title", label: "Visible", title: "Caller title" }
            Badge { id: "bd-icon", label: "Verified", icon: rsx! { Icon { size: IconSize::ExtraSmall, icon: rsx! { svg { view_box: "0 0 16 16", path { d: "M3 8l3 3 7-7", fill: "none", stroke: "currentColor", stroke_width: "2" } } } } } }
            IconDefaultSizeProvider { size: IconSize::Large,
                Badge { id: "bd-icon-context", label: "Caller-sized", icon: rsx! { Icon { label: "Caller glyph", icon: rsx! { svg { view_box: "0 0 16 16", circle { cx: "8", cy: "8", r: "6", fill: "currentColor" } } } } } }
            }
            Badge { id: "bd-native", label: "Native props", lang: "fr", "aria-describedby": "bd-note", onmounted: move |_| events += 1, attributes: vec![onclick(move |_| events += 1)] }
            p { id: "bd-note", "A static badge is not an action or a live region." }
            p { id: "bd-events", "Native events: {events}" }
            Button { id: "bd-update", label: "Update static badge", onclick: move |_| has_updated.toggle() }
            Badge { id: "bd-dynamic", label: if has_updated() { "Updated state" } else { "Initial state" } }
            h3 { "Content width, narrow rows and larger text" }
            div { id: "bd-grid", style: "display:grid;grid-template-columns:1fr;width:300px", Badge { label: "Grid label" } }
            div { id: "bd-stack", style: "display:flex;flex-direction:column;width:300px", Badge { label: "Stack label" } }
            div { id: "bd-row", style: "display:flex;width:180px;gap:8px", span { style: "flex:0 0 90px", "Fixed sibling" } Badge { id: "bd-long", variant: BadgeVariant::BLUE, label: "A very long label whose full tail remains available in the DOM" } }
            div { id: "bd-narrow", style: "width:18px", Badge { label: "Long" } }
            div { id: "bd-larger", style: "--text-supporting-size:32px",
                Badge { id: "bd-large-text", label: "Larger" }
                Badge { id: "bd-large-empty", label: BadgeLabel::Empty }
            }
            h3 { "RTL characters stay in order" }
            for direction in ["rtl","ltr"] {
                div { id: "bd-{direction}", dir: direction, style: "display:flex;flex-wrap:wrap;gap:8px",
                    for label in ["https://",".com","+3","@alice","iPhone جديد","example.com موقع"] { Badge { key: "{label}", label } }
                    Badge { label: BadgeLabel::Rich(rsx! { bdi { dir: "rtl", "שלום" } }) }
                }
            }
        }
    }
}
