# Fixtures and mock-server plan

**Issue:** [#33](https://github.com/rps321321/solpaper/issues/33)  
**Status:** plan only — implement fixtures when the owning feature lands  
**Privacy:** synthetic data only; never real Calendar titles, tokens, or paths with secrets

## Principles

1. **Determinism first** — fixed clocks, fixed random sequences, fixed file trees.
2. **No network in unit tests** — HTTP only through `CalendarTransport` / provider traits backed by a local mock.
3. **Temporary directories** for storage tests; never write into the user’s real profile from tests.
4. **One concern per fixture pack** — small, composable JSON/dirs over mega-snapshots.
5. **Redact by construction** — fixture fields that mirror production log allowlists.

## Proposed repository layout (when code arrives)

```text
crates/
  solpaper-core/tests/          # or #[cfg(test)] modules colocated
  solpaper-storage/tests/fixtures/
  solpaper-app/tests/           # process-level only if needed
tests/                          # workspace-level integration (optional later)
  mocks/
    calendar/                   # recorded/synthetic Google-like responses
    provider/                   # optional remote wallpaper HTTP shapes
  fixtures/
    layouts/
    pomodoro/
    wallpaper-folders/
docs/testing/evidence/          # physical runs only — not unit fixtures
```

Do not commit large binary wallpaper corpora. Prefer tiny generated PNGs or solid-color images under a size cap when image decode tests exist.

## Domain fixture packs

### Pomodoro

| Fixture | Content |
|---------|---------|
| `pomodoro_fresh_defaults` | Default durations; Idle/Ready baseline per #19 design |
| `pomodoro_mid_focus` | Running focus with known deadline via `Clock` |
| `pomodoro_missed_one` | Process-down across exactly one completion boundary |
| `pomodoro_pause_resume` | Paused remaining duration |

All times are absolute instants injected through `Clock`, not `std::time::SystemTime` in assertions.

### Layout / monitors

| Fixture | Content |
|---------|---------|
| `layout_single_100` | One widget, one monitor 1920×1080 @ 100% |
| `layout_dual_mixed` | Two monitors 100%/150%; widget on each |
| `layout_offscreen` | Anchor that resolves outside work area after topo change |
| `monitors_hotplug_sequence` | Ordered snapshots for unplug/primary change |

Geometry is pure data for `MonitorEnumerator` fakes.

### Storage / config

| Fixture | Content |
|---------|---------|
| `config_valid_vN` | Current schema settings |
| `config_corrupt_truncated` | Truncated JSON/TOML for recovery tests |
| `config_unsupported_future` | Unknown version for migrate/fail path |
| `db_empty` / `db_migrated_vN` | SQLite files generated in temp dirs at test start when DB exists |

Prefer **generating** SQLite in the test over checking in binary DBs unless migrations need golden files.

### Calendar (synthetic)

| Fixture | Content |
|---------|---------|
| `events_ordinary` | Titled `Team sync`, `Focus block` |
| `events_private_class` | Items that must project to `Private` |
| `events_busy_only` | Busy-only mode inputs |
| `events_all_day_recurring` | All-day + RRULE-like expanded instances as static lists |
| `events_cancelled` | Cancelled instances excluded from agenda |
| `sync_token_valid` / `sync_token_gone` | 410-style recovery path |

### Wallpaper folders

| Fixture | Content |
|---------|---------|
| `wall_local_small` | 1–2 tiny valid images |
| `wall_local_oversized_meta` | Metadata claiming > policy limit (reject before full decode where possible) |
| `wall_empty_dir` | Empty folder behavior |

## Local mock server plan

### When required

- Calendar HTTP integration (Alpha 2).
- Optional remote wallpaper provider (only if #22 retains a provider).

### Design defaults

| Topic | Default |
|-------|---------|
| Bind address | `127.0.0.1` only |
| Port | ephemeral, passed into `CalendarTransport` base URL |
| TLS in unit tests | not required for loopback mock; production remains HTTPS-only |
| Body size | enforce same caps as production policy tests |
| Concurrency | one mock per test process or mutexed port map |
| Lifetime | started in test setup, dropped on teardown |

### Scripted behaviors

| Script | Response |
|--------|----------|
| `oauth_token_ok` | Token endpoint success with **fake** tokens (never real) |
| `oauth_token_invalid_grant` | Revoked refresh |
| `calendar_list_ok` | Calendar list JSON |
| `events_incremental_ok` | Events + next sync token |
| `events_gone` | Sync token invalid |
| `rate_limited` | 429 with retry hints |
| `server_error` | 500/503 for backoff tests |
| `truncation` | Partial body / disconnect mid-stream |
| `redirect_http` | Ensure client rejects non-HTTPS or private redirect targets per #36 when client exists |

Implementation language: prefer **in-process** mock (e.g. hyper/axum or a tiny custom listener) inside the Rust test binary to avoid extra services. A separate Python/Node mock is a last resort and would need a supply-chain review (#38).

### Credential store fake

In-memory map with:

- `get` / `set` / `delete`
- injectable `Error::Unavailable` / `Error::AccessDenied`
- no persistence; purge tests assert empty map

Production Windows Credential Manager is exercised only in controlled manual/release tests, never in CI with real secrets.

### Desktop wallpaper fake

Records `set_wallpaper` calls; can fail once then succeed; exposes “current wallpaper” string for preservation assertions.

## CI integration

- Mock servers must not require admin rights or open external ports.
- Tests that bind sockets skip or fail clearly if bind is impossible (document `#[ignore]` only under flaky policy with an issue).
- Default `cargo test --workspace` remains offline with respect to the public internet.

## What this PR does not implement

- Actual mock crates or fixture files under `crates/**/tests`.
- HTTP client production code.
- Binary image corpora.

Those land with the feature issues (#20, #21, #23, storage work) consuming this plan.
