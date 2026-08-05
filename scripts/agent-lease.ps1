#Requires -Version 5.1
<#
.SYNOPSIS
  Atomic issue lease claim/heartbeat/release for Solpaper autonomous agents.

.DESCRIPTION
  Lease store: .agent/leases/issue-<N>.json
  Claim uses FileMode.CreateNew for exclusivity when no valid lease exists.
  Expired or released leases are reclaimable without deleting branch/PR work.

.EXAMPLE
  .\scripts\agent-lease.ps1 claim -Issue 31 -Owner 'agent:solpaper-dev-loop' -Branch 'issue-31-agent-governance' -Unit 'governance' -RiskClass LOW
  .\scripts\agent-lease.ps1 heartbeat -Issue 31 -Owner 'agent:solpaper-dev-loop'
  .\scripts\agent-lease.ps1 release -Issue 31 -Owner 'agent:solpaper-dev-loop'
  .\scripts\agent-lease.ps1 status -Issue 31
#>
[CmdletBinding()]
param(
    [Parameter(Position = 0, Mandatory = $true)]
    [ValidateSet('claim', 'heartbeat', 'release', 'status', 'list')]
    [string]$Command,

    [int]$Issue = 0,

    [string]$Owner = '',

    [string]$Branch = '',

    [string]$Unit = '',

    [ValidateSet('LOW', 'MEDIUM', 'HIGH', 'CRITICAL', '')]
    [string]$RiskClass = '',

    [Nullable[int]]$Pr = $null,

    [int]$TtlHours = 2,

    [string]$RepoRoot = ''
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

function Exit-Fail([string]$Message) {
    [Console]::Error.WriteLine($Message)
    exit 1
}

function Get-RepoRoot {
    if ($RepoRoot) { return (Resolve-Path $RepoRoot).Path }
    $here = $PSScriptRoot
    return (Resolve-Path (Join-Path $here '..')).Path
}

function Get-LeaseDir([string]$Root) {
    $dir = Join-Path $Root '.agent\leases'
    if (-not (Test-Path $dir)) {
        New-Item -ItemType Directory -Force -Path $dir | Out-Null
    }
    return $dir
}

function Get-LeasePath([string]$Root, [int]$IssueNumber) {
    return Join-Path (Get-LeaseDir $Root) ("issue-{0}.json" -f $IssueNumber)
}

function Read-Lease([string]$Path) {
    if (-not (Test-Path $Path)) { return $null }
    $raw = Get-Content -LiteralPath $Path -Raw -Encoding UTF8
    return $raw | ConvertFrom-Json
}

function Write-LeaseAtomic([string]$Path, $LeaseObject) {
    $json = $LeaseObject | ConvertTo-Json -Depth 6
    $dir = Split-Path -Parent $Path
    $tmp = Join-Path $dir (".lease-{0}.tmp" -f [guid]::NewGuid().ToString('N'))
    $bak = Join-Path $dir (".lease-{0}.bak" -f [guid]::NewGuid().ToString('N'))
    try {
        [System.IO.File]::WriteAllText($tmp, $json, [System.Text.UTF8Encoding]::new($false))
        if (Test-Path -LiteralPath $Path) {
            # Replace with explicit backup path (null backup is illegal on some .NET/Windows hosts)
            [System.IO.File]::Replace($tmp, $Path, $bak)
            Remove-Item -LiteralPath $bak -Force -ErrorAction SilentlyContinue
        }
        else {
            [System.IO.File]::Move($tmp, $Path)
        }
    }
    finally {
        if (Test-Path -LiteralPath $tmp) { Remove-Item -LiteralPath $tmp -Force -ErrorAction SilentlyContinue }
        if (Test-Path -LiteralPath $bak) { Remove-Item -LiteralPath $bak -Force -ErrorAction SilentlyContinue }
    }
}

function New-LeaseObject {
    param(
        [int]$IssueNumber,
        [string]$OwnerName,
        [string]$BranchName,
        [string]$UnitText,
        [string]$Risk,
        [Nullable[int]]$PrNumber,
        [int]$Ttl,
        [string]$Status = 'active'
    )
    $now = [DateTime]::UtcNow
    $exp = $now.AddHours($Ttl)
    return [pscustomobject]@{
        issue         = $IssueNumber
        owner         = $OwnerName
        branch        = $BranchName
        claimed_at    = $now.ToString('o')
        expires_at    = $exp.ToString('o')
        heartbeat_at  = $now.ToString('o')
        status        = $Status
        unit          = $UnitText
        pr            = if ($null -eq $PrNumber) { $null } else { [int]$PrNumber }
        risk_class    = $Risk
    }
}

function Test-LeaseExpired($Lease) {
    if ($null -eq $Lease) { return $true }
    if ($Lease.status -eq 'released') { return $true }
    if ($Lease.status -eq 'expired_reclaimed') { return $true }
    $exp = [DateTime]::Parse([string]$Lease.expires_at, $null, [System.Globalization.DateTimeStyles]::RoundtripKind)
    if ($exp.Kind -eq [DateTimeKind]::Unspecified) {
        $exp = [DateTime]::SpecifyKind($exp, [DateTimeKind]::Utc)
    }
    return ([DateTime]::UtcNow -ge $exp.ToUniversalTime())
}

function Assert-Issue {
    if ($Issue -le 0) { Exit-Fail "Issue number must be a positive integer." }
}

function Invoke-Claim {
    Assert-Issue
    if (-not $Owner) { Exit-Fail "Owner is required for claim." }
    if (-not $Branch) { Exit-Fail "Branch is required for claim." }
    if (-not $RiskClass) { Exit-Fail "RiskClass is required for claim (LOW|MEDIUM|HIGH|CRITICAL)." }
    if ($RiskClass -eq 'CRITICAL') {
        Exit-Fail "CRITICAL risk cannot be claimed by autonomous agents. Human-only."
    }

    $root = Get-RepoRoot
    $path = Get-LeasePath $root $Issue
    $existing = Read-Lease $path

    if ($null -ne $existing -and -not (Test-LeaseExpired $existing)) {
        if ($existing.owner -eq $Owner -and $existing.status -eq 'active') {
            # Same owner re-claim: refresh heartbeat/fields
            $existing.branch = $Branch
            $existing.unit = $(if ($Unit) { $Unit } else { $existing.unit })
            $existing.risk_class = $RiskClass
            if ($null -ne $Pr) { $existing.pr = [int]$Pr }
            $now = [DateTime]::UtcNow
            $existing.heartbeat_at = $now.ToString('o')
            $existing.expires_at = $now.AddHours($TtlHours).ToString('o')
            $existing.status = 'active'
            Write-LeaseAtomic $path $existing
            Write-Output "CLAIM_OK same_owner refresh issue=$Issue path=$path"
            return
        }
        Exit-Fail ("CLAIM_DENIED active lease held by '{0}' on branch '{1}' until {2}" -f $existing.owner, $existing.branch, $existing.expires_at)
    }

    $wasExpired = ($null -ne $existing) -and (Test-LeaseExpired $existing) -and ($existing.status -eq 'active')
    $lease = New-LeaseObject -IssueNumber $Issue -OwnerName $Owner -BranchName $Branch `
        -UnitText $(if ($Unit) { $Unit } else { '' }) -Risk $RiskClass -PrNumber $Pr -Ttl $TtlHours
    if ($wasExpired) {
        $lease | Add-Member -NotePropertyName prior_owner -NotePropertyValue $existing.owner -Force
        # status remains active; reclaim is recorded via prior_owner + new claimed_at
    }

    if (-not (Test-Path $path)) {
        # Atomic create: fail if another process created it first
        $json = $lease | ConvertTo-Json -Depth 6
        try {
            $fs = [System.IO.File]::Open($path, [System.IO.FileMode]::CreateNew, [System.IO.FileAccess]::Write, [System.IO.FileShare]::None)
            try {
                $bytes = [System.Text.UTF8Encoding]::new($false).GetBytes($json)
                $fs.Write($bytes, 0, $bytes.Length)
            }
            finally { $fs.Dispose() }
        }
        catch [System.IO.IOException] {
            $again = Read-Lease $path
            if ($null -ne $again -and -not (Test-LeaseExpired $again) -and $again.owner -ne $Owner) {
                Exit-Fail ("CLAIM_DENIED race: active lease held by '{0}'" -f $again.owner)
            }
            # Lost race but winner expired or same owner path — fall through to replace if still appropriate
            Write-LeaseAtomic $path $lease
        }
    }
    else {
        Write-LeaseAtomic $path $lease
    }

    $tag = if ($wasExpired) { 'reclaimed_expired' } else { 'new' }
    Write-Output "CLAIM_OK $tag issue=$Issue owner=$Owner branch=$Branch path=$path"
}

function Invoke-Heartbeat {
    Assert-Issue
    if (-not $Owner) { Exit-Fail "Owner is required for heartbeat." }
    $root = Get-RepoRoot
    $path = Get-LeasePath $root $Issue
    $lease = Read-Lease $path
    if ($null -eq $lease) { Exit-Fail "HEARTBEAT_DENIED no lease for issue $Issue" }
    if ($lease.owner -ne $Owner) { Exit-Fail ("HEARTBEAT_DENIED owner mismatch (have {0})" -f $lease.owner) }
    if ($lease.status -ne 'active') { Exit-Fail "HEARTBEAT_DENIED lease status is $($lease.status)" }
    if (Test-LeaseExpired $lease) { Exit-Fail "HEARTBEAT_DENIED lease already expired; re-claim required" }

    $now = [DateTime]::UtcNow
    $lease.heartbeat_at = $now.ToString('o')
    $lease.expires_at = $now.AddHours($TtlHours).ToString('o')
    if ($null -ne $Pr) { $lease.pr = [int]$Pr }
    if ($Unit) { $lease.unit = $Unit }
    Write-LeaseAtomic $path $lease
    Write-Output "HEARTBEAT_OK issue=$Issue expires_at=$($lease.expires_at)"
}

function Invoke-Release {
    Assert-Issue
    if (-not $Owner) { Exit-Fail "Owner is required for release." }
    $root = Get-RepoRoot
    $path = Get-LeasePath $root $Issue
    $lease = Read-Lease $path
    if ($null -eq $lease) {
        Write-Output "RELEASE_OK no_lease issue=$Issue"
        return
    }
    if ($lease.owner -ne $Owner -and -not (Test-LeaseExpired $lease)) {
        Exit-Fail ("RELEASE_DENIED active lease owned by '{0}'" -f $lease.owner)
    }
    $lease.status = 'released'
    $lease.heartbeat_at = [DateTime]::UtcNow.ToString('o')
    Write-LeaseAtomic $path $lease
    Write-Output "RELEASE_OK issue=$Issue path=$path"
}

function Invoke-Status {
    $root = Get-RepoRoot
    if ($Issue -gt 0) {
        $path = Get-LeasePath $root $Issue
        $lease = Read-Lease $path
        if ($null -eq $lease) {
            Write-Output "STATUS none issue=$Issue"
            return
        }
        $expired = Test-LeaseExpired $lease
        Write-Output ("STATUS issue={0} owner={1} branch={2} status={3} expired={4} expires_at={5} risk={6} pr={7}" -f `
                $lease.issue, $lease.owner, $lease.branch, $lease.status, $expired, $lease.expires_at, $lease.risk_class, $lease.pr)
        return
    }
    Invoke-List
}

function Invoke-List {
    $root = Get-RepoRoot
    $dir = Get-LeaseDir $root
    $files = Get-ChildItem -LiteralPath $dir -Filter 'issue-*.json' -ErrorAction SilentlyContinue
    if (-not $files) {
        Write-Output "LIST empty"
        return
    }
    foreach ($f in $files) {
        $lease = Read-Lease $f.FullName
        $expired = Test-LeaseExpired $lease
        Write-Output ("LIST issue={0} owner={1} status={2} expired={3} branch={4}" -f `
                $lease.issue, $lease.owner, $lease.status, $expired, $lease.branch)
    }
}

switch ($Command) {
    'claim' { Invoke-Claim }
    'heartbeat' { Invoke-Heartbeat }
    'release' { Invoke-Release }
    'status' { Invoke-Status }
    'list' { Invoke-List }
}
