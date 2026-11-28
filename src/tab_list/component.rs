use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use super::tab_list_context::{TabListContext, TabListState};
use crate::{ElementSize, SizeProvider, use_size};

/// The semantics of a horizontal tab strip.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum TabListMode {
    #[default]
    Navigation,
    Panels,
}

/// A controlled tab strip. The caller renders and associates any tab panels.
#[component]
pub fn TabList(
    value: String,
    on_change: EventHandler<String>,
    children: Element,
    #[props(default)] mode: TabListMode,
    #[props(default = "Tabs".to_owned())] label: String,
    #[props(default)] size: Option<ElementSize>,
    #[props(default)] has_divider: bool,
    #[props(default)] class: String,
    #[props(default)] style: String,
) -> Element {
    let is_panel_list = mode == TabListMode::Panels;
    let state = TabListState {
        value,
        on_change,
        is_panel_list,
    };
    let mut context = use_context_provider(|| TabListContext(Signal::new(state.clone())));
    if *context.0.peek() != state {
        context.0.set(state);
    }
    let size = use_size(size);
    let owner = use_hook(|| format!("astryx-tab-list-{}", current_scope_id().0));
    let mount_owner = owner.clone();
    let mut controllers = use_signal(|| None::<[document::Eval; 2]>);
    use_drop(move || {
        if let Some(evals) = *controllers.peek() {
            for eval in evals {
                let _ = eval.send(());
            }
        }
    });
    let mount = move |_| {
        if let Some(evals) = *controllers.peek() {
            for eval in evals {
                let _ = eval.send(());
            }
        }
        let focus = document::eval(include_str!("../hooks/use_list_focus.js"));
        let _ = focus.send([mount_owner.clone()]);
        let overflow = document::eval(concat!(
            include_str!("../utils/shared_resize_observer.js"),
            "\n",
            include_str!("tab_list_overflow.js"),
        ));
        let _ = overflow.send(&mount_owner);
        controllers.set(Some([focus, overflow]));
    };
    let class = format!("astryx-tab-list {class}");
    let divider = has_divider.then_some("true");
    if is_panel_list {
        rsx! { div { class, style, role: "tablist", "aria-label": label, "data-astryx-tab-list-owner": owner, "data-divider": divider,
            onmounted: mount, SizeProvider { size, {children} }
        } }
    } else {
        rsx! { nav { class, style, "aria-label": label, "data-astryx-tab-list-owner": owner, "data-divider": divider,
            onmounted: mount, SizeProvider { size, {children} }
        } }
    }
}
