# ime-border

`ime-border` is a Windows-first command-line tool that detects whether the active Microsoft Pinyin IME is currently in English mode and can project that state onto komorebi border colors.

## Download

GitHub Releases is the primary download path. Each tagged release publishes:

- `ime-border-vX.Y.Z-windows-x86_64.exe`
- `ime-border-vX.Y.Z-windows-x86_64.zip`
- `SHA256SUMS.txt`

## Commands

```powershell
.\ime-border.cmd
.\ime-border.cmd once --verbose
.\ime-border.cmd watch --interval 0.1
.\ime-border.cmd border-watch --verbose
```

Default `once` output is one of:

- `true`
- `false`
- `unknown`

## Portable build

Build the Windows release binary and copy it into `dist/`:

```powershell
.\tools\build-portable.ps1
```

The resulting executable is produced under `dist/ime-border.exe`.

## Release flow

Prepare a release locally with:

```powershell
.\tools\release.ps1 0.1.1
```

The helper script:

- updates `Cargo.toml`
- runs `cargo test --all-targets`
- creates a version bump commit
- creates the matching annotated tag

It does not push automatically. After it finishes, push the branch and tag yourself:

```powershell
git push origin HEAD
git push origin v0.1.1
```
