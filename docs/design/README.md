# Product design

**Issue:** [#34](https://github.com/rps321321/solpaper/issues/34)  
**Pack:** [`deterministic-execution-blueprint.md` § #34](../engineering/deterministic-execution-blueprint.md)  
**Related:** [#41](https://github.com/rps321321/solpaper/issues/41) a11y · [#18](https://github.com/rps321321/solpaper/issues/18) overlay · [#7](https://github.com/rps321321/solpaper/issues/7) tray · [#19](https://github.com/rps321321/solpaper/issues/19) Pomodoro · [#20](https://github.com/rps321321/solpaper/issues/20) Alpha 1 · [#21](https://github.com/rps321321/solpaper/issues/21) Calendar

| Document | Purpose |
|----------|---------|
| [user-flows.md](./user-flows.md) | First-run, tray, Edit Mode, Pomodoro, wallpaper, Calendar, errors |
| [interaction-states.md](./interaction-states.md) | Mode/surface state table and accidental-interaction rules |
| [keyboard-map.md](./keyboard-map.md) | Global and Edit Mode keys (pack DEFAULT) |
| [wireframes.md](./wireframes.md) | Low-cost text wireframes / prototype stand-in |
| [usability-script.md](./usability-script.md) | Required new-user script + findings template |
| [decisions-for-consumers.md](./decisions-for-consumers.md) | Explicit decisions for #7, #19, #20, #21 |
| [pomodoro-state-machine.md](./pomodoro-state-machine.md) | Domain machine (#19) — not UI chrome |
| [runtime-tray.md](./runtime-tray.md) | Tray runtime, autostart, single-instance (#7) |

## Hard rules (pack DEFAULT)

1. **Normal Mode:** widgets read-only and click-through; **no** direct overlay buttons in Alpha 1.
2. **Actions:** tray, settings, and keyboard only for core control.
3. **First run:** privacy → Pomodoro widget → optional local folder → tray guidance; **no** Calendar OAuth on Alpha 1 first launch.
4. **Edit Mode:** tray + default `Ctrl+Alt+F2`; Escape exits; clamp ≥48×48 DIP visible.
5. **Settings IA:** General → Widgets → Pomodoro → Wallpaper → Calendar → Diagnostics/About.
6. Visual design must not force architecture rejected by #18 / ADRs (Approach A, no WorkerW-only).
7. Usability review with humans is **MANUAL**; this pack delivers the script and design, not completed user study evidence.
