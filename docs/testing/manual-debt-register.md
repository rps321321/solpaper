# Manual evidence debt register

**Issue:** [#33](https://github.com/rps321321/solpaper/issues/33)  
**Seed source:** [#18](https://github.com/rps321321/solpaper/issues/18) / [`overlay-feasibility.md`](../research/overlay-feasibility.md)  
**Rules:** Autonomous merges may **add** rows. They must **not delete** or mark **cleared** without a linked evidence path and issue/PR trail.

## Status values

| Status | Meaning |
|--------|---------|
| `open` | Not yet run on a named environment |
| `in_progress` | Session scheduled or partial evidence |
| `cleared` | Evidence path filled; acceptance may cite it |
| `waived` | Human waiver with rationale and issue link (rare) |
| `blocked` | Waiting on hardware/operator/study constraints |

## Register

| ID | Scenario | Issue | Environment required | Owner / operator | Blocks release | Status | Evidence path | Expiry / retest trigger |
|----|----------|------:|----------------------|------------------|---------------:|--------|---------------|-------------------------|
| MD-001 | Sleep / resume — no duplicate windows; layout intact; Pomodoro recovery coherent | #18, #33, #13, #24 | Named Win11 x64 physical (`env-owner-primary` or successor) | owner | v1 (surface recovery) | `open` | — | Retest after Runtime HWND model or recovery policy change |
| MD-002 | Lock / unlock session — no duplicate Runtime/windows | #18, #33, #13, #24 | Named physical | owner | v1 | `open` | — | Retest after session-notify handling changes |
| MD-003 | Explorer restart recovery without undocumented-only path | #18, #33, #13, #24 | Named physical; disruptive | owner | v1 | `open` | — | Retest after surface create/destroy path changes |
| MD-004 | Two-monitor layout + cross-monitor drag | #18, #33, #13 | Dual-monitor physical | owner | v1 if multi-mon claimed | `open` | — | Retest after layout/monitor-binding changes |
| MD-005 | Mixed DPI (100% / 125% / 150%) usable layout | #18, #33, #13, #35 | Mixed-DPI physical | owner | v1 if mixed DPI claimed | `open` | — | Retest after DPI/DIP code changes |
| MD-006 | Monitor disconnect / reconnect / primary change — no stranded widgets | #18, #33, #13 | Hotplug-capable physical | owner | v1 | `open` | — | Retest after topology listeners change |
| MD-007 | Win+D and restore | #18, #33, #13 | Named physical | owner | v1 | `open` | — | Retest after z-order/style changes |
| MD-008 | Fullscreen game/video coverage (not permanent topmost) | #18, #33, #13 | Named physical | owner | v1 | `open` | — | Retest after topmost/style policy changes |
| MD-009 | Prolonged idle CPU/memory (≥10 min smoke; Beta 8 h soak later) | #18, #33, #35, #24 | Named physical; release profile | owner | budgets at claimed phase | `open` | — | Retest after timer/poll/render loop changes |
| MD-A11Y-01 | Keyboard-only core actions (tray/settings Pomodoro) | #41, #13, #20 | Named physical | owner | Alpha 1 | `open` | — | Retest after tray/command map changes |
| MD-A11Y-02 | Inspect UIA: settings Names + overlay Pane/Group | #41, #13 | Named physical + SDK Inspect | owner | Alpha 1 / Beta | `open` | — | Retest after UIA provider or settings toolkit changes |
| MD-A11Y-03 | Text scaling 100% / 150% / 200% usability | #41, #33, #13 | Named physical | owner | Alpha 1 (100/150); Beta (200) | `open` | — | Retest after layout/DPI changes |
| MD-A11Y-04 | High contrast readable settings + widget status | #41, #13 | Named physical | owner | Beta | `open` | — | Retest after theme/paint changes |
| MD-A11Y-05 | Narrator smoke Pomodoro + settings; no private title leak | #41, #13, #24 | Named physical | owner | v1 | `open` | — | Retest after projection/UIA/notification changes |
| MD-UX-01 | Human usability sessions for `docs/design/usability-script.md` | #34, #20 | Named physical + participant | owner | Alpha 1 gate (script pass) | `open` | — | Retest after major tray/Edit Mode changes |
| MD-PERF-01 | Cold start p95 ≤ 1.5 s; warm/cold settings open; shutdown ≤ 2 s (release build) | #35, #13, #20 | Named Win11 x64 physical; release profile | owner | Alpha 1 | `open` | — | Retest after tray/settings host or startup path changes |
| MD-PERF-02 | Idle working set with Calendar connected ≤ 100 MiB | #35, #21, #13 | Named physical; account connected | owner | Alpha 2 | `open` | — | Retest after Calendar sync/client changes |
| MD-PERF-03 | Beta soak: no crash/hang in 8 h on reference environment | #35, #24 | Named reference env; unattended | owner | Beta | `open` | — | Retest after timer/poll/render or process-model changes |

## How to clear a row

1. Run on a **named** environment from [windows-matrix.md](./windows-matrix.md).
2. Write evidence under `docs/testing/evidence/<issue>/<yyyy-mm-dd>/<environment>/` using the templates.
3. Set **Status** to `cleared`, fill **Evidence path**, link PR/issue comment.
4. Do not remove the row; cleared rows are the audit trail.

## How to add a row

- New hardware-dependent or disruptive claim without automation: add `MD-0xx` with blocking release phase.
- Prefer extending this table over burying debt only in chat or PR text.

## Waivers

Only a human may set `waived`. Record issue number, rationale, and which phase is no longer blocked. Autonomous agents must not waive.
