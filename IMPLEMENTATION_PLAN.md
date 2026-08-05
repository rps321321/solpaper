# Solpaper Implementation Plan

Regeneratable execution ledger. Do not duplicate GitHub issue bodies. Status on GitHub is authoritative; refresh this file each autonomous iteration.

**Canonical roadmap:** [Issue #1](https://github.com/rps321321/solpaper/issues/1)  
**Intended order:** #17 → #18 → #16 → #13 → (#7, #5, #19) → #20 → (#6, #21) → (#22, #23) → #24

## Current frontier

- **#18** — Prototype desktop overlay feasibility on Windows 11 (disposable spike under `spikes/desktop-overlay/`; research write-up `docs/research/overlay-feasibility.md`).
- Compare independent widget HWNDs vs monitor-sized surfaces. No production workspace. Do not claim physical sleep/monitor tests unless performed.

## Active work

- PR for #17 vocabulary/map mirror landing (this iteration). After merge: #18 is next implementation unit.

## Ready

| Issue | Title | Next action |
|------:|-------|-------------|
| #18 | Prototype desktop overlay feasibility on Windows 11 | Disposable spike; compare Approach A (per-widget HWND) vs B (monitor surface) |

## Blocked

| Issue | Title | Blocked by / wait for |
|------:|-------|------------------------|
| #16 | Record post-spike architecture and scaffold production workspace | #18 (+ human ADR approval) |
| #13 | Define measurable desktop-surface v1 acceptance criteria | #18 (+ human v1 boundary) |
| #7 | Decide tray runtime, autostart, and single-instance behaviour | #18, #16 |
| #5 | Research IDesktopWallpaper as wallpaper subsystem adapter | #16 |
| #19 | Design Pomodoro state machine and recovery semantics | #16 (+ human defaults) |
| #20 | Build Alpha 1: tray, layout, Pomodoro, local wallpapers | #16, #19, #5, #7 |
| #6 | Research secret storage and Google Calendar desktop OAuth | #20 (+ privacy default from #17) |
| #21 | Build Alpha 2: read-only Google Calendar agenda widget | #6, #20 |
| #22 | Research and select the first remote wallpaper provider | #20 |
| #23 | Build Beta wallpaper scheduling, cache, selected provider | #20, #22 |
| #24 | Harden, package, and validate Solpaper v1 | #13, #20, #21, #23, #7 |

## Manual evidence required

- None recorded yet.
- Expected later (do not automate during owner study sessions): physical sleep/resume, monitor disconnect/reconnect, multi-monitor hardware checks, lock/unlock, registry destruction, credential removal where destructive.

## Recently completed

- **#17** — Product destination locked: desktop-surface app; wallpaper peer subsystem; Pomodoro required; Calendar read-only Alpha 2 intended for v1; TUI not primary v1 UI; local wallpapers first; calendar privacy default + Busy-only. In-repo: `CONTEXT.md`, `docs/wayfinder/map.md`, `docs/wayfinder/tickets.md`, `README.md` aligned with Issue #1.
- Autonomous-development setup: `AGENTS.md`, this plan, `DEV_STATE.md`, `.grok/skills/solpaper-dev-loop`, `.grok/agents/solpaper-verifier.md`.

## Discovered defects

- None currently open. Prior stale mirrors (map/tickets/README/CONTEXT wallpaper-cycler+TUI copy) addressed in the #17 docs PR.

## Last verified repository state

- **Date (UTC):** 2026-08-05 (iteration completing #17 deliverables)
- **Branch:** `issue-17-product-destination` (docs only)
- **Working tree:** dirty until commit
- **Open PRs:** this iteration’s PR (when opened)
- **Production Cargo workspace:** absent
- **Open roadmap issues:** #1, #5–#7, #13, #16, #18–#24
- **Closed complete (product):** #17
- **Superseded (closed):** #2–#4, #8–#12, #14–#15
