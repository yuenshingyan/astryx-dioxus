# Kbd implementation and verification

## Baseline and ownership

Pinned authority: Meta Astryx [`476a3153417dbf75c565f7df05b0784e94ddd59e`](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e), `packages/core/src/Kbd/{Kbd.tsx,Kbd.doc.mjs,Kbd.test.tsx}`, the Kbd specification/Chromium audit, all four named templates, `utils/isApplePlatform.ts`, and `packages/core/locales/en.json`. Required Card/List/Item/Stack/Text source owners were inspected before matching native showcase composition.

`src/kbd/` owns parsing, glyph/spoken presentation, localized text props and the aggregate shortcut. The shared platform detector stays under `src/utils/is_apple_platform.{rs,js}`, its original owner, for future consumers. No new context, provider, dependency, suppression, or breaking existing public API was added. Supporting native example markup does not mark Card/List/Stack or internationalization migrations complete.

## Contract and adapters

- Required `keys` splits on `+`; each segment is ECMAScript-trimmed and lowercased before aliases `esc → escape` and `return → enter`. Repeat occurrences retain distinct identities before aliasing, so aliases/repeats never collapse or share keys. Unknown strings uppercase as text; no object-prototype lookup is used. Literal plus is spelled `plus`.
- Apple: mod/meta show ⌘; ctrl/alt/shift show ⌃/⌥/⇧. Elsewhere, and on the server/first client render, the corresponding text is Ctrl/Ctrl/Alt/Shift/Win. Enter/Backspace/Tab/arrows/plus keep ↵/⌫/⇥/↑/↓/←/→/+; Escape has localized short text, Esc by default.
- The group is one image with the computed spoken shortcut name; keycaps are native kbd elements hidden from AT. Mod is Command on Apple and Control elsewhere; Control/Alt/Shift and all special spoken labels follow text props. Source meta remains the spoken technical identifier **META**, even when its visual glyph is ⌘/Win. Unknown key names remain uppercased.
- Role and aria-label belong to Kbd and cannot be replaced by raw attributes. Aria-labelledby can name it from visible text, and aria-hidden can silence it. Native data/ARIA/class/style/ref via onmounted and additional event builders reach the group. Kbd adds no tab stop, tooltip/press bridge, live region or shortcut handler.
- Server/first-client rendering uses non-Apple text. A one-shot post-render scoped task reads the original platform detector; failures retain the default. There is no polling, listener, media subscription or shortcut registration. Dioxus cancels the task when its owner unmounts; key/locale changes do not re-read the platform. The platform is treated as stable for the lifetime of a mounted Kbd, not as an application-controlled setting.
- The detector prefers a named userAgentData.platform, matching `/mac/i`; blank/Unknown/non-string values fall through to navigator.platform's original Mac/iPhone/iPad/iPod test. Even the original named-iOS-versus-Mac distinction is preserved; it is not replaced with a new OS heuristic.

### Localized text

`text: KbdText` supplies all five short labels, thirteen spoken words and the pairwise shortcut template. Defaults reproduce the verified English catalog using borrowed Cow strings. Applications obtain translations from their own locale system and pass a changed value when locale changes; no incomplete global internationalization provider is introduced.

The template contains `{shortcut}` and `{key}` exactly once and can reorder them (for example `{key} mit {shortcut}`). Inserted words are not reparsed as template syntax. Nonempty translated words and template shape are validated at the public boundary; invalid values return recoverable KbdError variants. Apple symbols remain symbols while their spoken names localize.

Entirely empty shortcuts, including only empty `+` segments, return `KbdError::EmptyShortcut` rather than mounting an unnamed image. Empty internal segments in otherwise meaningful combinations remain as shipped (`ctrl++k` has three badges); use `plus` instead when the plus key is intended. This is explicit Rust boundary validation, not a claim of exact empty-input output parity with the source.

### Theming anatomy

Only the shortcut group carries `.astryx-kbd`. Individual keycaps have no public target or Astryx-prefixed class. The pinned specification leaves per-key target OQ1 open. Shared inherited tokens can change typography, fill, radius and border, but coloring the non-painting group itself does not automatically paint its keycaps. No invented size/color enum or public part API is added.

Keycaps are content-width with at least spacing-5 width/height, one-line minimum height, spacing-1 inline padding, radius-inner, neutral fill, primary text, medium supporting typography, and a 2px emphasized bottom border. Larger text grows the badge instead of freezing its height. Inline-flex baseline layout is retained; no vertical-align correction that changes the original inline prose is applied.

## All four original patterns

`examples/showcase/kbd.rs` ports:

- **KbdShowcase:** mod+k on the original muted presentation background.
- **Inline Instructions:** the exact four sentences, including the last sentence's three independent shortcuts and all surrounding spaces.
- **Menu Shortcuts:** Cut/Copy/Paste/Undo/Redo, native readonly list/card framing and exact compact row geometry; these are not menu actions or a reusable Card/List port.
- **Modifier Combinations:** all three rows (4/3/7 shortcuts), special keys/aliases and the original text label.

Native menus use the source's zero-padding Card border/radius, five 28px rows and four 2px gaps (150px Card height). Inline/Modifier columns remain intrinsic, not fixed source-measurement widths. Additional examples cover all special names/meta/plus, repeats/unknown keys, explicit naming/ref/events, reactive localization, changed and late/unmounted shortcuts, larger text and RTL.

## Source appearance evidence and published discrepancy

`tests/fixtures/kbd-neutral-reference.json` captures actual catalog evidence for all **26 original groups / 46 keycaps**, Apple and non-Apple, light and dark: 104 group / 184 keycap observations. Each platform navigation sets both navigator.platform and Client Hints platform before loading, and records actual navigator snapshots and loaded Figtree. This is platform emulation in a macOS-hosted Chrome, not native Windows font-rendering certification. The catalog advertises Core v0.7.16; it is not asserted identical to the pinned commit.

The published deployment has an `astryx-kbd-key` class on each keycap. The pinned implementation/spec/tests explicitly do not. Appearance comparison uses the actual geometry/paint but the port deliberately preserves pinned target ownership; tests separately assert absence of per-key Astryx targets. This observed deployment difference is not silently copied into the API.

Source profiles intern repeated styles/tokens/context paint. Tests compare the same DOM metrics for role/name, root dimensions/display/gap, every glyph and hidden state, keycap dimensions/positions and glyph Range rectangles, font/leading/weight, fill/text/bottom-border paint, radius/padding/minimums/user-selection, nearest presentation background, and native example dimensions. Geometry tolerance is one native layout unit (1/64px); selected nonnumeric properties are exact. Glyph Range rectangles are layout fragments, not ink outlines. Raw token expression spelling, whole-site chrome and unused fallback font stacks are not golden public contracts. Fixtures require no live service during tests.

## Verification

Run the five standard project checks and:

```sh
node tests/kbd.browser.mjs
node tests/button.browser.mjs
node tests/button_group.browser.mjs
node tests/icon_button.browser.mjs
node tests/link.browser.mjs
node tests/spinner.browser.mjs
node tests/badge.browser.mjs
node tests/divider.browser.mjs
node tests/spinner_rotor.browser.mjs
node tests/announcement.browser.mjs
node tests/kbd.contrast.mjs
```

Use the [existing Chrome/CDP setup](button-port.md#browser-checks). CDP_URL/SHOWCASE_URL override defaults; rebuild the watch-disabled showcase before browser checks.

Rust tests cover all key maps/platform presentation, aliases, repeated-source identities, unknown/prototype-like strings, Unicode uppercasing and ECMAScript whitespace, exact SSR Ctrl/Win output with **zero platform document reads**, computed-versus-caller naming, native attributes/events, localization/reordering and literal placeholder insertion, live locale props, recoverable validation and missing-keys compile failure.

Browser checks cover all 104 source group comparisons, all 184 keycaps and native contexts, actual preferred-platform selection, first committed Ctrl/K before Apple detection, AX one-name/hidden-glyph behavior, native callbacks, no own focus/press/live/shortcut handling, text/keys changing without platform polling, late mounts and removal requested during the platform read, larger text glyph containment, RTL order, narrow layout and forced-color border/role. The detector's absent/blank/Unknown/invalid/preferred/Apple/non-Apple cases are also checked at its external navigator boundary. Existing seven component suites and two renderer-boundary suites remain regression checks.

## Contrast and verification limits

`node tests/kbd.contrast.mjs` (optional --json) finds **0/12 modeled keycap text failures below 4.5:1** across core and showcase Neutral page/surface/muted backgrounds, light/dark, with translucent fills composited. Different shortcut/platform glyph strings share paint. Kbd is display-only, so the audit invents no interactive focus or essential keycap-edge requirement. Custom themes, arbitrary surrounding/control backgrounds, actual glyph antialiasing and native Windows fonts remain outside this result.

A real SSR hydration application, physical devices, Safari/Firefox and actual screen-reader delivery remain unrun. Separate SSR/first-client checks establish deterministic initial presentation, not a claim that a full hydration application was exercised. Chrome AX is not a screen-reader certificate. The original per-key theming question remains open; no full shared locale provider or hotkey system is claimed.
