mod app_shell_mobile_context;
mod component;

pub use component::{AppShell, AppShellProps, AppShellVariant};

pub use app_shell_mobile_context::{
    AppShellMobileContext, AppShellMobileProvider, AppShellMobileProviderProps,
    AppShellMobileState, use_app_shell_mobile,
};
