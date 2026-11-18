// Adapted from Astryx IconButton/IconButton.tsx. See THIRD_PARTY_NOTICES.md.

use dioxus::prelude::*;

use crate::button::{Button, ButtonAction, ButtonElevation, ButtonSize, ButtonType, ButtonVariant};
use crate::link::LinkComponentType;

/// An explicitly icon-only Button. `icon` and its accessible `label` are required.
///
/// All ordinary Button states, navigation, form props, callbacks, and inherited
/// group/size behavior are preserved. Visible children and end content are not
/// accepted. The explicit prop forwarding is Rust's counterpart of upstream's
/// Omit<ButtonProps, 'isIconOnly' | 'children' | 'endContent'> contract. This wrapper
/// adds no DOM element, state, effect, or listener. A label does not automatically
/// create a tooltip; supply `tooltip` when a visible hint is needed.
///
/// Content-mode overrides are intentionally rejected:
///
/// ```compile_fail
/// use astryx_dioxus::IconButton;
/// use dioxus::prelude::*;
/// let _ = rsx! { IconButton { label: "Settings", icon: rsx! { svg {} }, is_icon_only: false } };
/// ```
///
/// ```compile_fail
/// use astryx_dioxus::IconButton;
/// use dioxus::prelude::*;
/// let _ = rsx! { IconButton { label: "Settings", icon: rsx! { svg {} }, "Visible text" } };
/// ```
///
/// ```compile_fail
/// use astryx_dioxus::IconButton;
/// use dioxus::prelude::*;
/// let _ = rsx! { IconButton { label: "Settings", icon: rsx! { svg {} }, end_content: rsx! { span {} } } };
/// ```
///
/// ```compile_fail
/// use astryx_dioxus::IconButton;
/// use dioxus::prelude::*;
/// let _ = rsx! { IconButton { label: "Missing required icon" } };
/// ```
///
/// ```compile_fail
/// use astryx_dioxus::IconButton;
/// use dioxus::prelude::*;
/// let _ = rsx! { IconButton { icon: rsx! { svg {} } } };
/// ```
#[component]
pub fn IconButton(
    label: String,
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: Option<ButtonSize>,
    #[props(default)] elevation: ButtonElevation,
    #[props(default)] button_type: ButtonType,
    #[props(default)] is_disabled: bool,
    #[props(default)] is_loading: bool,
    #[props(default)] is_interruptible: bool,
    icon: Element,
    #[props(default = "Loading".to_string())] loading_label: String,
    #[props(default)] tooltip: Option<String>,
    #[props(default)] width: Option<String>,
    #[props(default)] href: Option<String>,
    #[props(default)] target: Option<String>,
    #[props(default)] rel: Option<String>,
    #[props(default)] render_link: Option<LinkComponentType>,
    #[props(default)] name: Option<String>,
    #[props(default)] value: Option<String>,
    #[props(default)] form: Option<String>,
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(default)] onclick: EventHandler<MouseEvent>,
    #[props(default)] onkeydown: EventHandler<KeyboardEvent>,
    #[props(default)] onmounted: EventHandler<MountedEvent>,
    #[props(default)] onfocus: Option<EventHandler<FocusEvent>>,
    #[props(default)] onblur: Option<EventHandler<FocusEvent>>,
    #[props(default)] onkeyup: Option<EventHandler<KeyboardEvent>>,
    #[props(default)] onpointerdown: Option<EventHandler<PointerEvent>>,
    #[props(default)] onpointerup: Option<EventHandler<PointerEvent>>,
    #[props(default)] onpointermove: Option<EventHandler<PointerEvent>>,
    #[props(default)] onpointercancel: Option<EventHandler<PointerEvent>>,
    #[props(default)] onpointerenter: Option<EventHandler<PointerEvent>>,
    #[props(default)] onpointerleave: Option<EventHandler<PointerEvent>>,
    #[props(default)] onpointerover: Option<EventHandler<PointerEvent>>,
    #[props(default)] onpointerout: Option<EventHandler<PointerEvent>>,
    #[props(default)] onmousedown: Option<EventHandler<MouseEvent>>,
    #[props(default)] onmouseup: Option<EventHandler<MouseEvent>>,
    #[props(default)] onmouseenter: Option<EventHandler<MouseEvent>>,
    #[props(default)] onmouseleave: Option<EventHandler<MouseEvent>>,
    #[props(default)] oncontextmenu: Option<EventHandler<MouseEvent>>,
    #[props(default)] ondblclick: Option<EventHandler<MouseEvent>>,
    #[props(default)] ontouchstart: Option<EventHandler<TouchEvent>>,
    #[props(default)] ontouchmove: Option<EventHandler<TouchEvent>>,
    #[props(default)] ontouchend: Option<EventHandler<TouchEvent>>,
    #[props(default)] ontouchcancel: Option<EventHandler<TouchEvent>>,
    #[props(default)] click_action: Option<ButtonAction>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        Button {
            is_icon_only: true,
            label,
            variant,
            size,
            elevation,
            button_type,
            is_disabled,
            is_loading,
            is_interruptible,
            icon,
            loading_label,
            tooltip,
            width,
            href,
            target,
            rel,
            render_link,
            name,
            value,
            form,
            class,
            style,
            onclick,
            onkeydown,
            onmounted,
            onfocus,
            onblur,
            onkeyup,
            onpointerdown,
            onpointerup,
            onpointermove,
            onpointercancel,
            onpointerenter,
            onpointerleave,
            onpointerover,
            onpointerout,
            onmousedown,
            onmouseup,
            onmouseenter,
            onmouseleave,
            oncontextmenu,
            ondblclick,
            ontouchstart,
            ontouchmove,
            ontouchend,
            ontouchcancel,
            click_action,
            attributes,
        }
    }
}
