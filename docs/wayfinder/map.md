# solpaper desktop-surface wayfinder map

**Canonical tracker issue:** https://github.com/rps321321/solpaper/issues/1  
**Label:** `wayfinder:map`

This file is the **in-repo mirror**. GitHub Issue #1 is authoritative when they disagree. Update this file when the map issue changes (especially Decisions so far).

---

## Destination

Working Windows 11 x64 application in Rust:

> Solpaper is a lightweight, local-first desktop-surface application. A user-session runtime owns desktop widget surfaces, productivity state, tray/settings interaction, and wallpaper management. Pomodoro and a read-only Google Calendar agenda are first-class widget use cases. Wallpaper fetching/cycling is one subsystem, not the product root.

The exact overlay-window model, renderer, normal-mode interactivity, process boundaries, Cargo workspace, and first remote wallpaper provider remain provisional until the overlay feasibility spike passes.

## Current status

**READY FOR TECHNICAL SPIKES** after product destination lock (#17 closed 2026-08-05).

There is no production implementation yet. Do not scaffold the final workspace or execute remote-provider/TUI work as the active frontier until #18 and #16 complete.

## Immediate frontier

1. ~~[#17 Redefine Solpaper as a Windows desktop-surface application](https://github.com/rps321321/solpaper/issues/17)~~ — **done** (product locks recorded).
2. [#18 Prototype desktop overlay feasibility on Windows 11](https://github.com/rps321321/solpaper/issues/18) — disposable spike under `spikes/desktop-overlay/`; write-up `docs/research/overlay-feasibility.md` (**current frontier**; recommends Approach A pending merge + ADR).
3. [#16 Record post-spike architecture and scaffold the production workspace](https://github.com/rps321321/solpaper/issues/16) — blocked by #18 (+ human ADR approval).
4. [#13 Define measurable desktop-surface v1 acceptance criteria](https://github.com/rps321321/solpaper/issues/13) — blocked by #18 (+ human v1 boundary).

## Blocked execution path

### Foundation

- [#7 Decide tray runtime, autostart, and single-instance behaviour](https://github.com/rps321321/solpaper/issues/7) — after the overlay/process architecture is known.
- [#5 Research IDesktopWallpaper as the wallpaper subsystem adapter](https://github.com/rps321321/solpaper/issues/5) — local files only, after the production boundary exists.
- [#19 Design the Pomodoro state machine and recovery semantics](https://github.com/rps321321/solpaper/issues/19) — platform-neutral domain design after #16.

### Alpha 1

- [#20 Build Alpha 1: tray, persistent layout, Pomodoro, and local wallpapers](https://github.com/rps321321/solpaper/issues/20).

### Alpha 2

- [#6 Research secret storage and Google Calendar desktop OAuth](https://github.com/rps321321/solpaper/issues/6).
- [#21 Build Alpha 2: read-only Google Calendar agenda widget](https://github.com/rps321321/solpaper/issues/21).

### Beta

- [#22 Research and select the first remote wallpaper provider](https://github.com/rps321321/solpaper/issues/22).
- [#23 Build Beta wallpaper scheduling, cache fallback, and selected remote provider](https://github.com/rps321321/solpaper/issues/23).

### v1 release

- [#24 Harden, package, and validate Solpaper v1 on Windows 11](https://github.com/rps321321/solpaper/issues/24).

## Locked principles

- Windows 11 x64 only for the initial effort.
- Rust.
- Local-first; no Solpaper cloud backend for v1.
- User-session application, not a Windows SCM service.
- Live Pomodoro/calendar content is rendered as UI, never baked into wallpaper files.
- Documented Win32 approaches are preferred.
- WorkerW/Progman or other undocumented Explorer techniques must never be the sole supported architecture.
- The TUI is not the primary v1 UI; tray, direct Edit Mode, and a visual settings surface are the default direction.
- Google Calendar is read-only and uses least-privilege scopes.
- Secrets and refresh tokens are not stored in plaintext configuration.
- Local-folder wallpapers precede remote-provider complexity.
- At most one remote wallpaper provider enters v1 without a new justification.
- Pomodoro is required for Alpha 1 and intended for v1.
- Calendar is Alpha 2 and remains intended for v1.
- Calendar privacy default: show ordinary titles; replace private details with `Private`; Busy-only mode must also exist.
- Wallpaper management is a peer subsystem, not the product root.

## Decisions so far

### 2026-08-05 — Issue #17 product destination

- **Destination confirmed:** desktop-surface application (Runtime + Surfaces + Widgets + wallpaper subsystem), not a wallpaper-cycler-with-TUI product.
- **Primary v1 UI:** tray + Edit Mode + visual settings surface. TUI deferred post-v1.
- **Pomodoro:** required (Alpha 1).
- **Google Calendar:** read-only; Alpha 2; intended for v1.
- **Calendar privacy:** default titles with private → `Private`; Busy-only mode required.
- **Wallpaper:** peer subsystem; local folders first; ≤1 remote provider in v1.
- **Platform locks retained:** Windows 11 x64, Rust, local-first, no Solpaper cloud.
- **Still provisional until #18:** window topology, renderer, process/Cargo boundaries.

### Superseded product (closed issues)

Closed as `not planned` because they described the old wallpaper/TUI architecture or prematurely selected provider/UI details:

- #2 Wallhaven API research.
- #3 Bing fetch research.
- #4 Unsplash research.
- #8 TUI↔agent IPC.
- #9 TUI information architecture.
- #10 cron semantics.
- #11 cache defaults.
- #12 purity/source defaults.
- #14 fixed 2560×1440-oriented fit policy.
- #15 TUI prototype.

Relevant questions re-enter only after their subsystem is reached under the new map.

## Decisions still requiring evidence

- Independent widget HWNDs vs monitor-sized surface HWNDs vs hybrid.
- Exact click-through and selective interaction model.
- Exact z-order and Win+D behaviour.
- Explorer restart recovery strategy.
- Per-monitor layout and monitor identity model.
- Rendering backend.
- Settings UI toolkit.
- One-process runtime details and whether local IPC is needed.
- Production Cargo crate boundaries.
- First remote wallpaper provider.
- Virtual-desktop behaviour.
- Whether Calendar is a hard v1 gate or may slip after Alpha 2 (default: intended for v1).

## Planned product slices

### Prototype 0

Overlay feasibility only: transparency, input, Edit Mode, dragging/resizing, DPI, display changes, Explorer restart, sleep/resume, and resource baseline.

### Alpha 1

Tray/runtime + persistent widget layout + Pomodoro widget + local-folder wallpaper apply.

### Alpha 2

Read-only Google Calendar OAuth, offline event cache, privacy modes, and agenda widget.

### Beta

One researched remote wallpaper provider, scheduling, cache/failure policy, packaging preparation, diagnostics, and reliability hardening.

### v1

Validated Windows 11 build with overlay, Pomodoro, Calendar if retained as required scope, wallpaper subsystem, installation/autostart, recovery behaviour, and measurable acceptance tests.

## Out of scope for v1

- Linux and macOS.
- Windows SCM service.
- Solpaper cloud sync or cross-machine profiles.
- Mobile companion or remote network control.
- AI-generated wallpapers.
- Write access to Google Calendar.
- Plugin marketplace.
- TUI as a required product surface.
- More than one remote wallpaper provider unless it is nearly free after the first is stable.
- Automatic screen-sharing detection unless a reliable documented Windows mechanism is approved.

## Map completion rule

This map can close only when #24 demonstrates every required v1 criterion from #13 with a packaged build. A successful overlay spike alone does not complete the map.
