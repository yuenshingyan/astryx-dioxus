# DropdownMenuGroup checkpoint

`src/dropdown_menu/dropdown_menu_group.rs` renders a `role="group"` with an optional, noninteractive heading. A rendered title supplies the group's `aria-labelledby`; an untitled group retains caller-provided naming attributes. Native root attributes and callbacks pass through without overriding the group role. The component owns no focus or typeahead behavior.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `DropdownMenu/DropdownMenuGroup.tsx` (Git blob `c2b7c2253d111966ac85146606307bdb9c6825bc`), its spec, and both catalog arrangements. The showcase adapts both arrangements with native controls instead of the unported popup and menu items.

All five standard Rust checks passed. Chrome verified the group name in the accessibility tree, heading association and spacing, and both arrangements. DropdownMenuDivider and showcase navigation regressions also passed. Data-mode parity, menu arrow navigation and typeahead, screenshot parity, and manual assistive technology remain unverified. The checklist stays open.

```sh
SHOWCASE_URL=http://127.0.0.1:8144 node tests/dropdown_menu_group.browser.mjs
```
