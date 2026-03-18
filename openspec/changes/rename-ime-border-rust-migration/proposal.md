# Proposal

## Why

The repository has a working Python implementation, but the product is intended to be a long-running Windows utility where memory footprint and native distribution matter more than rapid iteration. The current `ime-control` name also overstates control capabilities and is less clear than a name centered on IME state projection.

## What Changes

- Rename the product, repository-facing CLI identity, and portable artifact from `ime-control` to `ime-border`.
- Replace the Python runtime implementation and PyInstaller packaging flow with a Rust implementation that preserves the existing `once`, `watch`, and `border-watch` behavior.
- Keep the current machine-facing output contract and komorebi integration semantics stable while migrating the implementation language.
- Add repository-owned GitHub Actions workflows for Windows build and release automation around the Rust artifact.
