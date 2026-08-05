# Pomodoro state machine and recovery

**Issue:** [#19](https://github.com/rps321321/solpaper/issues/19)  
**Status:** implemented in `solpaper-core`  
**Code:** [`crates/solpaper-core/src/pomodoro.rs`](../../crates/solpaper-core/src/pomodoro.rs)  
**Pack:** `docs/engineering/deterministic-execution-blueprint.md` § #19 (sole decision store)

Platform-neutral domain design for Alpha 1. No overlay/UI choices here.

## Defaults (blueprint #19)

| Setting | Value |
|---------|------:|
| Focus | 25 minutes (range 1–180) |
| Short break | 5 minutes (range 1–60) |
| Long break | 15 minutes (range 1–120) |
| Focuses before long break | 4 (range 2–12) |
| Auto-start next phase | **false** |
| Skip increments focus count | **no** |
| Alpha 1 history / analytics | **off** |

## States

| Status | Meaning |
|--------|---------|
| `Idle` | No active phase |
| `Running { phase, deadline_utc_ms, phase_instance_id }` | Phase in progress; wall-clock deadline in UTC ms |
| `Paused { phase, remaining_ms, phase_instance_id }` | Phase paused; remaining duration stored (no deadline) |

`phase` ∈ { `Focus`, `ShortBreak`, `LongBreak` }.

There is **no** persisted permanent `Completed` state. Completion is an **event** (`PhaseCompleted` with `completion_id` = completed `phase_instance_id`).

## Commands

| Command | Idle | Running | Paused |
|---------|------|---------|--------|
| `Start` | → Running(Focus, new instance) | illegal | illegal (use Resume) |
| `Pause` | illegal | → Paused (remaining) | illegal |
| `Resume` | illegal | illegal | → Running (`now + remaining`) |
| `Skip` | illegal | end phase without focus credit; Idle or live auto-next | same |
| `Reset` | → Idle; **preserve** completed-focus count | same | same |
| `Sync` | no-op | if `now ≥ deadline`, complete **one** phase; **never** auto-start | no-op |
| `LiveTick` | no-op | same completion rule; may auto-start if configured | no-op |

`Sync` is the recovery path after restore/sleep/restart. `LiveTick` is for continuous process operation.

## Phase sequencing

After a **completed** Focus (via expiry):

1. Increment `completed_focuses_in_cycle`.
2. If `completed_focuses_in_cycle % focuses_before_long_break == 0` → next is LongBreak; else ShortBreak.
3. `LiveTick` + `auto_start_next` begins that break; `Sync` always leaves `Idle` (even with auto-start).

After a completed ShortBreak or LongBreak → next Focus (LongBreak completion zeroes the cycle count).

After **Skip** of Focus: do **not** increment focus count; next break is ShortBreak for that skip path.

## Time model

- Visible remaining time is derived from `deadline_utc_ms - now` while running (clamped to phase total), or stored `remaining_ms` while paused.
- Correctness does **not** depend on 1 Hz ticks.
- Time-zone changes do not alter an existing UTC deadline.
- Large clock jumps / recovery: `Sync` still completes **at most one** phase (no backlog replay).
- Monotonic display clock is an adapter concern for #20; domain accepts injected `now_utc_ms`.

## Recovery

| Situation | Policy |
|-----------|--------|
| App restart, deadline in future | Restore `Running`; countdown continues |
| App restart, deadline past | First `Sync` completes one phase; notify once; **no auto-start** |
| Sleep/hibernate across deadline | Same as missed deadline via `Sync` |
| Multiple theoretical phases missed | Still one completion only |
| Repeated `Sync` after completion | No events |

Notification dedupe: `last_completion_id` / `phase_instance_id`. Tray delivery is #7 (`Shell_NotifyIcon` / `NIF_INFO`); not implemented in this crate.

## Persistence shape

Serialize `PomodoroState`:

- `config` (duration snapshot + auto-start + cadence)
- `status` (Idle / Running+deadline+instance / Paused+remaining+instance)
- `completed_focuses_in_cycle`
- `last_transition_utc_ms`
- `last_completion_id`
- `phase_seq` (serde-visible private field for instance/completion identity)

No secrets. No SQLite history in Alpha 1.

## Widget view model

`PomodoroView`: phase label, remaining ms, progress 0–1, running/paused/idle flags, cycle count, `AvailableActions` { start, pause, resume, skip, reset }.

## Non-goals

- Overlay renderer / HWND widget chrome (#20)
- Tray menu wiring (#7 / #20)
- Sound playback implementation
- Multi-timer or concurrent sessions
- Claiming physical sleep/resume evidence (open under #33/#24)

## Acceptance mapping

| Criterion | Mechanism |
|-----------|-----------|
| No dependence on 1s ticks | Deadline math + tests |
| Restart before deadline restores | Persist Running+deadline; serde tests |
| Missed deadline ≤ one phase | `Sync` single completion + large-jump test |
| Recovery never auto-starts | `Sync` vs `LiveTick` |
| Skip default | No focus credit |
| Reset preserves focus count | Blueprint pack #19 |
| Config ranges | validate() 1–180 / 1–60 / 1–120 / 2–12 |
| Illegal transitions rejected | `CoreError::IllegalPomodoroTransition` |
