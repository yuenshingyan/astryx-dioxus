use dioxus::prelude::*;

/// A standalone command option. Composite keyboard navigation belongs to the palette.
#[component]
pub fn CommandPaletteItem(
    /// Item identifier reported on selection.
    value: String,
    /// Noninteractive option content.
    children: Element,
    /// Receives the item value on pointer activation.
    #[props(default)]
    on_select: Option<EventHandler<String>>,
    /// Runs before selection and may cancel it.
    #[props(default)]
    onclick: Option<EventHandler<MouseEvent>>,
    /// Highlights this option independently of selection.
    #[props(default)]
    is_highlighted: bool,
    /// Indicates selection in picker mode.
    #[props(default)]
    is_selected: bool,
    /// Suppresses activation.
    #[props(default)]
    is_disabled: bool,
    /// Additional root classes.
    #[props(default)]
    class: String,
    /// Inline root declarations.
    #[props(default)]
    style: Option<String>,
    /// Native attributes on the option, including mount and pointer events.
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attr| {
            !matches!(
                attr.name,
                "role"
                    | "aria-selected"
                    | "aria-disabled"
                    | "data-value"
                    | "data-highlighted"
                    | "data-selected"
                    | "data-disabled"
                    | "onclick"
            )
        })
        .collect();
    let activate = (!is_disabled).then(|| {
        EventHandler::new({
            let value = value.clone();
            move |event: MouseEvent| {
                if let Some(handler) = onclick {
                    handler.call(event.clone());
                }
                if event.default_action_enabled()
                    && let Some(handler) = on_select
                {
                    handler.call(value.clone());
                }
            }
        })
    });
    let mut attributes = attributes;
    if let Some(activate) = activate {
        attributes.push(dioxus::html::onclick(move |event| activate.call(event)));
    }
    rsx! {
        div { class: "astryx-command-palette-item {class}", style, role: "option",
            "aria-selected": if is_selected { "true" } else { "false" }, "aria-disabled": is_disabled.then_some("true"),
            "data-value": value,
            "data-highlighted": is_highlighted.then_some("highlighted"),
            "data-selected": is_selected.then_some("selected"), "data-disabled": is_disabled.then_some("disabled"),
            ..attributes, {children}
        }
    }
}
