use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::utils::is_renderable::is_renderable;
use crate::{Layout, LayoutContent, LayoutHeader, LayoutHeight, SpacingStep};

/// Surface treatment for the desktop shell.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum AppShellVariant {
    Wash,
    Surface,
    Section,
    #[default]
    Elevated,
}

/// Composes the desktop application frame using the shared Layout slots.
#[component]
pub fn AppShell(
    children: Element,
    #[props(default)] top_nav: Option<Element>,
    #[props(default)] side_nav: Option<Element>,
    #[props(default)] banner: Option<Element>,
    #[props(default)] variant: AppShellVariant,
    #[props(default = LayoutHeight::Fill)] height: LayoutHeight,
    #[props(default)] content_padding: Option<SpacingStep>,
    #[props(default = "Skip to content".to_owned())] skip_label: String,
    #[props(default)] class: String,
    #[props(default)] style: String,
) -> Element {
    let main_id = use_hook(|| format!("astryx-app-shell-main-{}", current_scope_id().0));
    let skip_href = format!("#{main_id}");
    let top_nav = top_nav.filter(is_renderable);
    let banner = banner.filter(is_renderable);
    let side_nav = side_nav.filter(is_renderable);
    let is_fill = height == LayoutHeight::Fill;
    let has_dividers = variant == AppShellVariant::Section;
    let header = (top_nav.is_some() || banner.is_some()).then(|| rsx! {
        LayoutHeader { role: "banner", class: "astryx-app-shell-header", padding: SpacingStep::Zero,
            {banner}
            {top_nav}
        }
    });
    let variant = match variant {
        AppShellVariant::Wash => "wash",
        AppShellVariant::Surface => "surface",
        AppShellVariant::Section => "section",
        AppShellVariant::Elevated => "elevated",
    };
    rsx! {
        div { class: format!("astryx-app-shell astryx-layout-root {class}"), style,
            "data-variant": variant, "data-height": if is_fill { "fill" } else { "auto" },
            a { class: "astryx-app-shell-skip", href: skip_href, "{skip_label}" }
            Layout { height, padding: SpacingStep::Zero, default_has_dividers: has_dividers, header, start: side_nav,
                LayoutContent { class: "astryx-app-shell-main", id: main_id, role: "main", tabindex: -1,
                    is_scrollable: is_fill, padding: content_padding.unwrap_or(SpacingStep::Zero),
                    {children}
                }
            }
        }
    }
}
