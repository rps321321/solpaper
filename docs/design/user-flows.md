# User flows

**Issue:** [#34](https://github.com/rps321321/solpaper/issues/34)  
**Pack source:** blueprint § #34 (DEFAULT)  
**A11y:** [docs/accessibility/requirements.md](../accessibility/requirements.md)

## Personas (minimal)

| Persona | Need |
|---------|------|
| New student/user | Install, see timer, edit layout once, quit without docs |
| Returning user | Tray actions, session recovery after restart/sleep |
| Calendar user (Alpha 2+) | Optional connect, privacy modes, offline clarity |

## F1 — First run (Alpha 1)

**Entry:** first successful process start with empty/default config.

| Step | UI | Behavior |
|-----:|----|----------|
| 1 | Onboarding / settings sheet | Local-first + privacy statement (no cloud backend; secrets policy one-liner) |
| 2 | Same flow | **Create Pomodoro widget by default** (layout defaults on primary work area) |
| 3 | Optional | Local wallpaper folder picker (skippable) |
| 4 | Finish | Close onboarding; **tray guidance** (balloon or one-time tip: “Solpaper is in the tray”) |
| — | Not in Alpha 1 first run | Google Calendar OAuth / calendar picker |

**Exit:** Runtime running; tray icon present; Pomodoro Idle on desktop (click-through).

**Failure:** If widget create fails, show error with primary action **Retry** or **Open diagnostics**; do not leave user without tray.

## F2 — Locate tray and open menu

| Step | Behavior |
|-----:|----------|
| 1 | User finds notify-icon (system tray / overflow) |
| 2 | Primary click or keyboard activation opens menu |
| 3 | Menu shows status line (e.g. Pomodoro Idle / Focus 12:04) + actions |

See [wireframes.md](./wireframes.md) tray IA.

## F3 — Enter / exit Edit Mode

| Path | Action |
|------|--------|
| Tray | **Edit layout** |
| Keyboard | `Ctrl+Alt+F2` (DEFAULT; remappable later only via explicit settings — not Alpha 1) |
| Exit | Tray **Done editing**, or **Escape**, or `Ctrl+Alt+F2` toggle |

**While in Edit Mode:**

- Selected widget: clear border (non-color-only cue + thickness), 24-DIP drag strip, 12-DIP resize grip.
- Desktop may receive fewer click-through regions on the active widget only.
- Other widgets: visible but not blocking whole desktop (Approach A per-widget HWND).

**Normal Mode after exit:** full click-through again; no overlay buttons.

## F4 — Drag, resize, monitor transfer, clamp

| Interaction | Behavior |
|-------------|----------|
| Drag (mouse) | 24-DIP strip; position in DIP; persist on release |
| Resize | 12-DIP grip; min size product-defined but clamp visibility 48×48 DIP |
| Keyboard move/resize | See [keyboard-map.md](./keyboard-map.md) |
| Monitor transfer | Drag across monitors; rebind to monitor identity (ADR-0004) |
| Snapping | Optional light snap to work-area edges in Edit Mode only; **off** if it fights user |
| Off-screen | Clamp so ≥48×48 DIP remains on **some** available work area after move/topology change |
| Reset layout | Tray or Settings → Widgets → **Reset layout** (confirm) |

## F5 — Pomodoro (Alpha 1)

Domain rules: [pomodoro-state-machine.md](./pomodoro-state-machine.md). UI only exposes commands.

| User goal | Path |
|-----------|------|
| Start focus | Tray **Start focus** / Settings Pomodoro / future hotkey |
| Pause / resume | Tray |
| Skip phase | Tray (does **not** credit focus count — domain) |
| Reset | Tray **Reset** → Idle; preserves focus count per domain |
| See remaining | Overlay text (Normal Mode) + tray status + UIA value |
| Completion | Text notification; sound optional; no color-only; at most one notify per completion_id |

**Recovery (visible):** After restart/sleep, overlay/tray show Idle or remaining coherently; pending break may wait for Start (auto-start next = false).

**Normal Mode:** no overlay Start/Pause buttons (pack DEFAULT).

## F6 — Local wallpaper folder

| Step | Behavior |
|-----:|----------|
| 1 | Settings → Wallpaper → **Choose folder** |
| 2 | OS folder picker |
| 3 | Success: path shown (not secrets); apply policy per #5 |
| Fail empty | Message + **Choose folder** primary |
| Fail unreadable | Message + **Choose folder** / **Open diagnostics** |
| Fail apply | Keep current wallpaper; error + **Retry** |

Not in first-run required path if user skipped step 3 of F1.

## F7 — Settings navigation

Order (pack DEFAULT):

1. General  
2. Widgets  
3. Pomodoro  
4. Wallpaper  
5. Calendar (Alpha 2+ enabled; Alpha 1 may show “Coming in Alpha 2” or disabled page)  
6. Diagnostics / About  

- Standard native controls; system colors; high contrast.
- No theme editor; no undocumented dark-mode API.
- Animation optional, **default off**.
- Every error: one clear primary recovery action.

## F8 — Calendar (Alpha 2 design now; implement later)

Not on Alpha 1 first run.

| Flow | Behavior |
|------|----------|
| Sign-in | Settings → Calendar → **Connect Google** → system browser OAuth (read-only scopes) |
| Calendar selection | Multi-select list after connect |
| Privacy | Default ordinary titles; Private → `Private`; Busy-only mode toggle |
| Offline / stale | Banner: last sync time; primary **Retry sync** |
| Error / re-auth | Primary **Reconnect**; secondary **Disconnect & purge tokens** |
| Disconnect | Confirm; purge credentials + local agenda cache |

Overlay shows projected agenda only; no OAuth on the widget itself.

## F9 — Empty, destructive, uninstall language

| Situation | Copy intent |
|-----------|-------------|
| No widgets | Settings Widgets: **Add Pomodoro** primary |
| Reset layout | Confirm: “Reset all widget positions to defaults?” |
| Disconnect Calendar | Confirm: “Remove Google access and cached agenda from this PC?” |
| Quit | Tray **Quit Solpaper** — stops runtime (not “close window only”) |
| Uninstall / purge | Installer/docs: removes settings DB; credentials purged from Credential Manager; no cloud account at Solpaper |

## F10 — High contrast, scaling, reduced motion, keyboard-only

| Concern | Flow impact |
|---------|-------------|
| High contrast | Settings + Edit Mode chrome use system colors; status not color-only |
| Text scale 100/150/200 | Layout clamp after scale; re-open settings if needed |
| Reduced motion | No required animation for state changes |
| Keyboard-only | Complete [usability-script.md](./usability-script.md) without mouse |

## Architecture constraints (from #18)

- Approach A widget HWNDs; not permanent topmost.
- Normal Mode must not block desktop icons/apps.
- No WorkerW/Progman-only design in flows.
- Live content never baked into wallpaper images.
