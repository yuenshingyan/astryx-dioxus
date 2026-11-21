# BreadcrumbItem checkpoint

`src/breadcrumbs/breadcrumb_item.rs` ports the non-menu forms: native list item with an anchor, button, or plain label; an explicitly current item is a noninteractive label with `aria-current="page"`. In a native ordered list, the last eligible item receives the automatic current-page marker unless an explicit current item exists. The browser controller updates that marker when siblings change and disconnects on unmount. The default slash separator is decorative, hidden for the first item, and mirrored in RTL.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Breadcrumbs/BreadcrumbItem.tsx` (Git blob `4d8893673f668c19bc4eff9919708809ab17bb4e`), the item spec, and both catalog templates. The examples use native navigation and ordered-list framing, with demo styling for the supporting variant and a local home illustration. Parent Breadcrumbs configuration, menu triggers and AppShell drawer integration remain pending. The checklist stays open.

All five standard Rust checks passed. Chrome verified both arrangements, keyboard activation, explicit and dynamic automatic current-page markers, and RTL separators. Token and showcase navigation regression suites passed. Published screenshot parity, other browsers, and manual assistive technology were not verified.

```sh
SHOWCASE_URL=http://127.0.0.1:8137 node tests/breadcrumb_item.browser.mjs
```
