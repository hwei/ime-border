# Proposal

## Why

The repository contains an initial IME English-mode probe, but it lacks the repository governance, product contract, and delivery shape needed for normal use as a portable desktop utility.

## What Changes

- Establish repository governance with OpenSpec-first workflow and metadata tooling.
- Formalize the CLI contract for `once`, `watch`, and komorebi-oriented `border-watch`.
- Implement a Windows CLI that monitors foreground Microsoft Pinyin English mode and projects changes onto komorebi borders.
- Add portable executable build support and repository-owned packaging instructions.
