# Zed Development Guide

## Commands
- **Build**: `cargo build -p zed`
- **Run**: `cargo run`
- **Lint**: `./script/clippy` (not `cargo clippy`)
- **Test all**: `cargo nextest run` or `cargo test`
- **Test single**: `cargo nextest run -p <crate> <test_name>` or `cargo test -p <crate> <test_name>`

## Code Style
- Avoid `unwrap()`; use `?` to propagate errors. Never silently discard errors with `let _ =`; use `.log_err()` if ignoring.
- No `mod.rs` files; use `src/module_name.rs`. New crates: set `[lib] path = "crate_name.rs"` in Cargo.toml.
- Full variable names (no abbreviations). Comments only explain "why", not "what".
- Clone before async: `cx.spawn({ let foo = foo.clone(); async move |cx| { ... } })`

## GPUI Framework
- Context arg named `cx`; Window arg named `window` (comes before `cx`). Callbacks come after `cx`.
- Use `Entity<T>`, `App`, `Context<T>`. Never use deprecated: `Model`, `View`, `AppContext`, `ModelContext`, `WindowContext`, `ViewContext`.
- Async: `cx.spawn(async move |cx| ...)` for foreground, `cx.background_spawn(async move { ... })` for background.
- Call `cx.notify()` when state changes affect rendering.

## Architecture
Key crates: `gpui` (UI framework), `editor` (text editing), `workspace` (window/layout), `project` (files/LSP), `ui` (components).
