// Adapted from Astryx utils/disabledInteraction.ts. See THIRD_PARTY_NOTICES.md.
use dioxus::prelude::*;

pub(crate) fn cancel_disabled_click<T: ?Sized>(event: &Event<T>) {
    event.prevent_default();
    event.stop_propagation();
}

pub(crate) fn cancel_disabled_key(event: &KeyboardEvent) {
    if event.key() == Key::Enter || event.key() == Key::Character(" ".into()) {
        event.prevent_default();
    }
}
