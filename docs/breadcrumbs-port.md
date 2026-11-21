# Breadcrumbs checkpoint

`src/breadcrumbs/component.rs` renders one named navigation landmark and a wrapping ordered list. Native ARIA naming attributes override the default label. Its layer-scoped context supplies the separator and variant to BreadcrumbItem; items outside the trail's layer receive defaults. The item still owns automatic current-page detection.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Breadcrumbs/Breadcrumbs.tsx` (Git blob `cfb69085a437c15b5b6b4bc7308a61a63265a71f`), `Breadcrumbs.spec.md` (`8b779c985ca3d303947f49c6b6a89085bc26dfcb`), and six catalog descriptors and templates. Five arrangements are available in the showcase. The menu arrangement remains pending the shared menu and popover components. The icon example uses local illustrations. Both BreadcrumbItem examples now use the public parent.

All five standard Rust checks passed. Chrome verified the landmark name, ordered list, repeated custom separators, narrow wrapping, and reactive variant and separator changes without replacing the current link. BreadcrumbItem and showcase navigation suites passed. Published screenshot parity, other browsers, and manual assistive technology were not verified. The migration checklist remains open.

```sh
SHOWCASE_URL=http://127.0.0.1:8138 node tests/breadcrumbs.browser.mjs
```
