use dioxus::prelude::*;

use super::list_context::{ListContext, ListDensity, ListEdge, ListMarkerStyle, ListSettings};
use crate::{Item, ItemAlign, ItemDensity, ItemLabel, ItemTag};

/// Static list row, with density, markers, dividers, and edge treatment inherited from List.
#[component]
pub fn ListItem(
    #[props(into)] label: ItemLabel,
    #[props(default)] description: Option<ItemLabel>,
    #[props(default)] start_content: Option<Element>,
    #[props(default)] end_content: Option<Element>,
    #[props(default)] align: ItemAlign,
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let settings = try_consume_context::<ListContext>().map_or(
        ListSettings {
            density: ListDensity::Balanced,
            edge: ListEdge::Padded,
            marker_style: ListMarkerStyle::None,
            has_dividers: false,
        },
        |context| (context.0)(),
    );
    let density = match settings.density {
        ListDensity::Compact => ItemDensity::Compact,
        ListDensity::Balanced => ItemDensity::Balanced,
        ListDensity::Spacious => ItemDensity::Spacious,
    };
    let edge_class = match settings.edge {
        ListEdge::Padded => "",
        ListEdge::Flush => "astryx-list-item--flush",
        ListEdge::Outset => "astryx-list-item--outset",
        ListEdge::Bleed => "astryx-list-item--bleed",
    };
    let divider_class = if settings.has_dividers {
        "astryx-list-item--divided"
    } else {
        ""
    };
    let marker = match settings.marker_style {
        ListMarkerStyle::None => None,
        style => {
            let kind = match style {
                ListMarkerStyle::Disc => "disc",
                ListMarkerStyle::Circle => "circle",
                ListMarkerStyle::Decimal => "decimal",
                ListMarkerStyle::None => "none",
            };
            Some(
                rsx! { span { class: "astryx-list-item__marker", "data-kind": kind, "aria-hidden": "true" } },
            )
        }
    };
    rsx! {
        Item { tag: ItemTag::Li, label, description, start_content, end_content, align, density, marker,
            class: format!("astryx-list-item {edge_class} {divider_class} {class}"), style, attributes,
        }
    }
}
