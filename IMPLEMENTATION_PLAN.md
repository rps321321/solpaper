# Solpaper Implementation Plan

Regeneratable execution ledger. Do not duplicate GitHub issue bodies. Status on GitHub is authoritative; refresh this file each autonomous iteration.

**Canonical roadmap:** [Issue #1](https://github.com/rps321321/solpaper/issues/1)  
**Intended order:** #17 → #18 → #16 → #13 → (#7, #5, #19) → #20 → (#6, #21) → (#22, #23) → #24

## Current frontier

- **#17** — Redefine Solpaper as a Windows desktop-surface application (product decision / grilling).
- Pre-approved answers exist in the `solpaper-dev-loop` skill (section D). Next action: record decisions, update vocabulary/map, close #17. No overlay implementation in that iteration.

## Active work

- None. Working tree clean on `main`. No open pull requests.

## Ready

| Issue | Title | Next action |
|------:|-------|-------------|
| #17 | Redefine Solpaper as a Windows desktop-surface application | Apply pre-approved product locks; update map/CONTEXT vocabulary; close #17 |

## Blocked

| Issue | Title | Blocked by / wait for |
|------:|-------|------------------------|
| #18 | Prototype desktop overlay feasibility on Windows 11 | #17 |
| #16 | Record post-spike architecture and scaffold production workspace | #17, #18 (+ human ADR approval) |
| #13 | Define measurable desktop-surface v1 acceptance criteria | #17, #18 (+ human v1 boundary) |
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

- Autonomous-development setup: `AGENTS.md`, this plan, `DEV_STATE.md`, `.grok/skills/solpaper-dev-loop`, `.grok/agents/solpaper-verifier.md`.

## Discovered defects

- **Stale in-repo mirrors:** `docs/wayfinder/map.md`, `docs/wayfinder/tickets.md`, root `README.md`, and `CONTEXT.md` still describe the superseded wallpaper-cycler + TUI product. Issue #1 is canonical. Repair during #17 / #16, not as product feature work.

## Last verified repository state

- **Date (UTC):** 2026-08-05T14:24:10Z
- **Branch:** `main` (up to date with `origin/main`)
- **Working tree:** clean
- **Open PRs:** none
- **Production Cargo workspace:** absent
- **Open roadmap issues:** #1, #5–#7, #13, #16–#24
- **Superseded (closed):** #2–#4, #8–#12, #14–#15
