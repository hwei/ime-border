# Repository Governance

## Purpose

Define the repository's canonical OpenSpec-first workflow, including change gating and durable-truth distillation.

## Requirements

### Requirement: OpenSpec is the canonical repository workflow

The repository SHALL use OpenSpec as the canonical system for project context, durable governance rules, and active change planning.

#### Scenario: Fresh contributor looks for process entry

- **WHEN** a contributor or agent needs the repository workflow entry point
- **THEN** `openspec/project.md` provides repository-wide context
- **AND** `openspec/specs/repository-governance/spec.md` defines the durable workflow rules

### Requirement: Substantial work requires OpenSpec change context

Substantial work SHALL be scoped through an OpenSpec change before implementation is treated as canonical. The minimum apply-ready artifact set MUST include proposal, specs, and tasks.

#### Scenario: Repository change is about to begin

- **WHEN** a contributor plans a substantial product or tooling change
- **THEN** the contributor creates or continues an OpenSpec change
- **AND** implementation does not proceed as normal repository workflow until the change includes proposal, specs, and tasks artifacts
