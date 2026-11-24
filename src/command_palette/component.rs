use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use super::{CommandPaletteEmpty, CommandPaletteInput, CommandPaletteItem, CommandPaletteList};
use crate::{Dialog, SizeValue, SpacingStep};

/// An entry in a local command source. Values are unique within the palette.
#[derive(Clone, PartialEq, Eq)]
pub struct CommandPaletteOption {
    /// Value sent to the selection callback.
    pub value: String,
    /// Text displayed and searched.
    pub label: String,
}

/// A controlled command palette with local, case-insensitive label search.
#[component]
pub fn CommandPalette(
    is_open: bool,
    on_open_change: EventHandler<bool>,
    options: Vec<CommandPaletteOption>,
    on_select: EventHandler<String>,
    #[props(default)] is_inline: bool,
    #[props(default = "Command palette".to_owned())] label: String,
    #[props(default = "Search commands".to_owned())] input_label: String,
    #[props(default = "No commands found".to_owned())] empty_label: String,
) -> Element {
    let id = use_hook(|| format!("astryx-command-palette-{}", current_scope_id().0));
    let mut query = use_signal(String::new);
    let mut active = use_signal(|| 0_usize);
    use_effect(use_reactive((&is_open,), move |(open,)| {
        if !open {
            query.set(String::new());
            active.set(0);
        }
    }));
    let needle = query.read().trim().to_lowercase();
    let matches: Vec<_> = options
        .iter()
        .enumerate()
        .filter(|(_, option)| option.label.to_lowercase().contains(&needle))
        .collect();
    let count = matches.len();
    let index = active().min(count.saturating_sub(1));
    let active_id = matches
        .get(index)
        .map(|(source_index, _)| format!("{id}-{source_index}"));
    let active_value = matches.get(index).map(|(_, option)| option.value.clone());
    let select = use_callback(move |value: String| {
        on_select.call(value);
        if !is_inline {
            on_open_change.call(false);
        }
    });
    rsx! {
        Dialog { is_open, on_open_change, is_inline, label, width: SizeValue::from(480), padding: SpacingStep::Zero,
            CommandPaletteInput { label: input_label, value: query.read().clone(), has_auto_focus: !is_inline,
                listbox_id: id.clone(), active_option_id: active_id,
                on_value_change: move |text| { query.set(text); active.set(0); },
                onkeydown: move |event: KeyboardEvent| {
                    if !event.default_action_enabled() || event.is_composing() || count == 0 { return; }
                    match event.key() {
                        Key::ArrowDown => active.set((index + 1) % count),
                        Key::ArrowUp => active.set((index + count - 1) % count),
                        Key::Enter => if let Some(value) = &active_value { select.call(value.clone()); },
                        _ => return,
                    }
                    event.prevent_default();
                },
            }
            CommandPaletteList { id: id.clone(),
                for (position, (source_index, option)) in matches.into_iter().enumerate() {
                    CommandPaletteItem { key: "{option.value}", id: "{id}-{source_index}", value: option.value.clone(),
                        is_highlighted: position == index, is_selected: position == index, on_select: select,
                        "{option.label}"
                    }
                }
            }
            if count == 0 { CommandPaletteEmpty { role: "status", "{empty_label}" } }
        }
    }
}
