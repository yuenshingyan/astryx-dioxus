# Astryx Component Migration Checklist

Source: [Astryx component catalog](https://astryx.atmeta.com/components).

107 tasks, one for each component card in the catalog, grouped in catalog order.
This inventory excludes additional subcomponents, hooks, and utilities that appear only in the documentation sidebar. Category order is not implementation priority.

Each task covers porting the component to Dioxus, matching its original styles and behavior, preserving accessibility, adding tests and showcase examples, and comparing against the original before marking it complete. Follow [AGENTS.md](AGENTS.md) and the [README migration workflow](README.md#migration-workflow).

All tasks start unchecked. The existing Button is a starter implementation, not a completed migration. Bottom Sheet Switcher is deprecated upstream; retain its inventory entry but confirm whether it should be ported before implementing it.

## Action

- [ ] Port [Button](https://astryx.atmeta.com/components/Button).
- [ ] Port [Button Group](https://astryx.atmeta.com/components/ButtonGroup).
- [ ] Port [Dropdown Menu](https://astryx.atmeta.com/components/DropdownMenu).
- [ ] Port [Icon Button](https://astryx.atmeta.com/components/IconButton).
- [ ] Port [Link](https://astryx.atmeta.com/components/Link).
- [ ] Port [More Menu](https://astryx.atmeta.com/components/MoreMenu).
- [ ] Port [Segmented Control](https://astryx.atmeta.com/components/SegmentedControl).
- [ ] Port [Toggle Button](https://astryx.atmeta.com/components/ToggleButton).
- [ ] Port [Toggle Button Group](https://astryx.atmeta.com/components/ToggleButtonGroup).
- [ ] Port [Toolbar](https://astryx.atmeta.com/components/Toolbar).

## Chat

- [ ] Port [Chat Composer](https://astryx.atmeta.com/components/ChatComposer).
- [ ] Port [Chat Layout](https://astryx.atmeta.com/components/ChatLayout).
- [ ] Port [Chat Message](https://astryx.atmeta.com/components/ChatMessage).
- [ ] Port [Chat Message Metadata](https://astryx.atmeta.com/components/ChatMessageMetadata).
- [ ] Port [Chat System Message](https://astryx.atmeta.com/components/ChatSystemMessage).
- [ ] Port [Chat Tool Calls](https://astryx.atmeta.com/components/ChatToolCalls).

## Container

- [ ] Port [Card](https://astryx.atmeta.com/components/Card).
- [ ] Port [Carousel](https://astryx.atmeta.com/components/Carousel).
- [ ] Port [Clickable Card](https://astryx.atmeta.com/components/ClickableCard).
- [ ] Port [Collapsible](https://astryx.atmeta.com/components/Collapsible).
- [ ] Port [Selectable Card](https://astryx.atmeta.com/components/SelectableCard).

## Content

- [ ] Port [Avatar](https://astryx.atmeta.com/components/Avatar).
- [ ] Port [Avatar Group](https://astryx.atmeta.com/components/AvatarGroup).
- [ ] Port [Blockquote](https://astryx.atmeta.com/components/Blockquote).
- [ ] Port [Citation](https://astryx.atmeta.com/components/Citation).
- [ ] Port [Code](https://astryx.atmeta.com/components/Code).
- [ ] Port [Code Block](https://astryx.atmeta.com/components/CodeBlock).
- [ ] Port [Empty State](https://astryx.atmeta.com/components/EmptyState).
- [ ] Port [Heading](https://astryx.atmeta.com/components/Heading).
- [ ] Port [Icon](https://astryx.atmeta.com/components/Icon).
- [ ] Port [Kbd](https://astryx.atmeta.com/components/Kbd).
- [ ] Port [Markdown](https://astryx.atmeta.com/components/Markdown).
- [ ] Port [Text](https://astryx.atmeta.com/components/Text).
- [ ] Port [Thumbnail](https://astryx.atmeta.com/components/Thumbnail).
- [ ] Port [Timer](https://astryx.atmeta.com/components/Timer).
- [ ] Port [Timestamp](https://astryx.atmeta.com/components/Timestamp).
- [ ] Port [Token](https://astryx.atmeta.com/components/Token).
- [ ] Port [Rich Text View](https://astryx.atmeta.com/components/RichTextView).

## Feedback & Status

- [ ] Port [Badge](https://astryx.atmeta.com/components/Badge).
- [ ] Port [Banner](https://astryx.atmeta.com/components/Banner).
- [ ] Port [Info Tip](https://astryx.atmeta.com/components/InfoTip).
- [ ] Port [Progress Bar](https://astryx.atmeta.com/components/ProgressBar).
- [ ] Port [Skeleton](https://astryx.atmeta.com/components/Skeleton).
- [ ] Port [Spinner](https://astryx.atmeta.com/components/Spinner).
- [ ] Port [Status Dot](https://astryx.atmeta.com/components/StatusDot).

## Form Controls

- [ ] Port [Calendar](https://astryx.atmeta.com/components/Calendar).
- [ ] Port [Checkbox Input](https://astryx.atmeta.com/components/CheckboxInput).
- [ ] Port [Complex Selector](https://astryx.atmeta.com/components/ComplexSelector).
- [ ] Port [Date Input](https://astryx.atmeta.com/components/DateInput).
- [ ] Port [Date Range Input](https://astryx.atmeta.com/components/DateRangeInput).
- [ ] Port [Date Time Input](https://astryx.atmeta.com/components/DateTimeInput).
- [ ] Port [Field](https://astryx.atmeta.com/components/Field).
- [ ] Port [File Input](https://astryx.atmeta.com/components/FileInput).
- [ ] Port [Multi Selector](https://astryx.atmeta.com/components/MultiSelector).
- [ ] Port [Number Input](https://astryx.atmeta.com/components/NumberInput).
- [ ] Port [Power Search](https://astryx.atmeta.com/components/PowerSearch).
- [ ] Port [Radio List](https://astryx.atmeta.com/components/RadioList).
- [ ] Port [Selector](https://astryx.atmeta.com/components/Selector).
- [ ] Port [Slider](https://astryx.atmeta.com/components/Slider).
- [ ] Port [Switch](https://astryx.atmeta.com/components/Switch).
- [ ] Port [Text Area](https://astryx.atmeta.com/components/TextArea).
- [ ] Port [Text Input](https://astryx.atmeta.com/components/TextInput).
- [ ] Port [Time Input](https://astryx.atmeta.com/components/TimeInput).
- [ ] Port [Tokenizer](https://astryx.atmeta.com/components/Tokenizer).
- [ ] Port [Typeahead](https://astryx.atmeta.com/components/Typeahead).
- [ ] Port [Typeahead Item](https://astryx.atmeta.com/components/TypeaheadItem).
- [ ] Port [Rich Text Editor](https://astryx.atmeta.com/components/RichTextEditor).
- [ ] Port [Rich Text Editor Auto-Link Plugin](https://astryx.atmeta.com/components/RichTextEditorAutoLinkPlugin).
- [ ] Port [Rich Text Editor Toolbar](https://astryx.atmeta.com/components/RichTextEditorToolbar).

## Layout

- [ ] Port [App Shell](https://astryx.atmeta.com/components/AppShell).
- [ ] Port [Aspect Ratio](https://astryx.atmeta.com/components/AspectRatio).
- [ ] Port [Divider](https://astryx.atmeta.com/components/Divider).
- [ ] Port [Form Layout](https://astryx.atmeta.com/components/FormLayout).
- [ ] Port [Grid](https://astryx.atmeta.com/components/Grid).
- [ ] Port [Layout](https://astryx.atmeta.com/components/Layout).
- [ ] Port [Resize Handle](https://astryx.atmeta.com/components/ResizeHandle).
- [ ] Port [Scrollable Area](https://astryx.atmeta.com/components/ScrollableArea).
- [ ] Port [Section](https://astryx.atmeta.com/components/Section).
- [ ] Port [Stack](https://astryx.atmeta.com/components/Stack).

## Navigation

- [ ] Port [Breadcrumbs](https://astryx.atmeta.com/components/Breadcrumbs).
- [ ] Port [Outline](https://astryx.atmeta.com/components/Outline).
- [ ] Port [Pagination](https://astryx.atmeta.com/components/Pagination).
- [ ] Port [Side Nav](https://astryx.atmeta.com/components/SideNav).
- [ ] Port [Stepper](https://astryx.atmeta.com/components/Stepper).
- [ ] Port [Tab List](https://astryx.atmeta.com/components/TabList).
- [ ] Port [Top Nav](https://astryx.atmeta.com/components/TopNav).
- [ ] Port [Top Nav Mega Menu](https://astryx.atmeta.com/components/TopNavMegaMenu).
- [ ] Port [Top Nav Mega Menu Featured Card](https://astryx.atmeta.com/components/TopNavMegaMenuFeaturedCard).
- [ ] Port [Top Nav Menu](https://astryx.atmeta.com/components/TopNavMenu).

## Overlay

- [ ] Port [Bottom Sheet](https://astryx.atmeta.com/components/BottomSheet).
- [ ] Resolve migration of [Bottom Sheet Switcher](https://astryx.atmeta.com/components/BottomSheetSwitcher) — **deprecated upstream**; confirm inclusion before porting.
- [ ] Port [Command Palette](https://astryx.atmeta.com/components/CommandPalette).
- [ ] Port [Dialog](https://astryx.atmeta.com/components/Dialog).
- [ ] Port [Drawer](https://astryx.atmeta.com/components/Drawer).
- [ ] Port [Hover Card](https://astryx.atmeta.com/components/HoverCard).
- [ ] Port [Lightbox](https://astryx.atmeta.com/components/Lightbox).
- [ ] Port [Overlay](https://astryx.atmeta.com/components/Overlay).
- [ ] Port [Popover](https://astryx.atmeta.com/components/Popover).
- [ ] Port [Toast](https://astryx.atmeta.com/components/Toast).
- [ ] Port [Tooltip](https://astryx.atmeta.com/components/Tooltip).

## Table & List

- [ ] Port [List](https://astryx.atmeta.com/components/List).
- [ ] Port [Metadata List](https://astryx.atmeta.com/components/MetadataList).
- [ ] Port [Overflow List](https://astryx.atmeta.com/components/OverflowList).
- [ ] Port [Table](https://astryx.atmeta.com/components/Table).
- [ ] Port [Table Selection Toolbar](https://astryx.atmeta.com/components/TableSelectionToolbar).
- [ ] Port [Tree List](https://astryx.atmeta.com/components/TreeList).

## Utility

- [ ] Port [VisuallyHidden](https://astryx.atmeta.com/components/VisuallyHidden).
