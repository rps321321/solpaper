# Solpaper Implementation Plan

Regeneratable execution ledger. Do not duplicate GitHub issue bodies. Status on GitHub is authoritative; refresh this file each autonomous iteration.

**Canonical roadmap:** [Issue #1](https://github.com/rps321321/solpaper/issues/1)  
**Engineering map:** [Issue #30](https://github.com/rps321321/solpaper/issues/30)  
**Governance:** [docs/engineering/agent-governance.md](docs/engineering/agent-governance.md)  
**ADRs:** [docs/adr/](docs/adr/)

**Product order:** #16 → #13 → (#7, #5, #19) → #20 → (#6, #21) → (#22, #23) → #24  
**Bootstrap remaining:** #32 → then #1/#30 frontier (#31 and #16 **done**)

## Current frontier

- **#32** — Establish CI, protected-main policy, and required quality gates (workspace exists under `crates/`).
- Foundation research unblocked: #7, #5, #19 (after or parallel with #32 per gates).
- Manual evidence debt from #18 remains open.
- **#16 is complete** — do not re-open product scaffold work as if pending.

## Active work

- **#32** — branch `issue-32-ci-quality-gates`, lease `agent:solpaper-dev-loop`, risk **MEDIUM**: Windows CI workflows, `docs/engineering/ci-policy.md`, branch-protection checklist + required-check matrix.

## Ready

| Issue | Title | Next action |
|------:|-------|-------------|
| #32 | Establish CI, protected-main policy, required quality gates | Windows Rust CI workflow + branch-protection checklist |
| #7 | Tray runtime, autostart, single-instance | Research/design against ADR-0002 |
| #5 | IDesktopWallpaper adapter research | After/with local wallpaper path planning |
| #19 | Pomodoro state machine design | Platform-neutral in solpaper-core |

## Blocked

| Issue | Title | Blocked by / wait for |
|------:|-------|------------------------|
| #13 | Define measurable desktop-surface v1 acceptance criteria | human v1 boundary |
| #20 | Build Alpha 1 | #19, #5, #7, #32… |
| #6 / #21 | Calendar path | #20 |
| #22 / #23 | Remote wallpaper path | #20 |
| #24 | v1 harden/package | release gates |

## Manual evidence required

From #18 (`docs/research/overlay-feasibility.md`): sleep/resume, lock/unlock, multi-monitor, mixed DPI, Explorer restart, Win+D/fullscreen, prolonged idle.

## Recently completed

- **#46** — Composable Solpaper engineering skills. PR #50 (validated: grok inspect, routing, standards+spec, verifier VERIFIED).
- **#16** — Post-spike ADRs + production workspace (`crates/solpaper-*`). PR #49.
- **#31** — Agent governance. PR #47.
- **#18** — Overlay spike. PR #28.
- **#17** — Product destination. PR #26.

## Discovered defects

- None currently open.

## Last verified repository state

- **Date (UTC):** 2026-08-05T15:57:00Z
- **Branch:** `main` (includes #50)
- **Open implementation PRs:** none
- **Production workspace:** present
- **Closed complete:** #17, #18, #31, #16, #46
