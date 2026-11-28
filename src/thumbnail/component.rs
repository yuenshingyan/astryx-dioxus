use dioxus::core::{AttributeValue, current_scope_id};
use dioxus::prelude::*;

use crate::button::{Button, ButtonSize, ButtonVariant};
use crate::hooks::use_announce_rendered_text::use_announce_rendered_text;
use crate::hooks::use_press_feedback::{PressFeedbackConfig, use_feedback_controller};
use crate::icon::{Icon, IconSize};
use crate::skeleton::{Skeleton, SkeletonRadius};
use crate::spinner::{Spinner, SpinnerShade, SpinnerSize};

/// When to show the remove action.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ThumbnailRemoveVisibility {
    /// Reveal on hover, keyboard focus, or coarse-pointer devices.
    #[default]
    Hover,
    /// Show without interaction.
    Always,
}

#[component]
fn PreviewImage(src: String, alt: Option<String>) -> Element {
    let mut has_error = use_signal(|| false);
    if has_error() {
        return rsx! { ImagePlaceholder {} };
    }
    let is_decorative = alt.as_deref().is_none_or(str::is_empty);
    rsx! { img { class: "astryx-thumbnail__image", src, alt: alt.unwrap_or_default(),
        role: is_decorative.then_some("presentation"), "aria-hidden": is_decorative.then_some("true"),
        onerror: move |_| has_error.set(true),
    } }
}

#[component]
fn ImagePlaceholder() -> Element {
    rsx! {
        span { class: "astryx-thumbnail__placeholder", "aria-hidden": "true",
            svg { width: "24", height: "24", view_box: "0 0 24 24", fill: "currentColor",
                path { d: "M21 19V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2M8.5 13.5l2.5 3 3.5-4.5 4.5 6H5l3.5-5.5z" }
            }
        }
    }
}

/// A square attachment preview with optional Open and Remove actions.
#[component]
pub fn Thumbnail(
    /// Browser image source.
    #[props(default)]
    src: Option<String>,
    /// Image description. Omit only when the image is decorative.
    #[props(default)]
    alt: Option<String>,
    /// Attachment name shown in the tooltip and used to name the group.
    #[props(default)]
    label: Option<String>,
    /// Open the preview when activated.
    #[props(default)]
    on_click: Option<EventHandler<MouseEvent>>,
    /// Remove the attachment when activated.
    #[props(default)]
    on_remove: Option<EventHandler<MouseEvent>>,
    /// Show loading artwork and temporarily disable Open.
    #[props(default)]
    is_loading: bool,
    /// Dim and disable both actions without removing them from the tab order.
    #[props(default)]
    is_disabled: bool,
    /// Visibility of the Remove action.
    #[props(default)]
    show_remove_on: ThumbnailRemoveVisibility,
    /// Localized fallback attachment name.
    #[props(default = "Thumbnail".into())]
    fallback_name: String,
    /// Override the full localized Open action name.
    #[props(default)]
    open_label: Option<String>,
    /// Override the full localized Remove action name.
    #[props(default)]
    remove_label: Option<String>,
    /// Localized upload announcement.
    #[props(default = "Loading".into())]
    loading_label: String,
    /// Classes on the outer group.
    #[props(default)]
    class: String,
    /// Inline CSS on the outer group.
    #[props(default)]
    style: String,
    /// Root attributes, mounted callback, and ordinary root listeners. The group role is fixed.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let id = format!("thumbnail-{}", current_scope_id().0);
    let name = match (&label, &alt) {
        (Some(label), Some(alt)) if !label.is_empty() && !alt.is_empty() => {
            format!("{label} — {alt}")
        }
        _ => label
            .clone()
            .or_else(|| alt.clone())
            .unwrap_or(fallback_name),
    };
    let open_label = open_label.unwrap_or_else(|| format!("Open {name}"));
    let remove_label = remove_label.unwrap_or_else(|| format!("Remove {name}"));
    let can_open = on_click.is_some() && !is_disabled && !is_loading;
    let is_uploading = is_loading && src.is_some();
    let announce = use_announce_rendered_text(
        format!("[data-astryx-thumbnail-busy='{id}']"),
        is_uploading,
        loading_label,
    );
    let tooltip_id = format!("{id}-tooltip");
    let tooltip = use_feedback_controller(
        PressFeedbackConfig {
            id: tooltip_id.clone(),
            label: name.clone(),
            tooltip: label.clone(),
            is_icon_only: false,
            is_loading: false,
            is_disabled: false,
        },
        false,
        label.is_some(),
    );
    let press = use_feedback_controller(
        PressFeedbackConfig {
            id: id.clone(),
            label: String::new(),
            tooltip: None,
            is_icon_only: false,
            is_loading: false,
            is_disabled: !can_open,
        },
        false,
        can_open,
    );
    let has_attr = |name| {
        attributes
            .iter()
            .any(|attr| attr.name == name && !matches!(attr.value, AttributeValue::None))
    };
    let root_name = (!has_attr("aria-label") && !has_attr("aria-labelledby")).then_some(name);
    // An overriding role such as img could hide the native actions from assistive technology.
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attr| {
            !matches!(attr.name, "class" | "style" | "role")
                && !(attr.name == "aria-label" && matches!(attr.value, AttributeValue::None))
        })
        .collect();
    let image = rsx! {
        if let Some(src) = src { PreviewImage { key: "{src}", src, alt } }
        else if is_loading { Skeleton { radius: SkeletonRadius::Two, style: "height:100%;width:100%" } }
        else { ImagePlaceholder {} }
    };
    rsx! {
        div { class: "astryx-thumbnail {class}", style: (!style.is_empty()).then_some(style), role: "group",
            "aria-label": root_name, "data-disabled": is_disabled.then_some("true"), ..attributes,
            div { class: "astryx-thumbnail__surface", "data-astryx-feedback-id": tooltip_id,
                "data-astryx-thumbnail-busy": id.clone(), onmounted: move |event: MountedEvent| { tooltip.call(event.clone()); announce.call(event); },
                div { class: "astryx-thumbnail__preview", "data-astryx-feedback-id": id, "data-pressable-container": can_open.then_some("true"), onmounted: press,
                    if let Some(on_click) = on_click {
                        button { r#type: "button", class: "astryx-thumbnail__open", "aria-label": open_label,
                            "aria-disabled": (!can_open).then_some("true"),
                            onkeydown: move |event: KeyboardEvent| { if !can_open && (event.key() == Key::Enter || event.key() == Key::Character(" ".into())) { event.prevent_default(); } },
                            onclick: move |event: MouseEvent| { if can_open { on_click.call(event); } else { event.prevent_default(); event.stop_propagation(); } },
                            {image}
                        }
                    } else { {image} }
                }
                if is_uploading {
                    span { class: "astryx-thumbnail__upload", "aria-hidden": "true", Spinner { size: SpinnerSize::Small, shade: SpinnerShade::OnMedia, is_decorative: true } }
                }
                if let Some(on_remove) = on_remove {
                    span { class: "astryx-thumbnail__remove", "data-reveal": (show_remove_on == ThumbnailRemoveVisibility::Hover).then_some("hover"),
                        Button { label: remove_label, size: ButtonSize::Small, variant: ButtonVariant::Secondary,
                            is_icon_only: true, is_disabled, class: "astryx-thumbnail__remove-button",
                            icon: rsx! { Icon { size: IconSize::ExtraSmall, icon: rsx! {
                                svg { view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", path { d: "m6 6 12 12M18 6 6 18" } }
                            } } },
                            onclick: move |event: MouseEvent| { event.stop_propagation(); on_remove.call(event); },
                        }
                    }
                }
            }
        }
    }
}
