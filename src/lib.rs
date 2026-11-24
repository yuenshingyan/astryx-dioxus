//! Reusable Astryx components for Dioxus.
//!
//! Include [`STYLES`] once using `document::Style` in your application root.

mod alert_dialog;
mod app_shell;
mod aspect_ratio;
mod avatar;
mod avatar_group;
mod badge;
mod banner;
mod blockquote;
mod bottom_sheet;
mod breadcrumbs;
mod button;
mod button_group;
mod card;
mod center;
mod chat;
mod checkbox_input;
mod checkbox_list;
mod citation;
mod clickable_card;
mod code;
mod code_block;
mod collapsible;
mod command_palette;
mod compatibility;
mod date_input;
mod date_range_input;
mod date_time_input;
mod dialog;
mod divider;
mod drawer;
mod dropdown_menu;
mod empty_state;
mod field;
mod field_status;
mod file_input;
mod form_layout;
mod grid;
mod h_stack;
mod heading;
mod hooks;
mod hover_card;
mod i18n;
mod icon;
mod icon_button;
mod indicator;
mod info_tip;
mod input_group;
mod interactive_role_context;
mod item;
mod kbd;
mod layer;
mod layout;
mod lightbox;
mod link;
mod list;
mod metadata_list;
mod mobile_nav;
mod nav_icon;
mod nav_menu;
mod number_input;
mod outline;
mod overlay;
mod pagination;
mod popover;
mod private_attributes;
mod progress_bar;
mod radio_list;
mod resizable;
mod scrollable_area;
mod section;
mod segmented_control;
mod selectable_card;
mod selector;
mod side_nav;
mod size_context;
mod skeleton;
mod slider;
mod spinner;
mod stack;
mod status_dot;
mod stepper;
mod switch;
mod tab_list;
mod table;
mod text;
mod text_area;
mod text_input;
mod theme;
mod thumbnail;
mod time_input;
mod timer;
mod timestamp;
mod toggle_button;
mod token;
mod toolbar;
mod tooltip;
mod top_nav;
mod typeahead;
mod utils;
mod v_stack;
mod visually_hidden;

pub use alert_dialog::{AlertDialog, AlertDialogProps};
pub use app_shell::{
    AppShell, AppShellMobileContext, AppShellMobileProvider, AppShellMobileProviderProps,
    AppShellMobileState, AppShellProps, AppShellVariant, use_app_shell_mobile,
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
pub use banner::{Banner, BannerContainer, BannerProps, BannerStatus, InvalidBannerIdentifier};
pub use blockquote::{Blockquote, BlockquoteCite};
pub use bottom_sheet::{BottomSheet, BottomSheetHeight, BottomSheetProps};
pub use breadcrumbs::{
    BreadcrumbItem, BreadcrumbItemProps, Breadcrumbs, BreadcrumbsProps, BreadcrumbsVariant,
    InvalidBreadcrumbsVariant,
};
pub use button::{Button, ButtonAction, ButtonElevation, ButtonSize, ButtonType, ButtonVariant};
pub use button_group::{ButtonGroup, ButtonGroupElevation, ButtonGroupOrientation};
pub use card::{Card, CardVariant, InvalidCardVariant};
pub use center::{Center, CenterAxis, CenterProps};
pub use chat::{
    ChatComposer, ChatComposerDrawer, ChatComposerDrawerProps, ChatComposerInput,
    ChatComposerInputProps, ChatComposerProps, ChatComposerToken, ChatComposerTokenElement,
    ChatComposerTokenElementProps, ChatDensity, ChatDictationButton, ChatDictationButtonProps,
    ChatDictationButtonSize, ChatDictationState, ChatLayout, ChatLayoutProps,
    ChatLayoutScrollAction, ChatLayoutScrollButton, ChatLayoutScrollButtonProps, ChatMessage,
    ChatMessageBubble, ChatMessageBubbleGroup, ChatMessageBubbleProps, ChatMessageBubbleVariant,
    ChatMessageList, ChatMessageListAlign, ChatMessageListProps, ChatMessageMetadata,
    ChatMessageMetadataProps, ChatMessageProps, ChatMessageSender, ChatMessageStatus,
    ChatSendButton, ChatSendButtonProps, ChatSendButtonSize, ChatSystemMessage,
    ChatSystemMessageProps, ChatSystemMessageVariant, ChatTokenizedText, ChatTokenizedTextProps,
    ChatToolCallItem, ChatToolCallStatus, ChatToolCalls, ChatToolCallsProps,
};
pub use checkbox_input::{CheckboxInput, CheckboxInputProps};
pub use checkbox_list::{CheckboxList, CheckboxListItem, CheckboxListItemProps, CheckboxListProps};
pub use citation::{Citation, CitationProps, CitationSource, CitationVariant};
pub use clickable_card::{ClickableCard, ClickableCardError};
pub use code::{Code, CodeColor, CodeProps, CodeSize};
pub use code_block::{CodeBlock, CodeBlockProps};
pub use collapsible::{
    Collapsible, CollapsibleChevronPosition, CollapsibleGroup, CollapsibleGroupDensity,
    CollapsibleGroupProps, CollapsibleGroupValue, CollapsibleProps,
};
pub use command_palette::{
    CommandPaletteEmpty, CommandPaletteEmptyProps, CommandPaletteFooter, CommandPaletteFooterProps,
    CommandPaletteGroup, CommandPaletteGroupProps, CommandPaletteInput, CommandPaletteInputProps,
    CommandPaletteItem, CommandPaletteItemProps, CommandPaletteList, CommandPaletteListProps,
};
pub use date_input::{DateInput, DateInputProps};
pub use date_range_input::{DateRange, DateRangeInput, DateRangeInputProps};
pub use date_time_input::{DateTimeInput, DateTimeInputProps};
pub use dialog::{
    Dialog, DialogHeader, DialogHeaderEdgeCompensation, DialogHeaderProps, DialogProps,
    DialogPurpose, DialogVariant,
};
pub use divider::{
    Divider, DividerLabel, DividerOrientation, DividerVariant, InvalidDividerVariant,
};
pub use drawer::{
    Drawer, DrawerHeader, DrawerHeaderEdgeCompensation, DrawerHeaderProps, DrawerProps, DrawerSide,
};
pub use dropdown_menu::{
    DropdownMenuCheckboxItem, DropdownMenuCheckboxItem as ContextMenuCheckboxItem,
    DropdownMenuCheckboxItemProps, DropdownMenuCheckboxItemProps as ContextMenuCheckboxItemProps,
    DropdownMenuDivider, DropdownMenuDivider as BreadcrumbMenuDivider,
    DropdownMenuDivider as ContextMenuDivider, DropdownMenuDividerProps,
    DropdownMenuDividerProps as BreadcrumbMenuDividerProps,
    DropdownMenuDividerProps as ContextMenuDividerProps, DropdownMenuGroup,
    DropdownMenuGroup as ContextMenuGroup, DropdownMenuGroupProps,
    DropdownMenuGroupProps as ContextMenuGroupProps, DropdownMenuItem,
    DropdownMenuItem as ContextMenuItem, DropdownMenuItemProps,
    DropdownMenuItemProps as ContextMenuItemProps, DropdownMenuItemVariant, DropdownMenuRadioGroup,
    DropdownMenuRadioGroup as ContextMenuRadioGroup, DropdownMenuRadioGroupProps,
    DropdownMenuRadioGroupProps as ContextMenuRadioGroupProps, DropdownMenuRadioItem,
    DropdownMenuRadioItem as ContextMenuRadioItem, DropdownMenuRadioItemProps,
    DropdownMenuRadioItemProps as ContextMenuRadioItemProps,
};
pub use empty_state::{EmptyState, EmptyStateProps};
pub use field::{
    Field, FieldLabel, FieldLabelProps, FieldProps, FieldStatusInput, FieldStatusPlacement,
};
pub use field_status::{
    FieldStatus, FieldStatusProps, FieldStatusVariant, InputStatusType, InvalidFieldStatusVariant,
};
pub use file_input::{FileInput, FileInputProps};
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
pub use hover_card::{HoverCard, HoverCardProps};
pub use i18n::{InternationalizationProvider, InternationalizationProviderProps, MessagesByLocale};
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
pub use info_tip::{InfoTip, InfoTipProps, InfoTipSize};
pub use input_group::{InputGroup, InputGroupProps, InputGroupText, InputGroupTextProps};
pub use interactive_role_context::{
    InteractiveRoleContext, InteractiveRoleProvider, use_interactive_role_context,
};
pub use item::{Item, ItemAlign, ItemDensity, ItemLabel, ItemLayout, ItemProps, ItemTag};
pub use kbd::{Kbd, KbdError, KbdText};
pub use layer::{LayerContentBoundary, LayerPlacement};
pub use layout::{
    Layout, LayoutContent, LayoutContentProps, LayoutFooter, LayoutFooterProps, LayoutHeader,
    LayoutHeaderProps, LayoutHeight, LayoutPanel, LayoutPanelProps, LayoutProps,
};
pub use lightbox::{Lightbox, LightboxMedia, LightboxProps};
pub use link::{
    Link, LinkComponentType, LinkDownload, LinkProvider, LinkRenderProps, LinkifyCaptures,
    LinkifyError, LinkifyMatch, LinkifyNode, LinkifyPattern, UseLinkifyOptions, linkify,
    use_link_component, use_linkify,
};
pub use list::{List, ListDensity, ListEdge, ListItem, ListItemProps, ListMarkerStyle, ListProps};
pub use metadata_list::{
    MetadataList, MetadataListColumns, MetadataListItem, MetadataListItemProps,
    MetadataListLabelConfig, MetadataListLabelPosition, MetadataListOrientation, MetadataListProps,
};
pub use mobile_nav::{
    MobileNav, MobileNavProps, MobileNavSide, MobileNavToggle, MobileNavToggleProps,
};
pub use nav_icon::{NavIcon, NavIconProps};
pub use nav_menu::{
    NavHeadingMenu, NavHeadingMenuItem, NavHeadingMenuItemProps, NavHeadingMenuProps,
};
pub use number_input::{NumberInput, NumberInputProps};
pub use outline::{Outline, OutlineDensity, OutlineItem, OutlineProps};
pub use overlay::{
    Overlay, OverlayAlign, OverlayPosition, OverlayProps, OverlayScrimMode, OverlayShowOn,
};
pub use pagination::{Pagination, PaginationProps, PaginationVariant};
pub use popover::{Popover, PopoverProps, PopoverTriggerRenderProps};
pub use progress_bar::{
    InvalidProgressBarVariant, ProgressBar, ProgressBarError, ProgressBarMark, ProgressBarState,
    ProgressBarVariant,
};
pub use radio_list::{
    RadioList, RadioListItem, RadioListItemProps, RadioListOrientation, RadioListProps,
};
pub use resizable::{ResizeDirection, ResizeHandle, ResizeHandleProps};
pub use scrollable_area::{
    ScrollAxis, ScrollOverscroll, ScrollableArea, ScrollableAreaProps, ScrollableAreaRole,
    ScrollableAreaStickyContainment,
};
pub use section::{InvalidSectionVariant, Section, SectionDivider, SectionProps, SectionVariant};
pub use segmented_control::{
    SegmentedControl, SegmentedControlItem, SegmentedControlItemProps, SegmentedControlLayout,
    SegmentedControlProps,
};
pub use selectable_card::{SelectableCard, SelectableCardError, SelectableCardProps};
pub use selector::{SelectorOption, SelectorOptionProps};
pub use side_nav::{
    SideNav, SideNavCollapseButton, SideNavCollapseButtonProps, SideNavControlledCollapsible,
    SideNavHeading, SideNavHeadingProps, SideNavItem, SideNavItemProps, SideNavProps,
    SideNavSection, SideNavSectionProps,
};
pub use size_context::{ElementSize, SizeContext, SizeProvider, use_size};
pub use skeleton::{Skeleton, SkeletonError, SkeletonLength, SkeletonRadius};
pub use slider::{Slider, SliderOrientation, SliderProps};
pub use spinner::{Spinner, SpinnerError, SpinnerShade, SpinnerSize};
pub use stack::{
    Stack, StackAlignment, StackCrossAlignment, StackDirection, StackElement, StackItem,
    StackItemCrossAlignSelf, StackItemProps, StackItemRoot, StackItemSize, StackMainAlignment,
    StackProps, StackRenderProps, StackWrap,
};
pub use status_dot::{
    InvalidStatusDotVariant, StatusDot, StatusDotError, StatusDotIcon, StatusDotVariant,
};
pub use stepper::{
    Step, StepIndicatorPreset, StepProps, Stepper, StepperOrientation, StepperProps,
};
pub use switch::{Switch, SwitchLabelPosition, SwitchLabelSpacing, SwitchProps};
pub use tab_list::{Tab, TabList, TabListMode, TabListProps, TabMode, TabProps};
pub use table::{
    Table, TableBody, TableBodyProps, TableCell, TableCellProps, TableCellScope, TableDensity,
    TableDividers, TableFooter, TableFooterProps, TableHeader, TableHeaderCell,
    TableHeaderCellProps, TableHeaderProps, TableProps, TableRow, TableRowProps,
    TableSelectionSummary, TableSelectionToolbar, TableSelectionToolbarProps, TableTextOverflow,
    TableVerticalAlign,
};
pub use text::{Text, TextTag, Truncation, use_truncation};
pub use text_area::{TextArea, TextAreaProps};
pub use text_input::{TextInput, TextInputProps, TextInputType};
pub use theme::{
    InvalidSyntaxColor, InvalidThemeName, MediaTheme, MediaThemeFallback, MediaThemeMode,
    MediaThemeProps, SyntaxColor, SyntaxTheme, SyntaxThemeDefinition, SyntaxThemePreset,
    SyntaxThemeProps, SyntaxTokenValue, Theme, ThemeMode, ThemeName, ThemeProps,
};
pub use theme::{
    InvalidTextName, TextColor, TextDisplay, TextJustify, TextSize, TextType, TextWeight, TextWrap,
    WordBreak,
};
pub use thumbnail::{Thumbnail, ThumbnailProps, ThumbnailRemoveVisibility};
pub use time_input::{TimeInput, TimeInputProps};
pub use timer::{Timer, TimerFormat, TimerProps};
pub use timestamp::{Timestamp, TimestampFormat, TimestampProps};
pub use toggle_button::{
    ToggleButton, ToggleButtonGroup, ToggleButtonGroupProps, ToggleButtonGroupSelection,
    ToggleButtonProps,
};
pub use token::{InvalidTokenColor, Token, TokenColor, TokenProps, TokenSize};
pub use toolbar::{Toolbar, ToolbarOrientation, ToolbarProps};
pub use tooltip::{Tooltip, TooltipProps};
pub use top_nav::{
    TopNav, TopNavHeading, TopNavHeadingProps, TopNavItem, TopNavItemProps, TopNavMegaMenu,
    TopNavMegaMenuFeaturedCard, TopNavMegaMenuFeaturedCardProps, TopNavMegaMenuItem,
    TopNavMegaMenuItemProps, TopNavMegaMenuProps, TopNavMenu, TopNavMenuItemData, TopNavMenuProps,
    TopNavProps,
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
