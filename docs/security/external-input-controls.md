# Externally controlled inputs — control matrix

**Issue:** [#36](https://github.com/rps321321/solpaper/issues/36)  
**Rule:** every externally controlled input documents **maximum size**, **parser/validation**, **timeout**, **error category**, **retry policy**, **log policy**, and **user-facing recovery**.

Numeric HTTP/image budgets align with [`non-functional-requirements.md`](../engineering/non-functional-requirements.md) and blueprint § #36 / #6 / #21. Tighten only with evidence; weaken Hard gates only with human approval.

## Legend

| Field | Meaning |
|-------|---------|
| Max size | Hard upper bound before reject/abort |
| Validation | Parser and semantic checks |
| Timeout | Wall-clock or I/O deadline |
| Error category | Stable typed category (not private payload) |
| Retry | Whether automatic retry is allowed |
| Log policy | What may appear in logs/diagnostics |
| Recovery | User-visible or automatic recovery |

---

## OAuth / identity

| Input | Max size | Validation | Timeout | Error category | Retry | Log policy | Recovery |
|-------|----------|------------|---------|----------------|-------|------------|----------|
| Loopback HTTP request line + headers | **8 KiB** total | Path must be `/oauth/callback`; method GET; first valid only | **120 s** overall connect wait | `OAuthCallbackInvalid`, `OAuthTimeout`, `OAuthPortBind` | No auto-retry of full connect without user action | **No** URL, query, code, state, verifier | Settings: cancel/reconnect; clear in-memory PKCE material |
| OAuth `state` query param | 32 bytes decoded (base64url form bounded by header limit) | Constant-time compare to session state | Within callback window | `OAuthStateMismatch` | No | Log mismatch **code only**, never values | User restarts Connect |
| OAuth `code` query param | Bounded by header limit | Non-empty; exchanged once | Token HTTP timeouts below | `OAuthExchangeFailed` | No silent re-use of code | Never log code | Reconnect |
| PKCE verifier | 32 random bytes → base64url | Generated locally only | Session lifetime ≤ 120 s | N/A internal | N/A | Never log | Drop on cancel/timeout |
| Token endpoint response JSON | **1 MiB** body (implementation default; must be bounded) | HTTPS; parse required fields; require refresh token on first connect | Connect **10 s**, total **30 s** (PERF-NET-01/02) | `OAuthExchangeFailed`, `OAuthMissingRefresh` | No tight loop | Status/error category only | `ReconnectRequired` |
| Google token revoke | Same HTTP bounds | Best-effort on disconnect | 10 s / 30 s | `OAuthRevokeFailed` (non-fatal) | Optional single retry max | Category only | Local credential delete **regardless** |

---

## Google Calendar API

| Input | Max size | Validation | Timeout | Error category | Retry | Log policy | Recovery |
|-------|----------|------------|---------|----------------|-------|------------|----------|
| `calendarList.list` page JSON | **5 MiB** per page (cap) | HTTPS; schema subset; `showHidden=false` | 10 s / 30 s | `CalendarHttp`, `CalendarParse` | Backoff 1,2,5,15 min cap 15; reset on success | No calendar names beyond allowlist fields if any; prefer IDs/codes | Keep last good list if any; isolate failure |
| Events list / incremental page | **5 MiB** per page | `singleEvents`, `showDeleted`, syncToken rules per #21 | 10 s / 30 s | `CalendarHttp`, `CalendarParse`, `CalendarSync410` | Same backoff; **410** → clear that calendar store + full resync | No raw titles | Stale after 30 min; keep committed cache |
| Stored instances per calendar | **50,000** | Cap before commit | N/A | `CALENDAR_TOO_LARGE` | No | Code only | Stop that calendar sync; other subsystems live |
| Event title/description/location | As provided by API up to page bound | Normalize to `AgendaItem`; privacy project **before** UI/UIA/log | N/A | N/A | N/A | **Never** raw private titles; projected or omit | Busy/`Private` modes |
| `nextSyncToken` / pageToken | Opaque string ≤ **4 KiB** | Store only after full successful page set | N/A | `CalendarSyncIncomplete` if partial | Do not expose partial as committed | Token not logged | Retry full page sequence |

---

## Remote wallpaper / provider HTTP (if retained)

| Input | Max size | Validation | Timeout | Error category | Retry | Log policy | Recovery |
|-------|----------|------------|---------|----------------|-------|------------|----------|
| Provider metadata JSON | **1 MiB** | HTTPS only; schema allowlist | 10 s / 30 s | `ProviderHttp`, `ProviderParse` | Bounded backoff; no infinite | No full URLs with secrets; host + status OK | Keep current wallpaper; local disable |
| Redirect chain | **3** redirects max | Each hop HTTPS; resolve host; **reject** loopback, private, link-local | Counted in total timeout | `ProviderRedirectRejected` | No follow beyond policy | Log reject category + hop count | Fail closed |
| Image download body | **30 MiB** compressed (PERF-WALL-02) | Content-type sanity; bounded reader | 10 s connect; overall download deadline **120 s** | `ProviderDownloadTooLarge`, `ProviderTimeout` | Limited; never on size reject | Bytes read / category | Abort; keep current |
| Cache filename | Generated ID/hash only | Never raw URL path as filename | N/A | `CacheIo` | Optional IO retry once | ID only, not user home path | Skip file |

---

## Local wallpaper / filesystem

| Input | Max size | Validation | Timeout | Error category | Retry | Log policy | Recovery |
|-------|----------|------------|---------|----------------|-------|------------|----------|
| User-selected folder path | OS path max | Canonicalize; must be directory; user-owned selection UI | Enumerate soft timeout **30 s** recommended | `WallpaperPathInvalid` | No | Directory name/hash or relative label—not full profile path by default | Prompt re-select |
| Image file (local) | **50 MiB** compressed (PERF-WALL-01) | Extension allowlist; canonicalize file under selected roots | Decode budget (impl) | `WallpaperFileTooLarge`, `WallpaperDecode` | No loop | Filename stem or ID | Skip file; keep current applied |
| Decoded pixels | **100 megapixels** (PERF-WALL-03) | Pre-check when decoder allows | N/A | `WallpaperDecode` | No | Dimensions + category | Keep current wallpaper |
| Settings / layout file | **1 MiB** recommended schema bound | Version field; schema validate | IO OS default | `SettingsCorrupt`, `SettingsIo` | No auto-migrate destructive | Schema version, error code | Preserve corrupt timestamped; load defaults; diagnostics recovery |

---

## Win32 / shell / notifications

| Input | Max size | Validation | Timeout | Error category | Retry | Log policy | Recovery |
|-------|----------|------------|---------|----------------|-------|------------|----------|
| HRESULT / COM failures | N/A | Map to typed errors; no panic on expected fail | API-specific | `Win32*`, `Com*` | Policy per call (wallpaper apply: no loop) | HRESULT + API name; no buffer dumps | Keep prior wallpaper/surface state |
| Monitor topology events | OS-defined | Clamp layout off-screen per ADR-0004 | N/A | `MonitorTopology` | N/A | Monitor count/IDs as available | Recompute layout |
| Notification strings | Short UI strings | Privacy-projected content only | N/A | N/A | Dedupe one completion notice | No private titles | Suppress duplicate |

---

## Logs and diagnostic bundle

| Input | Max size | Validation | Timeout | Error category | Retry | Log policy | Recovery |
|-------|----------|------------|---------|----------------|-------|------------|----------|
| Log fields | Five × **2 MiB**; 14-day (#40) | **Allowlist** only | N/A | N/A | N/A | Exclude: event title/description/location/attendee; OAuth URL/query/code/state/verifier/token; credential target contents; full personal paths | Rotation |
| Diagnostic zip | Bounded (impl target ≤ few MiB) | User-initiated; preview manifest | User cancel | `DiagnosticsIo` | No | Same exclusions + no raw DB/screenshots | User re-run with fewer options |

### Default log allowlist (positive)

Allowed examples: build version, commit, error **category/code**, correlation IDs, durations, HTTP status, retry count, calendar **count**, monitor count, wallpaper apply **success/fail code**, schema version.

Denied by default: anything in the exclusion list above. Redaction is by **not accepting** fields into the structured logger—not regex scrubbing alone.

---

## Activation / IPC (v1)

| Input | Policy |
|-------|--------|
| Named pipes, localhost control ports, multi-process RPC | **Not accepted in v1.** Any introduction is HIGH/CRITICAL scope and requires ADR + threat-model update before code. |

---

## Implementation test expectations

| Control family | Minimum automated coverage |
|----------------|----------------------------|
| OAuth callback | Fake server: success, state mismatch, path mismatch, denial, timeout, oversized, port-bind fail, exchange fail, missing refresh |
| Credential store | In-memory fake for domain tests; optional CM smoke with test-only target + cleanup |
| Redaction | Tests scan log/diagnostic output for token/code/state/verifier/callback query patterns used in fixtures |
| HTTP bounds | Mock oversized body, redirect-to-private, too many redirects |
| Image bounds | Fixture or mock exceeding compressed/pixel limits rejects without applying |
| Privacy | Busy/Private never appears as real title in UIA/log/notification fakes |

Do not store real OAuth secrets or private Calendar data in fixtures, issues, or evidence.
