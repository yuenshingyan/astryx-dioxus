# Inline Code

Authority: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, not the
published catalog. This port implements **Code**, not CodeBlock.

## Ownership and inventory

`packages/core/src/Code/Code.tsx` owns the single native `code` root. Its own
`Code/` directory, public target `astryx-code`, and `data-color` / optional
`data-size` reflection remain intact in `src/code/`. CodeBlock's compatibility
re-export does not transfer ownership to CodeBlock. No CodeBlock module is added.

| Source input | Dioxus input / behavior |
| --- | --- |
| required ReactNode children | required `Element` children, including rich/native children; escaped by Dioxus |
| color: primary (default), secondary, inherit | closed `CodeColor` enum |
| optional size: inherit | `Option<CodeSize>`; omission uses shared code scale, not surrounding size |
| ref | `onmounted: Option<EventHandler<MountedEvent>>`, on the root |
| className | `class`, appended to stable target |
| style / StyleX xstyle | native CSS `style` and consumer `class`, following the existing port convention; no StyleX runtime or StyleX object adapter |
| DOM / data / ARIA props | extended Dioxus `GlobalAttributes` / `attributes`, on the root |
| native events | direct `onclick` convenience prop; all other Dioxus event builders in `attributes`, including keyboard, pointer, focus, clipboard and drag handlers; no component interception |
| dir | defaults to `ltr`; caller replaces it, including `rtl` / `auto` |

Example of a native event without a dedicated convenience prop:

```rust
use astryx_dioxus::{Code, CodeColor, CodeSize};
use dioxus::events::onkeydown;
use dioxus::prelude::*;

let _ = rsx! {
    Code {
        color: CodeColor::Inherit,
        size: CodeSize::Inherit,
        attributes: vec![onkeydown(move |_| {})],
        "const count = 0"
    }
};
```

Visual reflection and owned class/style cannot be forged through the attribute
spread. Consumer CSS overrides compose after base paint; inline style wins as
ordinary native CSS. Dioxus's native global-attribute vocabulary is broader than
React BaseProps (which omits title, contentEditable, several obscure globals,
RDFa, microdata and dangerous HTML inputs); this is the existing port's native
attribute adapter, not a claim of identical React typing. There is no new
polymorphic root, custom color name, action, accessible name or focus target.

Code owns no context, hook, timer, listener, observer, state, task, subscription,
network operation, syntax highlighting or keyboard binding. Callers own any
work performed in their callbacks. Native content, selection and semantics stay
native. CSS is logical; the source's `dir=ltr` behavior is retained without
inventing extra bidi isolation properties.

## Tokens and appearance

Code consumes existing shared tokens; it defines **no duplicated token values**:

- typography: `--font-family-code`, `--text-code-size`;
- color: `--color-text-primary`, `--color-text-secondary`,
  `--color-background-muted`;
- geometry: `--spacing-1`, `--spacing-0`, `--radius-inner`.

Line height always inherits. Size inherits only when requested; font family
always remains code-family. Font weight is not reset (it inherits naturally).
Wrapping preserves both upstream `overflow-wrap:break-word` and
`word-break:break-word`. It does not introduce `pre`, white-space preservation,
block display, padding around line boxes, borders or min/max width.

CSS variables are read at the root and respond to theme/media/container
conditions. Code owns no breakpoint or theme compiler. Tests exercise viewport-
conditional token overrides and root-rem growth rather than duplicating shared
responsive theme infrastructure. Platform monospace fallback is intentionally
retained; no network font is required. The showcase's existing Figtree font is
bundled, though default source typography uses platform fonts.

## Original templates and source gaps

`examples/code_patterns.rs` includes all four original templates and their **17**
Code instances:

1. CodeShowcase: package command and variant references (5).
2. CodeInlineInParagraph: hooks in prose (3).
3. CodeVariousContent: variable, terminal, CSS, file path and shortcut text (5).
4. CodeAcrossTextSizes: heading, body, supporting, label (4).

Template content, including the historical `CodeBlock/Code.tsx` path and omitted
spaces after hook names in InlineInParagraph, is intentionally not silently
corrected. AcrossTextSizes' metadata says Code automatically matches the parent,
but its source omits `size`, while Code source/spec explicitly retain code-scale
size. This port follows the implementation/spec; additional cases test explicit
inheritance at 12/14/18/24px. The original templates import Code through the
CodeBlock compatibility entry; this port exports it directly without porting
CodeBlock.

Text and Heading frames belong to their respective owners. Existing Text is
reused; the heading frame in this isolated example is native `h3` with shared
heading tokens pending parent integration with the independent Heading port.
No Text/Heading/Stack/Layout implementations or tokens are copied into Code.

The upstream spec records testing gaps around wrapper roots, default typography,
paint, owned reflection precedence and RTL browser behavior. The new SSR/browser
checks explicitly cover these; upstream coverage is not presumed sufficient.

## Verification

Run from the root (all cargo commands use `CARGO_INCREMENTAL=0`):

```sh
cargo test --locked --test code
cargo check --locked --example code_patterns --target wasm32-unknown-unknown
dx serve --example code_patterns --web --port 8087
python3 /tmp/astryx-interactive-cards-reference/browser_lock.py node tests/code.browser.mjs
node tests/code.contrast.mjs
```

Browser default URL is port8087; override `CODE_URL` if needed. It creates and
closes only its own CDP target. Neither the parent's preview nor browser is stopped.
The browser test uses checked-in `tests/fixtures/code-source-reference.html`, an
**independently transcribed source projection**, not a React/StyleX render or a
catalog capture. `code-source-manifest.json` records 17 Git-blob-SHA1-verified
source/spec/doc/template/dependency files and the projection SHA256. No live
reference or remote asset is needed. The dx development overlay attempts a Google
Inter import; the suite blocks HTTPS and verifies every remote request failed at
the network boundary. No remote font participates in the comparison. Comparison covers each original code
instance's text, computed paint/typography/geometry and line fragments at 320 and
1000 CSS pixels in both color schemes. Prose frames model the typography needed
for Code comparison, not a completed audit of Text/Heading or template containers.
This is deterministic property/geometry comparison, **not pixel-diff proof of
the pinned React renderer**.

Additional browser assertions cover native root/mounted data, native click and
raw keyboard callback forwarding, absence of component roles/tab stops/names,
caller text in Chromium's AX tree, explicit color/size inheritance, native RTL
overrides, unbroken wrapping, rem/conditional token updates and forced-color
`auto` behavior. The component adds no focus handling. The suite does not certify
forced-color background distinction, real AT speech or device input behavior.

Local standard checks passed: fmt, renderer-neutral no-default-features library
check, full Rust tests, all-target Clippy with `-D warnings`, and WASM showcase
check. The focused WASM example check, browser suite and contrast model also
passed. Initial test escaping expectation and module-inception lint were fixed;
no diagnostic suppression was added. Parent must rerun combined checks after
integrating independently owned Text/Heading changes.

### Contrast and remaining boundaries

The source palettes are unchanged. The standalone contrast model passes 8/8
primary/secondary cases over source body/surface backdrops in light/dark, with
translucent muted background composited before calculation (minimum 5.425:1).
The background is decorative, not an interactive/essential graphic boundary.
Inherited/custom ink and arbitrary product backdrops are caller-dependent; this
is not a universal accessibility-conformance claim. No source colors were
silently repaired.

Unrun: pinned React/StyleX pixel capture, other browser engines/OS monospace
fallbacks, real-device touch/pen/selection, real screen-reader speech, exhaustive
user themes and shared responsive theme-compiler verification. Only Chromium
CDP's DOM/AX/computed layout was exercised. No migration checklist completion is
claimed pending parent integration of the shared typography frames.
