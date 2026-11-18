use std::future::Future;
use std::pin::Pin;

use astryx_dioxus::{
    Button, ButtonElevation, ButtonSize, ButtonType, ButtonVariant, Icon, IconButton, IconSize,
    LinkProvider, LinkRenderProps, SizeProvider,
};
use dioxus::prelude::*;

#[derive(Clone, Copy)]
enum Glyph {
    Search,
    Copy,
    Info,
    Menu,
    Close,
    More,
    Wrench,
    Plus,
}

// Neutral's Lucide glyphs and the original FAB's Heroicons Plus.
// SVG artwork only, not an icon-registry port. See THIRD_PARTY_NOTICES.md.
fn artwork(kind: Glyph) -> Element {
    rsx! {
        svg {
            view_box: "0 0 24 24", fill: "none", stroke: "currentColor",
            stroke_width: if matches!(kind, Glyph::Plus) { "1.5" } else { "2" },
            stroke_linecap: "round", stroke_linejoin: "round", "aria-hidden": "true",
            {match kind {
                Glyph::Search => rsx! { path { d: "m21 21-4.34-4.34" } circle { cx: "11", cy: "11", r: "8" } },
                Glyph::Copy => rsx! { rect { width: "14", height: "14", x: "8", y: "8", rx: "2", ry: "2" } path { d: "M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" } },
                Glyph::Info => rsx! { circle { cx: "12", cy: "12", r: "10" } path { d: "M12 16v-4" } path { d: "M12 8h.01" } },
                Glyph::Menu => rsx! { path { d: "M4 5h16" } path { d: "M4 12h16" } path { d: "M4 19h16" } },
                Glyph::Close => rsx! { path { d: "M18 6 6 18" } path { d: "m6 6 12 12" } },
                Glyph::More => rsx! { circle { cx: "12", cy: "12", r: "1" } circle { cx: "19", cy: "12", r: "1" } circle { cx: "5", cy: "12", r: "1" } },
                Glyph::Wrench => rsx! { path { d: "M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.106-3.105c.32-.322.863-.22.983.218a6 6 0 0 1-8.259 7.057l-7.91 7.91a1 1 0 0 1-2.999-3l7.91-7.91a6 6 0 0 1 7.057-8.259c.438.12.54.662.219.984z" } },
                Glyph::Plus => rsx! { path { d: "M12 4.5v15m7.5-7.5h-15" } },
            }}
        }
    }
}

fn glyph(kind: Glyph) -> Element {
    rsx! { Icon { icon: artwork(kind) } }
}

#[derive(Clone, Copy, PartialEq)]
enum LoadingAction {
    Copy,
    Search,
    Close,
}

#[derive(Clone, Copy)]
enum DemoState {
    Rest,
    Disabled,
    Loading,
    Interruptible,
    DisabledLoading,
}

impl DemoState {
    fn next(self) -> Self {
        match self {
            Self::Rest => Self::Disabled,
            Self::Disabled => Self::Loading,
            Self::Loading => Self::Interruptible,
            Self::Interruptible => Self::DisabledLoading,
            Self::DisabledLoading => Self::Rest,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Rest => "rest",
            Self::Disabled => "disabled",
            Self::Loading => "loading",
            Self::Interruptible => "interruptible",
            Self::DisabledLoading => "disabled-loading",
        }
    }
}

async fn demo_delay() {
    // Dropping the application-owned future closes recv and clears the timer.
    let _ = document::eval(
        "let timer; try { await Promise.race([new Promise(resolve => timer=setTimeout(resolve,1500)),dioxus.recv().catch(()=>null)]); } finally { clearTimeout(timer); } return true;",
    )
    .await;
}

fn router_link(props: LinkRenderProps) -> Element {
    rsx! {
        a {
            href: props.href, target: props.target, rel: props.rel,
            onclick: props.onclick, onkeydown: props.onkeydown, onmounted: props.onmounted,
            "data-adapter": "icon-router", ..props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn IconButtonExamples() -> Element {
    let mut pending = use_signal(|| None::<LoadingAction>);
    let _pending_timer = use_resource(move || async move {
        if pending().is_some() {
            demo_delay().await;
            pending.set(None);
        }
    });
    let mut state = use_signal(|| DemoState::Rest);
    let mut activations = use_signal(|| 0);
    let mut starts = use_signal(|| 0);
    let mut completions = use_signal(|| 0);
    let mut interrupts = use_signal(|| 0);
    let mut mounts = use_signal(|| 0);
    let mut focuses = use_signal(|| 0);
    let mut keys = use_signal(|| 0);
    let mut submits = use_signal(|| 0);
    let mut is_link_disabled = use_signal(|| false);
    let mut has_owned_action = use_signal(|| true);
    let variants = [
        (ButtonVariant::Primary, "primary", "Primary"),
        (ButtonVariant::Secondary, "secondary", "Secondary"),
        (ButtonVariant::Ghost, "ghost", "Ghost"),
        (ButtonVariant::Destructive, "destructive", "Destructive"),
    ];
    let sizes = [
        (ButtonSize::Small, "sm"),
        (ButtonSize::Medium, "md"),
        (ButtonSize::Large, "lg"),
    ];
    let elevations = [
        (ButtonElevation::None, "none"),
        (ButtonElevation::Low, "low"),
        (ButtonElevation::Medium, "med"),
        (ButtonElevation::High, "high"),
    ];
    let current = state();
    rsx! {
        section { id: "icon-button-examples", "data-astryx-theme": "neutral",
            h2 { "IconButton" }
            div { id: "ib-showcase", IconButton { id: "ib-settings", label: "Settings", icon: glyph(Glyph::Wrench), onclick: move |_| activations += 1 } }
            h2 { "IconButton — Action Bar" }
            div { id: "ib-action-bar", style: "display:flex;gap:var(--spacing-1)",
                IconButton { id: "ib-bar-search", label: "Search", icon: glyph(Glyph::Search), variant: ButtonVariant::Ghost, onclick: move |_| activations += 1 }
                IconButton { label: "Copy", icon: glyph(Glyph::Copy), variant: ButtonVariant::Ghost }
                IconButton { label: "Info", icon: glyph(Glyph::Info), variant: ButtonVariant::Ghost }
                IconButton { label: "Menu", icon: glyph(Glyph::Menu), variant: ButtonVariant::Ghost }
                IconButton { label: "Close", icon: glyph(Glyph::Close), variant: ButtonVariant::Ghost }
            }
            h2 { "IconButton — Floating" }
            div { style: "display:flex;flex-direction:column;gap:var(--spacing-4)",
                p { style: "margin:0;color:var(--color-text-secondary);font-size:var(--text-supporting-size);font-weight:var(--text-supporting-weight);line-height:var(--text-supporting-leading)", "FABs are usually icon-only — raise one with `elevation=\"high\"`" }
                div { id: "ib-floating", style: "display:flex;align-items:center;gap:var(--spacing-3)",
                    {variants.iter().map(|(variant, name, label)| rsx! { IconButton { id: "ib-fab-{name}", label: *label, variant: *variant, elevation: ButtonElevation::High, icon: glyph(Glyph::Plus) } })}
                }
            }
            h2 { "IconButton — Loading State" }
            div { id: "ib-loading-toggle", style: "display:flex;gap:var(--spacing-2)",
                IconButton { id: "ib-load-copy", label: "Copy", icon: glyph(Glyph::Copy), variant: ButtonVariant::Primary, is_loading: pending()==Some(LoadingAction::Copy), onclick: move |_| pending.set(Some(LoadingAction::Copy)) }
                IconButton { id: "ib-load-search", label: "Search", icon: glyph(Glyph::Search), is_loading: pending()==Some(LoadingAction::Search), onclick: move |_| pending.set(Some(LoadingAction::Search)) }
                IconButton { id: "ib-load-close", label: "Close", icon: glyph(Glyph::Close), variant: ButtonVariant::Ghost, is_loading: pending()==Some(LoadingAction::Close), onclick: move |_| pending.set(Some(LoadingAction::Close)) }
            }
            h2 { "IconButton — With Tooltips" }
            div { id: "ib-tooltips", style: "display:flex;gap:var(--spacing-2)",
                IconButton { id: "ib-tip-search", label: "Search", tooltip: "Search items", icon: glyph(Glyph::Search), variant: ButtonVariant::Ghost }
                IconButton { id: "ib-tip-copy", label: "Copy link", tooltip: "Copy to clipboard", icon: glyph(Glyph::Copy), variant: ButtonVariant::Ghost }
                IconButton { id: "ib-tip-more", label: "More options", tooltip: "More options", icon: glyph(Glyph::More), variant: ButtonVariant::Ghost }
            }
            h3 { "Variants, sizes, availability, and elevation" }
            Button { id: "ib-next-state", label: "Next icon state", onclick: move |_| { let next = state().next(); state.set(next); } }
            p { id: "ib-state", "{current.name()}" }
            div { class: "showcase-row",
                {variants.iter().flat_map(|(variant, name, _)| sizes.iter().map(move |(size, size_name)| rsx! {
                    IconButton { id: "ib-matrix-{name}-{size_name}", label: "{name} {size_name} action", icon: glyph(Glyph::Wrench), variant: *variant, size: *size,
                        is_disabled: matches!(current, DemoState::Disabled | DemoState::DisabledLoading),
                        is_loading: matches!(current, DemoState::Loading | DemoState::Interruptible | DemoState::DisabledLoading),
                        is_interruptible: matches!(current, DemoState::Interruptible),
                        onclick: move |_| activations += 1,
                    }
                }))}
                {elevations.iter().map(|(elevation, name)| rsx! { IconButton { id: "ib-elevation-{name}", label: "Elevation {name}", elevation: *elevation, icon: glyph(Glyph::Plus) } })}
                SizeProvider { size: ButtonSize::Small,
                    IconButton { id: "ib-inherited", label: "Inherited small", icon: glyph(Glyph::Info) }
                    IconButton { id: "ib-override", label: "Explicit large", size: ButtonSize::Large, icon: glyph(Glyph::Info) }
                }
                IconButton { id: "ib-explicit-icon", label: "Explicit glyph size", size: ButtonSize::Small, icon: rsx! { Icon { size: IconSize::Large, icon: artwork(Glyph::Info) } } }
                IconButton { id: "ib-width", label: "Explicit width", width: "96px", icon: glyph(Glyph::Search) }
                div { style: "--button-icon-only-aspect:2 / 1",
                    IconButton { id: "ib-aspect", label: "Theme aspect", icon: glyph(Glyph::Search) }
                    IconButton { id: "ib-aspect-width", label: "Width wins over aspect", width: "96px", icon: glyph(Glyph::Search) }
                }
                IconButton { id: "ib-mounted", label: "Callbacks", icon: glyph(Glyph::Info), onmounted: move |_| mounts += 1, onfocus: move |_| focuses += 1, onkeydown: move |_| keys += 1 }
                IconButton { id: "ib-disabled-tip", label: "Unavailable", tooltip: "Ask the owner for access", is_disabled: true, icon: glyph(Glyph::Info), onclick: move |_| activations += 100 }
                IconButton { id: "ib-self-colored", label: "Loading artwork", is_loading: true, icon: rsx! { svg { view_box: "0 0 24 24", rect { width: "24", height: "24", fill: "red" } } } }
            }
            h3 { "Async policy and native integration" }
            div { class: "showcase-row",
                if has_owned_action() {
                    IconButton { id: "ib-owned-action", label: "Save changes", tooltip: "Save changes", icon: glyph(Glyph::Copy),
                        click_action: move |_| -> Pin<Box<dyn Future<Output = ()>>> { Box::pin(async move { starts += 1; demo_delay().await; completions += 1; }) },
                    }
                }
                Button { id: "ib-remove-action", label: "Remove async icon", onclick: move |_| has_owned_action.set(false) }
                IconButton { id: "ib-interrupt", label: "Refresh preview", icon: glyph(Glyph::Search), is_interruptible: true,
                    click_action: move |_| -> Pin<Box<dyn Future<Output = ()>>> { Box::pin(async move { interrupts += 1; demo_delay().await; }) },
                }
                LinkProvider { component: router_link,
                    IconButton { id: "ib-router", label: "Open details", href: "#ib-destination", target: "_self", rel: "help", is_disabled: is_link_disabled(), icon: glyph(Glyph::Info), onclick: move |event: MouseEvent| { event.prevent_default(); activations += 1; } }
                }
                IconButton { id: "ib-native-link", label: "Jump to destination", href: "#ib-destination", icon: glyph(Glyph::Search) }
                Button { id: "ib-toggle-link", label: "Toggle icon link availability", onclick: move |_| is_link_disabled.toggle() }
            }
            form { id: "ib-form", onsubmit: move |event| { event.prevent_default(); submits += 1; },
                input { id: "ib-form-input", name: "title", initial_value: "Original", "aria-label": "Icon form title" }
                IconButton { id: "ib-submit", label: "Submit title", icon: glyph(Glyph::Copy), button_type: ButtonType::Submit, name: "action", value: "icon-submit", form: "ib-form" }
                IconButton { id: "ib-reset", label: "Reset title", icon: glyph(Glyph::Close), button_type: ButtonType::Reset, form: "ib-form" }
                IconButton { id: "ib-blocked-submit", label: "Unavailable submit", icon: glyph(Glyph::Copy), button_type: ButtonType::Submit, is_disabled: true }
            }
            p { id: "ib-activations", "Activations: {activations}" }
            p { id: "ib-starts", "Starts: {starts}" }
            p { id: "ib-completions", "Completions: {completions}" }
            p { id: "ib-interrupts", "Interrupts: {interrupts}" }
            p { id: "ib-callbacks", "Mounts: {mounts}; Focuses: {focuses}; Keys: {keys}" }
            p { id: "ib-submits", "Submits: {submits}" }
            div { id: "ib-destination", "Native link destination" }
        }
    }
}
