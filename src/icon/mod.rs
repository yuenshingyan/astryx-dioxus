#[path = "icon.rs"]
mod component;
mod icon_default_size_context;
mod icon_size;

pub use component::Icon;
pub use icon_default_size_context::{
    IconDefaultSizeContext, IconDefaultSizeProvider, use_icon_size,
};
pub use icon_size::IconSize;
