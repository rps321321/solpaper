# Generate CycloneDX JSON SBOMs for the Solpaper workspace and collect under OutDir.
# Requires: cargo-cyclonedx on PATH (pin version in CI/release docs; #38 uses 0.5.7).
# Usage:
#   powershell -NoProfile -File scripts/generate-sbom.ps1
#   powershell -NoProfile -File scripts/generate-sbom.ps1 -OutDir target/sbom
# Does not upload artifacts or sign anything. Never embed secrets.

[CmdletBinding()]
param(
    [string]$OutDir = "target/sbom"
)

$ErrorActionPreference = "Stop"
$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
Set-Location $repoRoot

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    throw "cargo not found on PATH"
}

$null = cargo cyclonedx --help 2>&1
if ($LASTEXITCODE -ne 0) {
    throw "cargo-cyclonedx not installed. Example: cargo install cargo-cyclonedx --locked --version 0.5.7"
}

New-Item -ItemType Directory -Force -Path $OutDir | Out-Null

Write-Host "Generating CycloneDX JSON SBOMs (workspace members)..."
cargo cyclonedx --format json --manifest-path Cargo.toml --all-features -q
if ($LASTEXITCODE -ne 0) {
    throw "cargo cyclonedx failed with exit $LASTEXITCODE"
}

# cargo-cyclonedx 0.5.x writes <crate>/<crate>.cdx.json next to each package.
$moved = 0
Get-ChildItem -Path (Join-Path $repoRoot "crates") -Filter "*.cdx.json" -Recurse -ErrorAction SilentlyContinue |
    ForEach-Object {
        $dest = Join-Path $OutDir $_.Name
        Move-Item -Path $_.FullName -Destination $dest -Force
        $moved++
        Write-Host "Moved $($_.Name) -> $dest"
    }

# Canonical name for the binary package SBOM (used by release-manifest defaults).
$appBom = Join-Path $OutDir "solpaper-app.cdx.json"
$canonical = Join-Path $OutDir "solpaper.cdx.json"
if (Test-Path $appBom) {
    Copy-Item $appBom -Destination $canonical -Force
    Write-Host "Canonical SBOM: $canonical"
}

if ($moved -eq 0 -and -not (Test-Path $canonical)) {
    throw "No .cdx.json files produced under crates/; check cargo-cyclonedx version and workspace layout."
}

Write-Host "Done ($moved file(s) in $OutDir). Hash with Get-FileHash -Algorithm SHA256 for the release manifest."
