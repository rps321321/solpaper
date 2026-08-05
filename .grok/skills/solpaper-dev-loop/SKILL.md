---
name: solpaper-dev-loop
description: >
  Perform exactly one bounded autonomous Solpaper iteration: recover durable
  state, select one frontier unit, route to a focused engineering skill, persist the result, and stop.
user-invocable: true
disable-model-invocation: false
---

# Solpaper autonomous loop controller

This skill coordinates work. It does not contain the detailed procedure for every engineering discipline.

## 1. Recover durable state

Read:

- `AGENTS.md`
- `IMPLEMENTATION_PLAN.md`
- `DEV_STATE.md`
- Git status, current branch, and worktrees
- open pull requests, CI, and unresolved review comments
- GitHub Issue #1
- GitHub Issue #30
- the active issue/spec and relevant ADRs

Reconcile stale state with Git/GitHub reality. Never discard unrelated changes.

If an active branch, PR, lease, or agent already owns the current work, continue that work or return `ACTIVE_WORK_ALREADY_RUNNING`. Do not create duplicate implementation.

## 2. Enforce governance

Read the active policy from Issue #31 or its repository mirror.

At minimum:

- one active builder and one active implementation PR;
- no direct push to `main`;
- no force-push/history rewrite;
- bounded verifier and failure retries;
- high-risk work may be prepared but not autonomously merged;
- critical actions are human-only;
- manual Windows evidence cannot be converted into a pass by prose.

Acquire or refresh the issue lease before editing. A lease records issue, owner/session, branch, timestamp, expiry, and heartbeat. Do not steal a valid lease. Reclaim an expired lease without deleting existing work.

## 3. Select one unit

Priority:

1. Failing or review-blocked active PR.
2. Incomplete leased work.
3. Earliest unblocked frontier item from Issue #1/#30.
4. Stale map, state, or documentation repair that blocks honest selection.

Choose exactly one:

- one complete small issue;
- one coherent vertical subtask of a large issue;
- one PR correction cycle;
- one research/prototype/decision artifact;
- one CI or state-recovery cycle.

Record the issue, branch, risk class, outcome, and evidence target in `DEV_STATE.md` before substantive work.

## 4. Route to the focused skill

Classify the selected unit:

- plan requires decomposition → `solpaper-ticketing`
- primary-source question → `solpaper-research`
- architecture/state/interaction assumption needs executable evidence → `solpaper-prototype`
- approved feature or internal implementation → `solpaper-implement`
- failing, flaky, broken, or slow behavior → `solpaper-diagnose`
- terminology, responsibilities, seams, or ADR choice → `solpaper-domain-design`
- completed diff or PR correction → `solpaper-review`
- merge/rebase conflict → `solpaper-resolve-conflicts`

Follow that skill's procedure. Do not duplicate it here.

## 5. Common publication rules

For any mutating work:

- use a focused issue branch;
- keep the change to the selected unit;
- run applicable checks and record exact results;
- inspect the full diff;
- remove debug, private, and machine-specific data;
- use `solpaper-review` before declaring implementation complete;
- push and open/update a PR with issue, risk, tests, tests not run, manual evidence, security/privacy impact, and known limitations.

Merge only when the risk class permits autonomous merge, required CI is green, material review findings are resolved, and evidence requirements for the current phase are satisfied. Use squash merge unless repository policy changes.

## 6. Persist and stop

Before exiting, update all affected durable state:

- originating issue and PR;
- `IMPLEMENTATION_PLAN.md`;
- `DEV_STATE.md`;
- Issue #1/#30 only when the roadmap or gate actually changed;
- `CONTEXT.md` or ADRs only when terminology/decisions changed;
- manual evidence register.

Release the lease only after state is durable. Never begin another issue in the same firing.

End with exactly one result:

- `TASK_COMPLETE`
- `PR_OPENED`
- `PR_UPDATED`
- `PR_MERGED`
- `ACTIVE_WORK_ALREADY_RUNNING`
- `CHANGES_REQUIRED`
- `EXTERNALLY_BLOCKED`
- `MANUAL_EVIDENCE_REQUIRED`
- `PROJECT_COMPLETE`
- `ARCHITECTURE_REJECTED`

## Project stop conditions

Stop autonomous implementation when:

- Issue #1's closure rule is satisfied;
- every remaining frontier item is externally or human blocked;
- an accepted decision reduces/abandons the desktop-surface goal;
- continuing requires a high/critical action without approval;
- governance detects runaway queues, repeated failures, or unsafe repository state.

Never fabricate completion to keep the loop moving.