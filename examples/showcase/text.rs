// Text-only patterns from the pinned CLI templates. Native frames stand in for
// Stack; this does not claim a public Stack, Table or Tooltip implementation.
use astryx_dioxus::{
    LayerPlacement, Link, Text, TextColor, TextDisplay, TextJustify, TextSize, TextTag, TextType,
    TextWeight, TextWrap, WordBreak, use_truncation,
};
use dioxus::prelude::*;

const LONG_TEXT: &str = "The design system provides a consistent set of typography tokens, spacing scales, and color palettes that ensure every surface in the product feels cohesive regardless of which team built it.";
const VERTICAL: &str = "display:flex;flex-direction:column;gap:var(--spacing-3)";

#[component]
fn HookCell(id: String, text: String, max_lines: u32) -> Element {
    let truncation = use_truncation(id.clone(), max_lines, false);
    let is_truncated = (truncation.is_truncated)();
    let full_text = (truncation.full_text)();
    let style = if max_lines == 1 {
        "display:block;max-width:120px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap"
            .to_string()
    } else {
        "display:-webkit-box;-webkit-line-clamp:2;-webkit-box-orient:vertical;overflow:hidden;max-width:120px;word-break:break-all".to_string()
    };
    rsx! {
        span {
            id: id.clone(), "data-astryx-truncation-id": id,
            onmounted: truncation.onmounted,
            tabindex: is_truncated.then_some(0), style,
            "{text}"
        }
        output { "data-hook-state": "{is_truncated}", "{full_text}" }
    }
}

#[component]
pub fn TextExamples() -> Element {
    let mut is_dark = use_signal(|| false);
    let mut has_tooltip = use_signal(|| true);
    let mut lines = use_signal(|| 2_u32);
    let mut is_present = use_signal(|| true);
    let mut native_events = use_signal(|| 0_u32);
    let mut content = use_signal(|| LONG_TEXT.to_string());
    rsx! {
        section {
            id: "text-examples", style: if is_dark() { "color-scheme:dark;background:#1f1f22;color:#dfe2e5;padding:24px" } else { "color-scheme:light;background:white;color:#0a1317;padding:24px" },
            h2 { "Text — pinned patterns" }
            button { id: "tx-theme", onclick: move |_| is_dark.toggle(), "Light / dark" }
            div { id: "tx-showcase", style: "display:flex;flex-direction:column;gap:var(--spacing-2)",
                for (text_type, text) in [
                    (TextType::BODY, "Body: The bulk of content"),
                    (TextType::LARGE, "Large: Emphasized content"),
                    (TextType::LABEL, "Label: Form and chart labels"),
                    (TextType::SUPPORTING, "Supporting: Helper text"),
                    (TextType::CODE, "Code: const x = 42;"),
                ] {
                    Text { text_type, "data-text-pattern": "showcase", "{text}" }
                }
            }
            div { id: "tx-types", style: VERTICAL,
                for (text_type, label, sample, has_strikethrough, has_tabular_numbers) in [
                    (TextType::BODY, "Body text", "Body text for paragraphs and general content", false, false),
                    (TextType::LARGE, "Large text", "Large text for introductions and callouts", false, false),
                    (TextType::LABEL, "Label text", "Label text for form fields and section titles", false, false),
                    (TextType::SUPPORTING, "Supporting text", "Supporting text for captions and metadata", false, false),
                    (TextType::CODE, "Code text", "const theme = defineTheme({})", false, false),
                    (TextType::BODY, "Strikethrough", "Body text with strikethrough decoration", true, false),
                    (TextType::BODY, "Tabular numbers", "1,234.56  78.90  100,000.00", false, true),
                ] {
                    div { style: "display:flex;flex-direction:column;gap:0",
                        Text { text_type: TextType::SUPPORTING, color: TextColor::SECONDARY, "{label}" }
                        Text { text_type, display: TextDisplay::Block, has_strikethrough, has_tabular_numbers, "data-text-pattern": "types", "{sample}" }
                    }
                }
            }
            div { id: "tx-colors", style: VERTICAL,
                for (color, text) in [
                    (TextColor::PRIMARY, "Primary — Default for headings and body text"),
                    (TextColor::SECONDARY, "Secondary — Supporting details and metadata"),
                    (TextColor::ACCENT, "Accent — Links, emphasis, and accent-colored text"),
                    (TextColor::DISABLED, "Disabled — Unavailable or inactive content"),
                    (TextColor::PLACEHOLDER, "Placeholder — Empty field hints"),
                ] { Text { color, "data-text-pattern": "colors", "{text}" } }
            }
            div { id: "tx-weight", style: VERTICAL,
                for (weight, label) in [(TextWeight::Normal,"Normal"),(TextWeight::Medium,"Medium"),(TextWeight::Semibold,"Semibold"),(TextWeight::Bold,"Bold")] {
                    Text { weight, display: TextDisplay::Block, "data-text-pattern": "weight", "{label}" }
                }
            }
            div { id: "tx-inline",
                Text { display: TextDisplay::Block, "data-text-pattern": "inline",
                    "Design tokens are " Text { text_type: TextType::CODE, "themeable" } " and shared across every surface."
                }
            }
            div { id: "tx-truncation", style: "display:flex;flex-direction:column;gap:16px;max-width:300px",
                for max_lines in [1_u32,2,3] {
                    div {
                        Text { text_type: TextType::SUPPORTING, color: TextColor::SECONDARY, display: TextDisplay::Block, if max_lines == 1 { "1 line" } else { "{max_lines} lines" } }
                        div { style: "border:1px solid var(--color-border);padding:8px",
                            Text { max_lines, "data-text-pattern": "truncation", "{LONG_TEXT}" }
                        }
                    }
                }
            }
            div { id: "tx-wrap", style: "display:flex;flex-direction:column;gap:16px;max-width:400px",
                for (text_wrap,label,sample) in [
                    (TextWrap::Wrap,"Wrap (default)","This text wraps normally at word boundaries when it reaches the edge."),
                    (TextWrap::NoWrap,"Nowrap","This text does not wrap and will overflow its container."),
                    (TextWrap::Balance,"Balance","This text is balanced for better visual appearance across lines."),
                    (TextWrap::Pretty,"Pretty","This text uses pretty wrap to avoid orphans at the end of paragraphs."),
                ] {
                    div {
                        Text { text_type: TextType::LABEL, display: TextDisplay::Block, "{label}" }
                        div { style: if text_wrap == TextWrap::NoWrap { "border:1px solid var(--color-border);padding:8px;width:200px;overflow:hidden" } else { "border:1px solid var(--color-border);padding:8px;width:200px" },
                            Text { text_wrap, "data-text-pattern": "wrap", "{sample}" }
                        }
                    }
                }
            }
            div { id: "tx-word-break", style: "display:flex;flex-direction:column;gap:16px;max-width:400px",
                for (word_break,label,sample) in [
                    (WordBreak::BreakWord,"Break-word (default for multi-line)","This is a verylongunbreakableword for a break-word example"),
                    (WordBreak::BreakAll,"Break-all (default for single-line)","Breaks anywhere: abcdefghijklmnopqrstuvwxyz0123456789"),
                ] {
                    div {
                        Text { text_type: TextType::LABEL, display: TextDisplay::Block, "{label}" }
                        div { style: "width:200px;border:1px solid var(--color-border);padding:8px",
                            Text { max_lines: 2, word_break, "data-text-pattern": "word-break", "{sample}" }
                        }
                    }
                }
            }
            h3 { "Native table hook adapter (Tooltip/Table ports not claimed)" }
            table { id: "tx-hook", thead { tr { th { "Asset" } th { "Path (measured full content below)" } } }
                tbody {
                    for (id,name,text,max_lines) in [
                        ("tx-hook-build","build","artifacts/release/build-20261103-120344/index.html",1),
                        ("tx-hook-bundle","bundle","reports/coverage/packages/core/src/components/Calendar/index.html",2),
                        ("tx-hook-readme","readme","README.md",1),
                    ] { tr { td { "{name}" } td { HookCell { id, text, max_lines } } } }
                }
            }
            h3 { "Native / reactive contracts" }
            button { id: "tx-toggle-tooltip", onclick: move |_| has_tooltip.toggle(), "Toggle tooltip" }
            button { id: "tx-lines", onclick: move |_| lines.set(if lines() == 2 {0} else {2}), "Toggle clamp" }
            button { id: "tx-content", onclick: move |_| content.set("Changed content in a fixed-width box that still overflows with its new complete text.".to_string()), "Change content" }
            button { id: "tx-drop", onclick: move |_| is_present.toggle(), "Mount / remove" }
            if is_present() {
                Text { id: "tx-dynamic", max_lines: lines(), has_truncate_tooltip: has_tooltip(), style: "width:150px", "{content}" input { id: "tx-kept-field", style: "width:40px" } }
            }
            div { style: "margin:100px;display:flex;flex-direction:column;align-items:center",
                for (id,placement) in [("above",LayerPlacement::Above),("below",LayerPlacement::Below),("start",LayerPlacement::Start),("end",LayerPlacement::End)] {
                    Text { id: "tx-{id}", max_lines: 1, truncate_tooltip_placement: placement, style: "width:180px;margin:20px", "Placement {id}: {LONG_TEXT}" }
                }
            }
            Text { id: "tx-native", tag: TextTag::Label, html_for: "tx-field", lang: "en", "data-testid": "native-text", class: "caller-text", style: "margin-inline:7px",
                onmounted: move |_| native_events += 1,
                onclick: move |_| native_events += 1,
                onwheel: move |_| native_events += 10,
                oncopy: move |_| native_events += 100,
                onanimationstart: move |_| native_events += 1_000,
                ontransitionend: move |_| native_events += 10_000,
                ondragstart: move |_| native_events += 100_000,
                "Native associated label"
            }
            input { id: "tx-field" }
            output { id: "tx-events", "{native_events}" }
            Text { id: "tx-capsize", max_lines: 2, has_capsize: true, style: "width:150px", "{LONG_TEXT}" }
            Text { id: "tx-override", text_type: TextType::CODE, size: TextSize::ExtraLarge, weight: TextWeight::Bold, "Override" }
            div { style: "text-align:end", Text { id: "tx-align", "Inherited start" } }
            div { dir: "rtl", Text { id: "tx-rtl", justify: TextJustify::End, "نص عربي" } }
            Text { id: "tx-unbroken", display: TextDisplay::Block, "https://example.com/abcdefghijklmnopqrstuvwxyz0123456789abcdefghijklmnopqrstuvwxyz0123456789abcdefghijklmnopqrstuvwxyz0123456789" }
            button { id: "tx-focus-host", style: "width:150px",
                Text { id: "tx-focus-text", max_lines: 1, "Keyboard focus can reach this clipped text through its caller-owned parent." }
            }
            Text { id: "tx-touch", max_lines: 1, style: "width:180px",
                "Touch text is long enough to clip: "
                button { id: "tx-touch-child", "Child" }
                input { id: "tx-touch-field", style: "width:40px" }
                " with trailing visible text content"
            }
            Text { id: "tx-nested", max_lines: 2, style: "width:180px", "Nested " Link { href: "#tx-showcase", "native link in clamped text" } " content" }
        }
    }
}
