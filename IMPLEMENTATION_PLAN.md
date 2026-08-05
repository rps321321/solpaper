# Solpaper Implementation Plan

Regeneratable execution ledger. Do not duplicate GitHub issue bodies. Status on GitHub is authoritative; refresh this file each autonomous iteration.

**Canonical roadmap:** [Issue #1](https://github.com/rps321321/solpaper/issues/1)  
**Engineering map:** [Issue #30](https://github.com/rps321321/solpaper/issues/30)  
**Governance:** [docs/engineering/agent-governance.md](docs/engineering/agent-governance.md)

**Product order (post-bootstrap):** #16 → #13 → (#7, #5, #19) → #20 → (#6, #21) → (#22, #23) → #24  
**Bootstrap remaining:** #16 → #32 → then #1/#30 frontier (#31 **done**)

## Current frontier

- **#16** — Record post-spike architecture and scaffold production workspace.
- Spike recommendation (from #18): **Approach A — independent widget HWNDs**; Approach B validated as fallback.
- Owner provisional ADRs (bootstrap notes): widget-sized HWND default; one user-session process; `windows` crate; smallest native placeholder renderer (no WebView2/wgpu/egui in scaffold); global opacity OK; layout = monitor match + anchor + DIP; settings versioned human-readable; secrets in Credential Manager only; crates max start solpaper-app/core/windows/storage.
- Human ADR acceptance still required before treating scaffold as frozen production architecture.
- Then **#32** — CI + protected main.
- Manual evidence debt remains in `docs/research/overlay-feasibility.md`.

## Active work

- None. Working tree should be clean on `main` after post-#31 state refresh. Next unit: #16.

## Ready

| Issue | Title | Next action |
|------:|-------|-------------|
| #16 | Record post-spike architecture and scaffold production workspace | Draft ADRs from #18 + owner provisional ADRs; human approval; scaffold |
| #32 | Establish CI, protected-main policy, required quality gates | After workspace exists (or staged CI policy) |

## Blocked

| Issue | Title | Blocked by / wait for |
|------:|-------|------------------------|
| #13 | Define measurable desktop-surface v1 acceptance criteria | human v1 boundary (+ #16) |
| #7 | Decide tray runtime, autostart, and single-instance behaviour | #16 |
| #5 | Research IDesktopWallpaper as wallpaper subsystem adapter | #16 |
| #19 | Design Pomodoro state machine and recovery semantics | #16 (+ human defaults) |
| #20 | Build Alpha 1: tray, layout, Pomodoro, local wallpapers | #16, #19, #5, #7, #32… |
| #6 | Research secret storage and Google Calendar desktop OAuth | #20 |
| #21 | Build Alpha 2: read-only Google Calendar agenda widget | #6, #20 |
| #22 | Research and select the first remote wallpaper provider | #20 |
| #23 | Build Beta wallpaper scheduling, cache, selected provider | #20, #22 |
| #24 | Harden, package, and validate Solpaper v1 | #13, #20, #21, #23, #7… |

## Manual evidence required

From #18 (`docs/research/overlay-feasibility.md`):

- Sleep/resume, lock/unlock
- Monitor disconnect/reconnect, primary change, dual-monitor + cross-monitor drag
- Mixed 100%/125%/150% DPI
- Explorer restart recovery
- Win+D / fullscreen coverage
- Prolonged idle CPU/memory

## Recently completed

- **#31** — Agent governance, risk classes, atomic leases, loop/verifier updates. PR #47.
- **#18** — Overlay feasibility spike. PR #28.
- **#17** — Product destination locked. PR #26.
- Autonomous-development setup PR #25.

## Discovered defects

- None currently open.

## Last verified repository state

- **Date (UTC):** 2026-08-05T15:30:00Z
- **Branch:** `main` (includes #47)
- **Working tree:** post-merge state refresh in progress
- **Open PRs:** none expected for #31
- **Production Cargo workspace:** absent
- **Spike:** `spikes/desktop-overlay/` present (disposable)
- **Governance:** `docs/engineering/agent-governance.md` + `scripts/agent-lease.ps1`
- **Open roadmap issues:** #1, #5–#7, #13, #16, #19–#24, #30, #32–#45
- **Closed complete:** #17, #18, #31
- **Superseded (closed):** #2–#4, #8–#12, #14–#15
