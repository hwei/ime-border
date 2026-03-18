# ime-border

[English](README.md) | 简体中文

`ime-border` 是一个面向 Windows 的命令行工具，用来检测当前活动微软拼音输入上下文是否处于英文模式，并把这个状态映射到 komorebi 的窗口边框颜色上。

## 下载

推荐直接从 GitHub Releases 下载。每个版本会提供：

- `ime-border-vX.Y.Z-windows-x86_64.exe`
- `ime-border-vX.Y.Z-windows-x86_64.zip`
- `SHA256SUMS.txt`

## 运行要求

- Windows
- 目标输入法为微软拼音
- 使用 `border-watch` 时，需要 `komorebic` 已在 `PATH` 中，或者通过 `--komorebic` 显式指定路径

## 快速开始

下载最新版本的 exe 后，可以直接运行：

```powershell
.\ime-border.exe once
.\ime-border.exe watch
.\ime-border.exe border-watch
```

`once` 默认输出以下三种之一：

- `true`
- `false`
- `unknown`

如果是在仓库里本地开发，可以运行：

```powershell
.\ime-border.cmd once --verbose
.\ime-border.cmd watch --interval 0.1
.\ime-border.cmd border-watch --verbose
```

## 本地构建

构建 Windows release 二进制，并复制到 `dist/`：

```powershell
.\tools\build-portable.ps1
```

生成的可执行文件位于 `dist/ime-border.exe`。

## 发布流程

本地准备一个新版本时，运行：

```powershell
.\tools\release.ps1 0.1.1
```

这个脚本会：

- 更新 `Cargo.toml`
- 运行 `cargo test --all-targets`
- 创建版本 bump commit
- 创建对应的 annotated tag

脚本不会自动 push。执行完成后，再手动运行：

```powershell
git push origin HEAD
git push origin v0.1.1
```
