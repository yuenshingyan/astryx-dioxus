use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use super::chat_layout_scroll_button::ChatLayoutScrollButton;
use super::chat_message::ChatDensity;
use super::use_chat_stream_scroll::use_chat_stream_scroll;

/// Override the default scroll-to-bottom action.
#[derive(Clone, Default, PartialEq)]
pub enum ChatLayoutScrollAction {
    #[default]
    Default,
    Hidden,
    Custom(Element),
}

/// Self-scrolling chat shell with a sticky composer dock.
#[component]
pub fn ChatLayout(
    composer: Element,
    #[props(default)] children: Option<Element>,
    #[props(default)] empty_state: Option<Element>,
    #[props(default)] scroll_button: ChatLayoutScrollAction,
    #[props(default)] density: ChatDensity,
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(default)] onmounted: EventHandler<MountedEvent>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let id = use_hook(|| current_scope_id().0.to_string());
    let (is_scrolled_up, mount_scroll, scroll_to_end) = use_chat_stream_scroll(id.clone());
    let density = match density {
        ChatDensity::Compact => "compact",
        ChatDensity::Balanced => "balanced",
        ChatDensity::Spacious => "spacious",
    };
    let attributes = attributes
        .into_iter()
        .filter(|attr| {
            !matches!(
                attr.name,
                "class"
                    | "style"
                    | "data-density"
                    | "data-astryx-chat-scroll"
                    | "tabindex"
                    | "onmounted"
            )
        })
        .collect::<Vec<_>>();
    rsx! {
        div { class: "astryx-chat-layout {class}", style, "data-density": density,
            "data-astryx-chat-scroll": id, tabindex: "-1",
            onmounted: move |event: MountedEvent| { mount_scroll.call(event.clone()); onmounted.call(event); },
            ..attributes,
            div { class: "astryx-chat-layout__messages",
                if let Some(children) = children { {children} }
                else if let Some(empty_state) = empty_state {
                    div { class: "astryx-chat-layout__empty", {empty_state} }
                }
            }
            div { class: "astryx-chat-layout__dock",
                match scroll_button {
                    ChatLayoutScrollAction::Default => rsx! { ChatLayoutScrollButton { is_visible: is_scrolled_up, onclick: scroll_to_end } },
                    ChatLayoutScrollAction::Hidden => rsx! {},
                    ChatLayoutScrollAction::Custom(button) => button,
                }
                div { class: "astryx-chat-layout__blur", "aria-hidden": "true" }
                div { class: "astryx-chat-layout__composer",
                    div { class: "astryx-chat-layout__composer-inner", {composer} }
                }
            }
        }
    }
}
