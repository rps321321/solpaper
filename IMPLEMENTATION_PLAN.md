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

- **#7** — Tray runtime, autostart, and single-instance behaviour (design/research + contract) per blueprint after #5.
- Manual evidence: physical matrix + MD-A11Y-* + MD-UX-01 + MD-PERF-* + MD-WP-01..06.
- **#5 / #40 / #38 / #36 / #35 / #34 / #41 / #33 / #19 complete.** Foundation + wallpaper adapter research landed.

## Active work

- None (post-#5 state sync only).

## Ready

| Issue | Title | Next action |
|------:|-------|-------------|
| #7 | Tray runtime / autostart / single instance | Claim lease; follow pack #7 LOCKED/DEFAULT |
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

- **#5** — IDesktopWallpaper research, trait, fake + COM adapter. PR #77 (HIGH, human-merged).
- **#40** — Diagnostics / logging policy / crash/safe-mode. PR #75.
- **#38** — Supply-chain. PR #74.
- **#36** — Threat model. PR #70.
- **#35** — NFR budgets. PR #68.
- **#34** — UX flows. PR #66.
- **#41** — Accessibility. PR #64.
- **#33** — Test strategy. PR #61.
- **#19** — Pomodoro. PR #58.

## Discovered defects

- None open.

## Last verified repository state

- **Date (UTC):** 2026-08-08
- **Branch:** `main` @ `e8e10e8` (feat #5)
- **Open implementation PRs:** none required for #5; docs draft #72 may coexist
- **Production workspace:** present
- **Closed complete (recent):** #33, #41, #34, #35, #36, #38, #40, #5
