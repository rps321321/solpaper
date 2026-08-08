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

- **#13** — Acceptance matrix (`docs/testing/acceptance-matrix.md`) per blueprint after #5/#7. Agent drafts rows; owner freezes v1 boundary and release-blocking waivers.
- After #13 draft: #20 Alpha 1 when acceptance rows + UX gates allow.
- Manual evidence: MD-* including MD-RT-01..05, MD-WP-*.
- **#7 / #5 / #40 / foundation packs complete.**

## Active work

- None (post-#7 state sync only).

## Ready

| Issue | Title | Next action |
|------:|-------|-------------|
| #13 | Acceptance matrix | Claim lease; follow pack #13; human freezes v1 boundary |
| #20 | Alpha 1 | after #13 rows + UX readiness |

## Blocked

| Issue | Title | Blocked by / wait for |
|------:|-------|------------------------|
| #20 | Alpha 1 | #13 acceptance rows + owner boundary + UX |
| #6 / #21 | Calendar path | #20 |
| #22 / #23 | Remote wallpaper | owner gate (#22 RECOMMENDATION) |
| #24 | v1 RC | release gates |

## Manual evidence required

MD-001..009, MD-A11Y-*, MD-UX-01, MD-PERF-*, MD-WP-01..06, MD-RT-01..05.

## Recently completed

- **#7** — Tray runtime design, second-launch activation, HKCU Run autostart. PR #79 (HIGH, human-merged).
- **#5** — IDesktopWallpaper research, trait, fake + COM adapter. PR #77.
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
- **Branch:** `main` @ `6c68eac` (feat #7)
- **Open implementation PRs:** none required for #7; docs #72 may coexist
- **Production workspace:** present
- **Closed complete (recent):** #33, #41, #34, #35, #36, #38, #40, #5, #7
