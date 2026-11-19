# Link implementation and verification

## Reference and ownership

Reference: Meta Astryx [`476a3153417dbf75c565f7df05b0784e94ddd59e`](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e), specifically `Link/{Link.tsx,computeTargetAndRel.ts,useLinkify.tsx}`, the Link tests/docs/templates, and their Text, Tooltip, Icon, interactive-role, and navigation utility owners. The contract itself is under `docs/specs/components/Link/Link.spec.md` upstream.

The port preserves these owners:

- `src/link/`: component, renderer/provider, target/rel normalization, linkification.
- `src/text/`: typography and stable inherited text-clamp context. `src/theme/types.rs` owns open validated type/color names and closed size/weight/display/wrapping options.
- `src/interactive_role_context/` and `src/hooks/use_interactive_role.rs`: independent inherited role and precedence, not a new combined control environment.
- `src/utils/safe_url.rs`: original shared navigation inspection; resource policy remains separate.
- `src/hooks/use_press_feedback/`: shared press/controller lifetime. The later [Spinner completion](spinner-port.md) moved rotor synchronization fully under Spinner and removed the old Button-specific adapter; all composed and standalone rings now share its phase batch.
- `src/tooltip/`: explicit above-tooltip presentation, name/description handling, hover/tap/focus interest, and lazy automatic Text tooltip behavior. This is not a placeholder public Tooltip component.
- `src/icon/global_icon_registry.rs`: the global icon-extension seam. Link resolves the `externalLink` fallback at its actual extra-small size. The showcase registers Neutral's exact Lucide artwork through that seam, rather than baking a theme glyph into Link.

Supporting Text/Tooltip/Icon/Theme/Card work does not mark those standalone component migrations complete. No dependency or diagnostic suppression was added. Existing Button URL acceptance is not widened by this task; Link/linkification use the pinned shared navigation rule.

## Approved API correction: optional renderer destinations

`LinkRenderProps.href` is now `Option<String>`, with explicit user approval. Upstream can call a custom renderer without a destination when an interactive owner supplies a link role. The former required String could not represent that path.

- Button producers pass `Some(href)`.
- Link passes `None` when the destination is absent. It never substitutes `""` or routes a rejected URL.
- Existing native forwarding, `href: props.href`, still works because Dioxus omits a `None` attribute.
- String-specific router logic must branch on the Option. The `ButtonLinkProps` alias has the same corrected field type.

```rust
fn render_link(props: LinkRenderProps) -> Element {
    rsx! {
        a {
            href: props.href, target: props.target, rel: props.rel,
            onclick: props.onclick, onkeydown: props.onkeydown,
            onmounted: props.onmounted, ..props.attributes,
            {props.children}
        }
    }
}
```

Callbacks that need their own hooks should return `rsx! { TheirRouterComponent { ... } }` to establish that component's scope. Preserve modified clicks, downloads, and non-self targets; do not indiscriminately prevent native navigation. Renderer resolution is explicit override → nearest provider → native fallback; an explicit native-rendering callback corresponds to upstream `as="a"`.

## Implemented Link contract

- All nine built-in text types plus validated custom names; eleven explicit sizes, four weights, six built-in colors plus custom names, inline/block display, arbitrary positive integer line clamps, standalone sizing, and underline defaults.
- Enabled href selects navigation before callbacks/context; otherwise a callback selects a native button, then an inherited role, then the disabled-anchor/no-destination fallbacks. Native actions always use type=button.
- Disabled navigation stays a focusable `<a role="link">` without href/target/rel and skips the router. Disabled actions use focusable aria-disabled buttons. Caller tab order is retained; role/aria-disabled cannot erase disabled exposure. Click and Enter/Space are stopped; other keys reach the caller. Focus survives both router-to-button and router-to-disabled-anchor replacements.
- Navigation inspection strips ASCII controls, trims ECMAScript whitespace, and case-inspects schemes. javascript, vbscript, and data:text/html are rejected before a router; ordinary relative/fragments/http(s)/mailto/tel/custom schemes and non-HTML downloads are accepted without rewriting the sink's input.
- External mode forces `_blank`, merges noopener/noreferrer without discarding caller tokens, appends a decorative 12px icon and localized `new_tab_label`, and does not add them to button forms. Explicit `_blank` is safe even without external mode. Download is an enum (off/on/suggested filename); target, rel, and referrer policy remain native string attributes.
- Children normally supply the name; `label` overrides it and caller aria-label can override label. New-tab text contributes to an unlabelled external link's name; an explicit label remains the whole name.
- Built-in Link colors are painted by the root, with inline inherited label paint so a Text theme does not steal Link's color. Hover-capable input underlines and tints the appropriate colors by 15%; pressed text surfaces use the original overlay, with no Button scale or Button hit growth.
- Long unbroken labels wrap in constrained anchors/buttons/external links; own clamps ellipsize inside a shrinking root. A flowing anchor reads the enclosing Text clamp and uses an inset focus outline; own block/clamp/button forms retain their boxes. LayerContentBoundary resets TextClampContext, as upstream specifies.
- Explicit tooltips open above (with viewport fallback), are hoverable, preserve additional descriptions, and dismiss on blur/Escape/removal. Automatic truncated Text tooltips are created only after hover/tap/keyboard interest, never duplicate the link's name/description, live-refresh while open, and close after widening removes overflow. No tooltip controller is registered for unclamped or explicitly disabled truncation tooltips. Active-only mutation/resize observers are disconnected on close/removal.

The shared bridge extraction preserves the legacy runtime Symbol and Button regressions. Its queued-scroll/hover regression was fixed: a pending scroll event after scrollIntoView must not cancel a newly entered hover. Text's automatic tip is placed outside an enclosing action root so it cannot become a second copy of the accessible name.

## All six named patterns

`examples/showcase/link.rs` ports Standalone, Inline Text, External Links, and With Tooltips, plus both hook patterns:

- **useLinkComponent — Article Cards**: provider routing with a visible location, explicit native override inside the provider, native fallback outside it. Native article framing uses the original default Card border/inset/background tokens; it is not a stub Card implementation or a completed Card migration.
- **useLinkify — Links in a Comment Thread**: exact URL/email/issue examples and unchanged plain note.

Additional cases demonstrate color/type options, disabled/native/custom forms, missing destinations, unsafe URL rejection, safe download/referrer attributes, dynamic focus transitions, long-label wrapping, own and ancestor clamps, layer reset, RTL, callbacks, and tooltip cleanup.

## Linkification adapter

`linkify` returns validated descriptors; `use_linkify` returns a memoized renderable fragment containing real Link components. Never turn descriptor destinations into unchecked raw anchors.

Rust has no built-in regex engine and no dependency was added. Custom patterns accept an application-owned matcher returning UTF-8 byte ranges/captures, followed by href/label callbacks. Retain the pattern/options across renders for memoization. Invalid, empty, reversed, out-of-bounds, or split-codepoint matches/captures return LinkifyError before builders run. Arbitrary matcher callbacks must terminate; the library does not own their cancellation.

Builtin URL and ASCII email scans are linear and follow the pinned regex boundaries, not a new punctuation/DNS heuristic. Custom matches precede builtins at equal starts; stable start ordering discards overlaps, keeps adjacent matches, and preserves Unicode surrounding text. Email, issue, and HTTP(S) links all go through Link's routing/safety boundary.

## Verification

Run all five standard project checks, then:

```sh
node tests/link.browser.mjs
node tests/button.browser.mjs
node tests/button_group.browser.mjs
node tests/icon_button.browser.mjs
```

Use the loopback Chrome setup in [Button browser checks](button-port.md#browser-checks), with optional CDP_URL/SHOWCASE_URL. Rebuild the watch-disabled showcase after changes.

Rust tests cover semantic forms/precedence/providers, actual None handoff, URL rejection and original-value acceptance, native attributes, safe rel/new-tab content, names/descriptions, typography/clamp/layer precedence, Text state preservation, all Text options, icon registry semantics, and linkification boundaries/ranges/cache/error paths. Browser assertions cover actual keyboard/mouse/touch/native routing, disabled focus transitions, accessibility names/states/descriptions, target/rel/download, glyph lookup, Text-theme isolation, hover/press, wrapping/clamping/inset focus, lazy/live tooltips, cleanup, and forced colors. Existing three browser suites are required regressions.

`tests/fixtures/link-neutral-reference.json` records the actual catalog's eight component-example links in Neutral light/dark. Tests compare label/href/target/rel, root/label paint, font/leading/weight, display, underline, gap, dimensions, and external-icon geometry. Only width permits 1/32 CSS pixel; other recorded properties match exactly. The registered external SVG viewBox/stroke/paths are also checked against the actual catalog artwork. The catalog's displayed release is not asserted identical to the pinned commit. The two hook templates are checked against their pinned sources and observable behavior, not whole-site screenshot equality. Tests do not depend on live services.

## Contrast and verification boundaries

`node tests/link.contrast.mjs` (or `--json`) audits 96 active color/state/backdrop cases: rest, hover, fine-pointer pressed and coarse-pointer pressed, six colors, light/dark, page/surface. Inherit assumes the default primary parent. There are **19 text failures below 4.5:1**, including active disabled-color paint and some accent pressed cases; **0/24 focus/backdrop failures below 3:1**. Merely choosing color=disabled does not make a link unavailable or exempt it from contrast.

The audit intentionally exits 1 and retains original colors under the existing [source-fidelity decision](button-port.md#core-palette-contrast-conflict), not a claim of passing contrast. Applications needing compliant active muted/pressed colors must strengthen the theme; use is_disabled for genuinely unavailable controls. This does not certify arbitrary custom themes/artwork/adjacent text, inline-link distinguishability, antialiasing, screen readers, physical devices, Safari, or Firefox. Global icon overrides are supported; a complete scoped Theme/icon-catalog implementation remains a separate migration.
