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

**Product order:** #16 → #13 → (#7, #5, #19) → #20 → (#6, #21) → (#22, #23) → #24  
**Blueprint order:** #33 → #41 → #34 → #35 → #36 → #38 → #40 → (#5, #7) → #13 → #20 → …

## Current frontier

- **#36** — Threat model and security architecture — **PR #70 open** (HIGH, human merge).
- After #70 merges: **#38** then **#40** per blueprint.
- Manual evidence: physical matrix + MD-A11Y-* + MD-UX-01 + MD-PERF-01..03.
- **#35 / #34 / #41 / #33 complete.**

## Active work

- **#36** — PR [#70](https://github.com/rps321321/solpaper/pull/70) on `issue-36-threat-model-security` (HIGH, no auto-merge). Lease active.

## Ready

| Issue | Title | Next action |
|------:|-------|-------------|
| #36 | Threat model / security architecture | Human merge PR #70 (CI green, VERIFIED, HIGH) |
| #38 / #40 | Foundation engineering packs | After #36 merge per blueprint |
| #5 / #7 | Wallpaper adapter / tray runtime | After foundation packs |

## Blocked

| Issue | Title | Blocked by / wait for |
|------:|-------|------------------------|
| #13 | Acceptance matrix | human v1 boundary + earlier packs |
| #20 | Alpha 1 | foundation + #5/#7/#19 + #13 + UX design |
| #6 / #21 | Calendar path | #20 |
| #22 / #23 | Remote wallpaper | owner gate (#22 RECOMMENDATION) |
| #24 | v1 RC | release gates + AT/UX MANUAL |

## Manual evidence required

Register: `docs/testing/manual-debt-register.md`.

## Recently completed

- **#35** — NFR and quality budgets. PR #68. (Physical PERF: MD-PERF-01..03, MD-009 open.)
- **#34** — UX flows, keyboard map, wireframes, usability script. PR #66. (Human sessions MD-UX-01 open.)
- **#41** — Accessibility. PR #64.
- **#33** — Test strategy. PR #61.
- **#19** — Pomodoro. PR #58.
- **#55** — Blueprint. PR #57.
- **#32** — CI. PR #53.
- **#16** — ADRs + workspace. PR #49.
- **#31** — Governance. PR #47.
- **#18** — Overlay spike. PR #28.
- **#17** — Product destination. PR #26.

## Discovered defects

- None currently open.

## Last verified repository state

- **Date (UTC):** 2026-08-06T10:44:00Z
- **Branch:** `main` (includes #68)
- **Open implementation PRs:** none
- **Closed complete (recent):** #17, #18, #31, #16, #46, #32, #55, #19, #33, #41, #34, #35
