$ErrorActionPreference = "Stop"

$cargo = Join-Path $env:USERPROFILE ".cargo\bin\cargo.exe"
if (-not (Test-Path $cargo)) {
    throw "cargo.exe not found at $cargo"
}

& $cargo build --release
if ($LASTEXITCODE -ne 0) {
    throw "cargo build --release failed with exit code $LASTEXITCODE"
}

$distDir = Join-Path $PSScriptRoot "..\dist"
New-Item -ItemType Directory -Path $distDir -Force | Out-Null

$sourceExe = Join-Path $PSScriptRoot "..\target\release\ime-border.exe"
$targetExe = Join-Path $distDir "ime-border.exe"
Copy-Item $sourceExe $targetExe -Force
Write-Output "Built $targetExe"
