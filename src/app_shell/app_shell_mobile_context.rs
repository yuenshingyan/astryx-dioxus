use dioxus::prelude::*;

/// Mobile layout and navigation state owned by an AppShell or its caller.
#[derive(Clone, Default, PartialEq)]
pub struct AppShellMobileState {
    /// Whether the shell is in its mobile layout.
    pub is_mobile: bool,
    /// Whether the shell has mobile navigation.
    pub is_mobile_nav_enabled: bool,
    /// Whether the mobile navigation is open.
    pub is_mobile_nav_open: bool,
    /// ID of the controlled navigation surface, if available.
    pub mobile_nav_id: Option<String>,
    /// Requests that the owner toggle the navigation.
    pub toggle_mobile_nav: Option<EventHandler<()>>,
}

/// Reactive state shared by the shell and its mobile navigation controls.
#[derive(Clone, Copy)]
pub struct AppShellMobileContext(pub ReadSignal<AppShellMobileState>);

/// Supplies caller-controlled mobile state without a wrapper element.
#[component]
pub fn AppShellMobileProvider(state: AppShellMobileState, children: Element) -> Element {
    let mut current = use_signal(|| state.clone());
    use_context_provider(|| AppShellMobileContext(current.into()));
    if *current.peek() != state {
        current.set(state);
    }
    children
}

/// Reads the closest mobile state. Outside a provider navigation is unavailable.
pub fn use_app_shell_mobile() -> AppShellMobileState {
    try_consume_context::<AppShellMobileContext>()
        .map_or_else(AppShellMobileState::default, |context| (context.0)())
}
