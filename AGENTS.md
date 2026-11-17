# Project Coding Conventions

## Languages & Stack

- Use Rust edition 2024, Dioxus 0.7, and CSS with the existing dependencies.
- Use `dioxus-ssr` for server-rendered component tests.
- Follow the setup, layout, and migration workflow in [README.md](README.md).

## Naming Conventions

- Use `snake_case` for Rust variables, ordinary functions, modules, and files.
- Use `PascalCase` for types, enum variants, and Dioxus component functions.
- Use `SCREAMING_SNAKE_CASE` for constants.
- Prefix boolean names with `is_`, `has_`, `should_`, or `can_`, as appropriate.
- Allow common abbreviations such as `config`, `ctx`, and `err`.
- Preserve the existing CSS naming: `astryx-` component prefixes and BEM-style modifiers, such as `.astryx-button--primary`.

## Namespaces & Imports

- Import symbols and use their unqualified names at use sites.
- Put `use` and `pub use` statements at the top of their containing module, not inside functions or other blocks.
- Group imports in this order, separating groups with blank lines: standard library, external crates, project imports. Let rustfmt order imports within groups.
- Allow `dioxus::prelude::*`; use explicit imports elsewhere.
- Use aliases to resolve imported-name collisions, such as `use std::io::Error as IoError;`.
- Use `crate::` and `super::` only in module-scope imports, including re-exports. Import names before using them in expressions, type annotations, or other non-import code.
- This path restriction does not extend to `self::`, `std::`, or external-crate paths.
- Re-export selected public component APIs through `src/lib.rs`.
- Qualify enum variants with their owning type or `Self`, such as `ButtonVariant::Primary` and `Self::Primary`.
- Use ordinary type-qualified or receiver syntax for associated members. Use explicit trait qualification when required by Rust.

```rust
use std::fmt::Debug;

use dioxus::prelude::*;

use crate::components::ButtonVariant;

fn primary_variant() -> ButtonVariant {
    ButtonVariant::Primary
}
```

## Formatting

- Use standard rustfmt defaults, including four-space indentation and its 100-column width target.
- Preserve existing CSS formatting, including two-space indentation.

## Functions & Code Structure

- Split functions by responsibility; impose no fixed function-length or parameter-count limit.
- Prefer early returns.
- Keep items private by default. Use `pub(crate)` for internal cross-module access and `pub` for intended public APIs.
- Keep reusable components focused on rendering and UI interaction.
- Let callers supply application-specific network, storage, and business operations through props or callbacks. For example, a Save button emits a click callback; the caller performs the save.

## Types & Safety

- Represent optional values with `Option<T>`.
- Keep bindings immutable unless mutation is needed.
- Validate external inputs at public boundaries using manual checks, in addition to compile-time prop types.
- Ask for approval before adding any diagnostic suppression, regardless of scope.

## Resource Ownership & Cleanup

- Clean up timers, event listeners, and subscriptions created by a component when that component is removed.
- Keep application-owned operations caller-owned.
- Defer task-lifetime, cancellation, and timeout decisions until a component needs them.

## Error Handling

- Use `Result<T, E>` for recoverable failures.
- Reserve panics for broken internal invariants; avoid `unwrap()` and `expect()` in production paths.
- Use category-based error enums where custom error types are needed.
- Write error messages that explain what failed and why.

## Text & Accessibility

- Accept user-facing text and accessible labels through props or children so applications can supply their own content.
- Preserve semantic HTML, keyboard interaction, and accessible names during migration, as required by the README.

## Comments & Documentation

- Comment why, rather than restating what the code does.
- Document public APIs with Rust doc comments.
- Include an issue link with TODO and FIXME markers.
- Keep the README's migration status accurate; do not describe starter implementations as faithful ports.

## Testing

- Keep rendering tests in `tests/`; allow colocated unit tests for private logic.
- Test each migrated component's states, rendered attributes, and accessibility behavior.
- Add showcase examples covering component states.
- Prefer observable behavior and public-contract assertions over implementation details.
- Prefer explicit assertions over snapshots.
- Keep tests independent of live network services. Mock only external boundaries when necessary.
- Fix flaky tests rather than hiding failures with automatic retries.

## Verification Workflow

- Run focused checks locally for the changes made; require the full existing check suite in CI.
- Run commands from the project root. See [README.md](README.md) for setup and [.github/workflows/ci.yml](.github/workflows/ci.yml) for CI.
- The full check suite is:

```sh
cargo fmt --check
cargo check --locked --lib --no-default-features
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo check --locked --example showcase --target wasm32-unknown-unknown
```

- Formatting and lint checks require rustfmt and Clippy. The WebAssembly check requires the `wasm32-unknown-unknown` target.
- Report checks run, unavailable or unrun checks, and pre-existing failures explicitly. Never claim an unrun check passed.

## File & Project Structure

- Keep each component in its own module under `src/components/`.
- Keep shared styles and tokens in the existing `assets/astryx.css` layout initially. Ask before reorganizing CSS.
- Keep showcase code in `examples/`; reusable components must not depend on the showcase.
- Ask before adding any dependency.

## Migration & CSS

- Obtain the original Astryx component source and design tokens before claiming migration fidelity.
- Inventory original components, dependencies, states, and tokens.
- Port shared tokens before matching component styles; preserve original Astryx tokens when porting.
- Reuse shared tokens for shared design values rather than duplicating literal values.
- Preserve appearance and behavior, and compare the port against the original before marking it migrated.
- Follow the README's migration sequence: Button and primitives first; defer complex focus-management components until primitives are stable.

## Git Workflow

- Use Conventional Commits, such as `feat: add badge component` or `fix: preserve button keyboard behavior`.

## Security

- Do not put secrets in source code, examples, or browser-delivered assets.
- Do not log sensitive prop values or user content.

## AI Collaboration

- Change only what was requested. Ask before unrelated refactors.
- Ask about consequential ambiguities; otherwise follow this guide and existing patterns.
- Ask for approval before breaking public component APIs or serialized formats.
- Do not leave placeholder implementations or mark unfinished ports complete. Report blockers instead.
- Keep completion reports brief: list changed files, checks run, unrun checks, and remaining blockers.
- Leave unanswered policies unspecified; ask when relevant work requires a decision.
