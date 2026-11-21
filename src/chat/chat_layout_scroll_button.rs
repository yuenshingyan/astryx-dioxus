use dioxus::prelude::*;

use crate::{Button, ButtonSize, ButtonVariant, Icon, IconSize};

/// Floating action for scrolling a transcript to its end. The caller owns scroll state.
#[component]
pub fn ChatLayoutScrollButton(
    /// Hidden controls are immediately inert, including during their exit transition.
    is_visible: bool,
    /// Optional visible label, such as “New messages”.
    #[props(default)]
    label: Option<String>,
    /// Localized accessible name for the icon-only control.
    #[props(default = "Scroll to bottom".to_owned())]
    scroll_label: String,
    /// Called when the visible button is activated.
    onclick: EventHandler<()>,
    #[props(default)] class: String,
    #[props(default)] style: String,
    /// Native attributes for the outer positioning wrapper.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let label = label.filter(|label| !label.trim().is_empty());
    let has_label = label.is_some();
    let attributes = attributes
        .into_iter()
        .filter(|attr| !matches!(attr.name, "class" | "style"))
        .collect::<Vec<_>>();
    rsx! {
        div { class: "astryx-chat-layout-scroll-button__wrapper {class}", style, ..attributes,
            div { class: "astryx-chat-layout-scroll-button", inert: (!is_visible).then_some(true),
                "data-visible": is_visible.then_some("true"), "data-expanded": has_label.then_some("true"),
                Button {
                    class: "astryx-chat-layout-scroll-button__action",
                    label: label.unwrap_or(scroll_label), variant: ButtonVariant::Ghost, size: ButtonSize::Medium,
                    is_icon_only: !has_label,
                    onclick: move |_| { if is_visible { onclick.call(()); } },
                    icon: rsx! { Icon { size: IconSize::Medium, icon: rsx! {
                        svg { view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round",
                            path { d: "m6 9 6 6 6-6" }
                        }
                    } } },
                }
            }
        }
    }
}
