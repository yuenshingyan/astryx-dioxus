# Lightbox: single-image checkpoint

`src/lightbox/` composes the fullscreen native Dialog with a fitted image, optional caption, and a Close button. The caller owns open state. The image mounts only when requested open. Close receives initial focus; Escape and Close request dismissal through the existing dialog path.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `Lightbox/Lightbox.tsx` (`a4ad1d89390e4532f7d1bb8a0b10433d0b100a8c`), its spec (`ee7e1f12801a8bb2f76ac917e36666f75ffe1d97`), and the showcase (`d3b07eff67e3411b0979bfe29e3ab1687baa87e5`). The local showcase uses the pinned coastline image (`c9d63bb0ac345205e28e06945e7530a855507805`) offline and retains the original alternative text and caption.

All five Rust checks and four Chrome suites passed. The focused suite verified viewport geometry, image loading, accessible dialog and image names, caption, initial focus, Escape and Close dismissal, and focus return. Gallery navigation, video, zoom and pan, caption expansion, touch gestures, backdrop behavior, and visual parity remain open. The checklist remains open.
