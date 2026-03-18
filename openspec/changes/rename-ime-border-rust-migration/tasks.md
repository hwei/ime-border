# Tasks

## 1. Governance and contract

- [x] 1.1 Update durable OpenSpec requirements from `ime-control` to `ime-border`.
- [x] 1.2 Update repository context and layout notes to describe the Rust implementation baseline.

## 2. Rust migration

- [x] 2.1 Add a Rust crate that implements `once`, `watch`, and `border-watch`.
- [x] 2.2 Port the Win32 IME state detection and komorebi projection logic without changing the command contract.
- [x] 2.3 Replace local wrapper scripts, docs, and build flow to use the Rust binary and `ime-border` naming.

## 3. Validation and release

- [x] 3.1 Add repository-owned validation for the Rust behavior and current command contract.
- [x] 3.2 Add GitHub Actions workflows for Windows build and release automation.
- [x] 3.3 Run local validation and commit the migration.
