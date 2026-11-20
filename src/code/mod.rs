//! Semantic inline code; block-code presentation belongs to CodeBlock.

#[path = "code.rs"]
mod component;

pub use component::{Code, CodeColor, CodeProps, CodeSize};
