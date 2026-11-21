# AvatarGroupOverflow checkpoint

`src/avatar_group/avatar_group_overflow.rs` renders a named static indicator or a native button. Negative counts clamp to zero, and custom children replace the visible count without changing its accessible name. The standalone indicator uses a 36px circular avatar size plus the 2px border on each side, and grows inline for wider content. Callers can supply a localized name.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `AvatarGroup/AvatarGroupOverflow.tsx` (Git blob `9268d3582e43211e43ef1a39435f717aee1cce1d`), the component spec, and all three catalog arrangements. The showcase now uses the public [AvatarGroup](avatar-group-port.md), which supplies sizing, overlap, and roving keyboard focus to an interactive overflow button. The group context resets at a layer boundary.

All five standard Rust checks passed. Chrome verified count clamping, pill growth, custom names, and native keyboard activation. Avatar, AvatarStatusDot, and showcase navigation regressions passed. Published screenshot parity, touch behavior, and manual assistive technology were not verified. The migration checklist remains open.

```sh
SHOWCASE_URL=http://127.0.0.1:8140 node tests/avatar_group_overflow.browser.mjs
```
