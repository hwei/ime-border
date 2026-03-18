# Design

## Summary

The product remains a Windows-first Python CLI packaged as a portable executable. It observes the foreground thread's default IME window, infers whether the active Microsoft Pinyin context is in English mode, and optionally projects that state onto komorebi border colors by invoking `komorebic.exe border-colour`.

## Product layers

- `ime_control.win32_ime`: foreground IME state acquisition and English-mode inference.
- `ime_control.komorebi`: komorebi command discovery and border-color application.
- `ime_control.watcher`: reusable polling loop with change detection and output projection.
- `ime_control.cli`: user-facing command tree.
