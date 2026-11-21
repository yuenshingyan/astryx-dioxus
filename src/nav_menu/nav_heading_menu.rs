use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::{ElementSize, SizeValue};

#[derive(Clone, Copy)]
pub(super) struct NavHeadingMenuContext(pub Signal<ElementSize>);

/// Groups navigation menu rows and supplies keyboard movement and activation.
/// The enclosing surface remains responsible for opening and dismissing the menu.
#[component]
pub fn NavHeadingMenu(
    children: Element,
    #[props(default)] size: ElementSize,
    #[props(default)] min_width: Option<SizeValue>,
    #[props(default)] class: String,
    #[props(default)] style: String,
    /// Native keyboard handler; prevent default to cancel menu handling.
    #[props(default)]
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    #[props(default)] onmounted: Option<EventHandler<MountedEvent>>,
    #[props(extends = div, extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let NavHeadingMenuContext(mut inherited) =
        use_context_provider(|| NavHeadingMenuContext(Signal::new(size)));
    if *inherited.peek() != size {
        inherited.set(size);
    }
    let owner = current_scope_id().0.to_string();
    let mut controller = use_signal(|| None::<document::Eval>);
    use_drop(move || {
        if let Some(eval) = *controller.peek() {
            let _ = eval.send(());
        }
    });
    let style = if let Some(width) = min_width {
        format!("{style};min-width:{}", width.as_css()?)
    } else {
        style
    };
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attr| {
            !matches!(
                attr.name,
                "role" | "data-size" | "data-astryx-nav-menu-owner" | "onkeydown" | "onmounted"
            )
        })
        .collect();
    rsx! {
        div { class: "astryx-nav-heading-menu {class}", style: (!style.is_empty()).then_some(style), role: "menu",
            "data-size": size.as_str(), "data-astryx-nav-menu-owner": owner.clone(),
            onkeydown: move |event| { if let Some(handler) = onkeydown { handler.call(event); } },
            onmounted: move |event| {
                if let Some(previous) = *controller.peek() { let _ = previous.send(()); }
                let eval = document::eval(include_str!("nav_heading_menu.js"));
                let _ = eval.send(owner.clone());
                controller.set(Some(eval));
                if let Some(handler) = onmounted { handler.call(event); }
            },
            ..attributes, {children}
        }
    }
}
