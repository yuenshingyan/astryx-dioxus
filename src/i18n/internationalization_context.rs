use dioxus::prelude::*;

use super::internationalization_provider::MessagesByLocale;

#[derive(Clone, PartialEq, Eq)]
pub(super) struct LocaleSettings {
    candidates: Vec<String>,
    messages: MessagesByLocale,
    overrides: MessagesByLocale,
}

impl LocaleSettings {
    pub(super) fn new(
        locale: &str,
        messages: MessagesByLocale,
        overrides: MessagesByLocale,
    ) -> Self {
        let locale = locale.to_ascii_lowercase();
        let mut candidates = Vec::new();
        if !locale.is_empty() {
            candidates.push(locale.clone());
            // Strip extensions together before considering parent locales.
            let mut base = locale
                .match_indices('-')
                .find_map(|(index, _)| {
                    (locale[index + 1..].split('-').next()?.len() == 1).then_some(&locale[..index])
                })
                .unwrap_or(&locale);
            if base != locale {
                candidates.push(base.to_owned());
            }
            while let Some((parent, _)) = base.rsplit_once('-') {
                candidates.push(parent.to_owned());
                base = parent;
            }
        }
        Self {
            candidates,
            messages: normalize(messages),
            overrides: normalize(overrides),
        }
    }

    fn resolve(&self, key: &str) -> Option<&str> {
        [&self.overrides, &self.messages]
            .into_iter()
            .find_map(|catalogs| {
                self.candidates
                    .iter()
                    .find_map(|locale| catalogs.get(locale)?.get(key).map(String::as_str))
            })
    }
}

fn normalize(catalogs: MessagesByLocale) -> MessagesByLocale {
    catalogs
        .into_iter()
        .map(|(locale, catalog)| (locale.to_ascii_lowercase(), catalog))
        .collect()
}

#[derive(Clone, Copy)]
pub(super) struct InternationalizationContext(pub(super) Signal<LocaleSettings>);

pub(crate) fn use_message(key: &str, english: &str) -> String {
    try_consume_context::<InternationalizationContext>().map_or_else(
        || english.to_owned(),
        |context| context.0.read().resolve(key).unwrap_or(english).to_owned(),
    )
}
