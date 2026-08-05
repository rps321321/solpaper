---
name: solpaper-dev-loop
description: >
  Perform exactly one bounded autonomous Solpaper iteration: recover durable
  state, enforce governance, select one frontier unit, route to a focused skill,
  persist the result, and stop.
user-invocable: true
disable-model-invocation: false
---

# Solpaper autonomous loop controller

This skill coordinates work. It does not duplicate the detailed procedure for every engineering discipline.

## 1. Recover durable state

Read:

- `AGENTS.md`;
- `docs/engineering/agent-governance.md`;
- `IMPLEMENTATION_PLAN.md`;
- `DEV_STATE.md`;
- Git status, current branch, worktrees, and recent history;
- open pull requests, CI, and unresolved review comments;
- GitHub Issues #1 and #30;
- the active issue/spec and relevant ADRs;
- `.agent/KILL` and the issue-lease store through `scripts/agent-lease.ps1`.

Reconcile stale state with Git and GitHub reality. Never discard unrelated changes.

The kill switch is engaged when **either** `.agent/KILL` exists **or** `DEV_STATE.md` status is `KILLED`. When engaged, return `GOVERNANCE_BLOCKED`.

When an active branch, PR, lease, or agent already owns the current work, continue it only when this agent owns or can validly reclaim the lease. Otherwise return `ACTIVE_WORK_ALREADY_RUNNING`. Do not create duplicate implementation.

## 2. Enforce governance

Before mutation:

1. Classify the unit as `LOW`, `MEDIUM`, or `HIGH` using governance. `CRITICAL` work is human-only and returns `GOVERNANCE_BLOCKED`.
2. Enforce the one-builder and one-implementation-PR limits.
3. Claim the issue lease:

```powershell
powershell -NoProfile -File scripts/agent-lease.ps1 claim `
  -Issue <N> -Owner 'agent:solpaper-dev-loop' `
  -Branch 'issue-<N>-<short-name>' -Unit '<one line>' -RiskClass <CLASS>
```

4. When claim is denied, persist any state correction and return `ACTIVE_WORK_ALREADY_RUNNING`.
5. Mirror the lease in `DEV_STATE.md` and heartbeat long-running work.

Never bypass retry limits, verifier-cycle limits, risk gates, lease ownership, or the kill switch.

## 3. Select exactly one unit

Priority:

1. failing or review-blocked active PR;
2. incomplete work under a valid lease;
3. earliest unblocked frontier item from Issues #1 and #30;
4. state, map, or documentation repair required for honest selection.

Choose one:

- one complete small issue;
- one coherent vertical subtask of a large issue;
- one PR correction cycle;
- one research, prototype, or decision artifact;
- one CI or state-recovery cycle.

Record issue, branch, lease, risk class, intended outcome, and evidence target in `DEV_STATE.md` before substantive work.

## 4. Route to one focused discipline

- plan requires decomposition → `solpaper-ticketing`
- primary-source question → `solpaper-research`
- architecture, state, interaction, or performance assumption needs executable evidence → `solpaper-prototype`
- approved feature or internal implementation → `solpaper-implement`
- failing, flaky, broken, or slow behavior → `solpaper-diagnose`
- terminology, responsibility, seam, or ADR choice → `solpaper-domain-design`
- completed diff or PR correction → `solpaper-review`, then `solpaper-verifier` for the final aggregate verdict
- merge or rebase conflict → `solpaper-resolve-conflicts`

Follow that skill's procedure. Do not reproduce its checklist here.

## 5. Common publication rules

For mutating work:

- use the issue branch named by the lease;
- keep the diff limited to the selected unit;
- run applicable checks and record exact results;
- inspect the full diff and remove debug, private, and machine-specific data;
- invoke `solpaper-review` (two independent axes) then `solpaper-verifier` (sole final aggregate) before declaring implementation complete;
- treat one full review→verifier sequence as one verifier cycle; max two cycles per unit;
- use `.github/PULL_REQUEST_TEMPLATE.md` and record issue, risk, lease, tests, tests not run, manual evidence, security/privacy impact, and known limitations.

When a PR exists, inspect CI once per firing. Do not poll in a long loop. Pending checks return `WAITING_FOR_CI`.

Merge only when:

- governance permits autonomous merge for the risk class;
- required CI is green;
- the final review verdict is `VERIFIED`;
- acceptance criteria for the current unit are met;
- no material review thread remains;
- no unrelated changes are present;
- the kill switch is not engaged.

`HIGH` work may reach a verified PR but requires explicit human merge approval. `CRITICAL` work is never executed autonomously. Never push to `main` directly or force-push. Use squash merge unless repository policy changes.

## 6. Persist and stop

Before exit, update every affected durable source:

- originating issue and PR;
- `IMPLEMENTATION_PLAN.md`;
- `DEV_STATE.md`;
- Issues #1 or #30 only when roadmap or gate state changed;
- `CONTEXT.md` or ADRs only when terminology or decisions changed;
- manual evidence register;
- issue lease status.

Release the lease only when the unit is complete or intentionally abandoned. Keep it active when the same unit legitimately awaits CI, review, or an authorized follow-up.

Never begin another issue in the same firing.

End with exactly one result:

- `TASK_COMPLETE`
- `PR_OPENED`
- `PR_UPDATED`
- `PR_MERGED`
- `ACTIVE_WORK_ALREADY_RUNNING`
- `WAITING_FOR_CI`
- `CHANGES_REQUIRED`
- `EXTERNALLY_BLOCKED`
- `MANUAL_EVIDENCE_REQUIRED`
- `GOVERNANCE_BLOCKED`
- `PROJECT_COMPLETE`
- `ARCHITECTURE_REJECTED`

## Stop conditions

Stop autonomous work when:

- Issues **#24** and **#1** are both closed complete (`PROJECT_COMPLETE` per `docs/engineering/agent-governance.md`);
- every remaining frontier item is externally or human blocked;
- an accepted decision reduces or abandons the desktop-surface goal;
- continuing requires unauthorized high or critical action;
- the kill switch is engaged;
- repeated failures, open PRs, or queued work exceed governance limits;
- repository state cannot be reconciled safely.

Never fabricate completion to keep the loop moving.