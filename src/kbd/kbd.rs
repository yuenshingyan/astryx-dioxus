// Adapted from Meta Astryx, baseline 476a3153417dbf75c565f7df05b0784e94ddd59e.
// See THIRD_PARTY_NOTICES.md.

use std::borrow::Cow;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

use dioxus::events::onmounted as mounted_attribute;
use dioxus::prelude::*;

use crate::utils::is_apple_platform::IS_APPLE_PLATFORM;

/// Application-owned localized key words and pairwise shortcut formatting.
///
/// Defaults reproduce the pinned English catalog. Apple modifier symbols are
/// fixed glyphs, while their spoken names still use these values. `meta` keeps
/// the source's spoken technical identifier "META"; Windows short text only
/// changes its visible badge. The template must contain `{shortcut}` and `{key}`
/// exactly once, in either order. Values are inserted without reparsing them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KbdText {
    /// Visible non-Apple Control/mod text.
    pub control_short: Cow<'static, str>,
    /// Visible non-Apple Alt text.
    pub alt_short: Cow<'static, str>,
    /// Visible non-Apple Shift text.
    pub shift_short: Cow<'static, str>,
    /// Visible non-Apple meta/Windows text.
    pub windows_short: Cow<'static, str>,
    /// Visible Escape text on every platform.
    pub escape_short: Cow<'static, str>,
    /// Spoken Control/mod text outside Apple platforms.
    pub control: Cow<'static, str>,
    /// Spoken mod text on Apple platforms.
    pub command: Cow<'static, str>,
    /// Spoken Alt text.
    pub alt: Cow<'static, str>,
    /// Spoken Shift text.
    pub shift: Cow<'static, str>,
    /// Spoken Enter text.
    pub enter: Cow<'static, str>,
    /// Spoken Backspace text.
    pub backspace: Cow<'static, str>,
    /// Spoken Escape text.
    pub escape: Cow<'static, str>,
    /// Spoken Tab text.
    pub tab: Cow<'static, str>,
    /// Spoken Up-arrow text.
    pub up: Cow<'static, str>,
    /// Spoken Down-arrow text.
    pub down: Cow<'static, str>,
    /// Spoken Left-arrow text.
    pub left: Cow<'static, str>,
    /// Spoken Right-arrow text.
    pub right: Cow<'static, str>,
    /// Spoken literal-plus text.
    pub plus: Cow<'static, str>,
    /// Pairwise localized message; for example `{key} mit {shortcut}`.
    pub shortcut_template: Cow<'static, str>,
}

impl Default for KbdText {
    fn default() -> Self {
        Self {
            control_short: Cow::Borrowed("Ctrl"),
            alt_short: Cow::Borrowed("Alt"),
            shift_short: Cow::Borrowed("Shift"),
            windows_short: Cow::Borrowed("Win"),
            escape_short: Cow::Borrowed("Esc"),
            control: Cow::Borrowed("Control"),
            command: Cow::Borrowed("Command"),
            alt: Cow::Borrowed("Alt"),
            shift: Cow::Borrowed("Shift"),
            enter: Cow::Borrowed("Enter"),
            backspace: Cow::Borrowed("Backspace"),
            escape: Cow::Borrowed("Escape"),
            tab: Cow::Borrowed("Tab"),
            up: Cow::Borrowed("Up arrow"),
            down: Cow::Borrowed("Down arrow"),
            left: Cow::Borrowed("Left arrow"),
            right: Cow::Borrowed("Right arrow"),
            plus: Cow::Borrowed("Plus"),
            shortcut_template: Cow::Borrowed("{shortcut} + {key}"),
        }
    }
}

impl KbdText {
    fn validate(&self) -> Result<(), KbdError> {
        if [
            &self.control_short,
            &self.alt_short,
            &self.shift_short,
            &self.windows_short,
            &self.escape_short,
            &self.control,
            &self.command,
            &self.alt,
            &self.shift,
            &self.enter,
            &self.backspace,
            &self.escape,
            &self.tab,
            &self.up,
            &self.down,
            &self.left,
            &self.right,
            &self.plus,
        ]
        .iter()
        .any(|text| text.trim().is_empty())
        {
            return Err(KbdError::EmptyTranslation);
        }
        if self.shortcut_template.matches("{shortcut}").count() != 1
            || self.shortcut_template.matches("{key}").count() != 1
        {
            return Err(KbdError::InvalidShortcutTemplate);
        }
        Ok(())
    }

    fn join(&self, shortcut: &str, key: &str) -> String {
        let pattern = self.shortcut_template.as_ref();
        // validate() proves both positions before any shortcut is formatted.
        let Some(left) = pattern.find("{shortcut}") else {
            return String::new();
        };
        let Some(right) = pattern.find("{key}") else {
            return String::new();
        };
        if left < right {
            format!(
                "{}{}{}{}{}",
                &pattern[..left],
                shortcut,
                &pattern[left + 10..right],
                key,
                &pattern[right + 5..]
            )
        } else {
            format!(
                "{}{}{}{}{}",
                &pattern[..right],
                key,
                &pattern[right + 5..left],
                shortcut,
                &pattern[left + 10..]
            )
        }
    }
}

/// Invalid shortcut/name input is reported through a recoverable render error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KbdError {
    /// There is no meaningful parsed key to name the shortcut.
    EmptyShortcut,
    /// Localized key words cannot leave a visible/spoken key unnamed.
    EmptyTranslation,
    /// The joining template needs exactly one occurrence of both placeholders.
    InvalidShortcutTemplate,
}
impl Display for KbdError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::EmptyShortcut=>"Kbd cannot name an empty shortcut; supply keys and spell a literal plus as plus",
            Self::EmptyTranslation=>"Kbd cannot render an empty translated key name; provide nonempty visible and spoken text",
            Self::InvalidShortcutTemplate=>"Kbd shortcut_template must contain {shortcut} and {key} exactly once",
        })
    }
}
impl Error for KbdError {}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Key {
    Mod,
    Control,
    Alt,
    Shift,
    Meta,
    Enter,
    Backspace,
    Escape,
    Tab,
    Up,
    Down,
    Left,
    Right,
    Plus,
    Other(String),
}
struct KeyPart {
    key: Key,
    identity: String,
}

fn is_ecmascript_space(character: char) -> bool {
    (character.is_whitespace() && character != '\u{85}') || character == '\u{feff}'
}

fn parse(keys: &str) -> Vec<KeyPart> {
    let mut occurrences = HashMap::new();
    keys.split('+')
        .map(|part| {
            let source = part.trim_matches(is_ecmascript_space).to_lowercase();
            let occurrence = occurrences.entry(source.clone()).or_insert(0_usize);
            let identity = format!("{source}:{occurrence}");
            *occurrence += 1;
            let key = match source.as_str() {
                "mod" => Key::Mod,
                "ctrl" => Key::Control,
                "alt" => Key::Alt,
                "shift" => Key::Shift,
                "meta" => Key::Meta,
                "enter" | "return" => Key::Enter,
                "backspace" => Key::Backspace,
                "escape" | "esc" => Key::Escape,
                "tab" => Key::Tab,
                "up" => Key::Up,
                "down" => Key::Down,
                "left" => Key::Left,
                "right" => Key::Right,
                "plus" => Key::Plus,
                _ => Key::Other(source),
            };
            KeyPart { key, identity }
        })
        .collect()
}

impl Key {
    fn glyph(&self, is_apple: bool, text: &KbdText) -> String {
        if is_apple {
            match self {
                Self::Mod | Self::Meta => return "⌘".to_string(),
                Self::Control => return "⌃".to_string(),
                Self::Alt => return "⌥".to_string(),
                Self::Shift => return "⇧".to_string(),
                _ => {}
            }
        }
        match self {
            Self::Mod | Self::Control => text.control_short.to_string(),
            Self::Alt => text.alt_short.to_string(),
            Self::Shift => text.shift_short.to_string(),
            Self::Meta => text.windows_short.to_string(),
            Self::Escape => text.escape_short.to_string(),
            Self::Enter => "↵".to_string(),
            Self::Backspace => "⌫".to_string(),
            Self::Tab => "⇥".to_string(),
            Self::Up => "↑".to_string(),
            Self::Down => "↓".to_string(),
            Self::Left => "←".to_string(),
            Self::Right => "→".to_string(),
            Self::Plus => "+".to_string(),
            Self::Other(value) => value.to_uppercase(),
        }
    }

    fn spoken(&self, is_apple: bool, text: &KbdText) -> String {
        match self {
            Self::Mod if is_apple => text.command.to_string(),
            Self::Mod | Self::Control => text.control.to_string(),
            Self::Alt => text.alt.to_string(),
            Self::Shift => text.shift.to_string(),
            Self::Enter => text.enter.to_string(),
            Self::Backspace => text.backspace.to_string(),
            Self::Escape => text.escape.to_string(),
            Self::Tab => text.tab.to_string(),
            Self::Up => text.up.to_string(),
            Self::Down => text.down.to_string(),
            Self::Left => text.left.to_string(),
            Self::Right => text.right.to_string(),
            Self::Plus => text.plus.to_string(),
            Self::Meta => "META".to_string(),
            Self::Other(value) => value.to_uppercase(),
        }
    }
}

/// Display-only, platform-aware shortcut notation. Does not handle any hotkey.
///
/// `keys` splits on +, trims/lowercases, normalizes esc/return and preserves each
/// occurrence. Use plus for a literal +. The aggregate role/name are owned;
/// aria-labelledby/aria-hidden and other native props remain caller-owned.
/// Text comes from localized `text` props, with pinned English defaults.
///
/// SSR and the first client render use non-Apple text (Ctrl/Win). A scoped,
/// one-shot post-render platform read selects Apple glyphs without a listener,
/// subscription, timer, polling or shortcut handler. Dioxus cancels that task
/// when the component unmounts. Failure to read the platform retains the default.
/// `onmounted` is the root-ref adapter; raw native event builders pass in attributes.
///
/// ```compile_fail
/// use astryx_dioxus::Kbd;
/// use dioxus::prelude::*;
/// let _=rsx!{Kbd{}};
/// ```
#[component]
pub fn Kbd(
    keys: String,
    #[props(default)] text: KbdText,
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(default)] onmounted: Option<EventHandler<MountedEvent>>,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let mut is_apple = use_signal(|| false);
    use_effect(move || {
        // The effect has no reactive dependencies. Scoped spawn cannot update a
        // disposed component, including when a delayed platform result arrives.
        spawn(async move {
            let result = document::eval(&format!("{IS_APPLE_PLATFORM}\nreturn isApplePlatform();"))
                .join::<bool>()
                .await;
            let Ok(value) = result else {
                return;
            };
            if *is_apple.peek() == value {
                return;
            }
            is_apple.set(value);
        });
    });
    text.validate()?;
    let parts = parse(&keys);
    if parts
        .iter()
        .all(|part| matches!(&part.key,Key::Other(value) if value.is_empty()))
    {
        return Err(KbdError::EmptyShortcut.into());
    }
    let is_apple = is_apple();
    let name = parts
        .iter()
        .enumerate()
        .fold(String::new(), |shortcut, (index, part)| {
            let word = part.key.spoken(is_apple, &text);
            if index == 0 {
                word
            } else {
                text.join(&shortcut, &word)
            }
        });
    let mut attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attribute| {
            !matches!(attribute.name, "class" | "style" | "role" | "aria-label")
                && (onmounted.is_none() || attribute.name != "onmounted")
        })
        .collect();
    if let Some(handler) = onmounted {
        attributes.push(mounted_attribute(move |event| handler.call(event)));
    }
    rsx! {
        span { class:"astryx-kbd {class}",style,role:"img","aria-label":name,..attributes,
            for part in parts {
                kbd { key:"{part.identity}","aria-hidden":"true","{part.key.glyph(is_apple,&text)}" }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{KbdError, KbdText, Key, parse};

    #[test]
    fn aliases_unknown_keys_and_repetitions_preserve_source_identity() {
        let parts = parse(" escape +esc+ESC+enter+return+return+constructor+__proto__+plus");
        let identities: Vec<_> = parts.iter().map(|part| part.identity.as_str()).collect();
        assert_eq!(
            identities,
            [
                "escape:0",
                "esc:0",
                "esc:1",
                "enter:0",
                "return:0",
                "return:1",
                "constructor:0",
                "__proto__:0",
                "plus:0"
            ]
        );
        assert_eq!(parts[1].key, Key::Escape);
        assert_eq!(parts[4].key, Key::Enter);
        let defaults = KbdText::default();
        assert_eq!(parts[6].key.glyph(false, &defaults), "CONSTRUCTOR");
        assert_eq!(parts[7].key.spoken(false, &defaults), "__PROTO__");
    }

    #[test]
    fn parsing_matches_ecmascript_whitespace_instead_of_rust_whitespace() {
        assert_eq!(parse("\u{feff}mod\u{feff}+ k ")[0].key, Key::Mod);
        assert_eq!(
            parse("\u{85}mod")[0].key,
            Key::Other("\u{85}mod".to_string())
        );
        assert_eq!(parse("ctrl++k").len(), 3);
        assert_eq!(parse("ß")[0].key.glyph(false, &KbdText::default()), "SS");
    }

    #[test]
    fn every_special_key_has_source_glyphs_and_spoken_words_on_both_platforms() {
        let text = KbdText::default();
        for (input, other, apple, label) in [
            ("ctrl", "Ctrl", "⌃", "Control"),
            ("alt", "Alt", "⌥", "Alt"),
            ("shift", "Shift", "⇧", "Shift"),
            ("meta", "Win", "⌘", "META"),
            ("enter", "↵", "↵", "Enter"),
            ("backspace", "⌫", "⌫", "Backspace"),
            ("escape", "Esc", "Esc", "Escape"),
            ("tab", "⇥", "⇥", "Tab"),
            ("up", "↑", "↑", "Up arrow"),
            ("down", "↓", "↓", "Down arrow"),
            ("left", "←", "←", "Left arrow"),
            ("right", "→", "→", "Right arrow"),
            ("plus", "+", "+", "Plus"),
        ] {
            let key = &parse(input)[0].key;
            assert_eq!(key.glyph(false, &text), other);
            assert_eq!(key.glyph(true, &text), apple);
            assert_eq!(key.spoken(false, &text), label);
            assert_eq!(key.spoken(true, &text), label);
        }
        assert_eq!(Key::Mod.spoken(true, &text), "Command");
        assert_eq!(Key::Mod.spoken(false, &text), "Control");
    }

    #[test]
    fn localized_pairwise_order_does_not_interpret_inserted_words_as_templates() {
        let mut text = KbdText {
            shortcut_template: "{key} mit {shortcut}".into(),
            ..KbdText::default()
        };
        assert_eq!(
            text.join("Steuerung", "Abbrechen"),
            "Abbrechen mit Steuerung"
        );
        assert_eq!(text.join("{key}", "{shortcut}"), "{shortcut} mit {key}");
        text.shortcut_template = "{shortcut} {shortcut} {key}".into();
        assert_eq!(text.validate(), Err(KbdError::InvalidShortcutTemplate));
        text.shortcut_template = "{shortcut} + {key}".into();
        text.command = " ".into();
        assert_eq!(text.validate(), Err(KbdError::EmptyTranslation));
    }
}
