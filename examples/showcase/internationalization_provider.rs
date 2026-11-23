use dioxus::prelude::*;

use astryx_dioxus::{InternationalizationProvider, MessagesByLocale, Pagination};

#[component]
pub(super) fn InternationalizationProviderExamples() -> Element {
    let mut is_portuguese = use_signal(|| true);
    let mut page = use_signal(|| 2);
    let locale = if is_portuguese() { "pt-BR" } else { "en" };
    let messages = MessagesByLocale::from([(
        "pt".into(),
        [
            (
                "@astryx.pagination.previous".into(),
                "Página anterior".into(),
            ),
            ("@astryx.pagination.next".into(), "Próxima página".into()),
        ]
        .into(),
    )]);
    rsx! {
        section { id: "internationalization-examples",
            h2 { "InternationalizationProvider" }
            button { r#type: "button", onclick: move |_| is_portuguese.toggle(), "Change preview language" }
            div { lang: locale,
                InternationalizationProvider { locale, messages,
                    Pagination { page: page(), total_pages: 3, on_change: move |value| page.set(value),
                        label: if is_portuguese() { "Paginação" } else { "Pagination" },
                    }
                }
            }
        }
    }
}
