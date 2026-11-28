# BreadcrumbItem checkpoint

`src/breadcrumbs/breadcrumb_item.rs` ports the non-menu forms: native list item with an anchor, button, or plain label; an explicitly current item is a noninteractive label with `aria-current="page"`. In a native ordered list, the last eligible item receives the automatic current-page marker unless an explicit current item exists. The browser controller updates that marker when siblings change and disconnects on unmount. The default slash separator is decorative, hidden for the first item, and mirrored in RTL.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Breadcrumbs/BreadcrumbItem.tsx` (Git blob `4d8893673f668c19bc4eff9919708809ab17bb4e`), the item spec, and both catalog templates. The examples use the public [Breadcrumbs](breadcrumbs-port.md) parent and a local home illustration. Menu triggers and AppShell drawer integration remain pending. The checklist stays open.

At the initial checkpoint, all five standard Rust checks passed. Chrome verified both arrangements, keyboard activation, explicit and dynamic automatic current-page markers, and RTL separators. Token and showcase navigation regression suites passed. Published screenshot parity, other browsers, and manual assistive technology were not verified.

```sh
SHOWCASE_URL=http://127.0.0.1:8137 node tests/breadcrumb_item.browser.mjs
```

## Separator follow-up

The pinned `BreadcrumbSeparator` treats an explicit bare `/` like the default
slash. `tests/breadcrumb_item.rs` now verifies that literal and interpolated
slash text use the existing RTL mirror class inside the `aria-hidden` wrapper,
while a custom element containing `/` is left alone. This is SSR markup
coverage, not new browser, visual, or assistive-technology evidence.

The checklist stays open. Beyond menu and drawer integration, spec OQ2 still
needs a decision: whether omitted `is_current` should keep auto-detection and,
if so, how the trail can supply that state at render time instead of inspecting
the DOM after mount.
