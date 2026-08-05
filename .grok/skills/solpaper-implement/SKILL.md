---
name: solpaper-implement
description: >
  Implement exactly one approved Solpaper issue or coherent vertical slice,
  using explicit seams, focused tests, independent review, and repository-backed state.
user-invocable: true
disable-model-invocation: false
---

# Implement one Solpaper unit

Implement one approved issue or one coherent subtask that can be verified independently.

## Entry conditions

Before editing, confirm:

- the originating issue/spec is identified and open;
- its blockers are complete;
- this agent owns a valid issue lease for the branch and unit;
- no other active lease, branch, PR, or agent owns the same work;
- the change-risk class is recorded;
- high and critical human gates are understood;
- the working tree and branch state are safe.

If the work is not ready, return the precise blocker instead of inventing scope.

## Understand the change

Read the issue, comments, `AGENTS.md`, governance policy, relevant `CONTEXT.md` terms, and relevant ADRs. Inspect existing code and tests at the intended seam.

Write a short plan containing:

- user-visible or domain behavior delivered;
- public seam under test;
- files or modules likely affected;
- risks and failure modes;
- explicit non-goals;
- evidence required.

Prefer making the change easy before making the easy change, but do not smuggle broad refactoring into the feature. Create a separate ticket when prefactoring is independently valuable.

## Work vertically

Build the narrowest complete path through the relevant layers. A completed slice must be independently demonstrable or testable.

Use `solpaper-tdd` for behavior that has a stable seam. One red-green cycle at a time:

1. Add one failing behavior test.
2. Run it and observe the intended failure.
3. Add only enough implementation to pass.
4. Run the focused test.
5. Repeat for the next behavior.

Do not write a horizontal batch of imagined tests before implementation.

## Solpaper constraints

- Keep platform-neutral state logic outside Win32 adapters.
- Encapsulate `unsafe` and COM ownership behind narrow interfaces.
- Do not block the UI thread with network or disk work.
- Do not create speculative IPC, provider, crate, plugin, or TUI architecture.
- Preserve data on recovery and use bounded retries.
- Never claim physical Windows behavior without evidence.
- Do not introduce secrets or private Calendar content into source, fixtures, logs, issues, PRs, screenshots, or diagnostics.

## Validate continuously

Run focused checks during development. Before review, run every applicable workspace check:

```powershell
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Use scoped equivalents for disposable spike crates. Record commands and exact outcomes.

## Review before publication

1. Invoke `solpaper-review` against a pinned base and head (two independent axes).
2. Invoke `solpaper-verifier` with both reports for the **sole final aggregate** verdict.
3. Address material findings, rerun relevant checks, and allow at most **two** full review→verifier cycles per unit (governance).

Before commit or PR:

- inspect the full diff;
- remove debug instrumentation and machine-specific values;
- update ADRs and docs only where behavior or decisions changed;
- list tests not run and manual evidence debt;
- ensure the PR risk class, lease metadata, and merge authority are explicit.

## Completion

Persist the issue, lease, branch, PR, tests, evidence, and next action in repository state. Release the lease only when governance says the unit is complete or abandoned.

End with one result:

- `TASK_COMPLETE`
- `PR_OPENED`
- `PR_UPDATED`
- `CHANGES_REQUIRED`
- `EXTERNALLY_BLOCKED`
- `MANUAL_EVIDENCE_REQUIRED`
- `GOVERNANCE_BLOCKED`

Do not start another issue after reaching a terminal result.