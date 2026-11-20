//! Offline Code patterns; prose frames belong to Text/Heading, not Code.
use astryx_dioxus::{Code, CodeColor, CodeSize, STYLES, Text, TextType};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut is_dark = use_signal(|| false);
    let mut count = use_signal(|| 0);
    let mut is_mounted = use_signal(|| false);
    rsx! {
        document::Style { "{STYLES}" }
        document::Style { r#"
          @font-face { font-family:Figtree; src:url('/assets/fonts/Figtree-latin.woff2') format('woff2'); }
          body { margin:0; background:var(--color-background-page); color:var(--color-text-primary); font-family:var(--font-family-body); }
          main { padding:24px; max-width:680px; color-scheme:inherit; }
          section { margin-block:24px; } p,h3 { margin-block:12px; } .stack { display:flex; flex-direction:column; gap:12px; }
        "# }
        main { id:"code-patterns", style: if is_dark() { "color-scheme:dark;background:var(--color-background-page)" } else { "color-scheme:light;background:var(--color-background-page)" },
            button { id:"code-theme", onclick:move |_| is_dark.toggle(), "Toggle theme" }
            section { id:"code-showcase", class:"stack",
                Text { "Run " Code { "npm install @astryxdesign/core" } " to add the package." }
                Text { "Use the " Code { "variant" } " prop to switch between " Code { "primary" } ", " Code { "secondary" } ", and " Code { "ghost" } " styles." }
            }
            section { id:"code-inline",
                Text { style:"max-width:400px", "Use " Code { "useState" } "for local state and " Code { "useEffect" } "for side effects. If you need shared state across components, consider " Code { "useContext" } "or a state management library." }
            }
            section { id:"code-content", class:"stack",
                for (label, content) in [("Variable", "const count = 0"), ("Terminal", "yarn build --watch"), ("CSS property", "border-radius: 8px"), ("File path", "packages/core/src/CodeBlock/Code.tsx"), ("Keyboard shortcut", "Ctrl+Shift+P")] {
                    Text { "{label}: " Code { "{content}" } }
                }
            }
            section { id:"code-sizes", class:"stack",
                // Native heading frame until the independently owned Heading port integrates.
                h3 { style:"font-size:var(--text-heading-3-size);line-height:var(--text-heading-3-line-height);font-weight:var(--text-heading-3-weight)", "Heading with " Code { "inline code" } }
                Text { "Body text with " Code { "inline code" } }
                Text { text_type:TextType::Supporting, "Supporting text with " Code { "inline code" } }
                Text { text_type:TextType::Label, "Label text with " Code { "inline code" } }
            }
            section { id:"code-extra",
                p { id:"code-color-parent", style:"color:rgb(120,40,90);font-size:24px;line-height:36px",
                    Code { id:"code-primary", "primary" } " "
                    Code { id:"code-secondary", color:CodeColor::Secondary, "secondary" } " "
                    Code { id:"code-inherit", color:CodeColor::Inherit, size:CodeSize::Inherit, "inherited" }
                }
                for size in [12,14,18,24] {
                    p { style:"font-size:{size}px;line-height:1.5", Code { size:CodeSize::Inherit, "inherited {size}" } }
                }
                p { id:"code-long", style:"width:180px", Code { "very_long_unbroken_identifier_that_must_wrap_without_widening_the_parent_or_document_0123456789" } }
                p { dir:"rtl", "טקסט " Code { id:"code-rtl", "const x = foo(1);" } " טקסט " Code { id:"code-dir", dir:"rtl", "אבג" } }
                Code { id:"code-native", class:"consumer-class", style:"opacity:0.75;margin-top:1px", "data-testid":"native-code",
                    onclick:move |_| count += 1,
                    onmounted:move |event:MountedEvent| { spawn(async move { if event.get_client_rect().await.is_ok() { is_mounted.set(true); } }); },
                    "const count = " strong { "0" }
                }
                output { id:"code-events", "{count}:{is_mounted}" }
            }
        }
    }
}
