//! Shared internationalization helpers and locale scope.

pub(crate) mod first_strong_direction;
mod internationalization_context;
mod internationalization_provider;

pub(crate) use internationalization_context::use_message;
pub use internationalization_provider::{
    InternationalizationProvider, InternationalizationProviderProps, MessagesByLocale,
};
