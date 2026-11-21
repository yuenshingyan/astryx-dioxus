// Adapted from Astryx Layer/layerScopedContext.tsx (LayerContentBoundary).
// See THIRD_PARTY_NOTICES.md.

use dioxus::prelude::*;

use crate::avatar::{AvatarSizeContext, AvatarStatusLabelContext};
use crate::avatar_group::avatar_group_context::AvatarGroupContext;
use crate::breadcrumbs::component::{BreadcrumbContext, BreadcrumbSettings};
use crate::button_group::button_group_context::{ButtonGroupContext, ConnectedButtonGroupContext};
use crate::compatibility::{ButtonEnvironment, ButtonEnvironmentContext};
use crate::form_layout::form_layout_context::{FormLayoutContext, FormLayoutSettings};
use crate::icon::{IconDefaultSizeContext, IconSize};
use crate::input_group::input_group_internal_context::{
    InputGroupInternalContext, InputGroupInternalState,
};
use crate::layout::layout_area_context::LayoutAreaContext;
use crate::layout::layout_divider_context::LayoutDividerContext;
use crate::layout::layout_slots_context::{LayoutSlots, LayoutSlotsContext};
use crate::metadata_list::metadata_list_context::MetadataListContext;
use crate::size_context::{ElementSize, SizeContext};
use crate::text::text_clamp_context::TextClampContext;
use crate::toggle_button::ToggleButtonGroupContext;

/// Starts a new layer's surface-context scope without adding a DOM element.
///
/// Size, icon-slot, group, and input-owned defaults from an outer surface do not
/// leak into the layer. New providers inside the boundary still work; shared
/// application routing continues to inherit. This is the context utility, not
/// a standalone Layer, Popover, Tooltip, or dialog implementation.
#[component]
pub fn LayerContentBoundary(children: Element) -> Element {
    // Rust's statically known contexts replace React's dynamic reset registry.
    // Keep the provider sequence stable so updates never remount layer content.
    use_context_provider(|| AvatarGroupContext(None));
    use_context_provider(AvatarSizeContext::default);
    use_context_provider(|| AvatarStatusLabelContext(None));
    use_context_provider(|| BreadcrumbContext(Signal::new(BreadcrumbSettings::default())));
    use_context_provider(|| FormLayoutContext::new(FormLayoutSettings::default()));
    use_context_provider(LayoutAreaContext::default);
    use_context_provider(|| LayoutDividerContext(Signal::new(false)));
    use_context_provider(|| LayoutSlotsContext(Signal::new(LayoutSlots::default())));
    use_context_provider(|| MetadataListContext {
        is_stacked: Signal::new(false),
    });
    use_context_provider(|| SizeContext(Signal::new(ElementSize::Medium)));
    use_context_provider(|| IconDefaultSizeContext(Signal::new(IconSize::Medium)));
    use_context_provider(|| ButtonGroupContext(Signal::new(None)));
    use_context_provider(|| ConnectedButtonGroupContext(Signal::new(None)));
    use_context_provider(|| {
        InputGroupInternalContext(Signal::new(InputGroupInternalState::default()))
    });
    use_context_provider(|| ButtonEnvironmentContext(Signal::new(ButtonEnvironment::default())));
    use_context_provider(|| TextClampContext(Signal::new(false)));
    use_context_provider(ToggleButtonGroupContext::empty);
    children
}
