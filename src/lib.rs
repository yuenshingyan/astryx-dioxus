//! Reusable Astryx components for Dioxus.
//!
//! Include [`STYLES`] once using `document::Style` in your application root.

mod app_shell;
mod aspect_ratio;
mod avatar;
mod avatar_group;
mod badge;
mod blockquote;
mod breadcrumbs;
mod button;
mod button_group;
mod card;
mod center;
mod chat;
mod citation;
mod clickable_card;
mod code;
mod command_palette;
mod compatibility;
mod dialog;
mod divider;
mod drawer;
mod dropdown_menu;
mod empty_state;
mod field;
mod field_status;
mod form_layout;
mod grid;
mod h_stack;
mod heading;
mod hooks;
mod i18n;
mod icon;
mod icon_button;
mod indicator;
mod input_group;
mod interactive_role_context;
mod kbd;
mod layer;
mod layout;
mod link;
mod metadata_list;
mod mobile_nav;
mod nav_icon;
mod nav_menu;
mod overlay;
mod private_attributes;
mod progress_bar;
mod scrollable_area;
mod section;
mod selectable_card;
mod side_nav;
mod size_context;
mod skeleton;
mod spinner;
mod stack;
mod status_dot;
mod table;
mod text;
mod theme;
mod thumbnail;
mod timer;
mod toggle_button;
mod token;
mod tooltip;
mod top_nav;
mod typeahead;
mod utils;
mod v_stack;
mod visually_hidden;

pub use app_shell::{
    AppShellMobileContext, AppShellMobileProvider, AppShellMobileProviderProps,
    AppShellMobileState, use_app_shell_mobile,
};
pub use aspect_ratio::{AspectRatio, AspectRatioFit, AspectRatioShape};
pub use avatar::{
    Avatar, AvatarPixelSize, AvatarProps, AvatarShape, AvatarSize, AvatarStatusDot,
    AvatarStatusDotProps, AvatarStatusDotVariant, AvatarTooltip, InvalidAvatarStatusDotVariant,
};
pub use avatar_group::{
    AvatarGroup, AvatarGroupOverflow, AvatarGroupOverflowProps, AvatarGroupProps,
};
pub use badge::{Badge, BadgeLabel, BadgeVariant, InvalidBadgeVariant};
pub use blockquote::{Blockquote, BlockquoteCite};
pub use breadcrumbs::{
    BreadcrumbItem, BreadcrumbItemProps, Breadcrumbs, BreadcrumbsProps, BreadcrumbsVariant,
    InvalidBreadcrumbsVariant,
};
pub use button::{Button, ButtonAction, ButtonElevation, ButtonSize, ButtonType, ButtonVariant};
pub use button_group::{ButtonGroup, ButtonGroupElevation, ButtonGroupOrientation};
pub use card::{Card, CardVariant, InvalidCardVariant};
pub use center::{Center, CenterAxis, CenterProps};
pub use chat::{
    ChatDensity, ChatDictationButton, ChatDictationButtonProps, ChatDictationButtonSize,
    ChatDictationState, ChatLayout, ChatLayoutProps, ChatLayoutScrollAction,
    ChatLayoutScrollButton, ChatLayoutScrollButtonProps, ChatMessage, ChatMessageBubble,
    ChatMessageBubbleGroup, ChatMessageBubbleProps, ChatMessageBubbleVariant, ChatMessageList,
    ChatMessageListAlign, ChatMessageListProps, ChatMessageMetadata, ChatMessageMetadataProps,
    ChatMessageProps, ChatMessageSender, ChatMessageStatus, ChatSendButton, ChatSendButtonProps,
    ChatSendButtonSize, ChatSystemMessage, ChatSystemMessageProps, ChatSystemMessageVariant,
};
pub use citation::{Citation, CitationProps, CitationSource, CitationVariant};
pub use clickable_card::{ClickableCard, ClickableCardError};
pub use code::{Code, CodeColor, CodeProps, CodeSize};
pub use command_palette::{
    CommandPaletteEmpty, CommandPaletteEmptyProps, CommandPaletteFooter, CommandPaletteFooterProps,
    CommandPaletteGroup, CommandPaletteGroupProps, CommandPaletteItem, CommandPaletteItemProps,
    CommandPaletteList, CommandPaletteListProps,
};
pub use dialog::{DialogHeader, DialogHeaderEdgeCompensation, DialogHeaderProps};
pub use divider::{
    Divider, DividerLabel, DividerOrientation, DividerVariant, InvalidDividerVariant,
};
pub use drawer::{DrawerHeader, DrawerHeaderEdgeCompensation, DrawerHeaderProps};
pub use dropdown_menu::{
    DropdownMenuDivider, DropdownMenuDivider as BreadcrumbMenuDivider,
    DropdownMenuDivider as ContextMenuDivider, DropdownMenuDividerProps,
    DropdownMenuDividerProps as BreadcrumbMenuDividerProps,
    DropdownMenuDividerProps as ContextMenuDividerProps, DropdownMenuGroup, DropdownMenuGroupProps,
};
pub use empty_state::{EmptyState, EmptyStateProps};
pub use field::{
    Field, FieldLabel, FieldLabelProps, FieldProps, FieldStatusInput, FieldStatusPlacement,
};
pub use field_status::{
    FieldStatus, FieldStatusProps, FieldStatusVariant, InputStatusType, InvalidFieldStatusVariant,
};
pub use form_layout::{
    EntryErrorBehavior, FormLayout, FormLayoutDirection, FormLayoutProps, FormOptionality,
};
pub use grid::{
    Grid, GridAlignment, GridColumns, GridProps, GridRepeat, GridSpan, GridSpanColumns,
    GridSpanProps,
};
pub use h_stack::{HStack, HStackProps};
pub use heading::{Heading, HeadingLevel, HeadingProps, HeadingType, InvalidHeadingLevel};
pub use hooks::use_interactive_role::{InteractiveRole, use_interactive_role};
pub use icon::{
    Icon, IconDefaultSizeContext, IconDefaultSizeProvider, IconEntry, IconRequest, IconSize,
    InvalidIconName, get_extended_icon, register_icons, use_icon_size,
};
pub use icon_button::IconButton;
pub use indicator::{
    CheckIndicator, CheckIndicatorProps, CheckIndicatorState, CheckboxIndicator,
    CheckboxIndicatorProps, CheckboxIndicatorState, IndicatorSize, RadioIndicator,
    RadioIndicatorProps, RadioIndicatorState,
};
pub use input_group::{InputGroup, InputGroupProps, InputGroupText, InputGroupTextProps};
pub use interactive_role_context::{
    InteractiveRoleContext, InteractiveRoleProvider, use_interactive_role_context,
};
pub use kbd::{Kbd, KbdError, KbdText};
pub use layer::{LayerContentBoundary, LayerPlacement};
pub use layout::{
    Layout, LayoutContent, LayoutContentProps, LayoutFooter, LayoutFooterProps, LayoutHeader,
    LayoutHeaderProps, LayoutHeight, LayoutPanel, LayoutPanelProps, LayoutProps,
};
pub use link::{
    Link, LinkComponentType, LinkDownload, LinkProvider, LinkRenderProps, LinkifyCaptures,
    LinkifyError, LinkifyMatch, LinkifyNode, LinkifyPattern, UseLinkifyOptions, linkify,
    use_link_component, use_linkify,
};
pub use metadata_list::{
    MetadataList, MetadataListColumns, MetadataListItem, MetadataListItemProps,
    MetadataListLabelConfig, MetadataListLabelPosition, MetadataListOrientation, MetadataListProps,
};
pub use mobile_nav::{MobileNavToggle, MobileNavToggleProps};
pub use nav_icon::{NavIcon, NavIconProps};
pub use nav_menu::{
    NavHeadingMenu, NavHeadingMenuItem, NavHeadingMenuItemProps, NavHeadingMenuProps,
};
pub use overlay::{
    Overlay, OverlayAlign, OverlayPosition, OverlayProps, OverlayScrimMode, OverlayShowOn,
};
pub use progress_bar::{
    InvalidProgressBarVariant, ProgressBar, ProgressBarError, ProgressBarMark, ProgressBarState,
    ProgressBarVariant,
};
pub use scrollable_area::{
    ScrollAxis, ScrollOverscroll, ScrollableArea, ScrollableAreaProps, ScrollableAreaRole,
    ScrollableAreaStickyContainment,
};
pub use section::{InvalidSectionVariant, Section, SectionDivider, SectionProps, SectionVariant};
pub use selectable_card::{SelectableCard, SelectableCardError, SelectableCardProps};
pub use side_nav::{
    SideNavCollapseButton, SideNavCollapseButtonProps, SideNavControlledCollapsible,
    SideNavHeading, SideNavHeadingProps, SideNavSection, SideNavSectionProps,
};
pub use size_context::{ElementSize, SizeContext, SizeProvider, use_size};
pub use skeleton::{Skeleton, SkeletonError, SkeletonLength, SkeletonRadius};
pub use spinner::{Spinner, SpinnerError, SpinnerShade, SpinnerSize};
pub use stack::{
    Stack, StackAlignment, StackCrossAlignment, StackDirection, StackElement, StackItem,
    StackItemCrossAlignSelf, StackItemProps, StackItemRoot, StackItemSize, StackMainAlignment,
    StackProps, StackRenderProps, StackWrap,
};
pub use status_dot::{
    InvalidStatusDotVariant, StatusDot, StatusDotError, StatusDotIcon, StatusDotVariant,
};
pub use table::{
    Table, TableBody, TableBodyProps, TableCell, TableCellProps, TableCellScope, TableDensity,
    TableDividers, TableFooter, TableFooterProps, TableHeader, TableHeaderCell,
    TableHeaderCellProps, TableHeaderProps, TableProps, TableRow, TableRowProps, TableTextOverflow,
    TableVerticalAlign,
};
pub use text::{Text, TextTag, Truncation, use_truncation};
pub use theme::{
    InvalidTextName, TextColor, TextDisplay, TextJustify, TextSize, TextType, TextWeight, TextWrap,
    WordBreak,
};
pub use theme::{MediaTheme, MediaThemeFallback, MediaThemeMode, MediaThemeProps};
pub use thumbnail::{Thumbnail, ThumbnailProps, ThumbnailRemoveVisibility};
pub use timer::{Timer, TimerFormat, TimerProps};
pub use toggle_button::{
    ToggleButton, ToggleButtonGroup, ToggleButtonGroupProps, ToggleButtonGroupSelection,
    ToggleButtonProps,
};
pub use token::{InvalidTokenColor, Token, TokenColor, TokenProps, TokenSize};
pub use top_nav::{
    TopNavHeading, TopNavHeadingProps, TopNavMegaMenuFeaturedCard, TopNavMegaMenuFeaturedCardProps,
    TopNavMegaMenuItem, TopNavMegaMenuItemProps,
};
pub use typeahead::{TypeaheadItem, TypeaheadItemContent, TypeaheadItemProps};

pub use utils::types::{Elevation, SizeValue, SizeValueError, SpacingStep};
pub use v_stack::{VStack, VStackProps};
pub use visually_hidden::{
    VisuallyHidden, VisuallyHiddenElement, VisuallyHiddenProps, VisuallyHiddenRenderProps,
};

// Compatibility spellings of provisional APIs. Core code uses upstream owners.
pub use compatibility::{
    ButtonEnvironment, ButtonEnvironmentContext, ButtonEnvironmentProvider, use_button_environment,
};
pub use link::{
    ButtonLinkProvider, LinkComponentType as ButtonLinkRenderer, LinkRenderProps as ButtonLinkProps,
};
pub use size_context::{ElementSize as ControlSize, SizeContext as ButtonSizeContext};

/// Base styles and design tokens. Include once in your application root.
pub const STYLES: &str = include_str!("../assets/astryx.css");
