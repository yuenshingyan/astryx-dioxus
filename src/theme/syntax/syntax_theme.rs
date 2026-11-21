use dioxus::prelude::*;

use super::define_syntax_theme::SyntaxThemeDefinition;

/// Scopes syntax colors to its descendants without changing the page color scheme.
#[component]
pub fn SyntaxTheme(theme: SyntaxThemeDefinition, children: Element) -> Element {
    let style = theme.declarations();
    rsx! { div { "data-astryx-syntax-theme": theme.name, style, {children} } }
}
