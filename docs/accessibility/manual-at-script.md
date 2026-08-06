# Manual assistive-technology script

**Issue:** [#41](https://github.com/rps321321/solpaper/issues/41)  
**MANUAL** before stable v1 (blueprint). Do not run disruptive shell tests during owner study.  
**Evidence layout:** `docs/testing/evidence/41/<yyyy-mm-dd>/<environment>/`

## Preconditions

- Named environment from [windows-matrix.md](../testing/windows-matrix.md)
- Build SHA recorded in `manifest.json`
- Synthetic Calendar data only (no real private events)
- Operator notes OS build, scale, theme, AT tools versions

## §1 Keyboard only (Alpha 1+)

Environment: single monitor 100%, default theme, mouse unplugged or unused.

| Step | Action | Pass criteria |
|-----:|--------|---------------|
| 1 | Start Solpaper | Tray icon present |
| 2 | Open tray menu with keyboard | Menu opens; items have readable names |
| 3 | Start Pomodoro | Timer running status available via tray/settings/UIA |
| 4 | Pause / resume / reset | Each succeeds without mouse |
| 5 | Open settings | Window focused; Tab moves between controls |
| 6 | Quit from tray | Process exits cleanly |

## §2 Edit Mode keyboard (#34 map)

Skip until Edit Mode ships; then:

| Step | Action | Pass criteria |
|-----:|--------|---------------|
| 1 | Enter Edit Mode | Mode indicated by non-color-only cue |
| 2 | Move focus / nudge widget per #34 | Widget moves; remains on work area or recoverable |
| 3 | Press Escape / documented exit | Returns to Normal Mode; no focus trap |

## §3 Text scaling

Repeat §1 smoke at **150%** (Alpha 1) and **200%** (Beta+).

| Pass criteria |
|---------------|
| Primary text readable |
| Widgets not permanently off-screen after clamp |
| Settings controls remain activatable |

## §4 High contrast

Enable a Windows high-contrast theme.

| Pass criteria |
|---------------|
| Settings text and controls visible |
| Pomodoro state not color-only |
| No essential UI missing contrast |

## §5 Narrator smoke (v1 gate)

Tools: Narrator (built-in). Optional: second SR if available.

| Step | Action | Pass criteria |
|-----:|--------|---------------|
| 1 | Focus settings | Control names announced sensibly |
| 2 | Pomodoro run / complete | Phase change understandable; not a flood every tick |
| 3 | Calendar Private fixture | Word “Private” or equivalent; **no** real title |
| 4 | Busy-only fixture | Busy/free only; **no** ordinary title leak |

## §6 Inspect / Insights smoke (Beta+)

| Tool | Check |
|------|-------|
| Inspect | Overlay widget ControlType Pane/Group; Name = type; Value = projected status |
| Inspect | Settings interactive controls named |
| Accessibility Insights (if available) | No critical name/type failures on settings |

## Recording results

1. Copy evidence templates from `docs/testing/evidence/`.
2. Fill scenario IDs `A11Y-*` from [acceptance-rows.md](./acceptance-rows.md).
3. Update manual-debt register when clearing MD-A11Y-* rows.
4. Never commit screenshots with real Calendar titles or tokens.

## Human / external review

Stable v1 expects either:

- This script completed on a named env **plus** brief qualified AT feedback, or  
- Explicit human waiver on #24/#44 with rationale.
