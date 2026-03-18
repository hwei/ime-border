# Repository Governance Delta

## MODIFIED Requirements

### Requirement: OpenSpec is the canonical repository workflow

The repository SHALL use OpenSpec as the canonical system for project context, durable governance rules, active change planning, and release helper tooling.

#### Scenario: Maintainer prepares a release

- **WHEN** a maintainer needs to bump the repository version and create a matching release tag
- **THEN** the repository provides a maintained helper script for the local release preparation steps
