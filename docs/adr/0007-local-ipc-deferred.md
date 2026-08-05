# ADR-0007: Local IPC deferred

## Status

**Accepted** (Issue #16).

## Context

No second local client exists (no TUI v1, no companion app). Single process model (ADR-0002) covers tray, surfaces, and settings.

## Decision

- **Do not** introduce a local IPC protocol, named pipe server, or multi-process agent split in the scaffold or Alpha 1 unless a real second client is approved on the roadmap.
- Single-instance enforcement uses OS primitives (mutex), not an IPC service API.

## Consequences

- Simpler security surface (no localhost protocol to auth).
- Future TUI or remote-control would require a new ADR and threat-model update (#36).
