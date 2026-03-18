# Design

## Scope

This change adds repository tooling only. It does not change the runtime behavior of `ime-border`.

## Script Contract

The helper script lives at `tools/release.ps1` and accepts a semantic version input with or without a leading `v`.

The script SHALL:

- normalize the input to `X.Y.Z`
- derive the matching Git tag as `vX.Y.Z`
- fail fast if the git worktree is not clean
- update the `version` field in `Cargo.toml`
- run `cargo test`
- create a commit for the version bump
- create the matching annotated Git tag

The script SHOULD support PowerShell `-WhatIf` behavior so maintainers can validate the steps without mutating the repository.

## Push Behavior

The helper does not push automatically. Pushing the branch and tag remains an explicit maintainer action.
