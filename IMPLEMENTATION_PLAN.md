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
**Research:** [docs/research/](docs/research/)

**Product order:** #16 → #13 → (#7, #5, #19) → #20 → (#6, #21) → (#22, #23) → #24  
**Blueprint order:** #33 → #41 → #34 → #35 → #36 → #38 → #40 → (#5, #7) → #13 → #20 → …

## Current frontier

- **#20** — Alpha 1 tracer **bullet 1** (Runtime tray host) on PR [#84](https://github.com/rps321321/solpaper/pull/84): **CI green**, blocked on **human HIGH merge**.
- Remaining #20 bullets (after #84 lands): widget host/Edit Mode → settings persistence polish → Pomodoro UI → wallpaper folders → diagnostics UI → physical evidence.
- **#13** — Matrix draft landed (PR #82); human freeze still open.
- Manual evidence: MD-* including MD-RT-01..05, MD-WP-*.

## Active work

- **#20** lease `agent:solpaper-dev-loop` branch `issue-20-runtime-tray-host` risk **HIGH** (Win32 tray/control HWND) — PR #84 awaiting human merge (all CI SUCCESS).

## Ready

| Issue | Title | Next action |
|------:|-------|-------------|
| #20 later bullets | Widget host, Pomodoro UI, wallpaper | after bullet 1 merges |
| #13 close | Acceptance freeze | owner v1 boundary |

## Blocked

| Issue | Title | Blocked by / wait for |
|------:|-------|------------------------|
| #13 close | Freeze matrix | owner v1 boundary approval |
| #6 / #21 | Calendar path | #20 complete |
| #22 / #23 | Remote wallpaper | owner gate |
| #24 | v1 RC | release gates |

## Manual evidence required

MD-001..009, MD-A11Y-*, MD-UX-01, MD-PERF-*, MD-WP-01..06, MD-RT-01..05.

## Recently completed

- **#13 draft** — Acceptance matrix. PR #82.
- **#7** — Tray design + adapters. PR #79.
- **#5** — IDesktopWallpaper. PR #77.
- **#40** — Diagnostics. PR #75.
- **#38** — Supply-chain. PR #74.
- **#36** — Threat model. PR #70.
- **#35** — NFR. PR #68.
- **#34** — UX flows. PR #66.
- **#41** — Accessibility. PR #64.
- **#33** — Test strategy. PR #61.
- **#19** — Pomodoro. PR #58.

## Discovered defects

- None open.

## Last verified repository state

- **Date (UTC):** 2026-08-08
- **Branch:** `issue-20-runtime-tray-host`
- **Checks (local):** clippy -D warnings PASS; workspace tests PASS; `solpaper --smoke` PASS
- **Open implementation PRs:** #20 bullet 1 in progress
