# Pomodoro state machine and recovery

**Issue:** [#19](https://github.com/rps321321/solpaper/issues/19)  
**Status:** implemented in `solpaper-core` (provisional defaults)  
**Code:** [`crates/solpaper-core/src/pomodoro.rs`](../../crates/solpaper-core/src/pomodoro.rs)

Platform-neutral domain design for Alpha 1. No overlay/UI choices here.

## Provisional defaults (human may re-approve)

| Setting | Value |
|---------|------:|
| Focus | 25 minutes |
| Short break | 5 minutes |
| Long break | 15 minutes |
| Focuses before long break | 4 |
| Auto-start next phase | **false** |
| Skip increments focus count | **no** |

These match common Pomodoro practice; Issue #19 listed them as human-input items. Changing defaults is a small config change, not a machine redesign.

## States

| Status | Meaning |
|--------|---------|
| `Idle` | No active phase |
| `Running { phase, deadline_utc_ms }` | Phase in progress; wall-clock deadline in UTC ms |
| `Paused { phase, remaining_ms }` | Phase paused; remaining duration stored (no deadline) |

`phase` ∈ { `Focus`, `ShortBreak`, `LongBreak` }.

There is **no** persisted permanent `Completed` state. Completion is an **event** (`PhaseCompleted` with `completion_id`).

## Commands

| Command | Idle | Running | Paused |
|---------|------|---------|--------|
| `Start` | → Running(Focus) | illegal | illegal (use Resume) |
| `Pause` | illegal | → Paused | illegal |
| `Resume` | illegal | illegal | → Running |
| `Skip` | illegal | end phase without focus credit; Idle or auto-next | same |
| `Reset` | no-op-ish Idle + clear cycle | → Idle, clear cycle count | same |
| `Sync` | no-op | if `now ≥ deadline`, complete **one** phase | no-op |

## Phase sequencing

After a **completed** Focus (via `Sync` expiry):

1. Increment `completed_focuses_in_cycle`.
2. If `completed_focuses_in_cycle % focuses_before_long_break == 0` → next is LongBreak; else ShortBreak.
3. If `auto_start_next`, begin that break; else `Idle`.

After a completed ShortBreak or LongBreak → next Focus (LongBreak completion also zeroes the cycle count).

After **Skip** of Focus: do **not** increment focus count; next break is always ShortBreak for that skip path (long-break cadence uses completed focuses only).

## Time model

- Visible remaining time is derived from `deadline_utc_ms - now` while running (or stored `remaining_ms` while paused).
- Correctness does **not** depend on 1 Hz ticks; the runtime may call `Sync` / `view` at any cadence.
- Time-zone changes do not alter an existing UTC deadline.
- Large clock jumps: `Sync` still completes **at most one** phase (no backlog replay).

## Recovery

| Situation | Policy |
|-----------|--------|
| App restart, deadline in future | Restore `Running`; countdown continues |
| App restart, deadline past | On first `Sync`, complete one phase; notify once (`completion_id`); do not auto-start unless config says so |
| Sleep/hibernate across deadline | Same as missed deadline: one completion, one notification id |
| Multiple theoretical phases missed | Still one completion only |
| Surprising clock anomaly | User can `Reset` / `Start`; no historical reconstruction |

Notification dedupe: `last_completion_id` / monotonic `completion_seq`. UI should ignore duplicate ids.

## Persistence shape

Serialize `PomodoroState`:

- `config` (duration snapshot + auto-start + cadence)
- `status` (Idle / Running+deadline / Paused+remaining)
- `completed_focuses_in_cycle`
- `last_transition_utc_ms`
- `last_completion_id`
- `completion_seq` (private field in Rust; still serde-visible for round-trip)

Storage lives with other non-secret app data (ADR-0005). **No** secrets.

## History (Alpha 1)

**Out of scope** for this ticket’s code: no append-only history log yet. Events are ephemeral for the session/UI. A future Alpha slice may add optional local history without dashboards.

## Widget view model

`PomodoroView`: phase label, remaining ms, progress 0–1, running/paused/idle flags, cycle count, `AvailableActions` { start, pause, resume, skip, reset }.

## Non-goals

- Overlay renderer / HWND widget chrome (#20)
- Tray menu wiring (#7 / #20)
- Sound asset selection (policy only: completion event exists)
- Multi-timer or concurrent sessions

## Acceptance mapping

| Criterion | Mechanism |
|-----------|-----------|
| No dependence on 1s ticks | Deadline math + tests |
| Restart before deadline restores | Persist Running+deadline; unit tests |
| Missed deadline ≤ one phase | `Sync` single completion + large-jump test |
| No multi-phase replay | Same |
| Duplicate completion notifications | `completion_id` |
| Illegal transitions rejected | `CoreError::IllegalPomodoroTransition` + tests |
| Skip default | No focus credit |

## Human gate (open)

Owner may still adjust default durations, cadence, auto-start, and skip semantics; update `PomodoroConfig::default` / constants only if they disagree with the provisional table above.
