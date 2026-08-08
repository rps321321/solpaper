# Write release-manifest.json for a candidate artifact (unsigned by default).
# Schema: docs/security/release-manifest.schema.md
# Usage example:
#   powershell -NoProfile -File scripts/write-release-manifest.ps1 `
#     -Version 0.0.0-dev -ArtifactPath target/release/solpaper.exe `
#     -SbomPath target/sbom/solpaper.cdx.json -OutPath target/release-manifest.json

[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)][string]$Version,
    [Parameter(Mandatory = $true)][string]$ArtifactPath,
    [string]$SbomPath = "",
    [string]$NoticesPath = "",
    [string]$OutPath = "target/release-manifest.json",
    [string]$Target = "x86_64-pc-windows-msvc",
    [string]$Profile = "release",
    [string]$BuildWorkflow = "",
    [string]$BuildRunUrl = "",
    [ValidateSet("unsigned", "signed")][string]$SigningState = "unsigned",
    [string]$Notes = "Development candidate only; not a public release."
)

$ErrorActionPreference = "Stop"
$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
Set-Location $repoRoot

if ($SigningState -eq "signed") {
    throw "Agents must not set signing_state=signed. Human release process only."
}

function Get-Sha256Hex([string]$Path) {
    if (-not (Test-Path $Path)) { throw "Missing file for hash: $Path" }
    return (Get-FileHash -Algorithm SHA256 -Path $Path).Hash.ToLowerInvariant()
}

$lockPath = Join-Path $repoRoot "Cargo.lock"
if (-not (Test-Path $lockPath)) { throw "Cargo.lock missing; lockfile must be committed." }

$sourceSha = (git rev-parse HEAD 2>$null)
if (-not $sourceSha) { $sourceSha = "unknown" }

$rustc = (rustc --version 2>$null)
$cargo = (cargo --version 2>$null)
if (-not $rustc) { $rustc = "unknown" }
if (-not $cargo) { $cargo = "unknown" }

$manifest = [ordered]@{
    schema_version     = "1"
    product            = "solpaper"
    version            = $Version
    source_sha         = $sourceSha.Trim()
    target             = $Target
    profile            = $Profile
    features           = @()
    rustc_version      = "$rustc".Trim()
    cargo_version      = "$cargo".Trim()
    cargo_lock_sha256  = Get-Sha256Hex $lockPath
    artifact_path      = $ArtifactPath.Replace("\", "/")
    artifact_sha256    = Get-Sha256Hex $ArtifactPath
    sbom_path          = $null
    sbom_sha256        = $null
    notices_path       = $null
    notices_sha256     = $null
    build_workflow     = $(if ($BuildWorkflow) { $BuildWorkflow } else { $null })
    build_run_url      = $(if ($BuildRunUrl) { $BuildRunUrl } else { $null })
    signing_state      = $SigningState
    built_at_utc       = (Get-Date).ToUniversalTime().ToString("o")
    notes              = $Notes
}

if ($SbomPath -and (Test-Path $SbomPath)) {
    $manifest.sbom_path = $SbomPath.Replace("\", "/")
    $manifest.sbom_sha256 = Get-Sha256Hex $SbomPath
}
if ($NoticesPath -and (Test-Path $NoticesPath)) {
    $manifest.notices_path = $NoticesPath.Replace("\", "/")
    $manifest.notices_sha256 = Get-Sha256Hex $NoticesPath
}

$outDir = Split-Path -Parent $OutPath
if ($outDir) { New-Item -ItemType Directory -Force -Path $outDir | Out-Null }

$json = $manifest | ConvertTo-Json -Depth 6
Set-Content -Path $OutPath -Value $json -Encoding utf8
Write-Host "Wrote $OutPath (signing_state=$SigningState)"
