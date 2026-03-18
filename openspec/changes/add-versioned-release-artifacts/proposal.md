# Proposal

## Why

The repository can already publish `ime-border.exe`, but the release artifact shape is still too thin for normal GitHub distribution. Users benefit from versioned filenames, a zip payload, and repository-owned checksums so downloads are easier to identify and verify.

## What Changes

- Define a repository-owned release artifact convention for GitHub releases.
- Update the release workflow to publish a versioned exe, a versioned zip archive, and `SHA256SUMS.txt`.
- Update the README to document GitHub Releases as the primary download path.
