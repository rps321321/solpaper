---
name: solpaper-dev-loop
description: >
  Perform exactly one bounded autonomous development iteration for Solpaper.
  Use when the user runs /solpaper-dev-loop, /loop with this skill, or asks for
  an autonomous Solpaper development turn. Owner is AFK for routine supervision;
  use GitHub, AGENTS.md, IMPLEMENTATION_PLAN.md, and DEV_STATE.md as persistent memory.
user-invocable: true
disable-model-invocation: false
---

# Solpaper autonomous development iteration

Perform exactly one bounded autonomous development iteration.

The repository owner is unavailable for routine supervision. Use GitHub, `AGENTS.md`, `IMPLEMENTATION_PLAN.md`, `DEV_STATE.md`, tests and git history as persistent project memory.

**Governance (authoritative):** [`docs/engineering/agent-governance.md`](../../../docs/engineering/agent-governance.md)  
Atomic leases live under `.agent/leases/` via `scripts/agent-lease.ps1`. `DEV_STATE.md` is a mirror only.

## A. Recover state

1. Read `AGENTS.md`.
2. Read `docs/engineering/agent-governance.md` (risk classes, limits, kill-switch).
3. Read `IMPLEMENTATION_PLAN.md`.
4. Read `DEV_STATE.md`.
5. If `.agent/KILL` exists or `DEV_STATE.md` status is `KILLED`, exit `GOVERNANCE_BLOCKED`.
6. Run `git status`.
7. Identify the current branch.
8. Inspect open PRs.
9. Inspect CI and review comments for any active Solpaper PR.
10. Read GitHub Issue #1, engineering map #30, and the candidate issue.
11. Run `powershell -NoProfile -File scripts/agent-lease.ps1 list` and `status` for the candidate issue.
12. Check for active background tasks or another agent already working on the same issue.
13. Never discard, overwrite or absorb unrelated changes.

If existing work is unfinished **and** this agent holds (or can reclaim) the lease, continue it before choosing a new issue.

## B. Prevent duplicate work / claim lease

Before any edit:

1. Choose risk class (`LOW` | `MEDIUM` | `HIGH`). Never claim `CRITICAL` autonomously → `GOVERNANCE_BLOCKED`.
2. Enforce concurrency: max **one** active builder and **one** active implementation PR unless finishing that PR.
3. Claim the issue lease:

```powershell
powershell -NoProfile -File scripts/agent-lease.ps1 claim `
  -Issue <N> -Owner 'agent:solpaper-dev-loop' `
  -Branch 'issue-<N>-<short-name>' -Unit '<one line>' -RiskClass <CLASS>
```

4. If claim fails with `CLAIM_DENIED` → update `DEV_STATE.md` if needed → exit `ACTIVE_WORK_ALREADY_RUNNING`.
5. Mirror the lease into `DEV_STATE.md` (issue, branch, owner, expiry, risk class).
6. Heartbeat on long units (≥ 30 minutes of work).

Do not queue speculative duplicate work.

## C. Choose one unit of work

Priority:

1. Fix a failing active PR (same lease/issue).
2. Address unresolved review comments.
3. Finish an active leased issue.
4. Bootstrap engineering gates when open: **#31 → #16 → #32**, then normal #1/#30 order.
5. Select the earliest unblocked issue from Issue #1 / #30.
6. Repair implementation-plan drift.
7. Repair documentation drift.

Perform only one of:

* One issue
* One coherent subtask of a large issue
* One PR correction cycle
* One planning/documentation gate
* One CI recovery cycle

Do not attempt the entire roadmap in one firing. Never finish multiple roadmap issues in one fire.

## D. Special treatment of Issue #17

The owner has pre-approved:

* Solpaper is a Windows desktop-surface application.
* Wallpaper is a peer subsystem.
* Pomodoro is required.
* Google Calendar is read-only and belongs in Alpha 2.
* Calendar remains intended for v1.
* TUI is not a v1 primary interface.
* Windows 11 x64, Rust, local-first operation and no cloud backend remain locked.
* Local folders are the first wallpaper source.
* Calendar default shows ordinary titles but replaces private details with `Private`.
* Busy-only mode must also exist.
* Window topology, renderer and Cargo boundaries remain provisional until #18 (spike complete; production ADRs on #16).

When #17 is the frontier:

1. Record these answers.
2. Update the map and repository vocabulary.
3. Update `IMPLEMENTATION_PLAN.md`.
4. Close #17 as completed.
5. Do not implement overlay code in the same iteration.
6. End with `TASK_COMPLETE`.

## E. Plan before editing

Before implementation:

1. Inspect relevant code and documentation.
2. Verify current APIs through primary documentation when needed.
3. Write a short implementation plan.
4. Identify:

   * Files affected
   * Tests required
   * Risks / risk class
   * Explicit non-goals
5. Create or use a focused branch:

   `issue-<number>-<short-name>`

Do not push directly to `main`. Do not force-push.

## F. Implementation rules

* Implement the smallest complete change.
* Do not refactor unrelated code.
* Do not create speculative abstractions.
* Do not create a crate per feature without evidence.
* Do not add IPC before a real second client exists.
* Do not add remote providers before local wallpapers are stable.
* Do not add TUI work.
* Do not hard-code one monitor resolution.
* Do not block the Windows UI thread with network work.
* Encapsulate every unsafe Win32 boundary.
* Do not hide errors.
* Do not create uncontrolled retries.
* Preserve user data on recovery.
* Do not expose Calendar titles in Busy-only mode.
* Do not automate sleep, lock, monitor disconnection, registry destruction or credential removal in ways that could interrupt the owner’s study session.
* Record those as manual evidence when physical interaction is required.
* Do not store secrets in source, config, SQLite, logs, issues, or PRs.

## G. Issue #18 overlay spike

Issue #18 is disposable evidence-gathering work (completed for product path; spike tree may remain).

Compare both:

1. Independent widget-sized top-level windows.
2. Monitor-sized transparent surfaces containing multiple widgets.

Do not assume a winner.

Evaluate separately:

* Global opacity
* Per-pixel transparency
* Whole-window click-through
* Selective interactive regions
* Edit Mode
* Dragging
* Resizing
* Focus stealing
* Taskbar visibility
* Alt+Tab visibility
* Desktop icon interaction
* Normal application coverage
* Win+D
* DPI scaling
* Cross-monitor movement
* Explorer restart
* Tray restoration
* Idle CPU and memory
* One-second updates
* Layout restoration

Use a clearly disposable spike location.

Do not create the production workspace during this issue.

Do not claim physical sleep/resume, monitor disconnect/reconnect or multi-monitor tests passed unless actually performed. Record missing checks under `Manual evidence required`.

The spike may recommend an architecture without perfect physical coverage when:

* Core input and window behaviour are proven.
* Remaining tests are clearly recorded.
* The selected architecture is reversible.
* No undocumented shell technique is the sole path.

## H. Test

Run the narrowest useful tests during implementation and every applicable full check before declaring completion.

For production Rust work:

```powershell
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

For spike work, run equivalent checks for the spike crate.

For governance/lease tooling:

```powershell
powershell -NoProfile -File scripts/tests/agent-lease.Tests.ps1
```

Record exact commands and results.

After the same failure occurs three times without a materially different attempted fix:

1. Stop retrying.
2. Record the failure signature in `DEV_STATE.md`.
3. Set terminal result `CHANGES_REQUIRED` or `EXTERNALLY_BLOCKED` / `GOVERNANCE_BLOCKED`.
4. Release the lease if abandoning the unit.
5. Continue only if a different independent task is available **in a later fire**.

## I. Independent verification

After implementation and local tests, spawn one independent general-purpose verifier subagent with a fresh context.

Prefer `subagent_type` `solpaper-verifier` when available; otherwise use `general-purpose` with the verifier contract from `.grok/agents/solpaper-verifier.md`.

Give the verifier:

* Linked issue
* Acceptance criteria
* Complete diff
* Test commands
* Declared risk class
* Relevant ADRs and product rules
* Governance doc path

The verifier must:

1. Attempt to disprove correctness.
2. Run relevant tests independently.
3. Check scope creep.
4. Check secret leakage.
5. Check failure recovery.
6. Check Windows assumptions.
7. Check documentation against implementation.
8. Check risk-class honesty and lease presence for the issue.
9. Return exactly one verdict:

   * `VERIFIED`
   * `CHANGES_REQUIRED`
   * `MANUAL_EVIDENCE_REQUIRED`

The verifier must not trust the builder’s summary.

When `CHANGES_REQUIRED`:

* Fix material findings.
* Re-run tests.
* Request one second verification.
* **Maximum two verifier cycles** per unit. After the second failure, stop with `CHANGES_REQUIRED`.

LOW trivial docs may use focused self-review only when the change is pure plan/state mirrors; prefer a verifier whenever scripts or policy tables change.

## J. Commit and PR

When locally verified:

1. Review the full diff.
2. Remove debug artefacts and machine-specific data.
3. Commit with a focused conventional message (no AI co-author trailers).
4. Push the branch.
5. Create or update a PR using `.github/PULL_REQUEST_TEMPLATE.md`.

The PR must include:

* Linked issue
* Summary
* **Change-risk class**
* Lease metadata
* Decisions
* Tests run
* Tests not run
* Manual evidence required
* Screenshots or recordings for visual work
* Security/privacy impact
* Known limitations

Heartbeat the lease with `-Pr <number>` after opening.

## K. CI and merge

When a PR already exists:

1. Inspect every check **once** per fire. If still pending → `WAITING_FOR_CI` (do not poll in a long loop).
2. Fix root causes.
3. Do not disable meaningful tests to obtain green CI.
4. Inspect unresolved review threads.
5. Run independent verification after the final change.

Merge rules by risk class:

| Class | Auto-merge when |
|-------|-----------------|
| LOW | Applicable checks green + focused review |
| MEDIUM | CI green + verifier `VERIFIED` |
| HIGH | **Never** without explicit human approval |
| CRITICAL | **Never** autonomous |

Also required for any merge:

* Acceptance criteria met
* No unresolved material review thread
* No unrelated changes
* Repository rules allow the merge
* Kill-switch not engaged

Use squash merge unless repository convention says otherwise.

If merge permission is unavailable:

* Leave the PR ready.
* Record the exact blocker as `EXTERNALLY_BLOCKED` when nothing else can proceed, else leave PR open and end `PR_OPENED` / `PR_UPDATED`.

Never push to `main` directly. Never force-push.

## L. Finish the iteration

Update:

* `IMPLEMENTATION_PLAN.md`
* `DEV_STATE.md`
* Relevant GitHub issue
* Issue #1 / #30 when roadmap state changed
* Relevant documentation or ADRs
* Release lease on true completion/abandon (`scripts/agent-lease.ps1 release`), or leave active if PR still owned by this unit

End with exactly one terminal result:

* `TASK_COMPLETE`
* `PR_OPENED`
* `PR_UPDATED`
* `PR_MERGED`
* `ACTIVE_WORK_ALREADY_RUNNING`
* `CHANGES_REQUIRED`
* `EXTERNALLY_BLOCKED`
* `MANUAL_EVIDENCE_REQUIRED`
* `WAITING_FOR_CI`
* `PROJECT_COMPLETE`
* `ARCHITECTURE_REJECTED`
* `GOVERNANCE_BLOCKED`

Do not begin another issue during the same firing after reaching a terminal result.

## M. Project stopping conditions

Stop autonomous implementation (and delete the scheduled loop task when applicable) when:

1. Issue #24 and Issue #1 are complete → `PROJECT_COMPLETE`.
2. Every remaining task is externally blocked with no independent unblocked task.
3. Overlay feasibility fails and the map records product reduction → `ARCHITECTURE_REJECTED`.
4. Continuing would require destructive or privacy-sensitive action not authorized here.
5. Kill-switch engaged.
6. Same failure signature ≥ 3 with no other independent task.
7. Queued iterations, open PRs, or repeated failures exceed limits in `docs/engineering/agent-governance.md`.

Do not fabricate completion to avoid a blocker.
