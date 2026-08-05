# CI policy and quality gates

**Issue:** [#32](https://github.com/rps321321/solpaper/issues/32)  
**Status:** complete (PR #53; branch protection applied 2026-08-05)  
**Related:** [agent-governance.md](./agent-governance.md) (change-risk classes)

This document is the repository policy for continuous integration, required checks, branch protection, artifacts, and flaky-test handling. Workflows live under `.github/workflows/`.

## Goals

- Prevent merge of formatting, clippy, build, or test failures for production code.
- Keep `main` protected against direct and force pushes where settings allow.
- Bound CI cost with concurrency cancellation and safe Rust caching.
- Never place secrets, tokens, or private Calendar data in logs or artifacts.
- Give the autonomous loop honest merge gates (green required checks + risk class).

## Workflows

| Workflow file | Name | When | Purpose |
|---------------|------|------|---------|
| `ci.yml` | **CI** | every PR; push to `main`; manual | Required quality gates |
| `release-build.yml` | **Release build check** | push to `main`; manual | Unsigned release binary build (not publication) |

Disposable spikes under `spikes/` are **excluded** from the production Cargo workspace and are not required to pass production CI. Spike authors run scoped checks locally when changing spike crates.

### CI jobs (stable check names)

These names are what branch protection and the autonomous loop must require:

| Check name | Runner | Mandatory for |
|------------|--------|---------------|
| `Windows Rust quality` | `windows-latest` | All PRs into `main`; push to `main` |
| `Governance tooling` | `windows-latest` | All PRs into `main`; push to `main` |
| `CI policy present` | `ubuntu-latest` | All PRs into `main`; push to `main` |
| `Dependency review` | `ubuntu-latest` | PRs only (soft gate; see below) |

`Windows Rust quality` runs, in order:

```text
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo build --workspace --all-targets
```

`RUSTFLAGS=-D warnings` is set for the job so warnings fail the build.

`Governance tooling` runs:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/tests/agent-lease.Tests.ps1
```

### Caching

- `Swatinem/rust-cache@v2` caches registry and `target/` keyed with lockfile awareness.
- Do not configure cache restore keys that hide `Cargo.lock` changes.
- Concurrency group `ci-${{ workflow }}-${{ pr number || ref }}` cancels superseded runs.

### Docs-only and cost control

The production workspace is small; **all PRs run the full CI job set** so required-check names stay stable and docs-only PRs cannot accidentally weaken merge gates.

If CI cost becomes material later:

1. Prefer path filters that **still produce a green check with the same required name** (explicit skip job that reports success only when production paths are untouched).
2. Never allow a missing check to satisfy branch protection.
3. Any path-filter change is at least **MEDIUM** and must update this document and the protection checklist together.

Workflow and `ci-policy.md` edits always run full CI (they are production merge-gate code).

## Required-check matrix by change-risk class

Risk classes are defined in [agent-governance.md](./agent-governance.md). Merge authority is repeated here only as it intersects CI.

| Risk class | Required CI checks | Additional gates | Agent merge |
|------------|--------------------|------------------|-------------|
| **LOW** | `Windows Rust quality`, `Governance tooling`, `CI policy present` | Focused review of docs/test/plan delta | May auto-merge when checks green |
| **MEDIUM** | Same as LOW | Independent `solpaper-review` + `solpaper-verifier` → `VERIFIED` | May auto-merge when CI green and `VERIFIED` |
| **HIGH** | Same as LOW; `Dependency review` must not be ignored if it reports critical findings | Verified PR; **human merge approval** | **No auto-merge** |
| **CRITICAL** | N/A autonomous | Human-only; do not execute | **No autonomous PR/merge** |

Notes:

- `Dependency review` is currently a **soft** PR job (`continue-on-error: true`) until an owner-approved license deny-list and severity policy land (related: engineering map #38). Critical findings must still be read and addressed on HIGH work.
- Spike-only changes that do not touch the production workspace should still open PRs against `main` with CI green; spike compile is not a production required check.
- Security-sensitive paths (Credential Manager, OAuth, autostart, installer, signing) remain **HIGH** regardless of CI green.

## Branch protection (protected `main`)

### Target settings

Apply on branch `main` (Settings → Branches, or API):

| Setting | Value |
|---------|--------|
| Require a pull request before merging | **Yes** |
| Required approving reviews | ≥ 0 for solo-owner bootstrap; raise when collaborators join |
| Dismiss stale pull request approvals when new commits are pushed | **Yes** when reviews > 0 |
| Require review from Code Owners | Optional until CODEOWNERS exists |
| Require status checks to pass before merging | **Yes** |
| Require branches to be up to date before merging | **Yes** preferred |
| Required status checks | `Windows Rust quality`, `Governance tooling`, `CI policy present` |
| Require conversation resolution before merging | **Yes** preferred |
| Require signed commits | Optional (owner policy) |
| Require linear history | Optional; squash-merge preferred by agents |
| Do not allow bypassing the above settings | **Yes** for non-admins; admins should avoid bypass except recovery |
| Restrict who can push to matching branches | No direct pushes; PR only |
| Allow force pushes | **No** |
| Allow deletions | **No** |

### Setup checklist (owner or admin)

Use this after the CI workflow has run at least once on `main` or on a PR so check names exist in the UI.

```text
[x] Confirm checks appear on a recent PR: Windows Rust quality, Governance tooling, CI policy present
[x] Settings → Branches → Add rule (or edit) for `main` (API PUT protection)
[x] Enable "Require a pull request before merging"
[x] Enable "Require status checks to pass before merging"
[x] Add required checks: Windows Rust quality, Governance tooling, CI policy present
[x] Enable "Require branches to be up to date before merging" (strict: true)
[x] Disable force pushes
[x] Disable branch deletions
[x] Restrict direct pushes / require PR
[x] Confirm admin bypass is used only for emergency recovery (enforce_admins: false)
[x] Record completion date and actor: 2026-08-05, agent:solpaper-dev-loop after PR #53
```

### API sketch (admin; optional)

```powershell
# Requires admin token. Adjust contexts to match job names after CI has run once.
$protectionPath = Join-Path $env:TEMP 'solpaper-main-protection.json'
@'
{
  "required_status_checks": {
    "strict": true,
    "contexts": [
      "Windows Rust quality",
      "Governance tooling",
      "CI policy present"
    ]
  },
  "enforce_admins": false,
  "required_pull_request_reviews": {
    "dismiss_stale_reviews": true,
    "required_approving_review_count": 0
  },
  "restrictions": null,
  "allow_force_pushes": false,
  "allow_deletions": false,
  "required_conversation_resolution": true
}
'@ | Set-Content -Path $protectionPath -Encoding utf8

gh api -X PUT repos/rps321321/solpaper/branches/main/protection `
  -H "Accept: application/vnd.github+json" `
  --input $protectionPath
```

> GitHub Free for private repos may limit branch protection features; public repos support the standard rule set. If the API returns an error, complete the checklist via the web UI.

**Human input:** Only repository settings that the agent cannot change (org policy, billing plan limits, or owner-locked rules) require the human owner. Admin agents may apply the checklist when permissions allow; still record the action on #32.

## Secrets, privacy, and artifacts

| Rule | Detail |
|------|--------|
| No secrets in source | Tokens, client secrets, refresh tokens never committed |
| No Calendar content in CI | Titles, attendees, free/busy never in fixtures, logs, or artifacts |
| Logs | Prefer structured local diagnostics later (#40); CI logs stay free of credentials |
| Artifacts | `release-build.yml` uploads **unsigned** `solpaper.exe` only; retention ≤ 14 days |
| Signing keys | Never present in CI secrets for autonomous workflows; public signing is CRITICAL / human-only |
| Env | Do not add repository secrets unless a HIGH ticket + human approval defines them |

GitHub secret scanning and push protection should remain enabled at the repository or org level when available.

## Dependency and license scanning

| Mechanism | Status |
|-----------|--------|
| `actions/dependency-review-action` on PRs | Soft gate; fails open on infra errors; `fail-on-severity: critical` |
| `cargo deny` / SBOM (#38) | Not required for #32; track under supply-chain workstream |
| License field on workspace packages | `MIT OR Apache-2.0` in root `Cargo.toml` |

Adding network, crypto, auth, or installer dependencies remains at least **MEDIUM** / often **HIGH** per governance, independent of soft dependency review.

## Flaky tests and external outages

- **Do not** normalize rerun-until-green as a merge strategy.
- A flaky test is a defect: quarantine with an issue, fix, or delete; do not leave silent ignores.
- If GitHub Actions or crates.io is down, return `WAITING_FOR_CI` or `EXTERNALLY_BLOCKED`; do not claim green.
- Re-running a failed job is allowed **once** after confirming infrastructure failure (runner abort, network blip). Product assertion failures require a code fix.
- Autonomously merging while required checks are missing or red is forbidden.

## Release-artifact build

`release-build.yml` proves the workspace builds in `--release` on Windows and uploads an **unsigned** binary artifact. It is **not**:

- a public release,
- a signed installer,
- permission to distribute,
- or a substitute for #24 / #39 / #44 human release gates.

Stable publication and signing-key use remain **CRITICAL** and human-only.

## Autonomous loop obligations

Before declaring a production PR mergeable, the loop must:

1. Observe required checks once (no long poll loops).
2. Treat pending checks as `WAITING_FOR_CI`.
3. Treat red or missing required checks as not mergeable.
4. Honor risk class: HIGH → verified PR only; CRITICAL → do not execute.
5. Keep at most one active implementation PR.

## Acceptance mapping (#32)

| Criterion | Mechanism |
|-----------|-----------|
| Format/clippy/build/test failures cannot merge | `Windows Rust quality` + branch protection required checks |
| Direct / force pushes to `main` prevented | Branch protection checklist (PR required, force push off) |
| Docs-only avoid waste without bypassing validation | Full CI kept for stable names; cost-control path documented above |
| CI artifacts have no secrets/Calendar data | Artifact path limited to unsigned exe; policy table |
| Loop cannot merge while checks missing/red | This policy + governance merge rules + `WAITING_FOR_CI` |

## Non-goals (this document)

- Full multi-OS matrix (Solpaper is Windows 11 local-first; expand only if crates become multi-target).
- Owner-approved license deny-list automation (#38).
- Production observability pipelines (#40).
- Public release engineering (#39, #24).
