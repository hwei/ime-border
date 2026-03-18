[CmdletBinding(SupportsShouldProcess = $true)]
param(
    [Parameter(Mandatory = $true, Position = 0)]
    [string]$Version
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

function Normalize-Version([string]$InputVersion) {
    if ($InputVersion -match '^v?(?<version>\d+\.\d+\.\d+)$') {
        return $Matches.version
    }
    throw "Version must look like X.Y.Z or vX.Y.Z. Got '$InputVersion'."
}

function Invoke-Git([string[]]$Arguments) {
    & git @Arguments
    if ($LASTEXITCODE -ne 0) {
        throw "git $($Arguments -join ' ') failed with exit code $LASTEXITCODE"
    }
}

function Invoke-Cargo([string[]]$Arguments) {
    $cargo = Join-Path $env:USERPROFILE ".cargo\bin\cargo.exe"
    if (-not (Test-Path $cargo)) {
        throw "cargo.exe not found at $cargo"
    }
    & $cargo @Arguments
    if ($LASTEXITCODE -ne 0) {
        throw "cargo $($Arguments -join ' ') failed with exit code $LASTEXITCODE"
    }
}

$normalizedVersion = Normalize-Version $Version
$tag = "v$normalizedVersion"
$repoRoot = Split-Path -Parent $PSScriptRoot
$cargoTomlPath = Join-Path $repoRoot "Cargo.toml"

Set-Location $repoRoot

$statusLines = git status --short
if ($LASTEXITCODE -ne 0) {
    throw "git status --short failed with exit code $LASTEXITCODE"
}
if ($statusLines) {
    throw "Git worktree must be clean before preparing a release."
}

$cargoToml = Get-Content $cargoTomlPath -Raw
$updatedCargoToml = [regex]::Replace(
    $cargoToml,
    '(?m)^version = "\d+\.\d+\.\d+"$',
    "version = `"$normalizedVersion`"",
    1
)
if ($updatedCargoToml -eq $cargoToml) {
    throw "Failed to locate the package version line in Cargo.toml."
}

if (-not $PSCmdlet.ShouldProcess($cargoTomlPath, "Set package version to $normalizedVersion")) {
    return
}

Set-Content -Path $cargoTomlPath -Value $updatedCargoToml -NoNewline

try {
    Invoke-Cargo @("test", "--all-targets")
    Invoke-Git @("add", "Cargo.toml")
    Invoke-Git @("commit", "-m", "Bump version to $normalizedVersion")
    Invoke-Git @("tag", "-a", $tag, "-m", "Release $tag")
}
catch {
    throw
}

Write-Output "Prepared release $tag"
Write-Output "Next steps:"
Write-Output "  git push origin HEAD"
Write-Output "  git push origin $tag"
