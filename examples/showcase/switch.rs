use dioxus::prelude::*;

use astryx_dioxus::{IndicatorSize, Switch, SwitchLabelPosition, SwitchLabelSpacing};

#[component]
pub(super) fn SwitchExamples() -> Element {
    let mut notifications = use_signal(|| true);
    let mut dark = use_signal(|| false);
    rsx! {
        section { id: "switch-examples",
            h2 { "Switch" }
            Switch { id: "notification-switch", label: "Enable notifications", value: notifications(), on_change: move |value| notifications.set(value) }
            div { dir: "rtl",
                Switch { id: "dark-mode-switch", label: "Dark mode", description: "Switch to a darker color scheme.",
                    value: dark(), on_change: move |value| dark.set(value), size: IndicatorSize::Small,
                    label_position: SwitchLabelPosition::Start, label_spacing: SwitchLabelSpacing::Spread,
                    width: Some("280px".into()),
                }
            }
            form { id: "switch-form",
                Switch { id: "premium-switch", label: "Premium feature", description: "Upgrade to enable this option",
                    value: true, is_disabled: true, html_name: "premium",
                }
                Switch { id: "managed-switch", label: "Managed preference", value: false }
            }
        }
    }
}
