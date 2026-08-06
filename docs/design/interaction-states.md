# Interaction state table

**Issue:** [#34](https://github.com/rps321321/solpaper/issues/34)

## Runtime surface modes

| Mode | Overlay input | Actions allowed on overlay | Desktop usable | How entered | How exited |
|------|---------------|----------------------------|----------------|-------------|------------|
| **Normal** | Click-through; no buttons | None (status display only) | Yes | Default; after Edit exit | — |
| **Edit** | Hit-test on active widget chrome | Drag, resize, select | Mostly yes (except active chrome) | Tray / `Ctrl+Alt+F2` | Escape, tray Done, toggle shortcut |
| **Onboarding** | N/A (settings-like UI) | Wizard controls | Yes | First run | Finish / skip-to-tray |
| **Settings open** | Overlay stays Normal unless Edit also on | N/A | Yes | Tray | Close settings |

**Decision:** Normal Mode controls are **not** interactive on the overlay in Alpha 1. All commands go through tray, settings, or keyboard. This reduces accidental desktop blocking and avoids inaccessible custom hit-targets on layered windows.

## Pomodoro UI × domain status

| Domain status | Overlay shows | Tray status | Primary tray actions |
|---------------|---------------|-------------|----------------------|
| Idle | “Ready” / idle label | Idle | Start focus |
| Running Focus | mm:ss + Focus | Focus mm:ss | Pause, Skip, Reset |
| Running ShortBreak | mm:ss + Break | Break mm:ss | Pause, Skip, Reset |
| Running LongBreak | mm:ss + Long break | Long break mm:ss | Pause, Skip, Reset |
| Paused * | mm:ss paused | Paused mm:ss | Resume, Skip, Reset |
| After completion event | Next pending or Idle per domain | Updated once | Start if pending |

Notifications: text required on phase completion; dedupe by `completion_id`.

## Focus and activation policy

| Event | Policy |
|-------|--------|
| Normal Mode widget click | Pass through — do not activate Solpaper |
| Enter Edit Mode | May activate for keyboard; not permanent topmost |
| Notification | Do not steal full-screen focus aggressively; use standard toast patterns |
| Settings | Normal app window activation |

## Accidental interaction risks

| Risk | Mitigation |
|------|------------|
| Overlay blocks desktop | Normal Mode full click-through; no Alpha 1 overlay buttons |
| Global hotkey conflicts | Default only `Ctrl+Alt+F2`; document; tray always works if conflict |
| Edit Mode leave-behind | Escape always exits; tray shows “Editing…” state |
| Off-screen widgets | Clamp 48×48 DIP; Reset layout |
| Notification spam | Domain single completion; UI must not re-fire |
| Privacy leak on shared screen | Busy-only / Private projection; UIA same strings |

## Opacity

| Control | Where |
|---------|--------|
| Per-widget opacity | Settings → Widgets (not Normal Mode gesture) |
| Global vs per-widget | Per-widget stored in layout (ADR-0004); Edit Mode may show % in settings only for Alpha 1 |

## Widget lifecycle states (UI)

| State | Meaning |
|-------|---------|
| Visible | On a work area |
| Hidden | Not shown; restore via Settings → Widgets |
| Deleted | Removed with confirm; Pomodoro can be re-added |

Hide/delete **only** via accessible settings (pack DEFAULT), not undocumented overlay gestures.
