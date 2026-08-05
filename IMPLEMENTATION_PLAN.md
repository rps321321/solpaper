# Solpaper Implementation Plan

Regeneratable execution ledger. Do not duplicate GitHub issue bodies. Status on GitHub is authoritative; refresh this file each autonomous iteration.

**Canonical roadmap:** [Issue #1](https://github.com/rps321321/solpaper/issues/1)  
**Engineering map:** [Issue #30](https://github.com/rps321321/solpaper/issues/30)  
**Governance:** [docs/engineering/agent-governance.md](docs/engineering/agent-governance.md)  
**ADRs:** [docs/adr/](docs/adr/)  
**CI policy:** [docs/engineering/ci-policy.md](docs/engineering/ci-policy.md)

**Product order:** #16 → #13 → (#7, #5, #19) → #20 → (#6, #21) → (#22, #23) → #24  
**Bootstrap remaining:** **none** (#31, #16, #32 done) → #1/#30 frontier

## Current frontier

- Foundation research unblocked: #7, #5, #19.
- Engineering systems next candidates: #33 test strategy, #35 budgets, #36 threat model, #40 diagnostics, #41 a11y (per #30 before Alpha 1).
- Manual evidence debt from #18 remains open.
- **#32 is complete** — CI workflows, ci-policy, protected `main` with required checks.

## Active work

- **#19** — branch `issue-19-pomodoro-state-machine`, lease `agent:solpaper-dev-loop`, risk **MEDIUM**: Pomodoro state machine + recovery in `solpaper-core`, design note, unit tests.

## Ready

| Issue | Title | Next action |
|------:|-------|-------------|
| #7 | Tray runtime, autostart, single-instance | Research/design against ADR-0002 |
| #5 | IDesktopWallpaper adapter research | After/with local wallpaper path planning |
| #19 | Pomodoro state machine design | Platform-neutral in solpaper-core |
| #33 | Test strategy / Windows matrix / evidence | Engineering map child |

## Blocked

| Issue | Title | Blocked by / wait for |
|------:|-------|------------------------|
| #13 | Define measurable desktop-surface v1 acceptance criteria | human v1 boundary |
| #20 | Build Alpha 1 | #19, #5, #7, … |
| #6 / #21 | Calendar path | #20 |
| #22 / #23 | Remote wallpaper path | #20 |
| #24 | v1 harden/package | release gates |

## Manual evidence required

From #18 (`docs/research/overlay-feasibility.md`): sleep/resume, lock/unlock, multi-monitor, mixed DPI, Explorer restart, Win+D/fullscreen, prolonged idle.

## Recently completed

- **#32** — CI, protected-main policy, required quality gates. PR #53; branch protection on `main` (required: Windows Rust quality, Governance tooling, CI policy present).
- **#46** — Composable Solpaper engineering skills. PR #50.
- **#16** — Post-spike ADRs + production workspace (`crates/solpaper-*`). PR #49.
- **#31** — Agent governance. PR #47.
- **#18** — Overlay spike. PR #28.
- **#17** — Product destination. PR #26.

## Discovered defects

- None currently open.

## Last verified repository state

- **Date (UTC):** 2026-08-05T16:35:00Z
- **Branch:** `main` (includes #53)
- **Open implementation PRs:** none
- **Production workspace:** present
- **CI:** workflows + protected main
- **Closed complete:** #17, #18, #31, #16, #46, #32
