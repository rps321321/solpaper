# Solpaper Implementation Plan

Regeneratable execution ledger. Do not duplicate GitHub issue bodies. Status on GitHub is authoritative; refresh this file each autonomous iteration.

**Canonical roadmap:** [Issue #1](https://github.com/rps321321/solpaper/issues/1)  
**Engineering map:** [Issue #30](https://github.com/rps321321/solpaper/issues/30)  
**Governance:** [docs/engineering/agent-governance.md](docs/engineering/agent-governance.md)  
**ADRs:** [docs/adr/](docs/adr/)  
**CI policy:** [docs/engineering/ci-policy.md](docs/engineering/ci-policy.md)  
**Deterministic packs:** [docs/engineering/deterministic-execution-blueprint.md](docs/engineering/deterministic-execution-blueprint.md)  
**Testing:** [docs/testing/](docs/testing/)

**Product order:** #16 → #13 → (#7, #5, #19) → #20 → (#6, #21) → (#22, #23) → #24  
**Blueprint order:** #33 → #41 → #34 → #35 → #36 → #38 → #40 → (#5, #7) → #13 → #20 → …

## Current frontier

- **#41** — Accessibility requirements / feasibility before UI toolkit freeze (first pack after #33).
- Follow blueprint required execution order; pack defaults are sole decision store.
- Manual evidence debt tracked in `docs/testing/manual-debt-register.md`.
- **#33 is complete** — test strategy, Windows matrix, evidence harness under `docs/testing/`.

## Active work

- None after #61 merge. Next unit: **#41**.

## Ready

| Issue | Title | Next action |
|------:|-------|-------------|
| #41 | Accessibility requirements | Execute pack #41 |
| #34 / #35 / #36 / #38 / #40 | Foundation engineering packs | After #41 per blueprint |
| #5 / #7 | Wallpaper adapter / tray runtime | After foundation packs |

## Blocked

| Issue | Title | Blocked by / wait for |
|------:|-------|------------------------|
| #13 | Acceptance matrix | human v1 boundary + earlier packs; consumes #33 mapping |
| #20 | Alpha 1 | foundation + #5/#7/#19 + #13 |
| #6 / #21 | Calendar path | #20 |
| #22 / #23 | Remote wallpaper | owner gate (#22 RECOMMENDATION) |
| #24 | v1 RC | release gates |

## Manual evidence required

Register: `docs/testing/manual-debt-register.md` (MD-001–MD-009). Source: #18 `docs/research/overlay-feasibility.md`.

## Recently completed

- **#33** — Test strategy, Windows matrix, evidence harness. PR #61.
- **#19** — Pomodoro state machine + recovery. PR #58 (pack-aligned, VERIFIED).
- **#55** — Deterministic execution blueprint. PR #57.
- **#32** — CI + protected main. PR #53.
- **#46** — Engineering skills. PR #50.
- **#16** — ADRs + workspace. PR #49.
- **#31** — Agent governance. PR #47.
- **#18** — Overlay spike. PR #28.
- **#17** — Product destination. PR #26.

## Discovered defects

- None currently open.

## Last verified repository state

- **Date (UTC):** 2026-08-06T07:25:00Z
- **Branch:** `main` (includes #61)
- **Open implementation PRs:** none
- **Production workspace:** present
- **Closed complete (recent):** #17, #18, #31, #16, #46, #32, #55, #19, #33
