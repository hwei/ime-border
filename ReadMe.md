# ime-border

`ime-border` is a Windows-first command-line tool that detects whether the active Microsoft Pinyin IME is currently in English mode and can project that state onto komorebi border colors.

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
