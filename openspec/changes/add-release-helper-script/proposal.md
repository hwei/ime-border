# Proposal

## Why

The repository can already build and publish tagged releases, but the maintainer still has to manually bump `Cargo.toml`, commit the version change, and create the matching Git tag. Those steps are mechanical and error-prone enough to deserve a repository-owned helper.

## What Changes

- Add a repository-owned PowerShell release helper that normalizes a version input, updates `Cargo.toml`, runs validation, commits the version bump, and creates the matching Git tag.
- Document the intended local release flow in the README.
- Record the helper as part of repository governance rather than product runtime behavior.
