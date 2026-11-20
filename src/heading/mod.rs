//! Semantic headings; shared typography and truncation remain owned by Text.

#[path = "heading.rs"]
mod component;

pub use component::{Heading, HeadingLevel, HeadingProps, HeadingType, InvalidHeadingLevel};
