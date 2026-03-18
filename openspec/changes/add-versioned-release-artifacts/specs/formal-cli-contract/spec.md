# Formal CLI Contract Delta

## MODIFIED Requirements

### Requirement: Portable build flow remains repository-owned

The repository SHALL define a portable executable build flow for `ime-border`, and the GitHub release process SHALL publish versioned Windows artifacts with repository-owned checksums.

#### Scenario: Maintainer publishes a tagged release

- **WHEN** a maintainer pushes a tag matching `v*`
- **THEN** the GitHub Release contains a versioned Windows exe, a versioned Windows zip archive, and `SHA256SUMS.txt`
