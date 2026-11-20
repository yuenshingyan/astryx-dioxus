# CheckIndicator port

CheckIndicator paints a decorative mark for a caller-owned selection. An unchecked item without a replacement renders nothing. A checked item uses the Icon root directly, with the host's class and native attributes on that same element. Both supported sizes use the small Icon box. The `check` artwork can be overridden through the existing global icon registry.

The Rust `replacement: Option<Element>` slot distinguishes absence from deliberate empty content. When supplied, it takes precedence in either state and reserves the same one-rem box. Both paths enforce `aria-hidden` even if a caller passes a conflicting value. Selection, form semantics, and focus remain with the host.

Authority: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Indicator/CheckIndicator.tsx` (blob `9903cf1c60f9f18529b516e1c1b24c52dc718881`) and its specification (blob `26ef6d4e7eb3061b30dd8fee5f1accccfbe75a90`). The pinned tree has no standalone CheckIndicator CLI template; the showcase contains contract examples, not a port of a host control or the theme replacement system.

All five standard Rust checks passed. Chrome verified the absent unchecked root, same-element Icon and host classes, reserved replacement geometry, and Neutral light and dark colors. IconButton and showcase navigation regressions also passed. Published appearance, other browsers, and manual assistive technology were not tested. The migration checklist remains open.
