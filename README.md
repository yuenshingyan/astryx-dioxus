# Astryx Dioxus

Reusable Astryx UI components for Dioxus 0.7.

## Status

Button implements the upstream variants, sizes, loading/actions, slots,
tooltips, touch behavior, link/form modes, and shared size/icon/spinner/group
integration. The showcase includes the documented Button patterns and a locally
bundled Figtree font. See [Button implementation and verification](docs/button-port.md)
for source references, tested behavior, API adapters, and verification boundaries.
The [component checklist](COMPONENT_CHECKLIST.md) records verification separately;
Button's implementation does not mark unrelated components or unrun audits complete.
Publishing remains disabled until package metadata and licensing are confirmed.
See [third-party notices](THIRD_PARTY_NOTICES.md) for the upstream MIT license.

## Run the showcase

Requires Rust (edition 2024), Dioxus CLI 0.7, and the WebAssembly target:

```sh
rustup target add wasm32-unknown-unknown
cargo install dioxus-cli --version '~0.7' --locked

dx serve --example showcase --web
```

## Use the library locally

```toml
[dependencies]
astryx-dioxus = { path = "../astryx-dioxus" }
dioxus = { version = "0.7.3", features = ["web"] }
```

```rust
use astryx_dioxus::{Button, STYLES};
use dioxus::prelude::*;

#[component]
fn App() -> Element {
    rsx! {
        document::Style { "{STYLES}" }
        Button { label: "Save", onclick: move |_| println!("clicked") }
    }
}
```

The library does not select a renderer. The showcase uses the web renderer.
CSS styles target HTML/webview output, not a native renderer.

## Layout

- `src/components/`: reusable components and public exports.
- `assets/astryx.css`: shared design tokens and component styles.
- `examples/showcase.rs`: interactive web showcase.
- `tests/`: server-rendered component tests.

## Checks

```sh
cargo fmt --check
cargo check --lib --no-default-features
cargo test
cargo clippy --all-targets -- -D warnings
cargo check --example showcase --target wasm32-unknown-unknown
```

For browser interaction checks, see [Button browser testing](docs/button-port.md#browser-checks).
The separate [core-palette contrast audit](docs/button-port.md#core-palette-contrast-conflict)
reports inherited upstream failures under an accepted source-fidelity exception;
run `node tests/button.contrast.mjs` (it exits 1). Those failures are not a claim
of passing contrast or accessibility compliance.

## Migration workflow

1. Inventory the original components, dependencies, states, and design tokens.
2. Port shared tokens before matching component styles.
3. Add each component in its own module and export it from `src/lib.rs`.
4. Preserve semantic HTML, keyboard interaction, and accessible names.
5. Add rendering tests and showcase examples for its states.
6. Compare appearance and behavior with the original before marking it migrated.

Start with Button, then primitives such as inputs, labels, badges, and cards.
Leave complex focus management (dialogs, menus, popovers) until primitives are stable.
