# Thumbnail checkpoint

`src/thumbnail/` renders a square attachment preview with a decorative or named image, source-specific error recovery, a Skeleton before a source is available during loading, and a decorative Spinner over an uploading image. Open and Remove are separate native actions. Loading disables Open without replacing it; disabled actions retain their tab stops. The label uses the existing tooltip controller, and upload state changes use the shared polite announcer.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Thumbnail.tsx` (Git blob `6306fa60fd78198b17ea9e73565303eee4473590`), the component spec, and all five catalog templates. Their embedded sample PNGs are saved under `tests/fixtures/thumbnail/` for offline use.

All five standard Rust checks passed. Chrome verified offline images, failed-source recovery, focus retention when Open becomes unavailable, keyboard activation suppression, upload announcements in the DOM, and separate Open and Remove behavior. FieldStatus and showcase navigation regressions passed. The initial-load announcement policy follows the existing shared reader; startup timing parity, authored tooltip trigger and touch parity, palette contrast, published screenshots, and spoken output were not independently verified. The upstream missing-image-name development warning is not implemented. The migration checklist remains open.

```sh
SHOWCASE_URL=http://127.0.0.1:8131 node tests/thumbnail.browser.mjs
```
