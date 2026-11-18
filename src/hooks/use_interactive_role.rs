// Adapted from Astryx hooks/useInteractiveRole.ts. See THIRD_PARTY_NOTICES.md.

use crate::interactive_role_context::use_interactive_role_context;

/// The element role resolved from destination, handler, then the nearest owner.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InteractiveRole {
    /// Navigation through a link renderer.
    Link,
    /// Native button interaction.
    Button,
    /// No interactive inputs.
    Inert,
}

/// An enabled destination wins, then a click handler, then inherited role.
pub fn use_interactive_role(
    has_href: bool,
    has_onclick: bool,
    is_disabled: bool,
) -> InteractiveRole {
    let inherited = use_interactive_role_context();
    if has_href && !is_disabled {
        return InteractiveRole::Link;
    }
    if has_onclick {
        return InteractiveRole::Button;
    }
    inherited.unwrap_or(InteractiveRole::Inert)
}
