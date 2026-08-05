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

## A. Recover state

1. Read `AGENTS.md`.
2. Read `IMPLEMENTATION_PLAN.md`.
3. Read `DEV_STATE.md`.
4. Run `git status`.
5. Identify the current branch.
6. Inspect open PRs.
7. Inspect CI and review comments for any active Solpaper PR.
8. Read GitHub Issue #1 and the currently active issue.
9. Check for active background tasks or another agent already working on the same issue.
10. Never discard, overwrite or absorb unrelated changes.

If existing work is unfinished, continue it before choosing a new issue.

## B. Prevent duplicate work

If another agent, command or queued turn is actively modifying the same branch or issue:

* Do not start another implementation.
* Inspect status only.
* Update `DEV_STATE.md` if needed.
* Exit the iteration with `ACTIVE_WORK_ALREADY_RUNNING`.

Do not queue speculative duplicate work.

## C. Choose one unit of work

Priority:

1. Fix a failing active PR.
2. Address unresolved review comments.
3. Finish an active issue.
4. Select the earliest unblocked issue from Issue #1.
5. Repair implementation-plan drift.
6. Repair documentation drift.

Perform only one of:

* One issue
* One coherent subtask of a large issue
* One PR correction cycle
* One planning/documentation gate
* One CI recovery cycle

Do not attempt the entire roadmap in one firing.

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
* Window topology, renderer and Cargo boundaries remain provisional until #18.

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
   * Risks
   * Explicit non-goals
5. Create or use a focused branch:

   `issue-<number>-<short-name>`

Do not push directly to `main`.

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

## G. Issue #18 overlay spike

Issue #18 is disposable evidence-gathering work.

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

Record exact commands and results.

After the same failure occurs three times without a materially different attempted fix:

1. Stop retrying.
2. Record the failure signature.
3. Set `DEV_STATE.md` to `CHANGES_REQUIRED` or `EXTERNALLY_BLOCKED`.
4. Continue only if a different independent task is available.

## I. Independent verification

After implementation and local tests, spawn one independent general-purpose verifier subagent with a fresh context.

Prefer `subagent_type` `solpaper-verifier` when available; otherwise use `general-purpose` with the verifier contract from `.grok/agents/solpaper-verifier.md`.

Give the verifier:

* Linked issue
* Acceptance criteria
* Complete diff
* Test commands
* Relevant ADRs and product rules

The verifier must:

1. Attempt to disprove correctness.
2. Run relevant tests independently.
3. Check scope creep.
4. Check secret leakage.
5. Check failure recovery.
6. Check Windows assumptions.
7. Check documentation against implementation.
8. Return exactly one verdict:

   * `VERIFIED`
   * `CHANGES_REQUIRED`
   * `MANUAL_EVIDENCE_REQUIRED`

The verifier must not trust the builder’s summary.

When `CHANGES_REQUIRED`:

* Fix material findings.
* Re-run tests.
* Request one second verification.
* Do not repeat verifier cycles indefinitely.

## J. Commit and PR

When locally verified:

1. Review the full diff.
2. Remove debug artefacts and machine-specific data.
3. Commit with a focused conventional message.
4. Push the branch.
5. Create or update a PR.

The PR must include:

* Linked issue
* Summary
* Decisions
* Tests run
* Tests not run
* Manual evidence required
* Screenshots or recordings for visual work
* Security/privacy impact
* Known limitations

## K. CI and merge

When a PR already exists:

1. Inspect every check.
2. Fix root causes.
3. Do not disable meaningful tests to obtain green CI.
4. Inspect unresolved review threads.
5. Run independent verification after the final change.

Merge only when:

* CI is green.
* The verifier returned `VERIFIED`, or only explicitly nonblocking manual evidence remains.
* Acceptance criteria are met.
* No unresolved material review thread remains.
* No unrelated changes are present.
* Repository rules allow autonomous merge.

Use squash merge unless repository convention says otherwise.

If merge permission is unavailable:

* Leave the PR ready.
* Record the exact blocker.
* Select another independent issue on a later iteration.

## L. Finish the iteration

Update:

* `IMPLEMENTATION_PLAN.md`
* `DEV_STATE.md`
* Relevant GitHub issue
* Issue #1 when roadmap state changed
* Relevant documentation or ADRs

End with exactly one terminal result:

* `TASK_COMPLETE`
* `PR_OPENED`
* `PR_UPDATED`
* `PR_MERGED`
* `ACTIVE_WORK_ALREADY_RUNNING`
* `CHANGES_REQUIRED`
* `EXTERNALLY_BLOCKED`
* `MANUAL_EVIDENCE_REQUIRED`
* `PROJECT_COMPLETE`
* `ARCHITECTURE_REJECTED`

Do not begin another issue during the same firing after reaching a terminal result.

## M. Project stopping conditions

Stop autonomous implementation when:

1. Issue #24 and Issue #1 are complete.
2. Every remaining task is externally blocked.
3. Overlay feasibility fails and the map is updated to a smaller fallback product.
4. Continuing would require destructive or privacy-sensitive action not authorized here.

Do not fabricate completion to avoid a blocker.
