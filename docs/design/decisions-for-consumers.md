# Explicit UX decisions for consumer issues

**Issue:** [#34](https://github.com/rps321321/solpaper/issues/34)  
**Authority:** blueprint § #34 DEFAULT + this document. Deviations need issue rationale.

## For #7 — tray runtime, autostart, single-instance

| Decision | Value |
|----------|--------|
| Tray is primary IA | Status + Pomodoro commands + Edit Mode + Settings + Quit |
| Single-instance | One runtime; second launch focuses tray/settings tip, does not duplicate widgets |
| Autostart | Opt-in in General settings (default **off** for first-run surprise avoidance unless owner changes later) |
| Edit Mode entry | Tray item + `Ctrl+Alt+F2` |
| Quit | Explicit **Quit Solpaper** ends process |
| Balloon/tip | One-time first-run tray guidance only |

## For #19 — Pomodoro (UI consumption)

| Decision | Value |
|----------|--------|
| No overlay buttons Alpha 1 | Commands via tray/settings only |
| Display | Remaining + phase label; paused indicated in text |
| Notifications | Text on completion; respect domain dedupe |
| Settings | Duration fields match domain ranges; auto-start next default **false** |
| Skip | Available in tray; does not credit focus (domain) |

Domain machine remains [pomodoro-state-machine.md](./pomodoro-state-machine.md); UI must not invent extra states.

## For #20 — Alpha 1 build scope (interaction)

| Decision | Value |
|----------|--------|
| First-run | Privacy → default Pomodoro widget → optional folder → tray tip |
| No Calendar OAuth in Alpha 1 first run | Calendar page stub/disabled OK |
| Normal Mode | Click-through, read-only overlay |
| Edit Mode | Pack geometry: 24-DIP drag, 12-DIP grip, Escape exit, clamp 48×48 |
| Settings IA | General → Widgets → Pomodoro → Wallpaper → Calendar → Diagnostics/About |
| Wallpaper | Local folder only |
| Usability | Script in usability-script.md is Alpha 1 acceptance aid |

## For #21 — Calendar Alpha 2 (interaction)

| Decision | Value |
|----------|--------|
| Connect only from Settings | System browser OAuth |
| Privacy default | Ordinary titles; offer Busy-only; Private → `Private` |
| Errors | Reconnect / Retry sync / Disconnect&purge with confirm |
| Overlay | Projected agenda only; no secrets; offline/stale banner semantics in settings + short overlay line |
| Not first-run | Optional later connection |

## For #41 — alignment

| Decision | Value |
|----------|--------|
| Keyboard map | [keyboard-map.md](./keyboard-map.md) is the Edit Mode map #41 references |
| Core actions mouse-free | Tray/settings paths mandatory |
| Non-color status | Phase labels in text |

## Out of scope / deferred

- Fancy theme editor, marketplace skins  
- Overlay-drawn Pomodoro buttons in Alpha 1  
- Mandatory global hotkeys for every command  
- Completed human usability study evidence (MANUAL sessions)
