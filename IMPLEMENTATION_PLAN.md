# Solpaper Implementation Plan

Regeneratable execution ledger. Do not duplicate GitHub issue bodies. Status on GitHub is authoritative; refresh this file each autonomous iteration.

**Canonical roadmap:** [Issue #1](https://github.com/rps321321/solpaper/issues/1)  
**Engineering map:** [Issue #30](https://github.com/rps321321/solpaper/issues/30)  
**Governance:** [docs/engineering/agent-governance.md](docs/engineering/agent-governance.md)  
**ADRs:** [docs/adr/](docs/adr/)

**Product order:** #16 → #13 → (#7, #5, #19) → #20 → (#6, #21) → (#22, #23) → #24  
**Bootstrap remaining:** #32 → then #1/#30 frontier (#31 and #16 **done** when this lands)

## Current frontier

- **#16** — ADRs + production workspace (this unit, branch `issue-16-architecture-scaffold`).
- After merge: **#32** CI + protected main (workspace now exists for Rust gates).
- Manual evidence debt from #18 remains open (non-blocking for scaffold).

## Active work

| Issue | Branch | Lease owner | Risk | Unit |
|------:|--------|-------------|------|------|
| #16 | `issue-16-architecture-scaffold` | `agent:solpaper-dev-loop` | MEDIUM | ADRs + crates scaffold |

## Ready

| Issue | Title | Next action |
|------:|-------|-------------|
| #32 | Establish CI, protected-main policy, required quality gates | After #16 merge; Windows Rust CI + protection checklist |
| #7 / #5 / #19 | Foundation research/design | Unblocked by #16 architecture |

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

- **#31** — Agent governance. PR #47.
- **#18** — Overlay spike. PR #28.
- **#17** — Product destination. PR #26.

## Discovered defects

- None currently open.

## Last verified repository state

- **Date (UTC):** 2026-08-05T15:45:00Z
- **Branch:** `issue-16-architecture-scaffold`
- **Gates (this unit):** `cargo fmt/check/test/clippy` green; `cargo run -p solpaper-app -- --smoke` exit 0
- **Production workspace:** present (`crates/solpaper-{app,core,windows,storage}`)
- **Spike:** still under `spikes/desktop-overlay/` (excluded from workspace)
