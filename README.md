# Astryx Dioxus

Reusable Astryx UI components for Dioxus 0.7.

## Status

Initial scaffold. The button and CSS are starter implementations, not faithful
ports of Astryx. The original component source and design tokens are needed
before migration can begin. Publishing is disabled until package metadata and
licensing are confirmed.

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
        Button { onclick: move |_| println!("clicked"), "Save" }
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

## Migration workflow

1. Inventory the original components, dependencies, states, and design tokens.
2. Port shared tokens before matching component styles.
3. Add each component in its own module and export it from `src/lib.rs`.
4. Preserve semantic HTML, keyboard interaction, and accessible names.
5. Add rendering tests and showcase examples for its states.
6. Compare appearance and behavior with the original before marking it migrated.

Start with Button, then primitives such as inputs, labels, badges, and cards.
Leave complex focus management (dialogs, menus, popovers) until primitives are stable.
