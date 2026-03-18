# Formal CLI Contract

## Purpose

Define the durable machine-facing contract for the `ime-border` CLI, including current-state querying, watch-mode output, komorebi border integration, and portable distribution expectations.

## Requirements

### Requirement: The CLI has one primary entry and flat command tree

The formal CLI SHALL use `ime-border` as its single primary entry. The authoritative flat command tree SHALL include `once`, `watch`, and `border-watch`.

#### Scenario: Agent discovers the CLI surface

- **WHEN** repository docs or help describe the CLI
- **THEN** `ime-border` is presented as the primary entry
- **AND** the help surface exposes `once`, `watch`, and `border-watch`

### Requirement: Single-state query returns a machine-usable English-mode result

`once` SHALL attempt to read the foreground input context and emit a machine-usable result for whether the active Microsoft Pinyin context is in English mode. The default stdout contract SHALL be `true`, `false`, or `unknown`.

#### Scenario: English mode is active

- **WHEN** `ime-border once` reads a foreground Microsoft Pinyin context whose conversion mode is alphanumeric
- **THEN** stdout emits `true`

### Requirement: Watch mode reports only state changes

`watch` SHALL poll the foreground state and emit one line when the machine-usable state changes.

#### Scenario: User toggles Chinese to English

- **WHEN** `ime-border watch` is running and the observed machine state changes from non-English to English
- **THEN** stdout emits exactly one new line for the new state until the next observed change

### Requirement: Border-watch projects state onto komorebi

`border-watch` SHALL support projecting the observed state onto komorebi border colors. When configured for komorebi control, the command SHALL invoke `komorebic.exe` only when the effective projected state changes.

#### Scenario: English mode change updates komorebi

- **WHEN** `ime-border border-watch` observes a change into English mode
- **THEN** the command invokes the configured komorebi border-color update for the English state

### Requirement: Portable build flow remains repository-owned

The repository SHALL define a portable executable build flow for `ime-border`.

#### Scenario: Maintainer builds a portable executable

- **WHEN** a maintainer follows the documented build command
- **THEN** the repository produces a Windows executable artifact for `ime-border`
