# ADR-0005: Storage split (settings, runtime, secrets, cache, logs)

## Status

**Accepted** (Issue #16).

## Context

Product locks forbid secrets in source, config, SQLite, logs, issues, or PRs. Spike used JSON layout only—no credentials.

Owner provisional: settings versioned human-readable; runtime relational state may use SQLite when justified; secrets only in Windows Credential Manager; cache/logs under LocalAppData.

## Decision

| Kind | Store | Notes |
|------|--------|------|
| **Settings** | Versioned human-readable file(s) under LocalAppData (e.g. TOML/JSON) | User-editable; no secrets |
| **Runtime / relational state** | SQLite under LocalAppData **when justified** | Optional until Pomodoro/history needs it; not required in scaffold |
| **Secrets / OAuth tokens** | **Windows Credential Manager only** | Never in settings files, DB, logs, or git |
| **Image cache** | LocalAppData cache directory | Wallpaper subsystem later |
| **Logs** | LocalAppData logs directory | No Calendar private titles; no tokens |

Scaffold implements path helpers and a minimal settings schema version field only—no Credential Manager writes, no OAuth.

## Consequences

- `solpaper-storage` owns paths and settings load/save shape.
- Calendar OAuth research (#6) consumes Credential Manager policy, not config files.
- Migrations that could destroy user data are HIGH risk (governance).
