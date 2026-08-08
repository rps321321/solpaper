# Solpaper Implementation Plan

Regeneratable execution ledger. Do not duplicate GitHub issue bodies. Status on GitHub is authoritative; refresh this file each autonomous iteration.

**Canonical roadmap:** [Issue #1](https://github.com/rps321321/solpaper/issues/1)  
**Engineering map:** [Issue #30](https://github.com/rps321321/solpaper/issues/30)  
**Governance:** [docs/engineering/agent-governance.md](docs/engineering/agent-governance.md)  
**ADRs:** [docs/adr/](docs/adr/)  
**CI policy:** [docs/engineering/ci-policy.md](docs/engineering/ci-policy.md)  
**Deterministic packs:** [docs/engineering/deterministic-execution-blueprint.md](docs/engineering/deterministic-execution-blueprint.md)  
**Testing:** [docs/testing/](docs/testing/)  
**Accessibility:** [docs/accessibility/](docs/accessibility/)  
**Design:** [docs/design/](docs/design/)  
**NFR / budgets:** [docs/engineering/non-functional-requirements.md](docs/engineering/non-functional-requirements.md)  
**Security:** [docs/security/](docs/security/) · [SECURITY.md](SECURITY.md)  
**Operations:** [docs/operations/](docs/operations/)

**Product order:** #16 → #13 → (#7, #5, #19) → #20 → (#6, #21) → (#22, #23) → #24  
**Blueprint order:** #33 → #41 → #34 → #35 → #36 → #38 → #40 → (#5, #7) → #13 → #20 → …

## Current frontier

- **#40** — Logging / diagnostics / crash recovery: PR [#75](https://github.com/rps321321/solpaper/pull/75) on `issue-40-diagnostics` (MEDIUM, CI pending).
- After #40 merges: foundation gates through #40 complete; next **#5 / #7** (wallpaper adapter / tray runtime) per blueprint.
- Manual evidence: physical matrix + MD-A11Y-* + MD-UX-01 + MD-PERF-01..03.
- **#38 / #36 / #35 / #34 / #41 / #33 / #19 complete.**

## Active work

- **#40** lease `agent:solpaper-dev-loop` branch `issue-40-diagnostics` PR **#75** risk **MEDIUM** — awaiting CI; review VERIFIED.

## Ready

| Issue | Title | Next action |
|------:|-------|-------------|
| #5 / #7 | Wallpaper adapter / tray runtime | After #40 merge per blueprint |
| #13 | Acceptance matrix | human v1 boundary + earlier packs |

## Blocked

| Issue | Title | Blocked by / wait for |
|------:|-------|------------------------|
| #13 | Acceptance matrix | human v1 boundary + earlier packs |
| #20 | Alpha 1 | foundation + #5/#7/#19 + #13 + UX design |
| #6 / #21 | Calendar path | #20 |
| #22 / #23 | Remote wallpaper | owner gate (#22 RECOMMENDATION) |
| #24 | v1 RC | release gates |

## Manual evidence required

From #18 and later packs: MD-001..009, MD-A11Y-01..05, MD-UX-01, MD-PERF-01..03; sleep/resume, multi-monitor, Explorer restart, Win+D, mixed DPI, prolonged idle.

## Recently completed

- **#38** — Supply-chain policy, MIT reconcile, deny/audit CI, SBOM scripts. PR #74 (HIGH, human-merged).
- **#36** — Threat model and security architecture. PR #70 (HIGH, human-merged).
- **#35** — NFR / quality budgets. PR #68.
- **#34** — UX flows. PR #66.
- **#41** — Accessibility requirements. PR #64.
- **#33** — Test strategy / evidence. PR #61.
- **#19** — Pomodoro state machine. PR #58.
- **#55** — Deterministic blueprint. PR #57.
- **#32** — CI + protected main. PR #53.

## Discovered defects

- None currently open for #40.

## Last verified repository state

- **Date (UTC):** 2026-08-08
- **Branch:** `issue-40-diagnostics` (from `main` @ `4bd7bb1`)
- **Open implementation PRs:** #40 in progress; docs draft #72 may coexist
- **Production workspace:** present
- **Closed complete (recent):** #33, #41, #34, #35, #36, #38
