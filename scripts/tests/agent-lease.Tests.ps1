#Requires -Version 5.1
# Self-contained tests for scripts/agent-lease.ps1
# Run: powershell -NoProfile -File scripts/tests/agent-lease.Tests.ps1
# Exit 0 on success; non-zero on failure. No Pester dependency.
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$script:Failures = 0
$script:Passes = 0

function Assert-True([bool]$Cond, [string]$Msg) {
    if ($Cond) {
        $script:Passes++
        Write-Host "  PASS: $Msg"
    }
    else {
        $script:Failures++
        Write-Host "  FAIL: $Msg"
    }
}

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot '..\..')
$leaseScript = Join-Path $repoRoot 'scripts\agent-lease.ps1'
$tempRoot = Join-Path ([System.IO.Path]::GetTempPath()) ("solpaper-lease-test-" + [guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory -Force -Path $tempRoot | Out-Null

$tempScripts = Join-Path $tempRoot 'scripts'
New-Item -ItemType Directory -Force -Path $tempScripts | Out-Null
Copy-Item -LiteralPath $leaseScript -Destination (Join-Path $tempScripts 'agent-lease.ps1')

function Invoke-Lease {
    param(
        [Parameter(Mandatory = $true)][string[]]$LeaseArgs
    )
    $psi = Join-Path $tempScripts 'agent-lease.ps1'
    $allArgs = @('-NoProfile', '-ExecutionPolicy', 'Bypass', '-File', $psi) + $LeaseArgs + @('-RepoRoot', $tempRoot)
    $prevEap = $ErrorActionPreference
    $ErrorActionPreference = 'Continue'
    try {
        $output = & powershell @allArgs 2>&1
        $code = $LASTEXITCODE
    }
    finally {
        $ErrorActionPreference = $prevEap
    }
    $text = ($output | ForEach-Object { "$_" }) -join "`n"
    return [pscustomobject]@{
        ExitCode = $code
        Output   = $text
    }
}

try {
    Write-Host "=== agent-lease tests ==="
    Write-Host "temp root: $tempRoot"

    Write-Host ""
    Write-Host "[claim new]"
    $r = Invoke-Lease -LeaseArgs @('claim', '-Issue', '31', '-Owner', 'agent:a', '-Branch', 'issue-31-x', '-Unit', 'test', '-RiskClass', 'LOW')
    Assert-True ($r.ExitCode -eq 0) 'first claim exit 0'
    Assert-True ($r.Output -match 'CLAIM_OK') 'first claim CLAIM_OK'

    Write-Host ""
    Write-Host "[second claim denied]"
    $r = Invoke-Lease -LeaseArgs @('claim', '-Issue', '31', '-Owner', 'agent:b', '-Branch', 'other', '-Unit', 'steal', '-RiskClass', 'LOW')
    Assert-True ($r.ExitCode -ne 0) 'different owner claim fails'
    Assert-True ($r.Output -match 'CLAIM_DENIED') 'different owner CLAIM_DENIED'

    Write-Host ""
    Write-Host "[same owner refresh]"
    $r = Invoke-Lease -LeaseArgs @('claim', '-Issue', '31', '-Owner', 'agent:a', '-Branch', 'issue-31-x', '-Unit', 'refresh', '-RiskClass', 'LOW')
    Assert-True ($r.ExitCode -eq 0) 'same owner re-claim ok'
    Assert-True ($r.Output -match 'CLAIM_OK') 'same owner CLAIM_OK'

    Write-Host ""
    Write-Host "[heartbeat]"
    $r = Invoke-Lease -LeaseArgs @('heartbeat', '-Issue', '31', '-Owner', 'agent:a')
    Assert-True ($r.ExitCode -eq 0) 'heartbeat exit 0'
    Assert-True ($r.Output -match 'HEARTBEAT_OK') 'heartbeat HEARTBEAT_OK'

    Write-Host ""
    Write-Host "[heartbeat wrong owner]"
    $r = Invoke-Lease -LeaseArgs @('heartbeat', '-Issue', '31', '-Owner', 'agent:b')
    Assert-True ($r.ExitCode -ne 0) 'wrong owner heartbeat fails'
    Assert-True ($r.Output -match 'HEARTBEAT_DENIED') 'wrong owner HEARTBEAT_DENIED'

    Write-Host ""
    Write-Host "[status active]"
    $r = Invoke-Lease -LeaseArgs @('status', '-Issue', '31')
    Assert-True ($r.ExitCode -eq 0) 'status exit 0'
    Assert-True ($r.Output -match 'expired=False') 'status not expired'
    Assert-True ($r.Output -match 'owner=agent:a') 'status owner'

    Write-Host ""
    Write-Host "[release]"
    $r = Invoke-Lease -LeaseArgs @('release', '-Issue', '31', '-Owner', 'agent:a')
    Assert-True ($r.ExitCode -eq 0) 'release exit 0'
    Assert-True ($r.Output -match 'RELEASE_OK') 'release RELEASE_OK'

    Write-Host ""
    Write-Host "[claim after release]"
    $r = Invoke-Lease -LeaseArgs @('claim', '-Issue', '31', '-Owner', 'agent:b', '-Branch', 'issue-31-y', '-Unit', 'next', '-RiskClass', 'MEDIUM')
    Assert-True ($r.ExitCode -eq 0) 'claim after release ok'
    Assert-True ($r.Output -match 'CLAIM_OK') 'claim after release CLAIM_OK'

    Write-Host ""
    Write-Host "[critical denied]"
    $r = Invoke-Lease -LeaseArgs @('claim', '-Issue', '99', '-Owner', 'agent:a', '-Branch', 'x', '-Unit', 'bad', '-RiskClass', 'CRITICAL')
    Assert-True ($r.ExitCode -ne 0) 'CRITICAL claim fails'
    Assert-True ($r.Output -match 'CRITICAL') 'CRITICAL message present'

    Write-Host ""
    Write-Host "[expired reclaim]"
    $leasePath = Join-Path $tempRoot '.agent\leases\issue-31.json'
    $obj = Get-Content -LiteralPath $leasePath -Raw | ConvertFrom-Json
    $obj.expires_at = ([DateTime]::UtcNow.AddHours(-1)).ToString('o')
    $obj.status = 'active'
    $obj.owner = 'agent:b'
    [System.IO.File]::WriteAllText($leasePath, ($obj | ConvertTo-Json -Depth 6), [System.Text.UTF8Encoding]::new($false))
    $workMarker = Join-Path $tempRoot 'work-branch-marker.txt'
    Set-Content -LiteralPath $workMarker -Value 'do-not-delete'
    $r = Invoke-Lease -LeaseArgs @('claim', '-Issue', '31', '-Owner', 'agent:c', '-Branch', 'issue-31-z', '-Unit', 'reclaim', '-RiskClass', 'LOW')
    Assert-True ($r.ExitCode -eq 0) 'expired reclaim exit 0'
    Assert-True ($r.Output -match 'CLAIM_OK') 'expired reclaim CLAIM_OK'
    Assert-True (Test-Path $workMarker) 'reclaim does not delete work marker'
    $new = Get-Content -LiteralPath $leasePath -Raw | ConvertFrom-Json
    Assert-True ($new.owner -eq 'agent:c') 'reclaim updates owner'

    Write-Host ""
    Write-Host "[list]"
    $r = Invoke-Lease -LeaseArgs @('list')
    Assert-True ($r.ExitCode -eq 0) 'list exit 0'
    Assert-True ($r.Output -match 'issue=31') 'list shows issue 31'

    Write-Host ""
    Write-Host ("Results: {0} passed, {1} failed" -f $script:Passes, $script:Failures)
    if ($script:Failures -gt 0) { exit 1 }
    exit 0
}
finally {
    Remove-Item -LiteralPath $tempRoot -Recurse -Force -ErrorAction SilentlyContinue
}
