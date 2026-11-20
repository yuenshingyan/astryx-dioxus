# Private typography Tooltip presentation

Required internal Text/Heading subset implemented; **not a public Tooltip/Layer
migration or completion certification**. Existing first-interest retention,
descriptions, actual-trigger focus gate, gestures and original press policies
remain in their owners. Explicit Tooltip, StatusDot, ProgressBar and Button's
undescribed cut-label presentation retain their existing paths.

## Authority

Pin: `476a3153417dbf75c565f7df05b0784e94ddd59e`. Independently verified all cached
`text/authority` files and the additional downloads against `tree.json` using
Git blob SHA-1 (header plus bytes). Principal additional sources:

| Source (under `packages/core/src/`) | Git blob |
| --- | --- |
| `Layer/layerAnimations.stylex.ts` | `200ee96f90dd6675812e471329e46c3211e0a4f0` |
| `Layer/layerSlideRules.ts` | `6e1ee52046a58714598d7f0d8b00cab4d9e2860a` |
| `Layer/layerViewportInset.stylex.ts` | `714bc4c88dcb3f3a2617a5a1db82fd0fc16b02b6` |
| `Layer/layerTextReset.stylex.ts` | `f104f328099e6681f9fed8103c6e1a27eb2722ff` |
| `Layer/layerStructureReset.stylex.ts` | `c3387247aa36fb62bbecc93be2d4905baf363061` |
| `Layer/layerHost.ts` | `ec677261afe692aa44d25d7fc651f92a4455f418` |
| `utils/moduleLoader.ts` | `b4708a1997a0f77f87035cd8dbaa4669f6a97f1c` |
| `motion/reducedMotion.stylex.ts` | `337af66f4a7947b57d90940278d48a08c21456c0` |
| `theme/themeMotion.ts` | `86fd0878bd494a209958e999d99244c46d985eb6` |

Existing Tooltip/useTooltip, Tooltip/Tooltip, Text/TruncationTooltip, useLayer
and tokens hashes are in [Text verification](text-port.md). The neutral theme
was also downloaded/verified (`69b053e64622468f34212b9b3b7dc274c8f38947`).

## Implementation boundary

- `src/text/truncation_tooltip.js` owns measured overflow and first interest.
  Overflow preloads presentation, without creating a layer or description.
- `src/tooltip/truncation_presentation.js` and
  `src/layer/truncation_presentation.js` are **separate Dioxus assets**, loaded
  by native dynamic imports. Neither module's implementation is concatenated
  into the eval bridge. Existing dependencies/features suffice.
- Tooltip owns the document-shared loaded/pending cache and pending presentation
  actions. Held hover starts its 200ms delay when code becomes available; focus
  and completed taps replay only while current. Leave, blur, press, Escape and
  outside release cancel pending interest. Disposal/overflow loss invalidates
  all callbacks; record identity prevents old code-resolution callbacks from
  opening a replacement registration. Failed imports attach no DOM and are
  caught; later interest retries using a fresh URL (failed imports are otherwise
  cached by browsers). Cache identity also includes the content-hashed asset
  URLs, so a controller from a newer build cannot consume an older cached module.
- Layer owns native named CSS anchors, logical centered placement, flip/slide
  options, above/below anchor-clear beside/fit options, viewport gutters/caps,
  and anchor cleanup. It does not run a per-frame positioning loop. A settled
  box is read before entrance animation starts, so travel cannot contaminate
  the gap measurement or cross the anchor. Native layout follows resize/scroll.
- Top-layer promotion escapes clipping. Ordinary sibling placement is retained
  to preserve existing Link/portal capture contracts; native-modal tips are
  hosted inside their modal. Theme/writing tokens are copied onto the surface.
- Tooltip body family/size/leading/weight and Layer's text/structural reset are
  separate from Heading/label formatting. Padding is spacing-1/spacing-2;
  **content** max-width is 300px, not a 300px outer border-box. Long text uses
  source `word-break: break-word`. Shared token additions supply fast-max timing.
- Entrance is source travel/0.95 scale/opacity about the actual landing side,
  capped to the trigger gap. Reduced motion with a fine pointer has no entrance;
  a coarse primary pointer uses the fixed 200ms easeInOut opacity fade.
- The source declares **no default exit animation**. Default hide is immediate,
  retaining the layer/description but releasing its anchor. Consumer-authored
  discrete display exits retain the anchor through finite running animations;
  generation guards prevent old completion from detaching a reopened layer.
  Disposal removes it immediately and releases resources.

No public API, new dependency, diagnostic suppression or ownership relocation.
This does not implement unrelated Layer alignment/custom-positioning APIs,
software-keyboard clearance, nested public layer stacks, a theme compiler, or a
non-anchor-engine geometry polyfill.

## Evidence and commands

All CDP runs use the shared browser lock and isolated targets. Only this task's
preview on **8095** was served/restarted. No shared/user preview was changed.

```sh
python3 /tmp/astryx-interactive-cards-reference/browser_lock.py node tests/tooltip_presentation.browser.mjs
python3 /tmp/astryx-interactive-cards-reference/browser_lock.py node tests/text_tooltip.browser.mjs
# Own 8095: use text_patterns, heading_patterns, then showcase as appropriate.
TEXT_URL=http://127.0.0.1:8095 python3 /tmp/astryx-interactive-cards-reference/browser_lock.py node tests/text.browser.mjs
HEADING_URL=http://127.0.0.1:8095 python3 /tmp/astryx-interactive-cards-reference/browser_lock.py node tests/heading.browser.mjs
```

The new localhost test serves production owner scripts and the **actual separate
module files** offline, holds/fails their HTTP responses and records requests.
It verifies no request before overflow, preload without DOM, held hover/focus/tap,
dismissed hover/Escape/outside release, removal, overflow loss, replaced records,
failed-fetch recovery, stale-build cache invalidation, and no unhandled asynchronous exceptions. It compares
settled geometry, computed typography and light/dark surface paint against
`tests/fixtures/truncation-tooltip-source-reference.html`, then checks LTR/RTL,
native flip/gutters, resize, clipped nested scrolling, modal-local hosting,
text reset, reduced media and default/custom exit behavior. The renderer Text
suite additionally asserts two real Dioxus asset resource requests.

The offline HTML fixture is a **source-derived native CSS reconstruction**, not
an executed upstream React/StyleX application or catalog screenshot certification.
The centered source fixture and top/left/right edge fallback cases are compared
numerically (with surface paint in both color schemes); scrolling/modal and
media/exit cases also have separate behavior assertions. Touch/pen
sequences are synthetic Chromium events, not real-device evidence.

All five standard Rust checks passed (`CARGO_INCREMENTAL=0`, shared Cargo target):
`cargo fmt --check`, `cargo check --locked --lib --no-default-features`,
`cargo test --locked`, `cargo clippy --locked --all-targets -- -D warnings`, and
`cargo check --locked --example showcase --target wasm32-unknown-unknown`.

Passing browser suites: Text, Heading, Text truncation, Text Tooltip, the new
presentation suite, typography animation, Button, Link, ButtonGroup, StatusDot,
StatusDot Tooltip, ProgressBar motion, ClickableCard, SelectableCard events and
layer capture, container press, native event boundary, **unchanged Skeleton and
ProgressBar**. Logs and exact local hashes are recorded in the task handoff.
The Heading geometry assertion now waits for the finite entrance to finish;
it still asserts strict logical separation. Its old immediate assertion sampled
the source entrance's first frame exactly at the trigger edge. An initial Text
suite invocation against showcase instead of text_patterns failed the expected
font-family fixture comparison; the correctly isolated run passed. Initial
Link sibling/entrance geometry failures were fixed, not classified as pre-existing.
A cache-identity change also exposed an isolated explicit-tooltip harness with
no private module URL binding; controller initialization now permits that unrelated
path without loading private code, while an actual private load requires its URLs.
No thresholds or idle-trace assertions were relaxed.

Unrun: remaining unaffected browser suites and separate interactive-card-boundary
preview; independent upstream React/StyleX build, Firefox/WebKit, physical touch/pen,
manual screen-reader/keyboard audits, Windows High Contrast and mobile software
keyboard behavior. Existing contrast findings and unrelated port completion
status are unchanged.
