# ADR-0004: Widget layout persistence and monitor binding

## Status

**Accepted** (Issue #16). Monitor identity refinement may deepen when multi-monitor evidence exists.

## Context

Spike #18 persisted JSON layout under `%LOCALAPPDATA%\solpaper-overlay-spike\` with virtual-screen coordinates; restore after process restart was proven. Stable monitor IDs and mixed-DPI multi-monitor remain manual debt.

Owner provisional: layout = stable monitor match + anchor + DIP offset + DIP size.

## Decision

1. **Coordinate model:** device-independent pixels (DIP) for size and offset; convert at the Win32 boundary using per-monitor DPI.
2. **Placement model:** each widget stores:
   - monitor match key (best-effort stable identity; fall back to primary / virtual coords when unknown),
   - anchor (e.g. work-area corner or center),
   - DIP offset from anchor,
   - DIP size.
3. **Persistence location:** under LocalAppData for the production app (not the spike path); human-readable settings may reference layout or layout may live in runtime store (see ADR-0005).
4. **Restore policy:** on startup, re-create widget HWNDs from saved layout; clamp onto available work areas if a monitor is missing.
5. **Do not** hard-code a single resolution (e.g. 2560×1440).

## Consequences

- `solpaper-core` owns pure layout math and types; `solpaper-windows` owns DPI and HWND placement.
- Multi-monitor edge cases stay on the manual evidence list until tested.
