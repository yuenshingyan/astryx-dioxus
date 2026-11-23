use std::borrow::Cow;
use std::fmt::{Display, Formatter};

use dioxus::prelude::*;

/// Name of a theme supplied by the application stylesheet.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ThemeName(Cow<'static, str>);

impl ThemeName {
    /// Core token defaults.
    pub const CORE: Self = Self(Cow::Borrowed("core"));
    /// Astryx brand overrides.
    pub const ASTRYX: Self = Self(Cow::Borrowed("astryx"));
    /// Neutral palette overrides.
    pub const NEUTRAL: Self = Self(Cow::Borrowed("neutral"));

    /// Validates a theme identifier without rewriting it.
    pub fn new(name: impl Into<String>) -> Result<Self, InvalidThemeName> {
        let name = name.into();
        if name.is_empty()
            || !name
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
        {
            return Err(InvalidThemeName);
        }
        Ok(Self(Cow::Owned(name)))
    }

    /// The name reflected on the theme scope.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// An invalid stylesheet theme identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvalidThemeName;
impl Display for InvalidThemeName {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(
            "Expected a nonempty theme name of ASCII letters, digits, hyphens, or underscores",
        )
    }
}
impl std::error::Error for InvalidThemeName {}

/// Color scheme for a theme subtree.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ThemeMode {
    #[default]
    System,
    Light,
    Dark,
}

#[derive(Clone, Copy)]
struct ThemeModeContext(Signal<ThemeMode>);

/// Applies a stylesheet-backed theme to a subtree without changing the document root.
#[component]
pub fn Theme(theme: ThemeName, children: Element, #[props(default)] mode: ThemeMode) -> Element {
    let parent_mode = try_consume_context::<ThemeModeContext>().map(|context| (context.0)());
    let (data_mode, scheme) = match mode {
        ThemeMode::Light => (Some("light"), Some("light")),
        ThemeMode::Dark => (Some("dark"), Some("dark")),
        ThemeMode::System if matches!(parent_mode, Some(ThemeMode::Light | ThemeMode::Dark)) => {
            (None, Some("light dark"))
        }
        ThemeMode::System => (None, None),
    };
    rsx! {
        div { class: "astryx-theme", "data-astryx-theme": theme.as_str(), "data-theme": data_mode,
            style: scheme.map(|scheme| format!("color-scheme:{scheme}")),
            ThemeScope { mode, {children} }
        }
    }
}

#[component]
fn ThemeScope(mode: ThemeMode, children: Element) -> Element {
    let mut context = use_context_provider(|| ThemeModeContext(Signal::new(mode)));
    if *context.0.peek() != mode {
        context.0.set(mode);
    }
    children
}
