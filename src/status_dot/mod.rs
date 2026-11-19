#[path = "status_dot.rs"]
mod component;
mod use_status_dot_tooltip;

pub use component::{
    InvalidStatusDotVariant, StatusDot, StatusDotError, StatusDotIcon, StatusDotVariant,
};
