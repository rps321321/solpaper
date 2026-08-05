# Solpaper Implementation Plan

Regeneratable execution ledger. Do not duplicate GitHub issue bodies. Status on GitHub is authoritative; refresh this file each autonomous iteration.

**Canonical roadmap:** [Issue #1](https://github.com/rps321321/solpaper/issues/1)  
**Engineering map:** [Issue #30](https://github.com/rps321321/solpaper/issues/30)  
**Governance:** [docs/engineering/agent-governance.md](docs/engineering/agent-governance.md)  
**ADRs:** [docs/adr/](docs/adr/)  
**CI policy:** [docs/engineering/ci-policy.md](docs/engineering/ci-policy.md)  
**Deterministic packs:** [docs/engineering/deterministic-execution-blueprint.md](docs/engineering/deterministic-execution-blueprint.md)

**Product order:** #16 → #13 → (#7, #5, #19) → #20 → (#6, #21) → (#22, #23) → #24  
**Blueprint order:** #33 → #41 → #34 → #35 → #36 → #38 → #40 → (#5, #7) → #13 → #20 → …

## Current frontier

- **#33** — Test strategy, Windows matrix, and evidence layout (first pack after #19 completion).
- Follow blueprint required execution order; pack defaults are sole decision store.
- Manual evidence debt from #18 remains open.
- **#19 is complete** — Pomodoro domain machine in `solpaper-core`.

## Active work

- None after #58 merge. Next unit: **#33**.

## Ready

| Issue | Title | Next action |
|------:|-------|-------------|
| #33 | Test strategy / Windows matrix / evidence | Execute pack #33 |
| #41 / #34 / #35 / #36 / #38 / #40 | Foundation engineering packs | After #33 per blueprint |
| #5 / #7 | Wallpaper adapter / tray runtime | After foundation packs |

## Blocked

| Issue | Title | Blocked by / wait for |
|------:|-------|------------------------|
| #13 | Acceptance matrix | human v1 boundary + earlier packs |
| #20 | Alpha 1 | foundation + #5/#7/#19 + #13 |
| #6 / #21 | Calendar path | #20 |
| #22 / #23 | Remote wallpaper | owner gate (#22 RECOMMENDATION) |
| #24 | v1 RC | release gates |

## Manual evidence required

From #18 (`docs/research/overlay-feasibility.md`): sleep/resume, lock/unlock, multi-monitor, mixed DPI, Explorer restart, Win+D/fullscreen, prolonged idle.

## Recently completed

- **#19** — Pomodoro state machine + recovery. PR #58 (pack-aligned, VERIFIED).
- **#55** — Deterministic execution blueprint. PR #57.
- **#32** — CI + protected main. PR #53.
- **#46** — Engineering skills. PR #50.
- **#16** — ADRs + workspace. PR #49.
- **#31** — Agent governance. PR #47.
- **#18** — Overlay spike. PR #28.
- **#17** — Product destination. PR #26.

## Discovered defects

- None currently open for #19.

## Last verified repository state

- **Date (UTC):** 2026-08-05T18:03:00Z
- **Branch:** `main` (includes #58)
- **Open implementation PRs:** none
- **Production workspace:** present
- **Closed complete (recent):** #17, #18, #31, #16, #46, #32, #55, #19
