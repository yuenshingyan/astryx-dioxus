# SideNav checkpoint

`src/side_nav/component.rs` renders a named expanded rail with header, top content, independently scrolling items, footer, and a small-size context for footer controls. Top and footer content stay outside the scroll area. Empty top and footer regions are omitted.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `SideNav/SideNav.tsx` (`173fdf660a8e28aedb1190196b783f094ca12c8a`), its spec (`2eda2f83e3b10d2e0e97bba957f1f766fc9c7388`), and the adapted showcase (`1ef7a384c06565154f2cc351201e7a687f9f69e9`). The sample substitutes local fragment links and omits the original icons and nested groups.

At the initial checkpoint, all five Rust checks and four Chrome suites passed. Chrome checked the named landmark, 260px default width, independent item scrolling, stationary header and footer, and inherited footer sizing.

The focused SSR follow-up verifies root naming: a supplied native `aria-label` overrides the `label` prop; an omitted or unset one keeps the prop (default `Side navigation`). The root emits one `aria-label`, still forwards `aria-labelledby`, and keeps its native navigation role. The showcase now exercises native `aria-label` with the same landmark name and current-page link. The existing `tests/side_nav.browser.mjs` AX and geometry checks should be rerun against the updated showcase by the integrator. Default-label localization, remaining catalog arrangements, published raster/visual parity, and spoken output remain unverified.

## Collapse handoff

In the pinned default rail, `SideNav` publishes `SideNavCollapseContext` when collapse is enabled. Its `{isCollapsed, isCollapsible, toggle}` comes from one `Resizable` region, internal even when there is no resize handle, or caller-owned when supplied. Resize config takes precedence over conflicting collapse config; an external region owns both collapse state and eligibility, with development warnings for conflicts. The mobile render branches do not publish this rail context. The separate `SideNavFocusTargetsContext` supports focus recovery (spec AR3); it is not a state source for the heading or section.

- `SideNavHeading` needs the resolved `isCollapsed`: its expanded text and trailing controls give way to an icon-only form, or no heading when there is no icon.
- `SideNavSection` combines `isCollapsed` with explicit `isHeaderHidden` using OR. It clips the header while retaining the title reference that names the group and keeping item DOM in place.

The Rust `SideNav` has no collapse or resize input and publishes no collapse context. `SideNavControlledCollapsible` supplies the external button with state and a callback but does not publish state to descendants; the current Rust `Resizable` module has a handle, not the region/state owner. The blocker is approval to port `SideNavCollapseContext` at its upstream owner and agree on the Rust SideNav collapse input/state-owner API. A CSS-only rail or extra heading/section flags would not provide the same shared state. No context or consumer changed here. Resizable ownership and warnings, collapse focus and motion, nested navigation, and AppShell integration remain open.
