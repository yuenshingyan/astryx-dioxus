use std::rc::Rc;

use dioxus::html::input_data::MouseButton;
use dioxus::prelude::*;

use astryx_dioxus::{
    Button, IconEntry, InteractiveRole, InteractiveRoleProvider, InvalidIconName,
    LayerContentBoundary, Link, LinkComponentType, LinkDownload, LinkProvider, LinkRenderProps,
    LinkifyMatch, LinkifyPattern, Text, TextColor, TextSize, TextType, TextWeight,
    UseLinkifyOptions, register_icons, use_link_component, use_linkify,
};

/// Neutral's externalLink alias is Lucide; core keeps its own fallback artwork.
pub fn register_neutral_icons() -> Result<(), InvalidIconName> {
    register_icons([(
        "externalLink".to_string(),
        IconEntry::Renderer(Rc::new(|_| {
            rsx! {
                svg { view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", "aria-hidden": "true",
                    path { d: "M15 3h6v6" }
                    path { d: "M10 14 21 3" }
                    path { d: "M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" }
                }
            }
        })),
    )])
}

#[derive(Clone, Copy)]
struct RouteLocation(Signal<String>);

fn native_root(props: LinkRenderProps) -> Element {
    rsx! { a { href: props.href, target: props.target, rel: props.rel,
        onclick: props.onclick, onkeydown: props.onkeydown, onmounted: props.onmounted,
        "data-adapter": "native", ..props.attributes, {props.children}
    } }
}

fn router_root(props: LinkRenderProps) -> Element {
    // A callback renderer has no independent component scope. Stateful adapters
    // should return their own Dioxus component; this one only consumes a signal.
    let route = try_consume_context::<RouteLocation>();
    let destination = props.href.clone();
    let has_destination = destination.is_some();
    let should_route = props
        .target
        .as_deref()
        .is_none_or(|target| target == "_self")
        && destination
            .as_deref()
            .is_some_and(|href| href.starts_with('/') && !href.starts_with("//"))
        && !props
            .attributes
            .iter()
            .any(|attribute| attribute.name == "download");
    rsx! { a { href: props.href, target: props.target, rel: props.rel,
        onclick: move |event: MouseEvent| {
            props.onclick.call(event.clone());
            if should_route && event.default_action_enabled() && event.modifiers().is_empty()
                && event.trigger_button() == Some(MouseButton::Primary)
                && let (Some(mut route), Some(destination)) = (route, destination.as_ref())
            {
                event.prevent_default();
                route.0.set(destination.clone());
            }
        },
        onkeydown: props.onkeydown, onmounted: props.onmounted,
        "data-adapter": "router", "data-has-destination": has_destination.to_string(),
        ..props.attributes, {props.children}
    } }
}

#[component]
fn ArticleCard(
    title: String,
    href: String,
    resolved_by: String,
    id: String,
    #[props(default)] component: Option<LinkComponentType>,
) -> Element {
    let renderer = use_link_component(component);
    let props = LinkRenderProps {
        href: Some(href),
        target: None,
        rel: None,
        attributes: vec![Attribute::new("id", id, None, false)],
        onclick: EventHandler::new(|_| {}),
        onkeydown: EventHandler::new(|_| {}),
        onmounted: EventHandler::new(|_| {}),
        children: rsx! { "{title}" },
    };
    let heading = if let Some(renderer) = renderer {
        renderer.call(props)
    } else {
        native_root(props)
    };
    rsx! { article { class: "demo-link-card", style: "box-sizing:border-box;min-width:0;overflow:clip;border:var(--border-width) solid var(--color-border);padding:calc(var(--spacing-3) - var(--border-width));border-radius:var(--radius-container);background:var(--color-background-card)",
        div { style: "display:flex;flex-direction:column;gap:var(--spacing-1)",
            Text { weight: TextWeight::Bold, {heading} }
            Text { text_type: TextType::SUPPORTING, color: TextColor::SECONDARY, "{resolved_by}" }
        }
    } }
}

fn issue_pattern() -> LinkifyPattern {
    LinkifyPattern::new(
        |text| {
            let bytes = text.as_bytes();
            let mut matches = Vec::new();
            let mut index = 0;
            while index < bytes.len() {
                if bytes[index] != b'#' {
                    index += 1;
                    continue;
                }
                let start = index;
                index += 1;
                let digits = index;
                while index < bytes.len() && bytes[index].is_ascii_digit() {
                    index += 1;
                }
                if index > digits {
                    matches.push(LinkifyMatch {
                        range: start..index,
                        captures: vec![Some(digits..index)],
                    });
                }
            }
            matches
        },
        |captures| {
            format!(
                "https://example.com/issues/{}",
                captures.get(1).unwrap_or_default()
            )
        },
    )
}

#[component]
fn Comment(text: String) -> Element {
    let options = use_hook(|| UseLinkifyOptions {
        patterns: vec![issue_pattern()],
        ..UseLinkifyOptions::default()
    });
    let nodes = use_linkify(text, options)?;
    rsx! { Text { color: TextColor::SECONDARY, {nodes} } }
}

#[component]
pub fn LinkExamples() -> Element {
    let route = use_context_provider(|| RouteLocation(Signal::new("/".to_string())));
    let mut actions = use_signal(|| 0);
    let mut key_count = use_signal(|| 0);
    let mut mounted = use_signal(|| 0);
    let mut is_disabled = use_signal(|| false);
    let mut has_dynamic = use_signal(|| true);
    let mut is_wide = use_signal(|| false);
    let colors = [
        TextColor::PRIMARY,
        TextColor::SECONDARY,
        TextColor::DISABLED,
        TextColor::PLACEHOLDER,
        TextColor::ACCENT,
        TextColor::INHERIT,
    ];
    let types = [
        TextType::BODY,
        TextType::LARGE,
        TextType::LABEL,
        TextType::SUPPORTING,
        TextType::CODE,
        TextType::DISPLAY_1,
        TextType::DISPLAY_2,
        TextType::DISPLAY_3,
        TextType::INHERIT,
    ];
    rsx! {
        section { id: "link-examples", "data-astryx-theme": "neutral", style: "color:var(--color-text-primary)",
            h2 { "Link" }
            div { id: "ln-showcase", style: "display:flex;width:max-content;max-width:100%", Link { id: "ln-documentation", href: "#", is_standalone: true, "Documentation" } }
            h2 { "Link — Inline Text" }
            Text { id: "ln-inline", "Read the " Link { id: "ln-inline-docs", href: "#", "documentation" } " for more information about using Astryx components." }
            h2 { "Link — External Links" }
            div { id: "ln-external", style: "display:flex;flex-direction:column;gap:var(--spacing-2);width:max-content;max-width:100%",
                Link { id: "ln-github", href: "https://github.com", is_external_link: true, is_standalone: true, "GitHub" }
                Link { id: "ln-mdn", href: "https://developer.mozilla.org", is_external_link: true, is_standalone: true, "MDN Web Docs" }
                Link { id: "ln-react", href: "https://react.dev", is_external_link: true, has_underline: true, is_standalone: true, "React Documentation" }
            }
            h2 { "Link — With Tooltips" }
            div { id: "ln-tooltips", style: "display:flex;gap:var(--spacing-4);align-items:center",
                Link { id: "ln-settings", href: "#", tooltip: "Configure your account settings", is_standalone: true, "Settings" }
                Link { id: "ln-profile", href: "#", tooltip: "View and edit your profile", is_standalone: true, "Profile" }
                Link { id: "ln-help", href: "#", tooltip: "Get help and support", color: TextColor::SECONDARY, is_standalone: true, "Help" }
            }
            h2 { "useLinkComponent — Article Cards" }
            div { id: "ln-cards", style: "display:flex;flex-direction:column;gap:var(--spacing-3)",
                Text { id: "ln-route", "Router location: {route.0}" }
                LinkProvider { component: router_root,
                    ArticleCard { id: "ln-article-router", title: "Getting started", href: "/articles/getting-started", resolved_by: "LinkProvider: the router's link, no page load" }
                    ArticleCard { id: "ln-article-native", title: "Style guide (PDF)", href: "#style-guide", resolved_by: "as=\"a\": a native anchor, even inside the provider", component: native_root }
                }
                ArticleCard { id: "ln-article-fallback", title: "Release notes", href: "#release-notes", resolved_by: "Outside the provider: the app's LinkProvider, or a native anchor" }
            }
            h2 { "useLinkify — Links in a Comment Thread" }
            div { id: "ln-comments", style: "display:flex;flex-direction:column;gap:var(--spacing-3);width:100%;max-width:360px",
                Comment { text: "See https://example.com/spec or email support@example.com." }
                Comment { text: "Fixed by #123, tracking the regression in #456." }
                Comment { text: "No links here, just a plain note." }
            }
            h3 { "Typography, colors, and native integration" }
            div { class: "showcase-row",
                {colors.iter().map(|color| rsx! { Link { id: "ln-color-{color}", href: "#ln-destination", color: color.clone(), "{color} link" } })}
                Link { id: "ln-underlined", href: "#ln-destination", has_underline: true, "Always underlined" }
                Link { id: "ln-action", onclick: move |_| actions += 1, "Link-styled action" }
                Link { id: "ln-disabled", href: "#not-visited", is_disabled: true, "Disabled navigation" }
                Link { id: "ln-disabled-action", href: "#not-visited", is_disabled: true, onclick: move |_| actions += 100, "Disabled action" }
                Link { id: "ln-blank", href: "#ln-destination", target: "_blank", rel: "help", "Explicit blank target" }
                Link { id: "ln-download", href: "data:text/plain,report", download: LinkDownload::FileName("report.txt".into()), referrer_policy: "no-referrer", "Download report" }
                Link { id: "ln-labelled", href: "#ln-destination", label: "Open documentation", "Visible docs" }
                Link { id: "ln-callbacks", href: "#ln-destination", onmounted: move |_| mounted += 1, onkeydown: move |_| key_count += 1, "Native callbacks" }
            }
            div { class: "showcase-row", {types.iter().map(|text_type| rsx! { Link { href: "#ln-destination", text_type: text_type.clone(), "{text_type}" } })} }
            Text { text_type: TextType::LARGE, "Large paragraph " Link { id: "ln-inherit", href: "#ln-destination", text_type: TextType::INHERIT, "inherits size" } }
            Link { id: "ln-size", href: "#ln-destination", size: TextSize::TwoExtraLarge, weight: TextWeight::Bold, "Explicit size and weight" }
            h3 { "Router safety, inherited roles, and focus transitions" }
            LinkProvider { component: router_root,
                Link { id: "ln-router", href: "/settings", is_disabled: is_disabled(), onclick: move |_| actions += 1, "Router navigation" }
                Link { id: "ln-router-passive", href: "/details", is_disabled: is_disabled(), "Navigation without a callback" }
                Link { id: "ln-rejected", href: "java\nscript:alert(1)", "Rejected destination" }
                InteractiveRoleProvider { role: InteractiveRole::Link,
                    Link { id: "ln-absent", role: "link", tabindex: 0, "Inherited role, no destination" }
                }
            }
            Button { id: "ln-toggle-disabled", label: "Toggle router availability", onclick: move |_| is_disabled.toggle() }
            h3 { "Wrapping, own clamp, ancestor clamp, and layer reset" }
            div { id: "ln-long-column", style: "width:200px",
                Link { id: "ln-long", href: "#ln-destination", "https://example.test/a-very-long-unbroken-destination-that-must-not-overflow-the-column" }
                Link { id: "ln-long-button", onclick: move |_| actions += 1, "https://example.test/a-very-long-unbroken-action-label-that-must-not-overflow-the-column" }
                Link { id: "ln-long-external", href: "#ln-destination", is_external_link: true, "https://example.test/a-very-long-unbroken-external-label-that-must-not-overflow-the-column" }
            }
            div { id: "ln-clamp-box", style: if is_wide() { "width:100%;" } else { "width:140px;" },
                Link { id: "ln-clamp-one", href: "#ln-destination", max_lines: 1, "A long clipped link that should offer its complete text on interest" }
                Link { id: "ln-clamp-two", href: "#ln-destination", max_lines: 2, "A longer multiline link label that must stop after precisely two constrained lines and expose its full text without a duplicate description" }
            }
            Button { id: "ln-widen", label: "Toggle clamp width", onclick: move |_| is_wide.toggle() }
            Text { id: "ln-parent-clamp", max_lines: 2, style: "width:200px", has_truncate_tooltip: false,
                "Before " Link { id: "ln-flow", href: "#ln-destination", "a flowing link inside the paragraph that should be clipped with its parent's line boxes" } " after."
                LayerContentBoundary { div { hidden: true, Link { id: "ln-layer-link", href: "#ln-destination", "Layer context reset" } } }
            }
            div { dir: "rtl", style: "width:200px;text-align:start", Link { id: "ln-rtl", href: "#ln-destination", "A wrapped link respects the parent's logical alignment and direction" } }
            if has_dynamic() {
                Link { id: "ln-dynamic-tip", href: "#ln-destination", tooltip: "Unmount must dismiss this tooltip", "Removable hint" }
            }
            Button { id: "ln-remove-tip", label: "Remove hinted link", onclick: move |_| has_dynamic.set(false) }
            p { id: "ln-actions", "Actions: {actions}" }
            p { id: "ln-callback-count", "Mounts: {mounted}; Keys: {key_count}" }
            div { id: "ln-destination", "Link destination" }
        }
    }
}
