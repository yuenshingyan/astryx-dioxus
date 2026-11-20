//! Layer-scoped default shared by Layout's header and footer.

#[derive(Clone, Copy, Default)]
pub(crate) struct LayoutDividerContext(pub bool);
