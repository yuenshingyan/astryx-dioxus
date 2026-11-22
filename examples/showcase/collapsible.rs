use dioxus::prelude::*;

use astryx_dioxus::{
    Collapsible, CollapsibleChevronPosition, CollapsibleGroup, CollapsibleGroupValue, Divider,
};

#[component]
pub(super) fn CollapsibleExamples() -> Element {
    let mut is_open = use_signal(|| true);
    rsx! {
        section { id: "collapsible-examples",
            h2 { "Collapsible" }
            h3 { "Without Card" }
            div { style: "max-width:480px;display:grid;gap:var(--spacing-3)",
                Collapsible { id: "deployment-disclosure", trigger: rsx! { "Deployment details" },
                    p { "Build 4,182 shipped to production from main." }
                    input { "aria-label": "Deployment note" }
                }
                Divider {}
                Collapsible { id: "environment-disclosure", trigger: rsx! { "Environment variables" }, default_is_open: false,
                    p { "12 variables configured across production and preview." }
                }
                Divider {}
                Collapsible { id: "rollback-disclosure", trigger: rsx! { "Rollback" }, is_disabled: true,
                    p { "Previous build 4,181 expires in 19h." }
                }
            }
            h3 { "Single accordion" }
            CollapsibleGroup { id: "single-accordion", has_dividers: true,
                default_value: CollapsibleGroupValue::Single(Some("general".into())),
                Collapsible { id: "accordion-general", value: "general", trigger: rsx! { "General" }, p { "Manage general settings." } }
                Collapsible { id: "accordion-privacy", value: "privacy", trigger: rsx! { "Privacy" }, p { "Manage profile visibility." } }
                Collapsible { value: "notifications", trigger: rsx! { "Notifications" }, p { "Choose notification preferences." } }
            }
            h3 { "Multiple accordion" }
            div { id: "multiple-accordion",
                CollapsibleGroup { default_value: CollapsibleGroupValue::Multiple(vec!["features".into()]),
                    Collapsible { value: "features", trigger: rsx! { "Features" }, p { "Explore included features." } }
                    Collapsible { value: "pricing", trigger: rsx! { "Pricing" }, p { "Compare plans." } }
                    Collapsible { value: "integrations", trigger: rsx! { "Integrations" }, p { "Connect other tools." } }
                }
            }
            h3 { "Controlled leading chevron" }
            div { dir: "rtl",
                Collapsible { id: "controlled-disclosure", trigger: rsx! { "פרטי פריסה" },
                    chevron_position: CollapsibleChevronPosition::Start,
                    is_open: is_open(), on_open_change: move |next| is_open.set(next),
                    input { "aria-label": "Deployment reference" }
                    button { onclick: move |_| is_open.set(false), "Close details" }
                }
            }
        }
    }
}
