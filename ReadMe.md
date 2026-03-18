# ime-control

`ime-control` is a Windows-first command-line tool that detects whether the active Microsoft Pinyin IME is currently in English mode and can project that state onto komorebi border colors.

## Commands

```powershell
.\ime-control.cmd
.\ime-control.cmd once --verbose
.\ime-control.cmd watch --interval 0.1
.\ime-control.cmd border-watch --verbose
```

Default `once` output is one of:

- `true`
- `false`
- `unknown`

## Portable build

Install the build dependency and emit a single-file exe:

```powershell
.\.conda\python.exe -m pip install -e .[build]
.\.conda\python.exe .\tools\build_portable_exe.py
```

The resulting executable is produced under `dist/ime-control.exe`.
