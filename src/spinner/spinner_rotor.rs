// Adapted from Astryx Spinner/spinnerRotor.ts. See THIRD_PARTY_NOTICES.md.

use dioxus::prelude::*;

/// Attach every rotor through the same realm-local, single-frame phase batch.
/// Geometry stays in CSS; the bridge never measures a ring.
pub(super) fn use_rotation_phase(id: String) -> EventHandler<MountedEvent> {
    let mut controller = use_signal(|| None::<document::Eval>);
    use_drop(move || {
        if let Some(controller) = *controller.peek() {
            let _ = controller.send("dispose");
        }
    });
    EventHandler::new(move |_| {
        if let Some(previous) = *controller.peek() {
            let _ = previous.send("dispose");
        }
        let next = document::eval(include_str!("spinner_rotor.js"));
        let _ = next.send(id.clone());
        controller.set(Some(next));
    })
}
