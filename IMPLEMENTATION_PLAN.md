# Solpaper Implementation Plan

Regeneratable execution ledger. Do not duplicate GitHub issue bodies. Status on GitHub is authoritative; refresh this file each autonomous iteration.

**Canonical roadmap:** [Issue #1](https://github.com/rps321321/solpaper/issues/1)  
**Engineering map:** [Issue #30](https://github.com/rps321321/solpaper/issues/30)  
**Governance:** [docs/engineering/agent-governance.md](docs/engineering/agent-governance.md)  
**ADRs:** [docs/adr/](docs/adr/)  
**CI policy:** [docs/engineering/ci-policy.md](docs/engineering/ci-policy.md)  
**Deterministic packs:** [docs/engineering/deterministic-execution-blueprint.md](docs/engineering/deterministic-execution-blueprint.md)

**Product order:** #16 → #13 → (#7, #5, #19) → #20 → (#6, #21) → (#22, #23) → #24  
**Blueprint order:** #33 → #41 → #34 → #35 → #36 → #38 → #40 → (#5, #7, #19) → #13 → #20 → …

## Current frontier

- **Active:** #19 Pomodoro state machine — PR #58 being reconciled with current `main` (blueprint #55 landed after PR open).
- **Next after #19:** **#33** (test strategy / Windows evidence). Do not start #33 while #19 is open.
- Manual evidence debt from #18 remains open.
- Deterministic blueprint is the sole pack decision store.

## Active work

- **#19** — branch `issue-19-pomodoro-state-machine`, lease `agent:solpaper-dev-loop`, risk **MEDIUM**, PR **#58**: Pomodoro state machine + recovery in `solpaper-core`, design note, unit tests; rebase/merge with post-#55 main.

## Ready

| Issue | Title | Next action |
|------:|-------|-------------|
| #33 | Test strategy / Windows matrix / evidence | After #19 merges |
| #41 / #34 / #35 / #36 / #38 / #40 | Foundation engineering packs | Per blueprint order after #33 |
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

From #18 (`docs/research/overlay-feasibility.md`): sleep/resume, lock/unlock, multi-monitor, mixed DPI, Explorer restart, Win+D/fullscreen, prolonged idle. Physical Pomodoro sleep/resume remains under #33/#24.

## Recently completed

- **#55** — Deterministic execution blueprint. PR #57.
- **#32** — CI, protected-main policy. PR #53.
- **#46** — Composable engineering skills. PR #50.
- **#16** — ADRs + production workspace. PR #49.
- **#31** — Agent governance. PR #47.
- **#18** — Overlay spike. PR #28.
- **#17** — Product destination. PR #26.

## Discovered defects

- None yet for the #19 reconcile pass.

## Last verified repository state

- **Date (UTC):** 2026-08-05T17:51:00Z
- **Branch:** `issue-19-pomodoro-state-machine` (merging `main` including #55/#59)
- **Open implementation PRs:** #58 only
- **Production workspace:** present
- **Closed complete (recent):** #17, #18, #31, #16, #46, #32, #55
