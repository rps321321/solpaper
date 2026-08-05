# Autonomous agent governance and change-risk controls

**Issue:** [#31](https://github.com/rps321321/solpaper/issues/31)  
**Status:** active  
**Owner-approved provisional policy:** 2026-08-05 (bootstrap priority from loop fire instructions)

This document is enforceable process for unattended Solpaper development.  
`DEV_STATE.md` is a human-readable mirror; it is **not** the atomic lease.

## Goals

- Prevent two builders from legitimately claiming the same issue.
- Bound retries, concurrent PRs, and wall-clock waste.
- Route high-risk work to humans; keep low-risk docs/tests autonomous.
- Recover from abandoned leases without deleting useful work.
- Stop the loop cleanly when governance or external limits block progress.

## Roles

| Role | Authority |
|------|-----------|
| **Builder** (solpaper-dev-loop) | One unit of work per fire; claim lease; implement; open/update PR; request verification |
| **Verifier** (solpaper-verifier) | Independent disprove pass; no merge authority; no scope expansion |
| **Human owner** | HIGH/CRITICAL approval; risk-class policy changes; kill-switch; public release |

Default concurrency: **at most one active builder** and **at most one active implementation PR** (product/code). Docs-only chore PRs may coexist only when they do not touch the same issue lease.

## Change-risk classes

Every PR and autonomous unit **must** declare a risk class in the PR body (see `.github/PULL_REQUEST_TEMPLATE.md`) and in `DEV_STATE.md`.

| Class | Examples | Agent authority | Merge |
|-------|----------|-----------------|-------|
| **LOW** | Docs, format, test-only, plan/state sync, wayfinder mirrors, governance wording | Full autonomous implement + verify | Auto-merge after focused review + applicable checks |
| **MEDIUM** | Ordinary internal impl, reversible architecture, non-destructive storage, justified deps | Implement + independent verifier + green required CI | May auto-merge when CI green and verifier `VERIFIED` |
| **HIGH** | OAuth/tokens, Credential Manager, Calendar privacy storage, autostart, installer, updater, substantive unsafe Win32, destructive-capable migrations, security policy | Implement + open verified PR | **No auto-merge** without explicit human approval |
| **CRITICAL** | Public release, signing keys, destructive migration approval, credential-policy weakening, accepted critical vuln, force-push/history rewrite, fundamental product reduction | **Do not execute or merge autonomously** | Human-only |

### Human-only gates (never autonomous)

1. Public release / stable publication  
2. Signing-key generation, import, or use for distribution  
3. Approval of destructive data migrations  
4. Weakening of credential or secret-storage policy  
5. Accepting a known critical vulnerability  
6. Force-push or history rewrite of shared branches  
7. Fundamental product reduction (e.g. dropping Pomodoro/Calendar from v1 without map update + owner)  
8. Architecture rejection that permanently shrinks the product below Issue #1 destination (record `ARCHITECTURE_REJECTED`, stop)  

### Risk-class decision rules

- If unsure between two classes, choose the **higher** class.  
- Adding a new dependency is at least **MEDIUM**; network/crypto/auth deps are **HIGH**.  
- Any path that can write secrets, autostart entries, or installer artifacts is **HIGH**.  
- Spike code under `spikes/` that cannot affect production defaults may be **MEDIUM** when disposable and documented.  
- Changing this governance document’s risk table or human-only list is **HIGH** until the owner re-approves.

## Atomic issue lease

### Location

```text
.agent/leases/issue-<number>.json
```

Directory `.agent/leases/` is the lease store. One file per issue number.

### Schema

```json
{
  "issue": 31,
  "owner": "agent:solpaper-dev-loop",
  "branch": "issue-31-agent-governance",
  "claimed_at": "2026-08-05T16:00:00Z",
  "expires_at": "2026-08-05T18:00:00Z",
  "heartbeat_at": "2026-08-05T16:30:00Z",
  "status": "active",
  "unit": "Implement governance docs and lease tooling",
  "pr": null,
  "risk_class": "LOW"
}
```

| Field | Meaning |
|-------|---------|
| `issue` | GitHub issue number (primary key) |
| `owner` | Agent id or human handle (`agent:…` / `human:…`) |
| `branch` | Working branch name |
| `claimed_at` | UTC claim time (ISO-8601) |
| `expires_at` | UTC expiry; default claim TTL **2 hours** |
| `heartbeat_at` | Last heartbeat UTC; refresh extends `expires_at` by claim TTL |
| `status` | `active` \| `released` \| `expired_reclaimed` |
| `unit` | One-line description of the single unit of work |
| `pr` | PR number if opened, else `null` |
| `risk_class` | `LOW` \| `MEDIUM` \| `HIGH` \| `CRITICAL` |

### Atomic claim rules

1. **Claim** only via `scripts/agent-lease.ps1 claim` (or equivalent CreateNew semantics).  
2. Claim **fails** if a non-expired `active` lease exists for that issue with a different owner.  
3. Same owner may re-claim / heartbeat their own lease.  
4. **Expired** leases are reclaimable: new claim overwrites metadata, sets `status` to `active`, and **must not** delete the previous branch, commits, or open PR.  
5. **Release** sets `status` to `released` and may leave the file for audit; a new claim may replace a released lease.  
6. Heartbeat at least every **30 minutes** during long work; default extension **+2 hours** from heartbeat time.  
7. `DEV_STATE.md` **mirrors** the active lease but is never authoritative for conflict resolution.

### Concurrent work limits

| Limit | Value |
|------:|-------|
| Active builders | 1 |
| Active implementation PRs | 1 |
| Verifier cycles per unit | 2 max |
| Materially identical failure retries | 3 then stop |
| Dependency additions per unit without separate justification | 1 (prefer zero) |
| Queued loop iterations while a PR awaits CI/human | prefer 0 new product units; maintenance allowed |

If limits would be exceeded: exit with `ACTIVE_WORK_ALREADY_RUNNING` or `EXTERNALLY_BLOCKED` (document which limit).

## Terminal results (loop)

Exactly one per fire (also listed in the skill):

| Result | When |
|--------|------|
| `TASK_COMPLETE` | Unit done without open PR (e.g. planning-only, issue closed with local state) |
| `PR_OPENED` / `PR_UPDATED` / `PR_MERGED` | PR lifecycle |
| `ACTIVE_WORK_ALREADY_RUNNING` | Lease held by another, or concurrent builder/PR limit |
| `CHANGES_REQUIRED` | Verifier or review found material defects; stop after recording |
| `EXTERNALLY_BLOCKED` | Needs human/settings/API/permission outside agent authority |
| `MANUAL_EVIDENCE_REQUIRED` | Physical/manual checks block honest completion |
| `WAITING_FOR_CI` | PR pushed; one CI poll done; checks still pending |
| `PROJECT_COMPLETE` | Issues #24 and #1 closed complete |
| `ARCHITECTURE_REJECTED` | Overlay/product reduction recorded; stop autonomous product work |
| `GOVERNANCE_BLOCKED` | Risk class CRITICAL, auto-merge forbidden, or runaway stop fired |

## Runaway stop conditions

Stop autonomous implementation (do not start a new unit) when any holds:

1. Same failure signature ≥ **3** times with no materially different fix.  
2. Continuous scope expansion across fires without roadmap issue support.  
3. Unexpected security-sensitive files touched without HIGH lease + plan (Credential Manager paths, token material, signing keys, autostart registry).  
4. Queued iterations / open autonomous PRs / repeated failures exceed limits above.  
5. Owner kill-switch engaged (see below).  
6. `PROJECT_COMPLETE` or `ARCHITECTURE_REJECTED`.  
7. All remaining work is `EXTERNALLY_BLOCKED` with no independent unblocked task.

Record the stop reason in `DEV_STATE.md` and Issue #1 / #30 as appropriate. Scheduler should `scheduler_delete` the loop task when project-level stop conditions hold.

## Kill-switch

### Engage (human)

1. Set `DEV_STATE.md` `Status:` to `KILLED` and `Next action:` to a clear stop message.  
2. Optionally create `.agent/KILL` containing reason + UTC timestamp.  
3. Cancel scheduled loop tasks if accessible.  
4. Close or mark draft any agent PR that must not merge.

### Agent behaviour when kill-switch is set

- Do **not** claim new leases.  
- Do **not** push or merge.  
- May only release own lease and update `DEV_STATE.md` to acknowledge.  
- Exit `GOVERNANCE_BLOCKED` or `EXTERNALLY_BLOCKED`.

### Clear (human only)

Remove `.agent/KILL`, set `DEV_STATE.md` back to `IDLE` or an intentional active status.

## Recovery: abandoned leases, branches, PRs

1. **Detect:** lease `expires_at` < now, or no heartbeat and branch idle, or PR stale with no owner activity.  
2. **Reclaim lease:** `scripts/agent-lease.ps1 claim -Issue N -Owner … -Branch … -ForceExpired` (only succeeds if expired/released).  
3. **Preserve work:** never delete remote branches or force-push; inspect existing PR/diff first.  
4. **Continue or supersede:** either finish the abandoned unit on the same branch, or open a new branch that builds on (or intentionally replaces) the old commits with a clear PR note.  
5. **Release:** when done or handing off, `release` the lease and update `DEV_STATE.md`.

Partial agent work is valid history until a human or a reclaiming agent decides otherwise.

## Worktree isolation

When parallel **read-only** agents run (search/audit), they must not write the same branch.  
If a second builder is ever authorized:

- Use isolated git worktrees or separate clones.  
- Still only **one** lease per issue.  
- Never share a dirty worktree between builders.

Default policy remains **single builder**.

## PR and verification requirements by class

| Class | Verifier | CI | Auto-merge allowed |
|-------|----------|----|--------------------|
| LOW | Focused review (verifier optional but preferred for non-trivial docs) | Applicable checks | Yes if green |
| MEDIUM | Required `VERIFIED` | Required green | Yes if green + verified |
| HIGH | Required `VERIFIED` | Required green | **No** — human approve |
| CRITICAL | N/A autonomous | N/A | **No** — human-only |

Max **two** verifier cycles per unit. After second `CHANGES_REQUIRED`, stop and set `CHANGES_REQUIRED` terminal result.

## Tooling

| Path | Role |
|------|------|
| `scripts/agent-lease.ps1` | claim / heartbeat / release / status |
| `scripts/tests/agent-lease.Tests.ps1` | exclusivity + expiry tests |
| `.agent/leases/` | lease files |
| `.agent/KILL` | optional kill-switch marker |
| `.grok/skills/solpaper-dev-loop/SKILL.md` | loop must claim lease before edit |
| `.grok/agents/solpaper-verifier.md` | verifier checks risk class + lease honesty |

## Non-goals

- Full multi-tenant agent fleet orchestration  
- Cloud-backed distributed locks  
- Replacing GitHub permissions or branch protection (#32)  
- Automating human HIGH/CRITICAL approval  

## Acceptance mapping (#31)

| Criterion | Mechanism |
|-----------|-----------|
| Two iterations cannot both claim same issue | Atomic CreateNew lease file + claim failure |
| Stale leases expire without deleting work | `expires_at` + reclaim overwrites lease only |
| High/critical cannot auto-merge | Risk table + PR template + loop/merge rules |
| Repeated identical failures stop | 3-strike rule + `GOVERNANCE_BLOCKED` / `CHANGES_REQUIRED` |
| Low-risk docs/tests remain autonomous | LOW class auto-merge path |
| Precise terminal state when blocked | Terminal result table including `GOVERNANCE_BLOCKED` / `WAITING_FOR_CI` |
