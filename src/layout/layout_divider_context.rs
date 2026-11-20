//! Layer-scoped default shared by Layout's header and footer.

use dioxus::prelude::*;

#[derive(Clone, Copy)]
pub(crate) struct LayoutDividerContext(pub Signal<bool>);
