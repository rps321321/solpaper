# ADR-0008: Narrow second-launch activation (not general IPC)

## Status

**Accepted** (Issue #7).

## Context

ADR-0002 requires a single long-running process. ADR-0007 forbids a general local IPC protocol in v1. Users still need a deterministic second-launch experience (open Settings / focus the existing Runtime) without spawning a second tray or widget set.

Blueprint pack #7 specifies: mutex already held → find control window → post `WM_APP_SHOW_SETTINGS` → exit 0.

## Decision

1. Register a hidden/message-only control window with class **`Solpaper.Runtime.Control.v1`** owned by the Runtime UI thread.
2. Define **one** application message: `WM_APP_SHOW_SETTINGS` (`WM_APP + 1`) with zero payload — show or create in-process Settings only.
3. Second process: acquire mutex fails → `FindWindowW` by class → `PostMessageW` → exit 0. Never create tray, surfaces, or domain workers in the second process.
4. If the control window is missing, still exit 0 without starting a second Runtime (avoid duplicates under crash races).
5. This remains **out of scope** for ADR-0007 “general IPC”: no named pipes, no command bus, no multi-verb protocol until a future ADR + threat-model update.

## Consequences

- `solpaper-app` wires second-launch activation today; full control HWND registration completes in #20 with the tray host.
- Security surface stays small (no authenticated localhost protocol).
- Autostart (`--background`) still goes through the same single-instance path.
