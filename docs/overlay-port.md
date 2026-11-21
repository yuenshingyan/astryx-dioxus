# Overlay desktop and controlled checkpoint

`src/overlay/component.rs` layers content over caller-provided media. The private scrim owns fill and strip placement, alignment, and backdrop paint. Dark and light scrims reuse MediaTheme; a transparent scrim preserves the ambient theme. Desktop hover and focus reveal use CSS. A controlled close overrides automatic reveal and keeps the content mounted but inert.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Overlay/Overlay.tsx` (`f1e7ba7263398dbd8c879ee6897f3cce9c783217`), `OverlayScrim.tsx` (`2a8972d76f5da1dd5e4bf624fcab8a795c589689`), `useOverlay.tsx`, and the three component templates with pinned offline images. Examples set the clipping radius on the container. Automatic child-radius copying, touch reveal behavior, the public hook and its example, and OverlayRegion remain open.

All five standard Rust checks passed. SSR verified rendering order, default media mode, and explicit-close precedence. Chrome verified offline images, fill and bottom-strip geometry, focus reveal, reduced motion, and controlled inertness and visibility. MediaTheme and showcase navigation regressions passed. Published visual parity and manual assistive technology were not verified. The migration checklist remains open.
