# ADR-0002: Process model and UI-thread ownership

## Status

**Accepted** (Issue #16).

## Context

Product destination (#17): a user-session Runtime owns Surfaces, productivity state, tray/settings, and wallpaper subsystem—not a Windows SCM service.

Spike #18 used a single process with a Win32 message loop; layout restore and low idle cost support a single long-running process. Sleep/resume duplicate-window risk is reduced by single-instance + re-create-from-layout, not multi-process shell attach (still manual-evidence open).

Owner provisional: one long-running user-session `solpaper` process; no local IPC until a real second client exists.

## Decision

1. **One** long-running user-session process (`solpaper` / `solpaper-app`) owns tray, overlay surfaces, domain state, and background coordination.
2. **UI thread:** Win32 message pump owns HWND lifetime and painting; network and heavy I/O must not block that thread (offload later when features need it).
3. **Single-instance:** enforce at startup (named mutex or equivalent) so sleep/resume and double-launch do not create duplicate overlay hosts.
4. Settings UI, when added, runs in-process or as owned windows of the same process—not a second agent process.

## Consequences

- No IPC bus in the scaffold (see ADR-0007).
- Tray (#7) and Pomodoro (#19/#20) plug into this process model.
- Crash recovery is process restart + layout restore, not multi-process handoff.
