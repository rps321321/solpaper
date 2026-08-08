# Tray runtime, autostart, and single-instance

**Issue:** [#7](https://github.com/rps321321/solpaper/issues/7)  
**Status:** design + pure seams + registry/activation + **Runtime tray host (#20 bullet 1)**  
**Pack:** [`deterministic-execution-blueprint.md` § #7](../engineering/deterministic-execution-blueprint.md)  
**Related:** ADR-0002 process · ADR-0007 IPC deferred · [ADR-0008](../adr/0008-second-launch-activation.md) · #40 diagnostics · #19 Pomodoro · #20 Alpha 1  
**Code:** `solpaper-core::tray`, `solpaper-windows::{activation,autostart,single_instance,runtime}`

## Purpose

Define how one long-running `solpaper.exe` owns the user session: tray, windows, background work, shutdown, recovery, and optional logon startup — without duplicate tray icons, widgets, or timers.

## Ownership diagram

```text
┌──────────────────── solpaper.exe (one process) ────────────────────┐
│  UI thread (STA COM for wallpaper)                                 │
│    Win32 message loop                                              │
│    ├─ Control HWND class Solpaper.Runtime.Control.v1               │
│    ├─ Tray icon (Shell_NotifyIconW) — #20 wires full menu host     │
│    ├─ Widget HWNDs (Approach A)                                    │
│    ├─ Settings / Diagnostics (in-process, lazy)                    │
│    └─ PostMessage completion from worker                           │
│  Worker thread (standard thread + typed channel)                   │
│    wallpaper prep, future Calendar I/O (async only when needed)    │
│  Named mutex Local\SolpaperSingleInstance_v1                       │
└────────────────────────────────────────────────────────────────────┘
```

| Concern | Owner |
|---------|--------|
| HWND lifetime / paint / tray | UI thread |
| COM `IDesktopWallpaper` | UI thread (STA) |
| Pomodoro wall-clock ticks | UI timer or worker + PostMessage to UI |
| Heavy decode / network | Worker only; never block UI |
| Settings | Same process, lazy create |

## Single-instance and second launch

| Step | Behavior |
|------|----------|
| 1 | `CreateMutexW` named `Local\SolpaperSingleInstance_v1` |
| 2 | If new → create control window, tray, surfaces; run message loop |
| 3 | If already exists → `FindWindowW(Solpaper.Runtime.Control.v1)` → `PostMessage(WM_APP_SHOW_SETTINGS)` → **exit 0** |
| 4 | If mutex held but window missing (crash race) → exit 0 without starting a second Runtime |

This is a **narrow activation signal**, not a general command protocol (ADR-0007 / ADR-0008).

Code: `activate_existing_show_settings`, `second_launch_outcome`, wired in `solpaper-app` main.

## Tray lifecycle

| Event | Action |
|-------|--------|
| Startup | `NIM_ADD` + `NIM_SETVERSION`; stable GUID identity (host #20) |
| Explorer / taskbar recreate | Listen `TaskbarCreated`; re-`NIM_ADD` only — **do not** recreate widget HWNDs via Explorer parenting |
| Menu | Fixed order from `build_tray_menu` (unavailable = **disabled**, not hidden) |
| Balloon | `Shell_NotifyIcon` + `NIF_INFO`; dedupe via `NotificationDeduper` / phase instance id |
| Shutdown | Remove icon before destroy windows |

Full `Shell_NotifyIconW` host is Alpha 1 (#20); this issue freezes menu order, enablement, and notification dedupe policy in `solpaper-core`.

### Menu (fixed order)

1. Open Settings  
2. Edit Mode toggle  
3. —  
4. Pomodoro Start/Pause/Resume  
5. Pomodoro Skip  
6. Pomodoro Reset  
7. —  
8. Wallpaper Next  
9. Wallpaper Hold  
10. —  
11. Start with Windows  
12. Diagnostics  
13. Quit  

## Autostart

| Rule | DEFAULT |
|------|---------|
| Default state | **Disabled** |
| Portable builds | No autostart UI (`portable_allows_autostart_ui() == false`) |
| Mechanism | `HKCU\Software\Microsoft\Windows\CurrentVersion\Run` only |
| Value name | `Solpaper` |
| Value data | `"<absolute installed exe>" --background` |
| Disable / uninstall | Delete **only** the Solpaper value |
| Forbidden | Task Scheduler, Windows service, machine-wide Run |

Adapters: `FakeAutostartStore`, `WindowsRunKeyAutostart` (HIGH: registry mutation).

## Notifications

- Path: tray balloon (`NIF_INFO`), not Windows App SDK toast-only for Alpha 1.
- Dedupe: at most one balloon per Pomodoro phase instance id.
- Never put Calendar titles into balloon text under privacy projection (#37 / #40).
- Auth/sync failures: same tray path + Diagnostics.

## Shutdown and recovery

Sequence (`SHUTDOWN_SEQUENCE`):

1. Stop accepting work  
2. Stop timers  
3. Atomic flush settings/runtime  
4. Stop worker (wait ≤ **2000 ms**)  
5. Remove tray icon  
6. Destroy windows  
7. Release mutex  

- No automatic crash restart (see #40 safe mode).  
- Next user/logon launch recovers durable state.  
- Explorer restart: tray re-add only.

## Failure table (link #40)

| Failure | Error path | Recovery |
|---------|------------|----------|
| Second launch, no control HWND | AlreadyRunningNoWindow | User starts again if process zombie; no duplicate Runtime |
| Autostart registry fail | `AutostartError::Registry` code | Settings shows disabled; keep prior state |
| Tray add fail after Explorer | surface/tray error code | Retry on TaskbarCreated; Diagnostics |
| Worker stop timeout | shutdown continues after 2 s | Log warn; still destroy windows |
| Panic loop | #40 crash markers | Safe mode; no autostart mutation in safe mode |

## Background work

- Alpha 1: **one** standard worker thread + typed channel; completion via `PostMessage` to UI.
- Async runtime only when Calendar networking requires it, **background thread only**.

## Requirements for #20

| ID | Requirement |
|----|-------------|
| RT-A1-01 | Register control window class `Solpaper.Runtime.Control.v1` before tray — **done** (`runtime.rs`) |
| RT-A1-02 | Tray icon + menu host using `build_tray_menu` — **done** (uid-based icon; product GUID resource later) |
| RT-A1-03 | TaskbarCreated re-add — **done** |
| RT-A1-04 | Autostart toggle only when installed_build — store ready; tray UI still scaffold-disabled |
| RT-A1-05 | Shutdown sequence honors 2 s worker wait — tray remove + destroy; worker wait when worker lands |
| RT-A1-06 | Balloon dedupe with phase instance id — pure deduper ready; NIF_INFO balloon wire later |
| RT-A1-07 | Second launch posts WM_APP_SHOW_SETTINGS — **done** (FindWindow finds control HWND) |
| RT-A1-08 | Approach A widget host class `Solpaper.Widget.Host.v1` — **done** (`widget_host.rs`, #20 bullet 2) |
| RT-A1-09 | Normal Mode click-through + Edit Mode tray / Ctrl+Alt+F2 / Escape — **done** (session geometry only; layout write is bullet 3) |

## Manual evidence

| ID | Scenario | Status |
|----|----------|--------|
| MD-RT-01 | Explorer restart recreates tray only | open |
| MD-RT-02 | Logon autostart (installed) | open |
| MD-RT-03 | Task Manager startup entry name | open |
| MD-RT-04 | Toggle off / uninstall removes only Solpaper Run value | open |
| MD-RT-05 | Second launch opens settings, no second tray | open (logic partial; needs control HWND) |

## Non-goals

No general IPC, no second process for settings, no TUI, no Task Scheduler autostart, no automatic crash restart loop.

## Acceptance criteria trace

| Criterion | Where |
|-----------|--------|
| Two starts never two runtimes/trays | Mutex + second_launch_outcome |
| Second-launch deterministic | ActivateShowSettings / exit 0 |
| Explorer restart path | Design + MD-RT-01 |
| Shutdown flush within budget | SHUTDOWN_* constants |
| UI not blocked by heavy work | Worker policy |
| Autostart opt-in, removable | WindowsRunKeyAutostart |
| No general IPC / TUI | ADR-0007 + this doc |
