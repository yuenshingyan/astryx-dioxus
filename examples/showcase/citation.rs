use dioxus::prelude::*;

use astryx_dioxus::{Citation, CitationSource, CitationVariant};

fn source(title: &str, url: Option<&str>, mark: Option<&str>) -> CitationSource {
    CitationSource {
        title: Some(title.to_owned()),
        url: url.map(str::to_owned),
        icon: mark.map(|mark| rsx! { span { style: "font-size:9px;font-weight:700", "{mark}" } }),
        ..CitationSource::default()
    }
}

#[component]
pub(super) fn CitationExamples() -> Element {
    rsx! {
        section { id: "citation-examples", "data-astryx-theme": "neutral",
            h2 { "Citation" }
            h3 { "Inline text" }
            div { style: "max-width:560px",
                p {
                    "React uses a virtual DOM to minimize expensive DOM operations"
                    Citation { source: source("React Documentation", Some("https://react.dev"), None), number: 1, variant: CitationVariant::Number }
                    ". This approach was inspired by earlier functional UI frameworks"
                    Citation { source: source("Elm Architecture", Some("https://guide.elm-lang.org/architecture/"), None), number: 2, variant: CitationVariant::Number }
                    "."
                }
                p {
                    "TypeScript adds static types to JavaScript for safer refactoring"
                    Citation { source: source("TypeScript Documentation", Some("https://www.typescriptlang.org"), Some("TS")), number: 3 }
                    "."
                }
            }
            h3 { "Showcase" }
            p { "Label: "
                Citation { source: source("React Documentation", Some("https://react.dev"), None), number: 1 }
                Citation { source: source("GitHub", Some("https://github.com"), Some("GH")), number: 2 }
                Citation { source: source("Internal reference", None, None), number: 3 }
            }
            p { "Number: "
                Citation { source: source("TypeScript Handbook", Some("https://typescriptlang.org"), None), number: 1, variant: CitationVariant::Number }
                " "
                Citation { source: source("MDN Web Docs", Some("https://developer.mozilla.org"), None), number: 2, variant: CitationVariant::Number }
                " "
                Citation { source: source("W3C Specification", None, None), number: 3, variant: CitationVariant::Number }
            }
            h3 { "Source list" }
            p { "Sources: "
                Citation { source: source("React Documentation", Some("https://react.dev"), Some("R")), number: 1 }
                Citation { source: source("TypeScript Handbook", Some("https://www.typescriptlang.org/docs/handbook/"), Some("TS")), number: 2 }
                Citation { source: source("MDN Web Docs", Some("https://developer.mozilla.org"), Some("MDN")), number: 3 }
                Citation { source: source("W3C WAI-ARIA Specification", Some("https://www.w3.org/TR/wai-aria/"), None), number: 4 }
            }
        }
    }
}
