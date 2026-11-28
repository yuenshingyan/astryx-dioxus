# Lightbox: single-image checkpoint

`src/lightbox/` composes the fullscreen native Dialog with a fitted image, optional caption, and a Close button. The caller owns open state. The image mounts only when requested open. Close receives initial focus; Escape and Close request dismissal through the existing dialog path.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `Lightbox/Lightbox.tsx` (`a4ad1d89390e4532f7d1bb8a0b10433d0b100a8c`), its spec (`ee7e1f12801a8bb2f76ac917e36666f75ffe1d97`), and the showcase (`d3b07eff67e3411b0979bfe29e3ab1687baa87e5`). The local showcase uses the pinned coastline image (`c9d63bb0ac345205e28e06945e7530a855507805`) offline and retains the original alternative text and caption.

## Caption disclosure (FR1, single image)

The caption uses body `Text` without a truncation tooltip. The shared truncation hook measures the two-line preview after layout; only an overflowing caption shows the link-styled `more` button. Its `aria-controls` points to the same full caption text that describes the dialog. The button stays mounted and changes to `less` on expansion. The dark caption panel grows over the fitted image, rather than resizing it; long expanded text scrolls independently of the action. Closing the viewer resets the caption. The `@astryx.lightbox.captionMore` and `@astryx.lightbox.captionLess` message keys can be overridden through `InternationalizationProvider`.

The showcase keeps the original short caption and adds a separate long-caption opener using the same offline image. `cargo test --locked --test lightbox` covers initial markup, including the full description relationship and the absence of disclosure for uncaptioned media. `tests/lightbox.browser.mjs` adds keyboard expansion and collapse, scroll and image geometry, full accessible description, Escape from the action, focus return, and reset on reopening. Browser coverage for this change was **not run**. The original single-image checkpoint had passing Rust and Chrome checks for image loading and fit, accessible names, initial focus, dismissal, and focus return.

## Remaining gaps

- The action occupies its own row, rather than the upstream measured inline placement on the last visible line. When the action is shown the reserved band is three lines, not two. Exact last-line wrapping, rich interactive caption descendants under the clamp, resize/font edge cases, and cross-browser visual and contrast parity are unverified.
- The fullscreen Dialog adapter, existing viewer name and Close button, and native dismissal/focus behavior are unchanged. Full upstream layer integration (including shared focus-region tab wrapping, document-specific dismissal, and modality-aware focus return), outside-media dismissal, and viewer-wide media theming remain open. Those runtime changes are out of scope for the caption port.
- Gallery navigation and shared caption bands, video, zoom and pan, touch gestures, motion, and gallery/zoom announcements remain open. The checklist remains open.
