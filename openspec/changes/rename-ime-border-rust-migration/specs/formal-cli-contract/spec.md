# Formal CLI Contract Delta

## MODIFIED Requirements

### Requirement: The CLI has one primary entry and flat command tree

The formal CLI SHALL use `ime-border` as its single primary entry. The authoritative flat command tree SHALL include `once`, `watch`, and `border-watch`.

### Requirement: Portable build flow remains repository-owned

The repository SHALL define a Rust-owned portable executable build flow for `ime-border`.

#### Scenario: Maintainer builds a portable executable

- **WHEN** a maintainer follows the documented repository build command
- **THEN** the repository produces `ime-border.exe` from the Rust release build
