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

$copied = $false
for ($attempt = 1; $attempt -le 5; $attempt++) {
    try {
        Copy-Item $sourceExe $targetExe -Force
        $copied = $true
        break
    }
    catch {
        if ($attempt -eq 5) {
            throw
        }
        Start-Sleep -Milliseconds 400
    }
}

if (-not $copied) {
    throw "Failed to copy $sourceExe to $targetExe"
}

Write-Output "Built $targetExe"
