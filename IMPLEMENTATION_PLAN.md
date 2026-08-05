# Solpaper Implementation Plan

Regeneratable execution ledger. Do not duplicate GitHub issue bodies. Status on GitHub is authoritative; refresh this file each autonomous iteration.

**Canonical roadmap:** [Issue #1](https://github.com/rps321321/solpaper/issues/1)  
**Intended order:** #17 → #18 → #16 → #13 → (#7, #5, #19) → #20 → (#6, #21) → (#22, #23) → #24

## Current frontier

- **#16** — Record post-spike architecture and scaffold production workspace.
- Spike recommendation (from #18): **Approach A — independent widget HWNDs**; Approach B validated as fallback. Human ADR approval required before production scaffold.
- Manual evidence debt remains in `docs/research/overlay-feasibility.md` (non-blocking for starting #16 ADR draft; physical checks still needed before hard freezes).

## Active work

- None. Working tree clean on `main` after #28. Next unit: #16 (human ADR gate).

## Ready

| Issue | Title | Next action |
|------:|-------|-------------|
| #16 | Record post-spike architecture and scaffold production workspace | Draft ADR from #18 recommendation; human approval; then scaffold |

## Blocked

| Issue | Title | Blocked by / wait for |
|------:|-------|------------------------|
| #13 | Define measurable desktop-surface v1 acceptance criteria | #18 done; still needs human v1 boundary |
| #7 | Decide tray runtime, autostart, and single-instance behaviour | #16 |
| #5 | Research IDesktopWallpaper as wallpaper subsystem adapter | #16 |
| #19 | Design Pomodoro state machine and recovery semantics | #16 (+ human defaults) |
| #20 | Build Alpha 1: tray, layout, Pomodoro, local wallpapers | #16, #19, #5, #7 |
| #6 | Research secret storage and Google Calendar desktop OAuth | #20 (+ privacy default from #17) |
| #21 | Build Alpha 2: read-only Google Calendar agenda widget | #6, #20 |
| #22 | Research and select the first remote wallpaper provider | #20 |
| #23 | Build Beta wallpaper scheduling, cache, selected provider | #20, #22 |
| #24 | Harden, package, and validate Solpaper v1 | #13, #20, #21, #23, #7 |

## Manual evidence required

From #18 (`docs/research/overlay-feasibility.md`):

- Sleep/resume, lock/unlock
- Monitor disconnect/reconnect, primary change, dual-monitor + cross-monitor drag
- Mixed 100%/125%/150% DPI
- Explorer restart recovery
- Win+D / fullscreen coverage
- Prolonged idle CPU/memory

## Recently completed

- **#18** — Overlay feasibility spike: `spikes/desktop-overlay/` (A + B), research note recommends Approach A; no WorkerW sole path; idle ~7–8 MB smoke. PR #28.
- **#17** — Product destination locked. PR #26.
- Autonomous-development setup: `AGENTS.md`, this plan, `DEV_STATE.md`, `.grok/skills/solpaper-dev-loop`, `.grok/agents/solpaper-verifier.md`.

## Discovered defects

- None currently open.

## Last verified repository state

- **Date (UTC):** 2026-08-05T15:10:00Z
- **Branch:** `main` (includes #28)
- **Working tree:** clean after state refresh
- **Open PRs:** none
- **Production Cargo workspace:** absent
- **Spike:** `spikes/desktop-overlay/` present (disposable)
- **Open roadmap issues:** #1, #5–#7, #13, #16, #19–#24
- **Closed complete (product):** #17, #18
- **Superseded (closed):** #2–#4, #8–#12, #14–#15
