use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{InternationalizationProvider, MessagesByLocale, Pagination};

#[test]
fn overrides_win_across_locale_levels_and_nested_providers_replace_context() {
    let mut dom = VirtualDom::new(|| {
        let messages = MessagesByLocale::from([(
            "pt-BR".into(),
            [
                ("@astryx.pagination.next".into(), "Avançar".into()),
                ("@astryx.pagination.previous".into(), "Voltar".into()),
            ]
            .into(),
        )]);
        let overrides = MessagesByLocale::from([(
            "pt".into(),
            [("@astryx.pagination.next".into(), "Próxima".into())].into(),
        )]);
        rsx! {
            InternationalizationProvider { locale: "pt-br-u-ca-gregory", messages, overrides,
                Pagination { page: 2, total_pages: 3, on_change: |_| {} }
                InternationalizationProvider { locale: "pt-BR",
                    Pagination { page: 2, total_pages: 3, on_change: |_| {} }
                }
            }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    for text in ["Próxima", "Voltar", "Next page", "Previous page"] {
        assert!(html.contains(text), "{html}");
    }
    assert!(
        !html.contains("Avançar") && !html.contains(" lang=") && !html.contains(" dir="),
        "{html}"
    );
}
