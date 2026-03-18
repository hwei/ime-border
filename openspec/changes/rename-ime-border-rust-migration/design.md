# Design

## Scope

This change is an equivalent rewrite plus product rename. It does not add IME state switching, WezTerm integration, config files, or daemon/service behavior.

## Product Identity

- Primary CLI entry becomes `ime-border`.
- Portable executable artifact becomes `ime-border.exe`.
- Existing subcommands remain `once`, `watch`, and `border-watch`.

## Implementation Strategy

- Introduce a single Rust binary crate as the product source of truth.
- Preserve the Win32 detection path based on foreground window, GUI thread focus resolution, `ImmGetDefaultIMEWnd`, and `WM_IME_CONTROL` queries.
- Preserve current English-mode semantics: `IME_CMODE_NATIVE == 0` means English mode.
- Preserve watch-mode semantics: emit only on effective state changes.
- Preserve komorebi projection semantics: invoke `komorebic` only when the observed machine state changes.

## Packaging

- Replace PyInstaller packaging with `cargo build --release`.
- Provide a repository wrapper script for local use that launches the Rust binary through `cargo run` in development.
- Publish GitHub Actions workflows that build Windows release artifacts and attach `ime-border.exe` to GitHub Releases on tags.

## Validation

- Recreate current repository-level tests against the Rust implementation.
- Run local smoke checks for `once` and the watcher flows after the migration.
