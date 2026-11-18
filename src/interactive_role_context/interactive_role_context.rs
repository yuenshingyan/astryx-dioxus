// Adapted from Astryx InteractiveRoleContext/InteractiveRoleContext.ts.

use dioxus::prelude::*;

use crate::hooks::use_interactive_role::InteractiveRole;

/// Optional role supplied by an interactive owner, independently of size/layers.
#[derive(Clone, Copy)]
pub struct InteractiveRoleContext(pub Signal<Option<InteractiveRole>>);

/// Supply an inherited interactive role. `None` clears an outer override.
#[component]
pub fn InteractiveRoleProvider(role: Option<InteractiveRole>, children: Element) -> Element {
    let mut context = use_context_provider(|| InteractiveRoleContext(Signal::new(role)));
    if *context.0.peek() != role {
        context.0.set(role);
    }
    children
}

/// Read only the nearest owner; most components use `use_interactive_role`.
pub fn use_interactive_role_context() -> Option<InteractiveRole> {
    try_use_context::<InteractiveRoleContext>().and_then(|context| *context.0.read())
}
