use astryx_dioxus::{Button, Kbd, KbdText, Text};
use dioxus::events::onclick;
use dioxus::prelude::*;

#[component]
fn NamedShortcut(keys: String, identity: String) -> Element {
    rsx! {Kbd{keys,"data-kbd-named":identity}}
}

fn german() -> KbdText {
    KbdText {
        control_short: "Strg".into(),
        control: "Steuerung".into(),
        command: "Befehl".into(),
        shift_short: "Umschalt".into(),
        shift: "Umschalttaste".into(),
        alt_short: "Alt Gr".into(),
        windows_short: "Windows".into(),
        escape_short: "Abbr".into(),
        escape: "Abbrechen".into(),
        shortcut_template: "{key} mit {shortcut}".into(),
        ..KbdText::default()
    }
}

#[component]
pub fn KbdExamples() -> Element {
    let mut is_dark = use_signal(|| false);
    let mut is_german = use_signal(|| false);
    let mut has_changed = use_signal(|| false);
    let mut is_shown = use_signal(|| false);
    let mut events = use_signal(|| 0);
    rsx! {
        document::Style { "
            .demo-kbd-context {{font-family:var(--font-family-body);font-size:var(--text-body-size);line-height:var(--text-body-leading);font-weight:400;}}
            .demo-kbd-column {{display:flex;flex-direction:column;gap:var(--spacing-3);width:max-content;max-width:100%;}}
            .demo-kbd-row {{display:flex;gap:var(--spacing-4);flex-wrap:nowrap;}}
            .demo-kbd-menu {{box-sizing:border-box;width:fit-content;max-width:100%;padding:0;border:var(--border-width) solid var(--color-border);border-radius:var(--radius-container);background:var(--color-background-card);overflow:clip;}}
            .demo-kbd-menu ul {{display:flex;flex-direction:column;gap:var(--spacing-0-5);list-style:none;padding:0;margin:0;}}
            .demo-kbd-menu li {{display:flex;align-items:center;gap:var(--spacing-2);padding:var(--spacing-1) var(--spacing-2);border-radius:var(--radius-element);}}
            .demo-kbd-menu-label {{display:flex;flex-direction:column;flex:1;min-width:0;}}
            .demo-kbd-menu-label > span {{display:block;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;}}
            .demo-kbd-menu-end {{display:flex;flex:0 0 auto;margin-inline-start:auto;}}
        " }
        section {
            id:"kbd-examples",class:"demo-kbd-context","data-astryx-theme":"neutral",
            style:if is_dark(){"color-scheme:dark;background:#1b1b1b;color:var(--color-text-primary);padding:1.5rem"}else{"color-scheme:light;background:#fff;color:var(--color-text-primary);padding:1.5rem"},
            h2 {"Kbd — original examples"}
            Button{id:"kb-theme",label:"Switch Kbd light/dark",onclick:move |_|is_dark.toggle()}
            h3 {"Kbd"}
            div {style:"display:flex;background:var(--color-background-muted);padding:var(--spacing-4);width:fit-content",
                NamedShortcut{keys:"mod+k",identity:"showcase"}
            }
            h3 {"Kbd — Inline Instructions"}
            div {class:"demo-kbd-column",id:"kb-inline",
                Text{"Press " NamedShortcut{keys:"mod+k",identity:"inline-command"} " to open the command palette."}
                Text{"Use " NamedShortcut{keys:"mod+shift+p",identity:"inline-all"} " to access all commands."}
                Text{"Press " NamedShortcut{keys:"escape",identity:"inline-escape"} " to close the dialog."}
                Text{"Navigate with " NamedShortcut{keys:"up",identity:"inline-up"} " and " NamedShortcut{keys:"down",identity:"inline-down"} " arrow keys, then press " NamedShortcut{keys:"enter",identity:"inline-enter"} " to select."}
            }
            h3 {"Kbd — Menu Shortcuts"}
            // Native sample list/card framing, not a public Card/List port.
            div {class:"demo-kbd-menu",id:"kb-menu",
                ul {
                    for (label,keys) in [("Cut","mod+x"),("Copy","mod+c"),("Paste","mod+v"),("Undo","mod+z"),("Redo","mod+shift+z")] {
                        li {key:"{label}",
                            div {class:"demo-kbd-menu-label",Text{"{label}"}}
                            div {class:"demo-kbd-menu-end",NamedShortcut{keys,identity:"menu-{label}"}}
                        }
                    }
                }
            }
            h3 {"Kbd — Modifier Combinations"}
            div {class:"demo-kbd-column",id:"kb-modifiers",
                div {class:"demo-kbd-row",
                    for (index,keys) in ["mod+k","shift+enter","ctrl+c","alt+tab"].iter().enumerate(){NamedShortcut{keys:*keys,identity:"modifier-a-{index}"}}
                }
                div {class:"demo-kbd-row",
                    for (index,keys) in ["mod+shift+z","ctrl+alt+delete","mod+shift+p"].iter().enumerate(){NamedShortcut{keys:*keys,identity:"modifier-b-{index}"}}
                }
                div {class:"demo-kbd-row",
                    Text{"Special keys and aliases:"}
                    for (index,keys) in ["escape","esc","enter","return","backspace","tab","space"].iter().enumerate(){NamedShortcut{keys:*keys,identity:"modifier-c-{index}"}}
                }
            }
            h3 {"All named keys, aliases and unknown identifiers"}
            div {style:"overflow:auto;max-width:100%",
                Kbd{id:"kb-all",keys:"mod+ctrl+alt+shift+meta+enter+backspace+escape+tab+up+down+left+right+plus"}
            }
            Kbd{id:"kb-duplicates",keys:"escape+esc+ESC+enter+return+return"}
            Kbd{id:"kb-prototype",keys:"constructor+__proto__+ß"}
            Kbd{id:"kb-plus",keys:"shift+plus"}
            h3 {"Native root inputs and computed naming"}
            Kbd{id:"kb-owned",keys:"mod+k",role:"presentation","aria-label":"Wrong name"}
            span{id:"kb-name","Visible shortcut description"}
            Kbd{id:"kb-referenced",keys:"mod+k","aria-labelledby":"kb-name"}
            Kbd{id:"kb-hidden",keys:"mod+k","aria-hidden":"true"}
            Kbd{id:"kb-native",keys:"ctrl+k",class:"caller-kbd",style:"margin-inline:7px",lang:"fr","data-testid":"kbd-native",onmounted:move |_|events+=1,attributes:vec![onclick(move |_|events+=1)]}
            p{id:"kb-events","Native events: {events}"}
            h3 {"Application-owned localization and key updates"}
            Button{id:"kb-locale",label:"Toggle translated key names",onclick:move |_|is_german.toggle()}
            Kbd{id:"kb-localized",keys:"mod+escape",text:if is_german(){german()}else{KbdText::default()}}
            Button{id:"kb-change",label:"Change displayed shortcut",onclick:move |_|has_changed.toggle()}
            Kbd{id:"kb-changing",keys:if has_changed(){"shift+plus"}else{"mod+k"}}
            Button{id:"kb-toggle",label:"Toggle late shortcut",onclick:move |_|is_shown.toggle()}
            if is_shown(){Kbd{id:"kb-late",keys:"mod+shift+meta+k"}}
            h3 {"Larger text and RTL"}
            div {id:"kb-larger",style:"--text-supporting-size:32px;overflow:auto;max-width:100%",Kbd{keys:"mod+shift+escape"}}
            div {dir:"rtl",id:"kb-rtl",Kbd{keys:"mod+shift+k"}}
        }
    }
}
