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

- **#5** — IDesktopWallpaper PR [#77](https://github.com/rps321321/solpaper/pull/77) **CI green**, risk **HIGH** → **human merge only** (no auto-merge).
- Then **#7** (tray runtime / autostart / single instance); **#19** already complete.
- Manual evidence: physical matrix + MD-A11Y-* + MD-UX-01 + MD-PERF-* + MD-WP-01..06.
- **Foundation complete through #40.**

## Active work

- **#5** lease `agent:solpaper-dev-loop` branch `issue-5-wallpaper-adapter` PR **#77** risk **HIGH** — blocked on human merge.

## Ready

| Issue | Title | Next action |
|------:|-------|-------------|
| #7 | Tray runtime / autostart / single instance | After #5 merge (blueprint #5 → #7) |
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

From #18 and later packs: MD-001..009, MD-A11Y-01..05, MD-UX-01, MD-PERF-01..03; wallpaper MD-WP-01..06 from #5.

## Recently completed

- **#40** — Diagnostics, logging policy, crash/safe-mode, issue templates. PR #75.
- **#38** — Supply-chain. PR #74.
- **#36** — Threat model. PR #70.
- **#35** — NFR budgets. PR #68.
- **#34** — UX flows. PR #66.
- **#41** — Accessibility. PR #64.
- **#33** — Test strategy. PR #61.
- **#19** — Pomodoro. PR #58.

## Discovered defects

- None currently open for #5.

## Last verified repository state

- **Date (UTC):** 2026-08-08
- **Branch:** `issue-5-wallpaper-adapter` (from `main` @ `580e0de`)
- **Open implementation PRs:** #5 in progress; docs draft #72 may coexist
- **Production workspace:** present
- **Closed complete (recent):** #33, #41, #34, #35, #36, #38, #40
