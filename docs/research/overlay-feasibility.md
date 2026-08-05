# Overlay feasibility — Issue #18

**Status:** spike implemented; core interactive behaviour exercised on one Windows 11 machine  
**Spike location:** `spikes/desktop-overlay/` (disposable; not production)  
**Date:** 2026-08-05  
**Environment:** Windows 11 x64 (build 10.0.26200), Rust 1.97, single primary monitor during automated smoke

## Purpose

Prove whether Solpaper can host persistent, transparent, draggable desktop widgets on Windows 11 with a **documented Win32-first** model, without baking live content into wallpaper images and without making WorkerW/Progman the sole architecture.

## Approaches implemented

| | Approach A | Approach B |
|---|------------|------------|
| Model | One top-level HWND per sample widget | One work-area-sized HWND containing multiple widgets |
| Styles | `WS_POPUP` + `WS_EX_LAYERED` + `WS_EX_TOOLWINDOW` + `WS_EX_NOACTIVATE` | Same |
| Opacity | Global alpha via `SetLayeredWindowAttributes` (`LWA_ALPHA`) | Same |
| Normal Mode input | Whole window click-through (`WS_EX_TRANSPARENT` + `HTTRANSPARENT`) | Full surface `HTTRANSPARENT` |
| Edit Mode input | Window client hit-test; drag title / resize grip | Selective `HTTRANSPARENT` outside widget rects; drag/resize inside |
| Z-order policy | Not permanently topmost; `SetWindowPos(HWND_BOTTOM)` after show | Same |
| Sample widgets | Timer card (1 Hz text) + calendar card | Same, painted into one surface |
| Layout persist | `%LOCALAPPDATA%\solpaper-overlay-spike\layout-a.json` | `layout-b.json` |
| Hotkeys | Ctrl+Alt+F2 / + / − / S / Esc (global; avoids bare-key steal) | Same |

**Not used:** WorkerW, Progman parenting, undocumented shell hooks, wallpaper pixel compositing, tray polish, OAuth, SQLite production schema.

### How to run

```powershell
cd spikes/desktop-overlay
cargo run --release -- --approach a
cargo run --release -- --approach b
```

## Environment tested

| Item | Value |
|------|--------|
| OS | Windows 11 Pro x64, 10.0.26200 |
| GPU / compositor | DWM (default session) |
| Monitors during smoke | 1 (primary work area) |
| DPI | Process set to Per-Monitor V2 (`SetProcessDpiAwarenessContext`) |
| Build | `cargo build --release` for `desktop-overlay-spike` |

## Automated checks

```text
cargo fmt --all -- --check          # pass
cargo test                          # 6/6 pass (layout hit-test + JSON)
cargo clippy --all-targets --all-features -- -D warnings  # pass
```

Smoke (start process, sample after ~3 s, terminate):

| Approach | Working set | CPU (≈3 s) | Responding |
|----------|-------------|------------|------------|
| A | ~7.7 MB | ~0.02 s | yes |
| B | ~7.0 MB | ~0.02 s | yes |

Idle resource use for two simple widgets is acceptable for a continuously running utility baseline.

## Observed / designed behaviour matrix

Legend: **P** = proven in this spike session (code path + smoke or unit test); **D** = designed and implemented, visually/interactively expected but not fully instrumented; **M** = manual evidence still required; **N/A** = not claimed.

| Concern | Approach A | Approach B | Notes |
|---------|------------|------------|-------|
| Adjustable transparency | P | P | Global window alpha (not per-pixel ARGB surfaces) |
| Per-pixel alpha | N/A | N/A | Not required for pass; `UpdateLayeredWindow` left for later |
| Passive Mode drag/resize | D | D | Title bar + bottom-right grip; unit-tested hit classification |
| Normal Mode desktop usable | D | D | Click-through via documented styles/hit-test |
| Selective interactive regions | Partial (whole widget HWND) | D | B is strictly better for “holes” between cards |
| Focus stealing | D | D | `WS_EX_NOACTIVATE` + `SW_SHOWNOACTIVATE` |
| Taskbar / Alt+Tab | D | D | `WS_EX_TOOLWINDOW` hides from taskbar/Alt+Tab |
| Apps cover Solpaper | D | D | No permanent `WS_EX_TOPMOST` |
| Desktop icon interaction | D | D | Depends on click-through + non-topmost z-order |
| Win+D | M | M | Expected to hide top-level popups with show-desktop |
| Fullscreen apps | M | M | Non-topmost should sit under fullscreen |
| Explorer restart recovery | M | M | Documented path: re-create HWNDs from saved layout (no shell parent) |
| Layout restore after process restart | P | P | JSON load on start; save on exit / Ctrl+Alt+S |
| Mixed DPI / cross-monitor move | M | M | DPI-aware process; hardware not exercised this run |
| Monitor disconnect | M | M | |
| Sleep/resume | M | M | Must not duplicate windows — single process model; needs physical test |
| Lock/unlock | M | M | |
| Virtual desktops | M | M | Record as observed later |
| 1 Hz update cost | P | P | `SetTimer` 1000 ms; low CPU in smoke |
| WorkerW-only dependency | P (absent) | P (absent) | Explicitly not used |

## Comparison

### Approach A — independent widget HWNDs

**Strengths**

- Natural unit of composition matches product language (Widget ≈ window).
- Cheap invalidation: only dirty widgets repaint.
- Drag/resize maps directly to `SetWindowPos` on that HWND.
- Failure isolation: one widget window dying does not destroy others.
- Lower composited pixel count when widgets are small.

**Weaknesses**

- N top-level windows to track (hotkeys registered on first HWND only in the spike).
- Inter-widget z-order and relative stacking need explicit policy as count grows.
- “Holes” are automatic (desktop between windows); overlapping widgets need z-order rules.

### Approach B — monitor-sized surface

**Strengths**

- Single HWND per monitor simplifies hotkeys, timers, and lifetime.
- Selective `HTTRANSPARENT` is the clean documented model for empty regions.
- Relative layout math is trivial (all widgets in one client space).
- Easier to draw shared chrome or snap guides later.

**Weaknesses**

- Work-area-sized layered window always exists (larger DWM surface even when mostly empty).
- Full-surface repaint on any widget tick (mitigable with region invalidation later).
- One window failure loses all widgets on that monitor.
- Global alpha applies to the entire surface (per-widget opacity needs per-pixel path).

### Hybrid

Reasonable production shape: **Approach A as default** for Alpha 1 (few widgets), with the option to host dense clusters on a surface HWND later. Hybrid is **not required** to pass the spike.

## Recommendation

**Primary recommendation: independent widget HWNDs (Approach A)** for Solpaper’s production direction after human ADR review (#16).

Rationale:

1. Alpha 1 needs a small number of widgets (Pomodoro + layout chrome); per-widget HWNDs match that scale.
2. Documented Win32 styles meet transparency, no-activate, toolwindow, and click-through needs without shell parenting.
3. Resource baseline is low (~8 MB class).
4. Explorer recovery is “re-create from layout file,” not “re-attach to WorkerW.”
5. Approach B remains a validated fallback if selective multi-region input or many widgets make N HWNDs awkward.

**Do not abandon** the desktop-surface goal based on this spike. Core input/window behaviour is feasible on documented APIs.

**Do not** adopt WorkerW/Progman as the sole or default architecture.

## Unresolved risks

1. **Stable z-order above desktop icons but below apps** — `HWND_BOTTOM` is a heuristic; icons may paint over widgets depending on Explorer state. Needs visual confirmation and possibly periodic re-assert without topmost.
2. **Win+D / show desktop** — top-level popups may hide; tray restore path belongs to #7.
3. **Per-pixel alpha and non-rectangular cards** — not proven; global alpha only.
4. **Multi-monitor identity** — layout stores virtual-screen coordinates, not stable monitor IDs (Monitor Binding TBD in production).
5. **Physical sleep/resume and display topology** — see manual evidence.
6. **Hotkey conflicts** — Ctrl+Alt combos can clash with other software; production should prefer tray + in-UI toggles (#7).

## Manual evidence required

Do **not** automate these during owner study sessions:

- [ ] Sleep / resume (no duplicate windows; layout intact)
- [ ] Lock / unlock
- [ ] Monitor disconnect / reconnect and primary change
- [ ] Two-monitor layout + cross-monitor drag
- [ ] Mixed 100% / 125% / 150% DPI
- [ ] Explorer restart (`taskkill /f /im explorer.exe` then restart) — overlay re-create path
- [ ] Win+D and restore
- [ ] Fullscreen game/video coverage behaviour
- [ ] Prolonged idle CPU/memory (10+ minutes)

## Pass criteria checklist (Issue #18)

| Criterion | Status |
|-----------|--------|
| Widgets render with adjustable transparency | **Met** (smoke) |
| Passive mode: desktop/app controls usable | **Met by design**; confirm manually on icons |
| Edit Mode drag/resize | **Implemented**; interactive confirm recommended |
| Normal apps cover Solpaper (not permanent topmost) | **Met by design** |
| Layout restored after process restart | **Met** |
| Mixed-DPI / cross-monitor not unusable | **Partial** — DPI-aware; multi-mon **manual** |
| Display topology changes do not strand widgets | **Manual** |
| Sleep/resume no duplicate / total loss | **Manual** |
| Explorer restart recovery without undocumented-only path | **Documented strategy**; physical **manual** |
| Idle resource use recorded | **Met** (~7–8 MB, low CPU in smoke) |

**Spike engineering verdict:** **PASS with manual evidence debt** — architecture recommendation stands; physical recovery tests remain before #16 ADR freeze.

## Explicit non-goals (honoured)

- Production Cargo workspace / crate graph
- Google OAuth, wallpaper APIs, tray polish, TUI, installer
- IPC, SQLite architecture, production themes

## Next steps

1. Human review of this recommendation (blocks ADR in #16).
2. Owner or later iteration records manual evidence checkboxes above.
3. #16 records window topology = **per-widget top-level HWNDs**, renderer TBD, no WorkerW sole path.
4. Keep `spikes/desktop-overlay/` until #16 scaffolds production; then archive or delete.
