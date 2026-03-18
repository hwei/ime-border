# Design

## Scope

This change prepares the repository for public GitHub publication. It does not change the runtime CLI behavior or release artifact format.

## Repository Identity

- The canonical English landing page becomes `README.md`.
- A Chinese landing page is added as `README.zh-CN.md`.
- The repository includes an MIT license in `LICENSE`.

## Documentation

The English README should explain:

- what the tool does
- the primary download path
- minimum requirements
- a short quick-start flow
- the local release helper flow

The Chinese README should cover the same core user-facing information with wording optimized for Chinese-speaking users.

## Branch Alignment

- The local repository branch is renamed from `master` to `main`.
- The CI workflow already targets `main`, so no behavior change is needed after the rename.
