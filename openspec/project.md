# Project Context

## Repository Identity

- This repository is the product source of truth for the Windows CLI `ime-border`.
- Product-facing durable requirements live in `openspec/specs/`.
- Active work is captured as OpenSpec changes under `openspec/changes/`.

## Environment

- The product targets Windows and relies on Win32 IME behavior.
- `./.env` is local-only and must not be committed; `./.env.example` is the tracked template.
- `.tmp/` is the repository-local location for transient validation probes and scratch scripts and is kept out of normal version control.
- `.conda/` may still be used for local tooling, but the product implementation and packaging baseline is Rust.

## Working Agreement

- OpenSpec is the canonical governance and planning system for this repository.
- Use `openspec/project.md` for repository-wide context and collaboration rules.
- Use `openspec/specs/` for durable requirements that should survive individual changes.
- Use `openspec/changes/` for scoped change work. Proposal, specs, and tasks are the default minimum before substantial implementation.
- Treat non-archived OpenSpec changes as the repository planning surface.
- Treat backlog specifically as changes whose `meta.yaml` status is `queued`.

## Repository Layout

- `openspec/project.md`: repository-wide context and collaboration guidance
- `openspec/specs/`: durable capability requirements
- `openspec/changes/`: active and archived change proposals
- `openspec/templates/change-meta.yaml`: repository-owned metadata template for non-archived changes
- `src/`: repo-owned Rust CLI implementation baseline
- `tools/new_openspec_change.py`: helper that creates a new OpenSpec change and seeds `meta.yaml`
- `tools/openspec_backlog.py`: filter and rank non-archived changes from `meta.yaml`
- `tests/`: canonical repository-level tests
