#[path = "link.rs"]
mod component;
mod compute_target_and_rel;
mod link_context;
mod link_provider;
mod types;
mod use_link_component;
mod use_linkify;

pub use component::{Link, LinkDownload};
pub use link_provider::{ButtonLinkProvider, LinkProvider};
pub use types::{LinkComponentType, LinkRenderProps};
pub use use_link_component::use_link_component;
pub(crate) use use_link_component::{
    render_navigation_anchor, render_navigation_anchor_with_click,
};
pub use use_linkify::{
    LinkifyCaptures, LinkifyError, LinkifyMatch, LinkifyNode, LinkifyPattern, UseLinkifyOptions,
    linkify, use_linkify,
};
