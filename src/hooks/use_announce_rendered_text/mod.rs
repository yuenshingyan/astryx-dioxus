// Adapted from Astryx useAnnounceRenderedText. See THIRD_PARTY_NOTICES.md.

use std::cell::Cell;
use std::rc::Rc;

use dioxus::prelude::*;

use super::use_announce::ANNOUNCE_CONTROLLER;

type WireConfig = (String, bool, String, u64);

fn start_controller(config: &WireConfig) -> document::Eval {
    let controller = document::eval(&format!(
        "{}\n{}",
        ANNOUNCE_CONTROLLER,
        include_str!("use_announce_rendered_text.js"),
    ));
    let _ = controller.send(config.clone());
    controller
}

/// Announces rendered content once per appearance/text change, politely.
///
/// `root_selector` must uniquely identify the live root across renders (including
/// root-tag changes). Attach the returned handler to that root's `onmounted`.
/// `is_active` is the caller's announcement policy, not merely DOM visibility:
/// composed/legacy-silent indicators pass false. The accessible fallback is used
/// only when the root contains no readable text or `aria-labelledby` name.
///
/// Text present during page load or the initial hydration render is remembered
/// silently. Hidden roots re-arm; DOM-only reveals are read on the next owner
/// render. There is no polling or subscription to child mutations.
pub(crate) fn use_announce_rendered_text(
    root_selector: String,
    is_active: bool,
    fallback_text: String,
) -> EventHandler<MountedEvent> {
    let mut controller = use_signal(|| None::<document::Eval>);
    let revision = use_hook(|| Rc::new(Cell::new(0_u64)));
    revision.set(revision.get().wrapping_add(1));
    let config = (root_selector, is_active, fallback_text, revision.get());
    use_drop(move || {
        if let Some(controller) = *controller.peek() {
            let _ = controller.send("dispose");
        }
    });
    let effect_config = config.clone();
    let mut effect = use_effect(move || {
        if let Some(controller) = *controller.peek() {
            let _ = controller.send(effect_config.clone());
        }
    });
    // Unlike use_reactive(props), this also sees rich child text changed while
    // every hook argument stayed equal. Effects run after DOM mutations, not
    // during component rendering. The wire revision dedupes the mount/effect.
    effect.mark_dirty();
    EventHandler::new(move |_| {
        if let Some(controller) = *controller.peek() {
            let _ = controller.send(config.clone());
        } else {
            controller.set(Some(start_controller(&config)));
        }
    })
}
