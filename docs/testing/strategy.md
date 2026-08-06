# Test strategy

**Issue:** [#33](https://github.com/rps321321/solpaper/issues/33)  
**Status:** initial strategy (Alpha 1 gate)  
**Pack source:** [`deterministic-execution-blueprint.md` § #33](../engineering/deterministic-execution-blueprint.md)  
**CI policy:** [`ci-policy.md`](../engineering/ci-policy.md)

## Goals

- Make every v1 acceptance concern executable or explicitly manual with named evidence.
- Keep platform-neutral domain logic under pure unit tests.
- Bound flaky tests: no rerun-until-green.
- Separate automated, manual, destructive, and hardware-dependent work so contributors can reproduce the matrix without hidden instructions.

## Test pyramid (layers)

| Layer | What | Where (convention) | Gate |
|------:|------|--------------------|------|
| 1 | **Pure core unit tests** — Pomodoro transitions, layout math, monitor matching, Calendar projection/privacy, cache policy pure logic | `crates/solpaper-core` (and pure modules in other crates) | CI required |
| 2 | **Storage integration** — temporary directories, transactional fixtures, migrations, corrupt-config recovery | `crates/solpaper-storage` | CI required |
| 3 | **Owned adapter contract tests with fakes** — wallpaper, credential store, monitor enumerator, notification sink | fakes in crate `tests/` or `#[cfg(test)]` modules | CI required |
| 4 | **HTTP integration with local mock server** — Google Calendar API shapes, optional remote provider errors/backoff | dedicated mock under `tests/mocks/` or crate integration tests | CI required when Calendar/remote code exists |
| 5 | **Win32 smoke/system tests** — safe, deterministic HWND/style/DPI helpers that do not disrupt the shell | `crates/solpaper-windows` optional tests; never sole production proof | CI when stable and non-disruptive |
| 6 | **Named physical Windows evidence** — sleep/resume, multi-monitor, Explorer restart, etc. | `docs/testing/evidence/<issue>/<date>/<env>/` | Manual / release |
| 7 | **Release tests** — clean install, upgrade, rollback, uninstall, autostart, credential purge, portable build | scripts + evidence under #24/#39 | Release / #24 |

Disposable spikes under `spikes/` use scoped local checks only; they are excluded from the production workspace CI ([ci-policy](../engineering/ci-policy.md)).

## Ownership by concern

| Concern | Primary layer | Owner crate / surface |
|---------|---------------|------------------------|
| Pomodoro state / recovery / single missed completion | 1 | `solpaper-core` |
| Widget layout math, hit-test, DIP geometry | 1 | `solpaper-core` |
| Monitor match / off-screen recovery math | 1 (+ 6 physical) | `solpaper-core` + evidence |
| Config / SQLite / migrations / atomic write | 2 | `solpaper-storage` |
| Credential store adapter | 3 (+ 7 purge) | adapter trait + fake; real CM on release |
| Calendar transport, sync token, privacy projection | 1 + 4 | core projection + mock HTTP |
| Desktop wallpaper apply path | 3 (+ 6/7) | fake contract; physical apply evidence |
| HWND styles, input pass-through, focus | 5 + 6 | windows crate smoke + physical matrix |
| Tray / single-instance / autostart | 3 + 6 + 7 | fakes where possible; physical for shell |
| Installer / upgrade / rollback | 7 | #39 / #24 |
| Performance / idle budgets | 6 (named hardware) | evidence + #35 budgets |
| Accessibility | manual + toolkit tests when UI exists | #41 |

## Required injectable seams

Production code that must support deterministic tests injects these (names are contractual for test design; exact Rust traits may evolve):

| Seam | Purpose in tests |
|------|------------------|
| `Clock` | Advance Pomodoro deadlines, stale Calendar indicators, backoff timers without wall sleep |
| `RandomSource` | OAuth state, IDs, jitter — fixed sequences in tests |
| `CredentialStore` | Success/failure/missing without touching real Credential Manager in unit/integration |
| `CalendarTransport` | Mock HTTP responses, 401/403/5xx, truncated bodies |
| `DesktopWallpaper` | Apply/fail/preserve-current without shell side effects in CI |
| `MonitorEnumerator` | Single/dual/mixed DPI topologies as pure data |
| `NotificationSink` | Assert at-most-once completion notifications |
| Owned filesystem/path services | Atomic write failure, corrupt config, permission denied |

If a defect cannot be regression-tested because a seam is missing, record the missing seam in the PR and open or extend a follow-up issue; do not claim “untestable” without that note.

## What runs where

| Environment | Runs |
|-------------|------|
| **GitHub Actions `windows-latest`** | Layers 1–4 (and 5 only when non-disruptive and green-stable). Full workspace: fmt, check, test, clippy, build. |
| **Developer machine (non-disruptive)** | Same as CI plus local fakes; optional Win32 smoke. |
| **Named physical matrix** | Layers 6–7 only on scheduled or human-driven sessions; never as silent merge proof. |

## Automated vs manual vs destructive

| Class | Definition | Merge / release rule |
|-------|------------|----------------------|
| **Automated** | Deterministic, no interactive desktop, no shell kill | Must pass CI for product PRs |
| **Manual (non-destructive)** | Operator interaction (Edit Mode drag, Win+D) without destroying user state | Evidence required for acceptance rows |
| **Hardware-dependent** | Multi-monitor, mixed DPI, sleep/resume, hotplug | Named environment + evidence; register until done |
| **Destructive** | Explorer kill/restart, uninstall, credential purge, migration rollback | Explicit consent; not during owner study; evidence + operator |

## Flaky-test policy

From the #33 pack (DEFAULT):

1. **No rerun-until-green.** A flaky failure is a failure.
2. **Quarantine** requires: issue number, owner, reason, observed failure rate, expiry date, and classification as **nonblocking** or **blocking**. Blocking quarantines cannot ship in the phase they gate.
3. Quarantine lives in issue text and, when applicable, `#[ignore]` with a comment linking the issue—not in silent CI “retry” config.
4. Every **fixed reproducible defect** needs a regression test unless the missing injectable seam is explicitly recorded on the issue/PR.

## Regression expectations (minimum)

Before Alpha 1 merges that touch the concern:

- Pomodoro: full transition table + restart/sleep recovery paths from #19 design.
- Layout: serialize/deserialize round-trip; off-monitor clamp math.
- Storage: open, migrate, corrupt-file recovery, atomic replace failure.
- Single-instance: contract test with fake lock where possible.

Before Alpha 2:

- Calendar privacy projection (ordinary / Private / Busy-only).
- Mock transport: auth failure, stale sync token, offline cache.
- No private titles in logs (allowlist field tests).

Before stable v1 (#24):

- Full acceptance mapping rows either automated-green or evidence-linked.
- Release suite executed on a named environment.
- Manual debt either cleared or human-waived with rationale on #13/#24.

## UI automation

| Reliable enough for automation (when UI exists) | Mandatory manual |
|-----------------------------------------------|------------------|
| Pure layout/hit-test math | Visual transparency and “desktop still usable” feel |
| Config form validation (non-shell) | Win+D, fullscreen game coverage |
| Accessibility tree queries for labels (toolkit-dependent, #41) | Sleep/resume, lock/unlock, multi-monitor drag |
| Notification sink call counts | Explorer restart recovery |
| Mock OAuth loopback with fake browser | Real system-browser OAuth once per major change |

Do not depend on brittle screenshot pixel diffs for CI gates. Screenshots belong in **evidence**, not as required CI assertions, unless a future ADR accepts a narrow golden-image suite.

## Privacy and security in tests

- Fixtures use synthetic Calendar titles only (`Focus block`, `Private`-class markers as strings—not real PII).
- No refresh tokens, client secrets, or Credential Manager material in repo or evidence.
- Evidence `manifest.json` must include a redaction confirmation field (see [evidence/](./evidence/)).
- Logs captured for evidence must be scanned for secrets before commit.

## Relationship to other issues

| Issue | Role |
|------:|------|
| #13 | Product acceptance rows; each maps here to a layer + evidence kind |
| #18 | Source of retained physical debt (seed for register) |
| #32 | CI enforces automated layers only |
| #35 | Numeric budgets measured on named hardware (layer 6) |
| #24 | Consumes this matrix for RC validation |
| #41 | Accessibility feasibility may add automated and manual rows |

## Non-goals (this issue)

- Implementing production Win32 system tests beyond strategy and seams.
- Running the full physical matrix in this PR.
- Freezing #13 row text (human v1 boundary still open).
- Adding UI automation frameworks or new runtime dependencies.
