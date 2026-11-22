use std::future::Future;
use std::pin::Pin;

use astryx_dioxus::{
    Button, ButtonElevation, ButtonSize, ButtonType, ButtonVariant, DropdownMenuDivider, STYLES,
};
use dioxus::prelude::*;

#[path = "showcase/aspect_ratio.rs"]
mod aspect_ratio;
#[path = "showcase/avatar.rs"]
mod avatar;
#[path = "showcase/avatar_group.rs"]
mod avatar_group;
#[path = "showcase/avatar_group_overflow.rs"]
mod avatar_group_overflow;
#[path = "showcase/avatar_status_dot.rs"]
mod avatar_status_dot;
#[path = "showcase/badge.rs"]
mod badge;
#[path = "showcase/banner.rs"]
mod banner;
#[path = "showcase/blockquote.rs"]
mod blockquote;
#[path = "showcase/breadcrumb_item.rs"]
mod breadcrumb_item;
#[path = "showcase/breadcrumbs.rs"]
mod breadcrumbs;
#[path = "showcase/button_contract.rs"]
mod button_contract;
#[path = "showcase/button_group.rs"]
mod button_group;
#[path = "showcase/button_patterns.rs"]
mod button_patterns;
#[path = "showcase/card.rs"]
mod card;
#[path = "showcase/center.rs"]
mod center;
#[path = "showcase/chat_dictation_button.rs"]
mod chat_dictation_button;
#[path = "showcase/chat_layout.rs"]
mod chat_layout;
#[path = "showcase/chat_layout_scroll_button.rs"]
mod chat_layout_scroll_button;
#[path = "showcase/chat_message.rs"]
mod chat_message;
#[path = "showcase/chat_message_bubble.rs"]
mod chat_message_bubble;
#[path = "showcase/chat_message_list.rs"]
mod chat_message_list;
#[path = "showcase/chat_message_metadata.rs"]
mod chat_message_metadata;
#[path = "showcase/chat_send_button.rs"]
mod chat_send_button;
#[path = "showcase/chat_system_message.rs"]
mod chat_system_message;
#[path = "showcase/chat_tokenized_text.rs"]
mod chat_tokenized_text;
#[path = "showcase/chat_tool_calls.rs"]
mod chat_tool_calls;
#[path = "showcase/check_indicator.rs"]
mod check_indicator;
#[path = "showcase/checkbox_indicator.rs"]
mod checkbox_indicator;
#[path = "showcase/checkbox_input.rs"]
mod checkbox_input;
#[path = "showcase/checkbox_list.rs"]
mod checkbox_list;
#[path = "showcase/citation.rs"]
mod citation;
#[path = "showcase/clickable_card.rs"]
mod clickable_card;
#[path = "showcase/collapsible.rs"]
mod collapsible;
#[path = "showcase/command_palette_empty.rs"]
mod command_palette_empty;
#[path = "showcase/command_palette_footer.rs"]
mod command_palette_footer;
#[path = "showcase/command_palette_group.rs"]
mod command_palette_group;
#[path = "showcase/command_palette_input.rs"]
mod command_palette_input;
#[path = "showcase/command_palette_item.rs"]
mod command_palette_item;
#[path = "showcase/command_palette_list.rs"]
mod command_palette_list;
#[path = "showcase/dialog_header.rs"]
mod dialog_header;
#[path = "showcase/divider.rs"]
mod divider;
#[path = "showcase/drawer_header.rs"]
mod drawer_header;
#[path = "showcase/dropdown_menu_checkbox_item.rs"]
mod dropdown_menu_checkbox_item;
#[path = "showcase/dropdown_menu_group.rs"]
mod dropdown_menu_group;
#[path = "showcase/dropdown_menu_item.rs"]
mod dropdown_menu_item;
#[path = "showcase/dropdown_menu_radio_group.rs"]
mod dropdown_menu_radio_group;
#[path = "showcase/empty_state.rs"]
mod empty_state;
#[path = "showcase/field.rs"]
mod field;
#[path = "showcase/field_label.rs"]
mod field_label;
#[path = "showcase/field_status.rs"]
mod field_status;
#[path = "showcase/form_layout.rs"]
mod form_layout;
#[path = "showcase/grid.rs"]
mod grid;
#[path = "showcase/h_stack.rs"]
mod h_stack;
#[path = "showcase/heading.rs"]
mod heading;
#[path = "showcase/icon_button.rs"]
mod icon_button;
#[path = "showcase/info_tip.rs"]
mod info_tip;
#[path = "showcase/input_group.rs"]
mod input_group;
#[path = "showcase/input_group_text.rs"]
mod input_group_text;
#[path = "showcase/item.rs"]
mod item;
#[path = "showcase/kbd.rs"]
mod kbd;
#[path = "showcase/layout.rs"]
mod layout;
#[path = "showcase/layout_content.rs"]
mod layout_content;
#[path = "showcase/layout_footer.rs"]
mod layout_footer;
#[path = "showcase/layout_header.rs"]
mod layout_header;
#[path = "showcase/layout_panel.rs"]
mod layout_panel;
#[path = "showcase/link.rs"]
mod link;
#[path = "showcase/link_provider.rs"]
mod link_provider;
#[path = "showcase/list.rs"]
mod list;
#[path = "showcase/media_theme.rs"]
mod media_theme;
#[path = "showcase/metadata_list.rs"]
mod metadata_list;
#[path = "showcase/metadata_list_item.rs"]
mod metadata_list_item;
#[path = "showcase/mobile_nav_toggle.rs"]
mod mobile_nav_toggle;
#[path = "showcase/nav_heading_menu_item.rs"]
mod nav_heading_menu_item;
#[path = "showcase/nav_icon.rs"]
mod nav_icon;
#[path = "showcase/outline.rs"]
mod outline;
#[path = "showcase/overlay.rs"]
mod overlay;
#[path = "showcase/pagination.rs"]
mod pagination;
#[path = "showcase/progress_bar.rs"]
mod progress_bar;
#[path = "showcase/radio_indicator.rs"]
mod radio_indicator;
#[path = "showcase/radio_list.rs"]
mod radio_list;
#[path = "showcase/resize_handle.rs"]
mod resize_handle;
#[path = "showcase/scrollable_area.rs"]
mod scrollable_area;
#[path = "showcase/section.rs"]
mod section;
#[path = "showcase/selectable_card.rs"]
mod selectable_card;
#[path = "showcase/selector_option.rs"]
mod selector_option;
#[path = "showcase/side_nav_collapse_button.rs"]
mod side_nav_collapse_button;
#[path = "showcase/side_nav_heading.rs"]
mod side_nav_heading;
#[path = "showcase/side_nav_item.rs"]
mod side_nav_item;
#[path = "showcase/side_nav_section.rs"]
mod side_nav_section;
#[path = "showcase/skeleton.rs"]
mod skeleton;
#[path = "showcase/spinner.rs"]
mod spinner;
#[path = "showcase/stack.rs"]
mod stack;
#[path = "showcase/status_dot.rs"]
mod status_dot;
#[path = "showcase/switch.rs"]
mod switch;
#[path = "showcase/syntax_theme.rs"]
mod syntax_theme;
#[path = "showcase/table.rs"]
mod table;
#[path = "showcase/table_header.rs"]
mod table_header;
#[path = "showcase/table_selection_toolbar.rs"]
mod table_selection_toolbar;
#[path = "showcase/text.rs"]
mod text;
#[path = "showcase/text_area.rs"]
mod text_area;
#[path = "showcase/text_input.rs"]
mod text_input;
#[path = "showcase/thumbnail.rs"]
mod thumbnail;
#[path = "showcase/timer.rs"]
mod timer;
#[path = "showcase/toggle_button.rs"]
mod toggle_button;
#[path = "showcase/toggle_button_group.rs"]
mod toggle_button_group;
#[path = "showcase/token.rs"]
mod token;
#[path = "showcase/toolbar.rs"]
mod toolbar;
#[path = "showcase/tooltip.rs"]
mod tooltip;
#[path = "showcase/top_nav_featured_card.rs"]
mod top_nav_featured_card;
#[path = "showcase/top_nav_heading.rs"]
mod top_nav_heading;
#[path = "showcase/top_nav_item.rs"]
mod top_nav_item;
#[path = "showcase/top_nav_mega_menu_item.rs"]
mod top_nav_mega_menu_item;
#[path = "showcase/typeahead_item.rs"]
mod typeahead_item;
#[path = "showcase/typography_animation.rs"]
mod typography_animation;
#[path = "showcase/v_stack.rs"]
mod v_stack;
#[path = "showcase/visually_hidden.rs"]
mod visually_hidden;

use aspect_ratio::AspectRatioExamples;
use avatar::AvatarExamples;
use avatar_group::AvatarGroupExamples;
use avatar_group_overflow::AvatarGroupOverflowExamples;
use avatar_status_dot::AvatarStatusDotExamples;
use badge::BadgeExamples;
use banner::BannerExamples;
use blockquote::BlockquoteExamples;
use breadcrumb_item::BreadcrumbItemExamples;
use breadcrumbs::BreadcrumbsExamples;
use button_contract::ButtonContractExamples;
use button_group::ButtonGroupExamples;
use button_patterns::DocumentedButtonPatterns;
use card::CardExamples;
use center::CenterExamples;
use chat_dictation_button::ChatDictationButtonExamples;
use chat_layout::ChatLayoutExamples;
use chat_layout_scroll_button::ChatLayoutScrollButtonExamples;
use chat_message::ChatMessageExamples;
use chat_message_bubble::ChatMessageBubbleExamples;
use chat_message_list::ChatMessageListExamples;
use chat_message_metadata::ChatMessageMetadataExamples;
use chat_send_button::ChatSendButtonExamples;
use chat_system_message::ChatSystemMessageExamples;
use chat_tokenized_text::ChatTokenizedTextExamples;
use chat_tool_calls::ChatToolCallsExamples;
use check_indicator::CheckIndicatorExamples;
use checkbox_indicator::CheckboxIndicatorExamples;
use checkbox_input::CheckboxInputExamples;
use checkbox_list::CheckboxListExamples;
use citation::CitationExamples;
use clickable_card::ClickableCardExamples;
use collapsible::CollapsibleExamples;
use command_palette_empty::CommandPaletteEmptyExamples;
use command_palette_footer::CommandPaletteFooterExamples;
use command_palette_group::CommandPaletteGroupExamples;
use command_palette_input::CommandPaletteInputExamples;
use command_palette_item::CommandPaletteItemExamples;
use command_palette_list::CommandPaletteListExamples;
use dialog_header::DialogHeaderExamples;
use divider::DividerExamples;
use drawer_header::DrawerHeaderExamples;
use dropdown_menu_checkbox_item::DropdownMenuCheckboxItemExamples;
use dropdown_menu_group::DropdownMenuGroupExamples;
use dropdown_menu_item::DropdownMenuItemExamples;
use dropdown_menu_radio_group::DropdownMenuRadioGroupExamples;
use empty_state::EmptyStateExamples;
use field::FieldExamples;
use field_label::FieldLabelExamples;
use field_status::FieldStatusExamples;
use form_layout::FormLayoutExamples;
use grid::{GridExamples, GridSpanExamples};
use h_stack::HStackExamples;
use heading::HeadingExamples;
use icon_button::IconButtonExamples;
use info_tip::InfoTipExamples;
use input_group::InputGroupExamples;
use input_group_text::InputGroupTextExamples;
use item::ItemExamples;
use kbd::KbdExamples;
use layout::LayoutExamples;
use layout_content::LayoutContentExamples;
use layout_footer::LayoutFooterExamples;
use layout_header::LayoutHeaderExamples;
use layout_panel::LayoutPanelExamples;
use link::LinkExamples;
use link_provider::LinkProviderExamples;
use list::ListExamples;
use media_theme::MediaThemeExamples;
use metadata_list::MetadataListExamples;
use metadata_list_item::MetadataListItemExamples;
use mobile_nav_toggle::MobileNavToggleExamples;
use nav_heading_menu_item::NavHeadingMenuItemExamples;
use nav_icon::NavIconExamples;
use outline::OutlineExamples;
use overlay::OverlayExamples;
use pagination::PaginationExamples;
use progress_bar::ProgressBarExamples;
use radio_indicator::RadioIndicatorExamples;
use radio_list::RadioListExamples;
use resize_handle::ResizeHandleExamples;
use scrollable_area::ScrollableAreaExamples;
use section::SectionExamples;
use selectable_card::SelectableCardExamples;
use selector_option::SelectorOptionExamples;
use side_nav_collapse_button::SideNavCollapseButtonExamples;
use side_nav_heading::SideNavHeadingExamples;
use side_nav_item::SideNavItemExamples;
use side_nav_section::SideNavSectionExamples;
use skeleton::SkeletonExamples;
use spinner::SpinnerExamples;
use stack::{StackExamples, StackItemExamples};
use status_dot::StatusDotExamples;
use switch::SwitchExamples;
use syntax_theme::SyntaxThemeExamples;
use table::TableExamples;
use table_header::TableHeaderExamples;
use table_selection_toolbar::TableSelectionToolbarExamples;
use text::TextExamples;
use text_area::TextAreaExamples;
use text_input::TextInputExamples;
use thumbnail::ThumbnailExamples;
use timer::TimerExamples;
use toggle_button::ToggleButtonExamples;
use toggle_button_group::ToggleButtonGroupExamples;
use token::TokenExamples;
use toolbar::ToolbarExamples;
use tooltip::TooltipExamples;
use top_nav_featured_card::TopNavFeaturedCardExamples;
use top_nav_heading::TopNavHeadingExamples;
use top_nav_item::TopNavItemExamples;
use top_nav_mega_menu_item::TopNavMegaMenuItemExamples;
use typeahead_item::TypeaheadItemExamples;
use v_stack::VStackExamples;
use visually_hidden::VisuallyHiddenExamples;

const FIGTREE: Asset = asset!("/assets/fonts/Figtree-latin.woff2");

fn main() {
    if let Err(err) = link::register_neutral_icons() {
        eprintln!("Showcase icon setup failed: {err}");
        return;
    }
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    if typography_animation::is_requested() {
        return rsx! { typography_animation::TypographyAnimation {} };
    }
    let mut count = use_signal(|| 0);
    let mut ancestor_count = use_signal(|| 0);
    let mut form_count = use_signal(|| 0);
    let mut is_loading = use_signal(|| false);
    let variants = [
        (ButtonVariant::Primary, "Primary"),
        (ButtonVariant::Secondary, "Secondary"),
        (ButtonVariant::Ghost, "Ghost"),
        (ButtonVariant::Destructive, "Destructive"),
    ];
    let sizes = [
        (ButtonSize::Small, "Small"),
        (ButtonSize::Medium, "Medium"),
        (ButtonSize::Large, "Large"),
    ];

    rsx! {
        document::Title { "Astryx Dioxus Showcase" }
        document::Style { "{STYLES}" }
        document::Style { "@font-face {{font-family:Figtree;src:url('{FIGTREE}') format('woff2');font-style:normal;font-weight:300 900;font-display:swap;}}" }
        document::Style { "
            body {{ font-family: Figtree, system-ui, sans-serif; margin: 0; background: #f8f4ed; }}
            main {{ max-width: 56rem; margin: 3rem auto; padding: 1.5rem; }}
            section {{ margin-block: 2rem; }}
            .showcase-row {{ display: flex; flex-wrap: wrap; align-items: center; gap: 1rem; }}
            .demo-count {{ font-size:12px;line-height:20px;padding:0 6px;border-radius:9999px;background:light-dark(#e2e2e2,#303030);color:light-dark(#1b1b1b,#d4d4d4); }}
            .demo-count--info {{ background:light-dark(#b9d7ff,#88bcff3d);color:light-dark(#003978,#a1caff); }}
            .demo-count--warning {{ background:light-dark(#fae19e,#e2b6233d);color:light-dark(#4b3900,#eec448); }}
        " }
        main { "data-astryx-theme": "astryx",
            h1 { "Astryx Dioxus Showcase" }
            p { "Documented patterns, interaction states, and integrations. See docs/ for each component's verification notes." }
            nav { "aria-label": "Showcase sections",
                a { href: "#button-group-examples", "ButtonGroup examples" }
                " · "
                a { href: "#icon-button-examples", "IconButton examples" }
                " · "
                a { href: "#link-examples", "Link examples" }
                " · "
                a { href: "#link-provider-examples", "LinkProvider examples" }
                " · "
                a { href: "#list-examples", "List examples" }
                " · "
                a { href: "#item-examples", "Item examples" }
                " · "
                a { href: "#selector-option-examples", "SelectorOption examples" }
                " · "
                a { href: "#layout-header-examples", "LayoutHeader examples" }
                " · "
                a { href: "#layout-footer-examples", "LayoutFooter examples" }
                " · "
                a { href: "#layout-content-examples", "LayoutContent examples" }
                " · "
                a { href: "#layout-panel-examples", "LayoutPanel examples" }
                " · "
                a { href: "#layout-examples", "Layout examples" }
                " · "
                a { href: "#spinner-examples", "Spinner examples" }
                " · "
                a { href: "#badge-examples", "Badge examples" }
                " · "
                a { href: "#banner-examples", "Banner examples" }
                " · "
                a { href: "#avatar-status-dot-examples", "AvatarStatusDot examples" }
                " · "
                a { href: "#avatar-examples", "Avatar examples" }
                " · "
                a { href: "#avatar-group-overflow-examples", "AvatarGroupOverflow examples" }
                " · "
                a { href: "#avatar-group-examples", "AvatarGroup examples" }
                " · "
                a { href: "#divider-examples", "Divider examples" }
                " · "
                a { href: "#drawer-header-examples", "DrawerHeader examples" }
                " · "
                a { href: "#dialog-header-examples", "DialogHeader examples" }
                " · "
                a { href: "#dropdown-menu-divider-example", "DropdownMenuDivider example" }
                " · "
                a { href: "#dropdown-menu-group-examples", "DropdownMenuGroup examples" }
                " · "
                a { href: "#dropdown-menu-radio-group-examples", "DropdownMenuRadioGroup examples" }
                " · "
                a { href: "#dropdown-menu-checkbox-item-examples", "DropdownMenuCheckboxItem examples" }
                " · "
                a { href: "#dropdown-menu-item-examples", "DropdownMenuItem examples" }
                " · "
                a { href: "#field-status-examples", "FieldStatus examples" }
                " · "
                a { href: "#field-label-examples", "FieldLabel examples" }
                " · "
                a { href: "#field-examples", "Field examples" }
                " · "
                a { href: "#text-area-examples", "TextArea examples" }
                " · "
                a { href: "#text-input-examples", "TextInput examples" }
                " · "
                a { href: "#input-group-text-examples", "InputGroupText examples" }
                " · "
                a { href: "#input-group-examples", "InputGroup examples" }
                " · "
                a { href: "#form-layout-examples", "FormLayout examples" }
                " · "
                a { href: "#thumbnail-examples", "Thumbnail examples" }
                " · "
                a { href: "#timer-examples", "Timer examples" }
                " · "
                a { href: "#typeahead-item-examples", "TypeaheadItem examples" }
                " · "
                a { href: "#table-header-examples", "TableHeader examples" }
                " · "
                a { href: "#table-footer-examples", "TableBody and TableFooter examples" }
                " · "
                a { href: "#table-examples", "Table examples" }
                " · "
                a { href: "#table-selection-toolbar-examples", "TableSelectionToolbar examples" }
                " · "
                a { href: "#toggle-button-examples", "ToggleButton examples" }
                " · "
                a { href: "#toolbar-examples", "Toolbar examples" }
                " · "
                a { href: "#toggle-button-group-examples", "ToggleButtonGroup examples" }
                " · "
                a { href: "#token-examples", "Token examples" }
                " · "
                a { href: "#breadcrumb-item-examples", "BreadcrumbItem examples" }
                " · "
                a { href: "#breadcrumbs-examples", "Breadcrumbs examples" }
                " · "
                a { href: "#empty-state-examples", "EmptyState examples" }
                " · "
                a { href: "#metadata-list-item-examples", "MetadataListItem examples" }
                " · "
                a { href: "#metadata-list-examples", "MetadataList examples" }
                " · "
                a { href: "#media-theme-examples", "MediaTheme examples" }
                " · "
                a { href: "#syntax-theme-examples", "SyntaxTheme examples" }
                " · "
                a { href: "#tooltip-examples", "Tooltip examples" }
                " · "
                a { href: "#info-tip-examples", "InfoTip examples" }
                " · "
                a { href: "#overlay-examples", "Overlay examples" }
                " · "
                a { href: "#kbd-examples", "Kbd examples" }
                " · "
                a { href: "#skeleton-examples", "Skeleton examples" }
                " · "
                a { href: "#status-dot-examples", "StatusDot examples" }
                " · "
                a { href: "#progress-bar-examples", "ProgressBar examples" }
                " · "
                a { href: "#blockquote-examples", "Blockquote examples" }
                " · "
                a { href: "#card-examples", "Card examples" }
                " · "
                a { href: "#collapsible-examples", "Collapsible examples" }
                " · "
                a { href: "#chat-system-message-examples", "ChatSystemMessage examples" }
                " · "
                a { href: "#chat-send-button-examples", "ChatSendButton examples" }
                " · "
                a { href: "#chat-message-metadata-examples", "ChatMessageMetadata examples" }
                " · "
                a { href: "#chat-message-examples", "ChatMessage examples" }
                " · "
                a { href: "#chat-message-bubble-examples", "ChatMessageBubble examples" }
                " · "
                a { href: "#chat-message-list-examples", "ChatMessageList examples" }
                " · "
                a { href: "#chat-layout-scroll-button-examples", "ChatLayoutScrollButton examples" }
                " · "
                a { href: "#chat-layout-examples", "ChatLayout examples" }
                " · "
                a { href: "#chat-dictation-button-examples", "ChatDictationButton examples" }
                " · "
                a { href: "#chat-tokenized-text-examples", "ChatTokenizedText examples" }
                " · "
                a { href: "#chat-tool-calls-examples", "ChatToolCalls examples" }
                " · "
                a { href: "#aspect-ratio-examples", "AspectRatio examples" }
                " · "
                a { href: "#stack-examples", "Stack examples" }
                " · "
                a { href: "#stack-item-examples", "StackItem examples" }
                " · "
                a { href: "#grid-examples", "Grid examples" }
                " · "
                a { href: "#outline-examples", "Outline examples" }
                " · "
                a { href: "#pagination-examples", "Pagination examples" }
                " · "
                a { href: "#resize-handle-examples", "ResizeHandle examples" }
                " · "
                a { href: "#grid-span-examples", "GridSpan examples" }
                " · "
                a { href: "#h-stack-examples", "HStack examples" }
                " · "
                a { href: "#v-stack-examples", "VStack examples" }
                " · "
                a { href: "#visually-hidden-examples", "VisuallyHidden examples" }
                " · "
                a { href: "#center-examples", "Center examples" }
                " · "
                a { href: "#citation-examples", "Citation examples" }
                " · "
                a { href: "#command-palette-empty-examples", "CommandPaletteEmpty examples" }
                " · "
                a { href: "#command-palette-group-examples", "CommandPaletteGroup examples" }
                " · "
                a { href: "#command-palette-input-examples", "CommandPaletteInput examples" }
                " · "
                a { href: "#command-palette-footer-examples", "CommandPaletteFooter examples" }
                " · "
                a { href: "#command-palette-list-examples", "CommandPaletteList examples" }
                " · "
                a { href: "#command-palette-item-examples", "CommandPaletteItem examples" }
                " · "
                a { href: "#check-indicator-examples", "CheckIndicator examples" }
                " · "
                a { href: "#radio-indicator-examples", "RadioIndicator examples" }
                " · "
                a { href: "#checkbox-indicator-examples", "CheckboxIndicator examples" }
                " · "
                a { href: "#checkbox-input-examples", "CheckboxInput examples" }
                " · "
                a { href: "#switch-examples", "Switch examples" }
                " · "
                a { href: "#checkbox-list-examples", "CheckboxList examples" }
                " · "
                a { href: "#radio-list-examples", "RadioList examples" }
                " · "
                a { href: "#section-examples", "Section examples" }
                " · "
                a { href: "#side-nav-heading-examples", "SideNavHeading examples" }
                " · "
                a { href: "#side-nav-item-examples", "SideNavItem examples" }
                " · "
                a { href: "#mobile-nav-toggle-examples", "MobileNavToggle examples" }
                " · "
                a { href: "#side-nav-section-examples", "SideNavSection examples" }
                " · "
                a { href: "#side-nav-collapse-button-examples", "SideNavCollapseButton examples" }
                " · "
                a { href: "#top-nav-heading-examples", "TopNavHeading examples" }
                " · "
                a { href: "#top-nav-item-examples", "TopNavItem examples" }
                " · "
                a { href: "#top-nav-featured-card-examples", "TopNavMegaMenuFeaturedCard examples" }
                " · "
                a { href: "#top-nav-mega-menu-item-examples", "TopNavMegaMenuItem examples" }
                " · "
                a { href: "#nav-icon-examples", "NavIcon examples" }
                " · "
                a { href: "#nav-heading-menu-item-examples", "NavHeadingMenuItem examples" }
                " · "
                a { href: "#scrollable-area-examples", "ScrollableArea examples" }
            }
            DocumentedButtonPatterns {}
            section { "aria-labelledby": "variants-heading",
                h2 { id: "variants-heading", "Variants" }
                div { class: "showcase-row",
                    {variants.iter().map(|(variant, label)| rsx! {
                        Button { label: *label, variant: *variant }
                    })}
                }
            }
            section { "aria-labelledby": "sizes-heading",
                h2 { id: "sizes-heading", "Sizes" }
                div { class: "showcase-row",
                    {sizes.iter().map(|(size, label)| rsx! {
                        Button { label: *label, size: *size }
                    })}
                }
            }
            section { "aria-labelledby": "states-heading",
                h2 { id: "states-heading", "Interaction and loading" }
                div { class: "showcase-row", onclick: move |_| ancestor_count += 1,
                    Button { id: "counter", label: format!("Clicked {} times", count()), onclick: move |_| count += 1 }
                    Button { id: "disabled", label: "Disabled", is_disabled: true, onclick: move |_| count += 100 }
                    Button { id: "loading", label: "Save", is_loading: true, loading_label: "Loading" }
                    Button { id: "toggle-loading", label: "Toggle loading", onclick: move |_| is_loading.toggle() }
                    Button { id: "controlled-loading", label: "Controlled loading", is_loading: is_loading(), loading_label: "Loading" }
                    Button {
                        id: "async-action",
                        label: "Async action",
                        loading_label: "Loading",
                        click_action: move |_| -> Pin<Box<dyn Future<Output = ()>>> {
                            Box::pin(async move {
                                count += 1;
                                let _ = document::eval("await new Promise(resolve => setTimeout(resolve, 800)); return true;").await;
                            })
                        },
                    }
                    Button {
                        id: "interruptible-action",
                        label: "Interruptible action",
                        is_interruptible: true,
                        loading_label: "Loading",
                        click_action: move |_| -> Pin<Box<dyn Future<Output = ()>>> {
                            Box::pin(async move {
                                count += 1;
                                let _ = document::eval("await new Promise(resolve => setTimeout(resolve, 800)); return true;").await;
                            })
                        },
                    }
                    Button {
                        id: "prevented-action",
                        label: "Prevented action",
                        onclick: move |event: MouseEvent| event.prevent_default(),
                        click_action: move |_| -> Pin<Box<dyn Future<Output = ()>>> {
                            Box::pin(async move { count += 100; })
                        },
                    }
                }
                p { id: "click-count", "Click count: {count}" }
                p { id: "ancestor-count", "Ancestor clicks: {ancestor_count}" }
            }
            section { "aria-labelledby": "content-heading",
                h2 { id: "content-heading", "Content and width" }
                div { class: "showcase-row",
                    Button { label: "Add", icon: rsx! { span { "aria-hidden": "true", "+" } } }
                    Button { id: "icon-only", label: "Add item", is_icon_only: true, icon: rsx! { span { "aria-hidden": "true", "+" } } }
                    Button { label: "Messages", end_content: rsx! { span { "3" } } }
                    Button { label: "Save changes", strong { "Custom content" } }
                    Button { label: "Floating action", elevation: ButtonElevation::High }
                }
                div { style: "width: 140px; margin-top: 1rem;",
                    Button { id: "truncated", label: "A long label that must truncate", width: "100%" }
                }
            }
            section { "aria-labelledby": "links-heading",
                h2 { id: "links-heading", "Links and forms" }
                div { class: "showcase-row",
                    Button { id: "link", label: "Go to variants", href: "#variants-heading" }
                    Button { id: "disabled-link", label: "Disabled link", href: "#variants-heading", is_disabled: true }
                    Button { id: "busy-link", label: "Busy link", href: "#variants-heading", is_loading: true, loading_label: "Loading" }
                }
                form {
                    id: "demo-form",
                    onsubmit: move |event| { event.prevent_default(); form_count += 1; },
                    Button { id: "submit", label: "Submit", button_type: ButtonType::Submit }
                    Button { id: "disabled-submit", label: "Disabled submit", button_type: ButtonType::Submit, is_disabled: true }
                    Button { id: "busy-submit", label: "Busy submit", button_type: ButtonType::Submit, is_loading: true }
                }
                p { id: "form-count", "Submissions: {form_count}" }
            }
            ButtonContractExamples {}
            ButtonGroupExamples {}
            IconButtonExamples {}
            LinkExamples {}
            LinkProviderExamples {}
            ListExamples {}
            ItemExamples {}
            SelectorOptionExamples {}
            MetadataListItemExamples {}
            MetadataListExamples {}
            MediaThemeExamples {}
            SyntaxThemeExamples {}
            TooltipExamples {}
            InfoTipExamples {}
            OverlayExamples {}
            LayoutHeaderExamples {}
            LayoutFooterExamples {}
            LayoutContentExamples {}
            LayoutPanelExamples {}
            LayoutExamples {}
            SpinnerExamples {}
            BadgeExamples {}
            BannerExamples {}
            AvatarStatusDotExamples {}
            AvatarExamples {}
            AvatarGroupOverflowExamples {}
            AvatarGroupExamples {}
            DividerExamples {}
            DrawerHeaderExamples {}
            DialogHeaderExamples {}
            DropdownMenuGroupExamples {}
            DropdownMenuRadioGroupExamples {}
            DropdownMenuCheckboxItemExamples {}
            DropdownMenuItemExamples {}
            section { id: "dropdown-menu-divider-example", "data-astryx-theme": "neutral",
                h2 { "DropdownMenuDivider" }
                div { style: "max-width:240px",
                    div { "Edit" }
                    DropdownMenuDivider { id: "dropdown-menu-divider-demo" }
                    div { "Delete" }
                }
            }
            FieldStatusExamples {}
            FieldLabelExamples {}
            FieldExamples {}
            TextAreaExamples {}
            TextInputExamples {}
            InputGroupTextExamples {}
            InputGroupExamples {}
            FormLayoutExamples {}
            KbdExamples {}
            SkeletonExamples {}
            StatusDotExamples {}
            TextExamples {}
            ThumbnailExamples {}
            TimerExamples {}
            TypeaheadItemExamples {}
            TableHeaderExamples {}
            TableExamples {}
            TableSelectionToolbarExamples {}
            ToggleButtonExamples {}
            ToolbarExamples {}
            ToggleButtonGroupExamples {}
            TokenExamples {}
            BreadcrumbItemExamples {}
            BreadcrumbsExamples {}
            EmptyStateExamples {}
            HeadingExamples {}
            ProgressBarExamples {}
            BlockquoteExamples {}
            CardExamples {}
            CollapsibleExamples {}
            ChatSystemMessageExamples {}
            ChatSendButtonExamples {}
            ChatMessageMetadataExamples {}
            ChatMessageExamples {}
            ChatMessageBubbleExamples {}
            ChatMessageListExamples {}
            ChatLayoutScrollButtonExamples {}
            ChatLayoutExamples {}
            ChatDictationButtonExamples {}
            ChatTokenizedTextExamples {}
            ChatToolCallsExamples {}
            ClickableCardExamples {}
            SelectableCardExamples {}
            AspectRatioExamples {}
            StackExamples {}
            StackItemExamples {}
            GridExamples {}
            OutlineExamples {}
            PaginationExamples {}
            ResizeHandleExamples {}
            GridSpanExamples {}
            HStackExamples {}
            VStackExamples {}
            VisuallyHiddenExamples {}
            CenterExamples {}
            CitationExamples {}
            CommandPaletteEmptyExamples {}
            CommandPaletteGroupExamples {}
            CommandPaletteInputExamples {}
            CommandPaletteFooterExamples {}
            CommandPaletteListExamples {}
            CommandPaletteItemExamples {}
            CheckIndicatorExamples {}
            RadioIndicatorExamples {}
            CheckboxIndicatorExamples {}
            CheckboxInputExamples {}
            SwitchExamples {}
            CheckboxListExamples {}
            RadioListExamples {}
            SectionExamples {}
            SideNavHeadingExamples {}
            SideNavItemExamples {}
            MobileNavToggleExamples {}
            SideNavSectionExamples {}
            SideNavCollapseButtonExamples {}
            TopNavHeadingExamples {}
            TopNavItemExamples {}
            TopNavFeaturedCardExamples {}
            TopNavMegaMenuItemExamples {}
            NavIconExamples {}
            NavHeadingMenuItemExamples {}
            ScrollableAreaExamples {}
        }
    }
}
