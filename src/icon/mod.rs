#[path = "icon.rs"]
mod component;
pub(crate) mod global_icon_registry;
mod icon_default_size_context;
mod icon_size;
pub use global_icon_registry::{
    IconEntry, IconRequest, InvalidIconName, get_extended_icon, register_icons,
};

pub use component::Icon;
pub(crate) use component::render_icon;
pub use icon_default_size_context::{
    IconDefaultSizeContext, IconDefaultSizeProvider, use_icon_size,
};
pub use icon_size::IconSize;
