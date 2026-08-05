# Solpaper Implementation Plan

Regeneratable execution ledger. Do not duplicate GitHub issue bodies. Status on GitHub is authoritative; refresh this file each autonomous iteration.

**Canonical roadmap:** [Issue #1](https://github.com/rps321321/solpaper/issues/1)  
**Engineering map:** [Issue #30](https://github.com/rps321321/solpaper/issues/30)  
**Governance:** [docs/engineering/agent-governance.md](docs/engineering/agent-governance.md)

**Product order (post-bootstrap):** #16 → #13 → (#7, #5, #19) → #20 → (#6, #21) → (#22, #23) → #24  
**Bootstrap priority (until done):** #31 → #16 → #32 → then #1/#30 frontier

## Current frontier

- **#31** — Autonomous-agent governance and change-risk controls (in progress on `issue-31-agent-governance`).
- Next after #31: **#16** — post-spike ADRs + production workspace (owner provisional ADRs recorded in loop fire notes; human ADR acceptance still required for treating scaffold as final).
- Then **#32** — CI + protected main.
- Spike recommendation (from #18): **Approach A — independent widget HWNDs**; Approach B validated as fallback.
- Manual evidence debt remains in `docs/research/overlay-feasibility.md`.

## Active work

| Issue | Branch | Lease owner | Risk | Unit |
|------:|--------|-------------|------|------|
| #31 | `issue-31-agent-governance` | `agent:solpaper-dev-loop` | LOW | Governance docs, lease tooling, loop/verifier updates |

## Ready

| Issue | Title | Next action |
|------:|-------|-------------|
| #16 | Record post-spike architecture and scaffold production workspace | After #31; draft ADRs from #18 + owner provisional ADRs; human approval; scaffold |
| #32 | Establish CI, protected-main policy, required quality gates | After workspace exists (or minimal docs-only CI policy if staged) |

## Blocked

| Issue | Title | Blocked by / wait for |
|------:|-------|------------------------|
| #13 | Define measurable desktop-surface v1 acceptance criteria | #18 done; still needs human v1 boundary (+ #16) |
| #7 | Decide tray runtime, autostart, and single-instance behaviour | #16 |
| #5 | Research IDesktopWallpaper as wallpaper subsystem adapter | #16 |
| #19 | Design Pomodoro state machine and recovery semantics | #16 (+ human defaults) |
| #20 | Build Alpha 1: tray, layout, Pomodoro, local wallpapers | #16, #19, #5, #7, #32, #33… |
| #6 | Research secret storage and Google Calendar desktop OAuth | #20 (+ privacy default from #17) |
| #21 | Build Alpha 2: read-only Google Calendar agenda widget | #6, #20 |
| #22 | Research and select the first remote wallpaper provider | #20 |
| #23 | Build Beta wallpaper scheduling, cache, selected provider | #20, #22 |
| #24 | Harden, package, and validate Solpaper v1 | #13, #20, #21, #23, #7, #44, #45… |

Engineering children #33–#45: see Issue #30; not all blocked on #16 but product Alpha merges need their gates.

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

- **Date (UTC):** 2026-08-05T15:26:00Z
- **Branch:** `issue-31-agent-governance` (from `main` @ 781d691)
- **Working tree:** dirty with #31 governance work
- **Open PRs:** none yet (this iteration opening)
- **Production Cargo workspace:** absent
- **Spike:** `spikes/desktop-overlay/` present (disposable)
- **Open roadmap issues:** #1, #5–#7, #13, #16, #19–#24, #30–#45
- **Closed complete (product):** #17, #18
- **Superseded (closed):** #2–#4, #8–#12, #14–#15
