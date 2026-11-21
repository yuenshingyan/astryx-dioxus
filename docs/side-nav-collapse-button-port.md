# SideNavCollapseButton checkpoint

`src/side_nav/side_nav_collapse_button.rs` ports the externally controlled form. It delegates to the icon-only ghost Button, requests the opposite of the caller's state, and lets a caller click handler cancel that request with `prevent_default()`. It switches the default English action label with state, accepts a custom label and glyph, and uses separate transforms for direction mirroring and collapse rotation. Without a state source it renders nothing.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `SideNav/SideNavCollapseButton.tsx` (`d4951cbe7be72101affb1554bf15ba403e92fd38`), its spec (`222d316358e7436e091c8263f4956c63f8fadc21`), and both catalog templates. The examples adapt the original footer placements with a caller-controlled demo rail and native links. SideNav context, the deprecated imperative handle, focus registration, AppShell mobile hiding, and translation-provider integration remain pending. Reduced motion switches immediately, as in the pinned implementation; the spec's touch cross-fade remains open.

All five standard Rust checks passed. SSR verified the missing-state and custom-icon forms. Chrome verified native keyboard activation, caller cancellation, retained button focus, accessible action names, RTL mirroring, and immediate reduced-motion rotation. SideNavSection and showcase navigation regressions passed. Published visual parity and manual assistive technology were not verified; the migration checklist remains open.

```sh
SHOWCASE_URL=http://127.0.0.1:8152 node tests/side_nav_collapse_button.browser.mjs
```
