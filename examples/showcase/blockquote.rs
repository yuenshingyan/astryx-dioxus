// Named patterns adapted from pinned Meta Astryx templates; see notices.
use astryx_dioxus::{Blockquote, Card, SpacingStep};
use dioxus::events::onclick;
use dioxus::prelude::*;

const DESIGN: &str =
    "Design is not just what it looks like and feels like. Design is how it works.";
const PEOPLE: &str =
    "The people who are crazy enough to think they can change the world are the ones who do.";

#[component]
pub fn BlockquoteExamples() -> Element {
    let mut is_dark = use_signal(|| false);
    let mut is_rtl = use_signal(|| false);
    let mut is_visible = use_signal(|| true);
    let mut has_cite = use_signal(|| true);
    let mut mounts = use_signal(|| 0);
    let mut clicks = use_signal(|| 0);
    rsx! {
        document::Style { "
            .demo-bq-stack {{ display:flex;flex-direction:column;gap:var(--spacing-4);max-width:500px; }}
            #bq-showcase {{ width:500px;max-width:100%; }}
            .demo-bq-grid {{ display:grid;grid-template-columns:repeat(2,1fr);gap:var(--spacing-3);max-width:600px; }}
            .demo-bq-span {{ grid-column:span 2; }}
            #blockquote-examples {{ padding:var(--spacing-4);background:var(--color-background-body);color:var(--color-text-primary);font-size:var(--font-size-base);line-height:var(--text-body-leading); }}
            .demo-bq-override {{ color:rebeccapurple;margin-block:9px;padding-inline-start:23px; }}
        " }
        section {
            id: "blockquote-examples", "data-astryx-theme": "neutral",
            style: if is_dark() { "color-scheme:dark" } else { "color-scheme:light" },
            dir: if is_rtl() { "rtl" } else { "ltr" },
            h2 { "Blockquote" }
            p { "Pinned patterns; native Stack/Grid framing only. Testimonial surfaces use the existing Card." }
            button { id: "bq-theme", onclick: move |_| is_dark.toggle(), "Toggle theme" }
            button { id: "bq-direction", onclick: move |_| is_rtl.toggle(), "Toggle direction" }
            h3 { "Blockquote — Showcase" }
            div { id: "bq-showcase", class: "demo-bq-stack", "data-bq-pattern": "Showcase",
                Blockquote { "{DESIGN}" }
                Blockquote { cite: "Steve Jobs", "{PEOPLE}" }
            }
            h3 { "Blockquote — With Attribution" }
            div { id: "bq-with-cite", class: "demo-bq-stack", "data-bq-pattern": "WithCite",
                Blockquote { "{DESIGN}" }
                Blockquote { cite: "Steve Jobs", "{PEOPLE}" }
            }
            h3 { "Blockquote — Testimonials" }
            div { id: "bq-testimonials", class: "demo-bq-grid", "data-bq-pattern": "Testimonials",
                Card { padding: SpacingStep::Four,
                    Blockquote { cite: "Sarah K., Lead Engineer", "Shipping UI has never been this fast. Our team went from design to production in a single afternoon." }
                }
                Card { padding: SpacingStep::Four,
                    Blockquote { cite: "Marcus T., Product Designer", "The token system means every new screen just looks right — no manual color or spacing decisions." }
                }
                div { class: "demo-bq-span",
                    Card { padding: SpacingStep::Four,
                        Blockquote { cite: "Priya L., Engineering Manager", "Onboarding a new engineer used to take a week of design review. Now they're shipping accessible, polished components on day one." }
                    }
                }
            }
            h3 { "Native content, shallow attribution and caller operations" }
            Blockquote { id: "bq-rich", cite: rsx! { a { href: "#bq-source", "Author and source" } },
                p { "First paragraph" }
                p { "Second paragraph" }
            }
            div { id: "bq-scalars",
                Blockquote { cite: false, "False attribution" }
                Blockquote { cite: true, "True attribution" }
                Blockquote { cite: "", "Empty text attribution" }
                Blockquote { cite: " ", "Whitespace attribution" }
                Blockquote { cite: 0, "Zero attribution" }
                Blockquote { cite: rsx! {}, "Empty rich attribution" }
            }
            Blockquote { id: "bq-long", style: "width:180px", cite: "A source with a notably long attribution line that has to wrap",
                "https://www.example.com/research/2026/design-systems/the-very-long-report-slug/appendix"
            }
            Blockquote { id: "bq-class", class: "demo-bq-override", "Caller class" }
            Blockquote { id: "bq-style", class: "demo-bq-override", style: "padding-inline-start:31px;color:rgb(40,80,120)", "Caller inline style" }
            button { id: "bq-toggle", onclick: move |_| is_visible.toggle(), "Mount/unmount quote" }
            button { id: "bq-cite-toggle", onclick: move |_| has_cite.toggle(), "Toggle attribution" }
            if is_visible() {
                Blockquote {
                    id: "bq-native", tabindex: 0, lang: "fr", "data-caller": "forwarded",
                    "aria-label": "Caller named quote",
                    cite: if has_cite() { "Dynamic author" } else { "" },
                    onmounted: move |_| mounts += 1,
                    attributes: vec![onclick(move |_| clicks += 1)],
                    "Native mounted quote"
                }
            }
            output { id: "bq-events", "Mounts: {mounts}; clicks: {clicks}" }
            span { id: "bq-source", "Caller-owned source destination" }
        }
    }
}
