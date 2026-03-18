## Environment setup

- `./.env` is local-only and must not be committed; use `./.env.example` as the tracked template.
- `.conda/` is a repository-local development environment and should stay out of normal source changes.

## OpenSpec entry points

- `openspec/project.md` is the canonical repository-wide context file.
- `openspec/specs/` holds durable requirements that survive individual changes.
- `openspec/changes/` holds active and archived change artifacts.
- `tests/` is the canonical repository-level test location.
- `.tmp/` is the preferred local-only location for transient validation probes and scratch scripts.

## Change metadata

- Every non-archived OpenSpec change should carry a repository-owned `meta.yaml`.
- `meta.yaml` currently tracks `status`, `change_type`, `priority`, `blocked_by`, `assumption_state`, `evidence`, and `updated_at`.
- Allowed `status` values: `queued`, `active`, `blocked`, `superseded`.
- Allowed `change_type` values: `feature`, `harness`, `validation`, `refactor`, `spike`.
- Allowed `priority` values: `P0`, `P1`, `P2`.
- Allowed `assumption_state` values: `valid`, `needs-review`, `invalid`.
- Allowed `evidence` values: `tests`, `host-validation`, `cli-transcript`, `manual-check`.

## Apply closeout

- Every apply session should end with an explicit closeout finding summary.
- The summary must say either `No new follow-up work identified` or `New follow-up candidates identified`.
- If new follow-up candidates are identified, discuss them with the human before promoting them into further work.
