# Produce a third-party notices stub for release candidates.
# Full license-text aggregation may use cargo-deny / cargo-about once tooling is pinned.
# Usage:
#   powershell -NoProfile -File scripts/generate-third-party-notices.ps1
#   powershell -NoProfile -File scripts/generate-third-party-notices.ps1 -OutPath target/THIRD_PARTY_NOTICES.txt

[CmdletBinding()]
param(
    [string]$OutPath = "target/THIRD_PARTY_NOTICES.txt"
)

$ErrorActionPreference = "Stop"
$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
Set-Location $repoRoot

New-Item -ItemType Directory -Force -Path (Split-Path -Parent $OutPath) | Out-Null

$lines = @(
    "Solpaper third-party notices",
    "============================",
    "",
    "Solpaper application source is licensed under the MIT License (see LICENSE).",
    "This file summarizes dependency licensing for redistribution packages.",
    "",
    "Machine-checked license policy: deny.toml (cargo deny check licenses).",
    "Policy: docs/security/supply-chain.md",
    "",
    "To regenerate a detailed dependency license list when cargo-deny is installed:",
    "  cargo deny list",
    "",
    "Do not place secrets, tokens, or private Calendar data in this file.",
    "Generated (UTC): $((Get-Date).ToUniversalTime().ToString('o'))",
    ""
)

if (Get-Command cargo -ErrorAction SilentlyContinue) {
    $deny = Get-Command cargo-deny -ErrorAction SilentlyContinue
    if ($deny) {
        $lines += "--- cargo deny list (licenses) ---"
        $lines += ""
        try {
            $out = cargo deny list 2>&1 | Out-String
            $lines += $out
        } catch {
            $lines += "(cargo deny list failed: $_)"
        }
    } else {
        $lines += "(cargo-deny not on PATH; install to expand this notices file.)"
    }
}

Set-Content -Path $OutPath -Value ($lines -join "`r`n") -Encoding utf8
Write-Host "Wrote $OutPath"
