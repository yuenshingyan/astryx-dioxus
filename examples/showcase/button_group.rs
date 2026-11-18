use astryx_dioxus::{
    Button, ButtonElevation, ButtonGroup, ButtonGroupElevation, ButtonGroupOrientation,
    ButtonVariant, ElementSize, Icon, IconButton, LayerContentBoundary,
};
use dioxus::prelude::*;

#[derive(Clone, Copy)]
enum Glyph {
    Copy,
    Cut,
    Paste,
    Chevron,
}

// Heroicons v2.2.0, as used by the original named showcase. MIT; see notices.
fn glyph(kind: Glyph) -> Element {
    let data = match kind {
        Glyph::Copy => {
            "M8.25 7.5V6.108c0-1.135.845-2.098 1.976-2.192.373-.03.748-.057 1.123-.08M15.75 18H18a2.25 2.25 0 0 0 2.25-2.25V6.108c0-1.135-.845-2.098-1.976-2.192a48.424 48.424 0 0 0-1.123-.08M15.75 18.75v-1.875a3.375 3.375 0 0 0-3.375-3.375h-1.5a1.125 1.125 0 0 1-1.125-1.125v-1.5A3.375 3.375 0 0 0 6.375 7.5H5.25m11.9-3.664A2.251 2.251 0 0 0 15 2.25h-1.5a2.251 2.251 0 0 0-2.15 1.586m5.8 0c.065.21.1.433.1.664v.75h-6V4.5c0-.231.035-.454.1-.664M6.75 7.5H4.875c-.621 0-1.125.504-1.125 1.125v12c0 .621.504 1.125 1.125 1.125h9.75c.621 0 1.125-.504 1.125-1.125V16.5a9 9 0 0 0-9-9Z"
        }
        Glyph::Cut => {
            "m7.848 8.25 1.536.887M7.848 8.25a3 3 0 1 1-5.196-3 3 3 0 0 1 5.196 3Zm1.536.887a2.165 2.165 0 0 1 1.083 1.839c.005.351.054.695.14 1.024M9.384 9.137l2.077 1.199M7.848 15.75l1.536-.887m-1.536.887a3 3 0 1 1-5.196 3 3 3 0 0 1 5.196-3Zm1.536-.887a2.165 2.165 0 0 0 1.083-1.838c.005-.352.054-.695.14-1.025m-1.223 2.863 2.077-1.199m0-3.328a4.323 4.323 0 0 1 2.068-1.379l5.325-1.628a4.5 4.5 0 0 1 2.48-.044l.803.215-7.794 4.5m-2.882-1.664A4.33 4.33 0 0 0 10.607 12m3.736 0 7.794 4.5-.802.215a4.5 4.5 0 0 1-2.48-.043l-5.326-1.629a4.324 4.324 0 0 1-2.068-1.379M14.343 12l-2.882 1.664"
        }
        Glyph::Paste => {
            "M15.666 3.888A2.25 2.25 0 0 0 13.5 2.25h-3c-1.03 0-1.9.693-2.166 1.638m7.332 0c.055.194.084.4.084.612v0a.75.75 0 0 1-.75.75H9a.75.75 0 0 1-.75-.75v0c0-.212.03-.418.084-.612m7.332 0c.646.049 1.288.11 1.927.184 1.1.128 1.907 1.077 1.907 2.185V19.5a2.25 2.25 0 0 1-2.25 2.25H6.75A2.25 2.25 0 0 1 4.5 19.5V6.257c0-1.108.806-2.057 1.907-2.185a48.208 48.208 0 0 1 1.927-.184"
        }
        Glyph::Chevron => "m19.5 8.25-7.5 7.5-7.5-7.5",
    };
    rsx! { Icon { icon: rsx! { svg { view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "1.5", "aria-hidden": "true", path { stroke_linecap: "round", stroke_linejoin: "round", d: data } } } } }
}

#[component]
pub fn ButtonGroupExamples() -> Element {
    let mut action = use_signal(|| "None".to_string());
    let mut activations = use_signal(|| 0);
    let mut bubbles = use_signal(|| 0);
    let mut has_first = use_signal(|| true);
    let mut has_extra = use_signal(|| false);
    let mut is_busy = use_signal(|| false);
    let mut is_disabled = use_signal(|| false);
    let mut should_cancel_keys = use_signal(|| false);
    let mut root_focuses = use_signal(|| 0);
    let mut root_keys = use_signal(|| 0);
    let mut is_menu_open = use_signal(|| false);
    let mut is_inner_layer_open = use_signal(|| false);
    let mut layer_keys = use_signal(|| 0);
    let mut has_owned_group = use_signal(|| true);
    let sizes = [
        (ElementSize::Small, "sm"),
        (ElementSize::Medium, "md"),
        (ElementSize::Large, "lg"),
    ];
    let elevations = [
        (ButtonGroupElevation::None, "none"),
        (ButtonGroupElevation::Low, "low"),
        (ButtonGroupElevation::Medium, "med"),
        (ButtonGroupElevation::High, "high"),
    ];
    rsx! {
        section { id: "button-group-examples", "data-astryx-theme": "neutral",
            h2 { "ButtonGroup — Basic" }
            div { onclick: move |_| bubbles += 1,
                ButtonGroup { id: "bg-basic", label: "Text editing actions",
                    Button { id: "bg-copy", label: "Copy", onclick: move |_| { action.set("Copy".into()); activations += 1; } }
                    Button { id: "bg-cut", label: "Cut", onclick: move |_| { action.set("Cut".into()); activations += 1; } }
                    Button { id: "bg-paste", label: "Paste", onclick: move |_| { action.set("Paste".into()); activations += 1; } }
                }
            }
            p { id: "bg-activations", "Activations: {activations}" }
            p { id: "bg-bubbles", "Bubbled clicks: {bubbles}" }
            h2 { "ButtonGroup — Floating" }
            div { style: "display:flex;flex-direction:column;align-items:flex-start;gap:var(--spacing-3)",
                p { style: "margin:0;color:var(--color-text-secondary);font-size:var(--text-supporting-size);font-weight:var(--text-supporting-weight);line-height:var(--text-supporting-leading)", "The whole group shares one raised surface — a floating action bar" }
                ButtonGroup { id: "bg-floating", label: "Zoom controls", elevation: ButtonGroupElevation::Medium,
                    Button { label: "Zoom out" }
                    Button { label: "Reset" }
                    Button { label: "Zoom in" }
                }
            }
            h2 { "ButtonGroup — Showcase" }
            div { class: "showcase-row", style: "gap:var(--spacing-6)",
                ButtonGroup { id: "bg-clipboard", label: "Clipboard actions",
                    Button { label: "Copy", icon: glyph(Glyph::Copy) }
                    Button { label: "Cut", icon: glyph(Glyph::Cut) }
                    Button { label: "Paste", icon: glyph(Glyph::Paste) }
                }
                ButtonGroup { id: "bg-save-options", label: "Save options",
                    Button { label: "Save", variant: ButtonVariant::Primary }
                    IconButton { id: "bg-icon-options", label: "Save options", variant: ButtonVariant::Primary, icon: glyph(Glyph::Chevron), onclick: move |_| action.set("Icon options".into()) }
                }
            }
            h3 { "Sizes, elevation, orientation, and member overrides" }
            div { class: "showcase-row",
                {sizes.iter().map(|(size, name)| rsx! {
                    ButtonGroup { id: "bg-size-{name}", label: "Size {name}", size: *size,
                        Button { label: "First" }
                        Button { label: "Second" }
                    }
                })}
                ButtonGroup { id: "bg-size-override", label: "Member sizing", size: ElementSize::Small,
                    Button { id: "bg-inherited-size", label: "Small" }
                    Button { id: "bg-explicit-size", label: "Large", size: ElementSize::Large }
                }
            }
            div { class: "showcase-row", style: "margin-top:1rem",
                {elevations.iter().map(|(elevation, name)| rsx! {
                    ButtonGroup { id: "bg-elevation-{name}", label: "Elevation {name}", elevation: *elevation,
                        Button { label: "First", elevation: ButtonElevation::High }
                        Button { label: "Second", elevation: ButtonElevation::High }
                    }
                })}
                ButtonGroup { id: "bg-vertical", label: "Vertical actions", orientation: ButtonGroupOrientation::Vertical,
                    Button { id: "bg-up", label: "Up" }
                    Button { id: "bg-middle", label: "Middle" }
                    Button { id: "bg-down", label: "Down" }
                }
                ButtonGroup { id: "bg-rtl", label: "RTL actions", dir: "rtl",
                    Button { id: "bg-rtl-first", label: "First" }
                    Button { id: "bg-rtl-last", label: "Last" }
                }
            }
            h3 { "Roving focus and caller callbacks" }
            div { class: "showcase-row",
                Button { id: "bg-before", label: "Before group" }
                ButtonGroup {
                    id: "bg-keyboard", label: "Keyboard actions",
                    onfocus: move |_| root_focuses += 1,
                    onkeydown: move |event: KeyboardEvent| {
                        root_keys += 1;
                        if should_cancel_keys() && event.key() == Key::ArrowRight { event.prevent_default(); }
                    },
                    Button { id: "bg-key-first", label: "First" }
                    Button { id: "bg-key-disabled", label: "Disabled", is_disabled: true }
                    Button { id: "bg-key-link", label: "Link", href: "#button-group-examples" }
                    button { id: "bg-key-native", r#type: "button", "Native member" }
                }
                Button { id: "bg-after", label: "After group" }
                Button { id: "bg-cancel-keys", label: "Toggle caller key cancellation", onclick: move |_| should_cancel_keys.toggle() }
            }
            p { id: "bg-callbacks", "Focus: {root_focuses}; keys: {root_keys}" }
            h3 { "Busy, disabled, insertion/removal, and focus-preserving link replacement" }
            div { class: "showcase-row",
                ButtonGroup { id: "bg-dynamic", label: "Dynamic actions", is_disabled: is_disabled(),
                    if has_first() {
                        Button { id: "bg-dyn-first", label: "Start pending", is_loading: is_busy(), onclick: move |_| is_busy.set(true) }
                    }
                    Button { id: "bg-dyn-link", label: "Dynamic link", href: "#button-group-examples" }
                    if has_extra() { Button { id: "bg-dyn-extra", label: "Inserted action" } }
                }
                Button { id: "bg-remove-first", label: "Remove first member", onclick: move |_| has_first.set(false) }
                Button { id: "bg-insert", label: "Insert member", onclick: move |_| has_extra.set(true) }
                Button { id: "bg-toggle-disabled", label: "Toggle whole group disabled", onclick: move |_| is_disabled.toggle() }
                Button { id: "bg-reset", label: "Reset dynamic group", onclick: move |_| { has_first.set(true); has_extra.set(false); is_busy.set(false); is_disabled.set(false); } }
            }
            h3 { "Wrapped members and trailing layers" }
            ButtonGroup { id: "bg-wrapped", label: "Wrapped actions",
                span { style: "display:contents", Button { id: "bg-wrap-first", label: "First" } }
                span { style: "display:contents", Button { id: "bg-wrap-middle", label: "Middle", tooltip: "A member-owned tooltip" } }
                span { style: "display:contents", Button { id: "bg-wrap-last", label: "Last" } }
                div { popover: "manual", "A trailing layer is not a member" }
                template { "Trailing template" }
                dialog { "Trailing dialog" }
            }
            h3 { "Member-owned layers and context boundaries" }
            p { "The layer below is caller-owned native popover content, not a completed DropdownMenu port." }
            ButtonGroup { id: "bg-menu-group", label: "Layer owners", size: ElementSize::Small,
                Button { id: "bg-menu-first", label: "First", onclick: move |_| action.set("Menu sibling".into()) }
                Button {
                    id: "bg-menu-trigger", label: "Open member layer", "aria-haspopup": "menu", "aria-controls": "bg-member-layer", "aria-expanded": is_menu_open().to_string(),
                    onclick: move |_| {
                        is_menu_open.toggle();
                        let method = if is_menu_open() { "showPopover" } else { "hidePopover" };
                        let target = if is_menu_open() { "bg-layer-action" } else { "bg-menu-trigger" };
                        let _ = document::eval(&format!("document.querySelector('#bg-member-layer').{method}(); document.getElementById('{target}').focus();"));
                    },
                }
                LayerContentBoundary {
                    div { id: "bg-member-layer", popover: "manual", role: "menu", "aria-label": "Caller menu",
                        onkeydown: move |event: KeyboardEvent| {
                            layer_keys += 1;
                            if matches!(event.key(), Key::ArrowUp | Key::ArrowDown | Key::Home | Key::End) {
                                event.prevent_default();
                                let _ = document::eval("document.querySelector('#bg-layer-action').focus();");
                            }
                            if event.key() == Key::Escape {
                                is_menu_open.set(false);
                                let _ = document::eval("document.querySelector('#bg-member-layer').hidePopover(); document.querySelector('#bg-menu-trigger').focus();");
                            }
                        },
                        Button { id: "bg-layer-action", role: "menuitem", label: "Independent layer action", elevation: ButtonElevation::High }
                    }
                }
            }
            Button {
                id: "bg-inner-layer-open", label: "Open caller dialog with a group", "aria-haspopup": "dialog", "aria-controls": "bg-inner-layer", "aria-expanded": is_inner_layer_open().to_string(),
                onclick: move |_| {
                    is_inner_layer_open.set(true);
                    let _ = document::eval("document.querySelector('#bg-inner-layer').showPopover(); document.querySelector('#bg-inner-first').focus();");
                },
            }
            LayerContentBoundary {
                div { id: "bg-inner-layer", popover: "manual", role: "dialog", "aria-label": "Caller dialog",
                    onkeydown: move |event: KeyboardEvent| {
                        if event.key() == Key::Escape {
                            is_inner_layer_open.set(false);
                            let _ = document::eval("document.querySelector('#bg-inner-layer').hidePopover(); document.querySelector('#bg-inner-layer-open').focus();");
                        }
                    },
                    ButtonGroup { id: "bg-inner-group", label: "Inner layer actions",
                        Button { id: "bg-inner-first", label: "Inner first" }
                        Button { id: "bg-inner-last", label: "Inner last" }
                    }
                    Button { label: "Close caller dialog", onclick: move |_| {
                        is_inner_layer_open.set(false);
                        let _ = document::eval("document.querySelector('#bg-inner-layer').hidePopover(); document.querySelector('#bg-inner-layer-open').focus();");
                    } }
                }
            }
            p { id: "bg-layer-keys", "Layer keys: {layer_keys}" }
            h3 { "Expanded targets bounded by neighbours" }
            div { style: "display:flex;align-items:center;gap:4px",
                ButtonGroup { id: "bg-tiny", label: "Tiny target", size: ElementSize::Small,
                    IconButton { id: "bg-tiny-action", label: "Tiny action", icon: glyph(Glyph::Chevron), onclick: move |_| action.set("Tiny action".into()) }
                }
                Button { id: "bg-neighbour", label: "Wide neighbouring control", size: ElementSize::Small, width: "200px", onclick: move |_| action.set("Neighbour".into()) }
            }
            ButtonGroup { id: "bg-wide", label: "Wide group with empty space", size: ElementSize::Small, style: "width:340px;margin-top:1rem",
                IconButton { id: "bg-wide-action", label: "Wide group's action", icon: glyph(Glyph::Chevron), onclick: move |_| action.set("Wide action".into()) }
            }
            h3 { "Scoped controller cleanup" }
            Button { id: "bg-mount-toggle", label: "Mount or unmount group", onclick: move |_| has_owned_group.toggle() }
            if has_owned_group() {
                ButtonGroup { id: "bg-disposable", label: "Disposable group",
                    Button { id: "bg-dispose-first", label: "First" }
                    Button { id: "bg-dispose-last", label: "Last" }
                }
            }
            Button { id: "bg-clear-action", label: "Clear recorded action", onclick: move |_| action.set("None".into()) }
            p { id: "bg-action", "Last group action: {action}" }
        }
    }
}
