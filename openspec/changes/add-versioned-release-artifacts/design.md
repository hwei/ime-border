# Design

## Scope

This change only affects release packaging and documentation. It does not change the runtime CLI contract.

## Artifact Convention

For a release tag `vX.Y.Z`, the Windows GitHub Release SHALL attach:

- `ime-border-vX.Y.Z-windows-x86_64.exe`
- `ime-border-vX.Y.Z-windows-x86_64.zip`
- `SHA256SUMS.txt`

The zip archive contains the exe artifact.

## Workflow

- Reuse the existing Windows release build.
- Rename the built executable to the versioned release filename.
- Produce a zip archive containing that executable.
- Produce SHA256 hashes for the exe and zip artifacts.
- Upload all three artifacts to the GitHub Release for the pushed tag.
