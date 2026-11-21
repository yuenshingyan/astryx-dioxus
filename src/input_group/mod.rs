// Only the state contract needed by existing controls is implemented here.
// This does not claim a standalone InputGroup rendering implementation.
mod component;
pub(crate) mod input_group_internal_context;
mod input_group_text;

pub use component::{InputGroup, InputGroupProps};
pub use input_group_text::{InputGroupText, InputGroupTextProps};
