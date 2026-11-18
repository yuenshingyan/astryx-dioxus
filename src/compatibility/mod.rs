//! Existing provisional APIs, retained without defining core component ownership.
mod button_environment;

pub(crate) use button_environment::publish_group_environment;

pub use button_environment::{
    ButtonEnvironment, ButtonEnvironmentContext, ButtonEnvironmentProvider, use_button_environment,
};
