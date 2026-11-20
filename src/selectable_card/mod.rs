#[path = "selectable_card.rs"]
mod component;
mod selectable_card_input;

#[cfg(target_arch = "wasm32")]
mod native;

pub use component::{SelectableCard, SelectableCardError, SelectableCardProps};
