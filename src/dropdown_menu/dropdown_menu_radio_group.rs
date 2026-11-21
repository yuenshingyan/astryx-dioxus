use dioxus::prelude::*;

use super::dropdown_menu_context::{DropdownMenuRadioGroupContext, RadioGroupSettings};

/// Names a controlled group of radio rows. The enclosing menu owns keyboard navigation.
#[component]
pub fn DropdownMenuRadioGroup(
    /// Accessible fallback name. A caller may reference visible text with `aria-labelledby`.
    label: String,
    /// Current selected value; `None` represents no selection.
    #[props(default)]
    value: Option<String>,
    /// Receives the value requested by a descendant radio item.
    on_change: EventHandler<String>,
    /// Whether the menu should close after selecting a radio item.
    #[props(default = true)]
    has_close_on_select: bool,
    children: Element,
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let settings = RadioGroupSettings {
        value,
        on_change,
        has_close_on_select,
    };
    let mut context =
        use_context_provider(|| DropdownMenuRadioGroupContext(Signal::new(settings.clone())));
    if *context.0.peek() != settings {
        context.0.set(settings);
    }
    let attributes = attributes
        .into_iter()
        .filter(|attr| !matches!(attr.name, "class" | "style" | "role" | "aria-label"))
        .collect::<Vec<_>>();
    rsx! {
        div { class: "astryx-dropdown-menu-radio-group {class}", style, role: "group", "aria-label": label, ..attributes, {children} }
    }
}

#[cfg(test)]
mod tests {
    use dioxus::prelude::*;
    use dioxus_ssr::render;

    use super::DropdownMenuRadioGroup;
    use super::DropdownMenuRadioGroupContext;

    #[component]
    fn Probe() -> Element {
        let ctx = use_context::<DropdownMenuRadioGroupContext>();
        let state = (ctx.0)();
        rsx! { span { "{state.value.as_deref().unwrap_or_default()}:{state.has_close_on_select}" } }
    }

    #[test]
    fn closest_group_supplies_controlled_value_and_close_policy() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                DropdownMenuRadioGroup { label: "Outer", value: "recent", on_change: |_| {},
                    Probe {}
                    DropdownMenuRadioGroup { label: "Inner", value: "name", has_close_on_select: false, on_change: |_| {}, Probe {} }
                }
            }
        });
        dom.rebuild_in_place();
        let html = render(&dom);
        assert!(
            html.contains("recent:true") && html.contains("name:false"),
            "{html}"
        );
    }
}
