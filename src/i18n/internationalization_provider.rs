use std::collections::BTreeMap;

use dioxus::prelude::*;

use super::internationalization_context::{InternationalizationContext, LocaleSettings};

/// Plain-text messages indexed by locale tag and message key.
pub type MessagesByLocale = BTreeMap<String, BTreeMap<String, String>>;

/// Supplies a locale and plain-text catalogs without changing DOM language or direction.
#[component]
pub fn InternationalizationProvider(
    locale: String,
    children: Element,
    #[props(default)] messages: MessagesByLocale,
    #[props(default)] overrides: MessagesByLocale,
) -> Element {
    let settings = LocaleSettings::new(&locale, messages, overrides);
    let mut context =
        use_context_provider(|| InternationalizationContext(Signal::new(settings.clone())));
    if *context.0.peek() != settings {
        context.0.set(settings);
    }
    children
}
