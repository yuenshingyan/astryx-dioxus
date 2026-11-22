use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use super::list_context::{ListContext, ListDensity, ListEdge, ListMarkerStyle, ListSettings};

/// Static list container for caller-supplied list rows.
#[component]
pub fn List(
    children: Element,
    #[props(default)] density: ListDensity,
    #[props(default)] has_dividers: bool,
    #[props(default)] edge: ListEdge,
    #[props(default)] header: Option<Element>,
    #[props(default)] list_style: ListMarkerStyle,
    #[props(default)] start: Option<i32>,
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let settings = ListSettings {
        density,
        edge,
        marker_style: list_style,
        has_dividers,
    };
    let mut context = use_context_provider(|| ListContext(Signal::new(settings)));
    if *context.0.peek() != settings {
        context.0.set(settings);
    }
    let header_id = use_hook(|| format!("astryx-list-header-{}", current_scope_id().0));
    let (has_header, is_ordered) = (header.is_some(), list_style == ListMarkerStyle::Decimal);
    let label_id = has_header.then(|| header_id.clone());
    let density_name = match density {
        ListDensity::Compact => "compact",
        ListDensity::Balanced => "balanced",
        ListDensity::Spacious => "spacious",
    };
    let edge_name = match edge {
        ListEdge::Padded => "padded",
        ListEdge::Flush => "flush",
        ListEdge::Outset => "outset",
        ListEdge::Bleed => "bleed",
    };
    let marker_name = match list_style {
        ListMarkerStyle::None => "none",
        ListMarkerStyle::Disc => "disc",
        ListMarkerStyle::Circle => "circle",
        ListMarkerStyle::Decimal => "decimal",
    };
    let style = if list_style == ListMarkerStyle::None {
        style
    } else {
        format!(
            "counter-reset:astryx-list {};{style}",
            i64::from(start.unwrap_or(1)) - 1
        )
    };
    let attributes = attributes
        .into_iter()
        .filter(|attr| {
            !matches!(
                attr.name,
                "class"
                    | "style"
                    | "role"
                    | "start"
                    | "data-density"
                    | "data-list-style"
                    | "data-edge"
                    | "data-dividers"
                    | "data-container-corner"
            ) && !(has_header && attr.name == "aria-labelledby")
        })
        .collect::<Vec<_>>();
    let corner = (edge == ListEdge::Bleed && !has_header).then_some("true");
    let class = format!("astryx-list {class}");
    let dividers = has_dividers.then_some("true");
    let list = if is_ordered {
        rsx! { ol { class, style, role: "list", "aria-labelledby": label_id, start: start.filter(|&value| value != 1),
            "data-density": density_name, "data-list-style": marker_name, "data-edge": edge_name, "data-dividers": dividers,
            "data-container-corner": corner, ..attributes, {children}
        } }
    } else {
        rsx! { ul { class, style, role: "list", "aria-labelledby": label_id,
            "data-density": density_name, "data-list-style": marker_name, "data-edge": edge_name, "data-dividers": dividers,
            "data-container-corner": corner, ..attributes, {children}
        } }
    };
    if let Some(header) = header {
        rsx! {
            div { class: "astryx-list__wrapper", "data-container-corner": (edge == ListEdge::Bleed).then_some("true"),
                div { class: "astryx-list__header", id: header_id, {header} }
                {list}
            }
        }
    } else {
        list
    }
}
