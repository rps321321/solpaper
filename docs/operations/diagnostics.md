# Observability, diagnostics, crash recovery, and supportability

**Issue:** [#40](https://github.com/rps321321/solpaper/issues/40)  
**Status:** initial design (policy + pure logic; production file logging lands with Runtime)  
**Pack source:** [`deterministic-execution-blueprint.md` § #40](../engineering/deterministic-execution-blueprint.md)  
**Related:** NFR PERF-LOG-*, PERF-REL-04 · [threat-model.md](../security/threat-model.md) AC-LOG-* · [#37](https://github.com/rps321321/solpaper/issues/37) privacy · [#20](https://github.com/rps321321/solpaper/issues/20) Alpha 1 · [#21](https://github.com/rps321321/solpaper/issues/21) Calendar · root issue templates

## Purpose

Make local failures diagnosable **without** a debugger, a Solpaper cloud service, or private Calendar/OAuth data leaving the machine by default. This document is the decision store for logging, diagnostics UI, support bundles, crash/safe-mode policy, and supportability requirements consumed by later issues.

## Authority

| Rule | Policy |
|------|--------|
| Pack defaults | Blueprint § #40 LOCKED/DEFAULT; this file + `solpaper-core::diagnostics` encode them |
| Numeric budgets | [`non-functional-requirements.md`](../engineering/non-functional-requirements.md) PERF-LOG-*, PERF-REL-04 |
| Privacy projection | Blueprint § #37 — same projection before UI, UIA, notifications, **logs**, export |
| Secrets location | ADR-0005 — never logs, settings, SQLite, bundles |
| Security log abuse cases | AC-LOG-01, AC-LOG-02 in threat model |
| Telemetry / remote crash | **Forbidden in v1** without explicit human approval (owner gate) |

Deviation requires new primary-source or repository evidence, an issue-linked rationale, and the applicable risk/human gate.

## Scope and non-goals

### In scope

- Structured event taxonomy, components, levels, correlation IDs.
- Log rotation, size, retention, and **field allowlist** redaction.
- User-visible health/status (Diagnostics UI fields and recovery actions).
- Privacy-safe diagnostic bundle (user-initiated, previewable).
- Crash markers, safe-mode recommendation, hang policy for a single process.
- Safe operational counters.
- Public bug/crash issue templates and reproduction guidance.
- Requirements mapping for #20, #21, #23, #24.

### Non-goals (explicit)

- No product telemetry and no remote crash upload in v1 (`TELEMETRY_ENABLED = false`, `REMOTE_CRASH_UPLOAD = false` in core).
- No automatic endless restart after crash; no separate watchdog process in v1.
- Not a full production `tracing` subscriber wiring (MEDIUM dependency unit when Runtime lands).
- Not legal certainty for support data under every jurisdiction; default is minimize + redact + user consent.
- Physical Windows matrix evidence remains MANUAL (MD-* register).

---

## Logging stack (when implementation begins)

| Decision | DEFAULT |
|----------|---------|
| Libraries | `tracing` + `tracing-subscriber` (justify in the MEDIUM dep unit that adds them) |
| Location | `%LOCALAPPDATA%\solpaper\logs\` (`AppPaths.logs`) |
| Format | Structured (JSON lines or equivalent key=value) + human-readable fallback only if needed for early Alpha |
| Default level | `INFO` in release; `DEBUG` available via explicit user/diagnostic toggle (not a silent global) |

Do not add a second logging family. Do not log to the desktop wallpaper image path.

---

## Components

Every structured event carries `component` from this set (stable tokens):

| Component | Token | Responsibility |
|-----------|-------|----------------|
| Runtime | `runtime` | Process lifetime, single-instance, startup/shutdown |
| Tray | `tray` | Notification area icon and menu |
| Surface | `surface` | Widget HWND create/destroy/z-order |
| Layout | `layout` | Geometry, monitor binding, off-screen clamp |
| Pomodoro | `pomodoro` | Timer domain transitions and projections |
| Wallpaper | `wallpaper` | Local/remote cycle, decode, apply |
| Calendar | `calendar` | Sync, cache, stale/offline |
| Auth | `auth` | OAuth connect/disconnect/reconnect |
| Storage | `storage` | Settings/layout/cache IO |
| Migration | `migration` | Schema forward migrations |
| Diagnostics | `diagnostics` | Bundle export, safe mode, health surface |

Implemented as `solpaper_core::Component`.

---

## Correlation IDs

Mint a fresh opaque ID at the start of each scope; attach it to all events for that operation:

| Scope | Token | When |
|-------|-------|------|
| Startup | `startup` | Process start → tray/surface ready or failed |
| Calendar sync | `calendar_sync` | Each sync attempt (poll or manual) |
| Wallpaper cycle | `wallpaper_cycle` | Each apply/rotate attempt |
| Migration | `migration` | Each schema migration run |

IDs are random/opaque (not derived from user content). Field name: `correlation_id` (or scope-specific aliases `startup_id`, `sync_id`, `wallpaper_cycle_id`, `migration_id` when clearer).

---

## Log levels

| Level | Use |
|-------|-----|
| ERROR | User-visible failure or data-loss risk; always has `error_category` + `error_code` |
| WARN | Degraded but continuing (stale cache, single monitor missing, backoff) |
| INFO | Lifecycle milestones (started, sync ok, wallpaper applied, shutdown) |
| DEBUG | Detailed flow for local diagnosis; still allowlisted fields only |
| TRACE | Not required for v1; if enabled, same allowlist |

---

## Structured field policy (redaction by construction)

**Allowlist only.** Unknown keys must not be written. Forbidden keys must not be written even if an implementer aliases them.

### Allowlisted fields

See `solpaper_core::ALLOWED_LOG_FIELDS` (authoritative list in code). Summary: timestamp, level, component, event, correlation IDs, error_category, error_code, os_error, http_status, duration_ms, counts/retries, safe_mode, schema/build/config versions, monitor/widget counts, phase/timer_status, host (not full URL), path_kind, redacted_path, short safe message.

### Forbidden fields (non-exhaustive; code list is closed)

Event title, description, location, attendees, OAuth URL/query/code/state/verifier, access/refresh tokens, credential target contents, full personal file paths, account email.

**Rule:** privacy projection (§ #37) runs **before** any log site that could touch Calendar titles. Prefer logging `error_code` only.

Unit tests: `validate_log_fields`, allowlist/forbidden disjointness, path redaction (`redact_user_path`).

---

## Rotation and retention

| Budget | Value | NFR |
|--------|-------|-----|
| Files | **5** | PERF-LOG-01 |
| Size per file | **2 MiB** | PERF-LOG-01 |
| Total size | **10 MiB** | PERF-LOG-01 |
| Age | **14 days** | PERF-LOG-02 |
| Bundle log tail | **≤ 512 KiB** | PERF-LOG-03 (design bound) |

Policy helpers: `needs_rotation_before_write`, `log_files_to_delete` (newest-first input). When over policy, drop oldest first; always drop files older than retention.

---

## Error categories

Stable categories so network failures are distinguishable from auth, parse, storage, and provider-policy failures (acceptance criterion):

| Category | Token | Examples |
|----------|-------|----------|
| Network | `network` | HTTP/TLS/timeout/offline |
| Auth | `auth` | OAuth mismatch, reconnect required |
| Parse | `parse` | JSON/schema |
| Storage | `storage` | IO, cache disk |
| Provider policy | `provider_policy` | too large, rate limit, redirect rejected |
| Surface | `surface` | HWND/tray host |
| Layout | `layout` | geometry/monitor |
| Pomodoro | `pomodoro` | timer host |
| Migration | `migration` | schema |
| Config | `config` | corrupt settings recovered |
| Internal | `internal` | panic path, unexpected |

Mapping helper: `categorize_error_code`. Specific codes remain those defined in security control matrices (`OAuthStateMismatch`, `CalendarHttp`, …).

---

## Diagnostics UI (requirements)

Settings → Diagnostics (or equivalent) must show:

| Field | Notes |
|-------|-------|
| Version | Marketing/semver from build |
| Source commit / build id | From build metadata |
| Config schema version | Settings/layout schema |
| Last startup | Timestamp + success/fail + correlation id |
| Last successful Calendar sync | Timestamp or “never” / “not connected” |
| Last wallpaper cycle | Timestamp + local/remote kind (no full path) |
| Active error codes | Codes + categories only |
| Safe mode | Yes/no + why |
| Data / cache / log locations | Display **redacted** paths; copy actions must warn |
| Recovery actions | Explicit buttons (below) |

### Recovery actions (minimum)

| Symptom (user language) | Documented path | Primary action |
|-------------------------|-----------------|----------------|
| Widget disappeared | Surface/layout health | Recreate surfaces; clamp off-screen; open Edit Mode |
| Tray missing | Runtime/single-instance | Restart app; check second-instance activation |
| Calendar not updating | Auth vs network vs parse | Show category; Reconnect if auth; retry if network; keep cache offline |
| Wallpaper stuck | Wallpaper cycle errors | Re-scan local folder; keep current on failure |
| App keeps crashing on start | Crash markers / safe mode | Launch safe mode; open Diagnostics; export bundle |
| Corrupt settings | PERF-STOR-02 | Load defaults; preserve corrupt file; show recovery banner |

Offline: all of the above work **without** any Solpaper cloud service.

---

## Diagnostic bundle

| Rule | DEFAULT |
|------|---------|
| Initiation | **User only** (no silent background upload) |
| Preview | Show manifest of included files/sizes **before** write |
| Format | Zip under user-chosen path |
| Log tail | Bounded (`BUNDLE_LOG_TAIL_MAX_BYTES`) |
| Settings | Redacted copy (no secrets; paths redacted) |
| System | OS build, arch, monitor count/geometry/DPI, memory summary (no username required) |
| Build | Version, git SHA, rustc/cargo if available, target |

### Default exclusions

Tokens, OAuth callback data, event titles/details, raw database/SQLite, screenshots, full personal paths, Credential Manager material. Helper: `is_forbidden_bundle_entry_name`.

### Include by default

`manifest.json`, redacted settings summary, active error summaries, bounded log tail, monitor topology summary, safe-mode flag, counter snapshot (below).

---

## Crash behavior

| Decision | DEFAULT |
|----------|---------|
| Panic hook | Write a **minimal redacted crash marker** (timestamp, build SHA, component if known, error_code `InternalPanic`) under LocalAppData; no stacks with paths/tokens |
| Auto-restart | **None** in v1 (`AUTO_RESTART_ON_CRASH = false`) |
| Safe mode | ≥ **3** startup crash markers within **5 minutes** → recommend/launch safe mode |
| Safe mode disables | Widgets, Calendar, remote provider, autostart mutation |
| Safe mode keeps | Settings, Diagnostics |
| Watchdog process | **None** in v1 |
| Hang policy | No multi-minute UI-thread hang design: network/disk off UI thread (NFR); no separate hang-killer process |

Core helpers: `should_recommend_safe_mode`, `SafeModePolicy::RESTRICTED`.

Next **user** or logon launch recovers from durable state (settings, layout, Pomodoro). Explorer restart only recreates tray registration (blueprint shutdown/recovery).

---

## Safe counters (no private payloads)

Maintain process-local (and optionally durable) counters for support:

| Counter | Meaning |
|---------|---------|
| `failed_syncs` | Calendar sync failures (by category optional) |
| `provider_cooldowns` | Remote provider cooldown entries |
| `duplicate_prevention` | Single-instance / duplicate tray suppressions |
| `migrations_run` | Successful migrations |
| `migrations_failed` | Failed migrations |
| `surface_recreates` | Surface recreation events |
| `safe_mode_entries` | Times safe mode engaged |
| `crash_markers` | Count of markers in current window (derived) |

Never attach event titles or tokens to counters.

---

## Hang / long-work policy

| Rule | DEFAULT |
|------|---------|
| UI thread | No network or large disk work |
| Timeouts | Align with NFR PERF-NET-* and external-input matrix |
| Detection | Optional future: “still working” UI for long local scans; not a watchdog kill |
| Beta soak | PERF-IDLE-06 / MD-PERF-03 — zero crash/hang in 8 h on reference env |

---

## Troubleshooting scenarios

### “My widget disappeared”

1. Diagnostics → active errors (`surface` / `layout`?).
2. Recovery → Recreate surfaces / clamp off-screen.
3. Edit Mode → confirm layout.
4. If after Explorer crash: restart Solpaper once; check single-instance.
5. Export bundle if still broken; attach to bug template (no screenshots of calendar).

### “Calendar not syncing”

1. Category `auth` → Reconnect (do not paste tokens).
2. Category `network` → check offline; wait backoff; last cache retained.
3. Category `parse` / `provider_policy` → report error_code only.
4. Never paste OAuth URLs or event titles into issues.

### “App crashes on startup”

1. If safe mode offered → accept; open Diagnostics.
2. Note crash marker count and build SHA.
3. Export bundle; file crash issue template.
4. Do not enable autostart mutation while diagnosing loops.

---

## Bug-report data guidance

Public GitHub templates (see `.github/ISSUE_TEMPLATE/`):

- **Bug report** — steps, expected/actual, build SHA, OS build, error codes.
- **Crash report** — safe mode?, marker window, redacted bundle optional.
- **Diagnostics / support** — non-sensitive counter and health fields.

**Never** in public issues: refresh/access tokens, OAuth callback URLs, Calendar event titles/descriptions, emails, full `C:\Users\<you>\...` paths, raw DB files.

Security vulnerabilities: root [`SECURITY.md`](../../SECURITY.md) only — not public issue templates.

---

## Requirements consumed by later issues

### #20 — Alpha 1

| Req ID | Requirement | Evidence intent |
|--------|-------------|-----------------|
| OPS-A1-01 | Startup correlation id; runtime/tray/surface lifecycle events allowlisted | Unit + code review |
| OPS-A1-02 | Log directory under AppPaths; rotation policy constants enforced when writer lands | Unit (policy) + later integration |
| OPS-A1-03 | Panic hook writes redacted crash marker; no auto-restart | Unit + manual smoke |
| OPS-A1-04 | Safe-mode policy gates widgets; settings/diagnostics remain | Unit policy + Alpha UI |
| OPS-A1-05 | Diagnostics UI baseline: version, build, last startup, errors, paths redacted, recovery | Manual Alpha |
| OPS-A1-06 | “Widget disappeared” recovery path documented and reachable | UX + manual |
| OPS-A1-07 | No telemetry / remote crash upload | Dependency + feature audit |

### #21 — Calendar Alpha 2

| Req ID | Requirement | Evidence intent |
|--------|-------------|-----------------|
| OPS-CAL-01 | Each sync has correlation id; failures categorized network/auth/parse/policy | Unit |
| OPS-CAL-02 | Never log titles/attendees/locations; projection before log | Redaction tests |
| OPS-CAL-03 | Last successful sync timestamp on Diagnostics | UI |
| OPS-CAL-04 | Counters for failed syncs without payloads | Unit |

### #23 — Remote wallpaper (if retained)

| Req ID | Requirement | Evidence intent |
|--------|-------------|-----------------|
| OPS-REM-01 | Wallpaper cycle correlation id; provider errors categorized | Unit |
| OPS-REM-02 | No full image URLs with secrets; host + status OK | Review + tests |
| OPS-REM-03 | Cooldown counter; keep current wallpaper on failure | Unit |

### #24 — v1 RC

| Req ID | Requirement | Evidence intent |
|--------|-------------|-----------------|
| OPS-RC-01 | PERF-LOG-* and PERF-REL-04 rows executed or waived | Evidence pack |
| OPS-RC-02 | Bundle preview + exclusion tests green | Automated + manual |
| OPS-RC-03 | Public templates present; SECURITY.md path for vulns | Repo audit |
| OPS-RC-04 | Beta soak no crash/hang (MD-PERF-03) | Manual |

---

## Acceptance criteria traceability

| Criterion | Where satisfied |
|-----------|-----------------|
| “Widget disappeared” has diagnostic path | Troubleshooting + Diagnostics UI recovery |
| Logs bounded; redact secrets/titles by automated test | Core unit tests + field allowlist |
| Repeated startup crashes do not uncontrolled-loop | No auto-restart + safe mode at 3/5 min |
| Network vs auth vs parse vs storage vs policy | `ErrorCategory` + `categorize_error_code` |
| Useful offline; no cloud | Local logs, markers, bundle, UI |

---

## Implementation seam

| Layer | Owns |
|-------|------|
| `solpaper-core::diagnostics` | Policy constants, allowlist, redaction, safe mode, rotation decisions |
| `solpaper-storage` | Paths for logs/markers; future atomic marker write |
| `solpaper-app` / Runtime | `tracing` subscriber, panic hook install, Diagnostics UI, bundle zip |
| Docs / templates | This file, troubleshooting, GitHub issue templates |

Production logging dependency addition remains a **MEDIUM** unit when Runtime wiring begins; this issue does **not** add `tracing` crates.

## Known limitations

- Marker persistence format and multi-version migration of crash files are left to the Runtime unit.
- Hang “detection” is design-level (thread affinity + timeouts), not a kill switch.
- Full physical evidence for crash loops and soak remains open manual debt (MD-PERF-03, PERF-REL-04).
