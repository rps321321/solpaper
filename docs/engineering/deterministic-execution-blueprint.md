# Deterministic execution blueprint

**Issue:** #55  
**Status:** proposed execution defaults  
**Applies to:** all remaining Solpaper roadmap work  
**Last researched:** 2026-08-05

This file removes avoidable design choice from autonomous implementation. It is not permission to bypass governance, accepted ADRs, tests, physical evidence, external policy, or human-only gates.

## Authority and interpretation

Use this order when instructions disagree:

1. `docs/engineering/agent-governance.md` and the kill switch.
2. Accepted ADRs under `docs/adr/`.
3. GitHub Issue #1 for product scope/order and Issue #30 for engineering gates.
4. The originating issue and its acceptance criteria.
5. This blueprint's execution defaults.
6. Agent preference.

An agent must follow a selected default below unless new primary-source evidence or repository behavior proves it unsafe or impossible. In that case, stop the unit, record the contradiction, and route through `solpaper-research` or `solpaper-domain-design`; do not silently choose another architecture.

### Decision labels

- **LOCKED:** already fixed by product, governance, or accepted ADR.
- **DEFAULT:** implement as written unless evidence contradicts it.
- **RECOMMENDATION — OWNER GATE:** do not change roadmap scope until the owner accepts it.
- **MANUAL:** evidence or action cannot be fabricated or safely automated.
- **EXTERNAL:** depends on an account, repository setting, signing service, provider term, or participant.

## Required execution order

Finish or merge the existing post-#32 state PR before starting a new implementation lease. Then use this sequence:

1. **#55** — merge this blueprint after validation.
2. **Foundation gates:** #33 → #41 → #34 → #35 → #36 → #38 → #40.
3. **Alpha 1 decisions/components:** #5 → #7 → #19.
4. **Acceptance matrix first usable revision:** #13.
5. **Alpha 1:** #20.
6. **Calendar preparation:** #6 → #37 → Google section of #42.
7. **Calendar Alpha 2:** #21.
8. **Public-scope validation:** #43.
9. **Remote provider decision:** #22. Do not start #23 until the owner explicitly retains a remote provider.
10. **Release system:** #39 → remaining #42 → #45.
11. **External validation:** #44.
12. **Release candidate:** #24, with #13 as the authoritative evidence matrix.

One autonomous firing still performs one bounded issue or coherent subtask. The sequence is a dependency order, not permission to merge multiple issues together.

---

# Issue execution packs

## #5 — IDesktopWallpaper adapter

### Locked/default decisions

- **LOCKED:** wallpaper is a peer subsystem and uses local files first.
- Use the existing `windows` crate; do not add a second Windows binding.
- Initialize COM on the UI thread as STA with `CoInitializeEx(..., COINIT_APARTMENTTHREADED)` and release it with normal RAII ownership.
- Activate `IDesktopWallpaper` once for the runtime; recreate the COM object once after a server-disconnected/transient COM failure, then surface the error.
- Adapter interface:

```rust
pub trait DesktopWallpaper {
    fn monitors(&self) -> Result<Vec<WallpaperMonitor>, WallpaperError>;
    fn current(&self, monitor: &WallpaperMonitorId) -> Result<Option<PathBuf>, WallpaperError>;
    fn apply(&self, monitor: &WallpaperMonitorId, owned_file: &Path) -> Result<(), WallpaperError>;
    fn position(&self) -> Result<WallpaperPosition, WallpaperError>;
    fn set_position(&self, position: WallpaperPosition) -> Result<(), WallpaperError>;
}
```

- Use `GetMonitorDevicePathAt` identifiers for `IDesktopWallpaper` calls. Use `GetMonitorRECT` to distinguish attached from detached entries.
- Persist a separate best-effort monitor fingerprint: normalized monitor device path first; EDID manufacturer/product + friendly name + connector second; geometry/orientation fallback last.
- Wallpaper position is global, not per monitor. Default to **Fill**.
- When per-monitor crop differs, pre-render a monitor-sized image into Solpaper's cache; do not pretend Windows exposes per-monitor positioning.
- Accept local `.jpg`, `.jpeg`, `.png`, and `.bmp` initially.
- Canonicalize the source path, decode under the limits in #35, render/copy into a Solpaper-owned cache file, then call `SetWallpaper` with the full owned path.
- Pin every file currently applied to a monitor. Cache cleanup must never delete a pinned file.
- Invalid file/path/decode/HRESULT: keep the existing wallpaper and return one typed error. No retry loop.

### Tests/evidence

- Fake adapter contract: enumerate/query/apply/error mapping.
- Windows integration smoke on one monitor using a generated solid-color image, then restore the original path.
- **MANUAL:** two distinct images on two monitors, detach/reconnect, monitor rename/identity, and global-position behavior.
- Verify no overlay/window implementation detail leaks into the wallpaper monitor interface.

### Non-goals

No remote provider, scheduler, cache eviction policy, image recommendation, or universal resolution filter.

---

## #6 — Google OAuth and credential storage

### OAuth flow

- **LOCKED:** desktop installed-app OAuth, read-only, system browser, no Solpaper cloud.
- Create a Google OAuth client of type **Desktop app**.
- Bind `TcpListener` to `127.0.0.1:0` before opening the browser. Callback path: `/oauth/callback`.
- Launch the system browser through the Windows shell; never embed a webview for login.
- Authorization code + PKCE S256:
  - verifier: 32 cryptographically random bytes, base64url without padding;
  - state: independent 32 random bytes;
  - callback timeout: 120 seconds;
  - maximum request line + headers: 8 KiB;
  - accept only the first valid callback with exact state and expected path;
  - never log callback URL, query, code, verifier, state, access token, or refresh token.
- Request exactly:
  - `https://www.googleapis.com/auth/calendar.calendarlist.readonly`
  - `https://www.googleapis.com/auth/calendar.events.readonly`
- Use `access_type=offline`; use `prompt=consent` only when a refresh token is absent during first connect/reconnect.
- Support one Google account in v1.

### Credential interface and storage

```rust
pub trait CredentialStore {
    fn load_refresh_token(&self, account: &AccountKey) -> Result<Option<SecretString>, CredentialError>;
    fn save_refresh_token(&self, account: &AccountKey, token: &SecretString) -> Result<(), CredentialError>;
    fn delete_refresh_token(&self, account: &AccountKey) -> Result<(), CredentialError>;
}
```

- Windows Credential Manager target: `Solpaper/GoogleCalendar/v1/default`.
- Store refresh token only. Access tokens remain in memory and are discarded on exit.
- OAuth client ID may be configuration/build metadata; a desktop-app client secret is not treated as a protected secret.
- Disconnect: best-effort Google token revocation, then local credential deletion regardless of remote result.
- Missing, malformed, revoked, or `invalid_grant` refresh credentials transition to `ReconnectRequired`; no uncontrolled refresh loop.

### Google distribution limits

- Development/testing may use explicitly listed test users.
- Document that external Testing mode can impose user and refresh-token lifetime limits and is not a public-release configuration.
- Public Calendar distribution requires the production consent-screen/policy/verification work in #42 and a published privacy policy.

### Tests

- Local fake OAuth server for success, state mismatch, callback path mismatch, denial, timeout, oversized request, port-bind failure, exchange failure, and missing refresh token.
- In-memory fake CredentialStore for all domain tests; a Windows Credential Manager integration smoke stores a synthetic token under a test-only target and removes it in cleanup.
- Redaction tests scan logs/diagnostic output for token, code, state, verifier, and callback query.

### Human/external gates

Google Cloud project/client creation, consent-screen details, test users, and public verification are **EXTERNAL/HUMAN**. No agent may place real credentials in GitHub or prompts.

---

## #7 — tray runtime, autostart, and single instance

### Runtime

- **LOCKED:** one long-running user-session process; Win32 UI thread owns HWNDs; no general IPC in v1.
- Retain the named mutex for exclusivity.
- Add a hidden/message-only control window with class `Solpaper.Runtime.Control.v1`.
- Second launch behavior: mutex indicates an existing instance → find control window → post `WM_APP_SHOW_SETTINGS` → exit 0. This is a narrow activation signal, not a general command protocol.
- Native tray with `Shell_NotifyIconW`, stable GUID identity, `NIM_ADD`, then `NIM_SETVERSION`.
- Register the `TaskbarCreated` message and re-add the icon after Explorer/taskbar recreation.
- Settings windows are in-process and created lazily.
- Alpha 1 background work: one standard worker thread and a typed channel; completion returns to UI by `PostMessage`. Add an async runtime only when Calendar networking requires it, on a background thread only.

### Tray menu, fixed order

1. Open Settings
2. Edit Mode toggle
3. Separator
4. Pomodoro Start/Pause/Resume
5. Pomodoro Skip
6. Pomodoro Reset
7. Separator
8. Wallpaper Next
9. Wallpaper Hold
10. Separator
11. Start with Windows
12. Diagnostics
13. Quit

Unavailable feature actions are disabled, not hidden.

### Autostart

- Disabled by default.
- Installed build only; portable builds do not expose autostart.
- Use current-user Run key: `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`.
- Value name: `Solpaper`.
- Value: quoted absolute installed executable plus `--background`.
- Toggle-off and uninstall remove only Solpaper's value.
- Do not use Task Scheduler, a Windows service, or machine-wide registration for v1.

### Shutdown and recovery

- Stop accepting new work; stop timers; atomically flush settings/runtime state; ask worker to stop; wait at most 2 seconds; remove tray icon; destroy windows; release mutex.
- No automatic crash restart in v1. The next user/logon launch recovers from durable state.
- Explorer restart only recreates tray registration; top-level widget windows are not Explorer children.

### Tests/evidence

- Process test: second launch exits and activates settings without a duplicate tray/surface.
- Registry adapter fake plus test-key integration cleanup.
- Tray command routing unit tests.
- **MANUAL:** Explorer restart, logon autostart, Task Manager startup listing, uninstall/toggle cleanup.

---

## #19 — Pomodoro state machine

### Defaults

- Focus: 25 minutes.
- Short break: 5 minutes.
- Long break: 15 minutes.
- Long break after every 4 **completed** focus sessions.
- Auto-start next phase: OFF.
- Notifications: ON.
- Sound: ON.
- Skipped focus does not increment completion count.
- Alpha 1 history: OFF; do not add analytics or SQLite history.

Allowed configuration ranges:

- focus 1–180 minutes;
- short break 1–60 minutes;
- long break 1–120 minutes;
- cadence 2–12 completed focus sessions.

### Domain model

Use explicit phase and run status rather than a single ambiguous enum:

```rust
Phase = Focus | ShortBreak | LongBreak
RunStatus = Idle | Running { deadline_utc, phase_instance_id } | Paused { remaining, phase_instance_id }
```

Persist: phase, status/deadline or remaining duration, duration snapshot, completed focus count within cycle, auto-start preference, last transition UTC, phase instance ID, last-notified phase instance ID.

### Command rules

- `Start`: Idle → Running with new phase instance.
- `Pause`: Running → Paused with remaining duration.
- `Resume`: Paused → Running with `now + remaining`.
- `Skip`: current phase outcome `skipped`; focus skip does not increment; choose the next phase; next phase is Idle unless auto-start applies to an ordinary live skip.
- `Reset`: current phase becomes Idle at full configured duration; preserve completed-focus count.
- Live completion: completed focus increments count and chooses short/long break; completed break chooses focus.
- Recovered expired completion: complete at most one phase and leave the next phase Idle, even when auto-start is enabled.

### Time/recovery

- While process is alive, use a monotonic clock for displayed remaining time and UTC for durable deadline/recovery.
- Persist every semantic transition, not every one-second tick.
- Restart/resume before deadline continues the same phase.
- Restart/resume after deadline completes at most one phase; never replay multiple missed cycles.
- Detect live UTC/monotonic divergence greater than 2 minutes; log a redacted clock-adjustment event and continue using monotonic elapsed time until the next durable transition.
- Notification uses the tray balloon/system notification route selected by #7; dedupe by phase instance ID.

### Tests

Table-test every command from every state, exact-deadline boundary, restart before/after deadline, large sleep-like jump, clock forward/back, skip cadence, reset semantics, auto-start live vs recovered, and duplicate notification suppression. Use an injected `Clock` and `NotificationSink`.

---

## #34 — UX and interaction defaults

### First run

One simple settings/onboarding flow:

1. Local-first/privacy statement.
2. Create Pomodoro widget by default.
3. Optional local wallpaper-folder selection.
4. Finish and show tray guidance.

Calendar remains an optional later connection and is not requested on first Alpha 1 launch.

### Normal Mode

- Widget surfaces are read-only and click-through.
- No direct overlay buttons in Alpha 1.
- Actions are available through tray, settings, and keyboard paths.
- This prevents accidental desktop blocking and avoids inaccessible custom action controls.

### Edit Mode

- Enter/exit through tray; default shortcut `Ctrl+Alt+F2`.
- Selected widget shows a clear border, 24-DIP drag strip, and 12-DIP resize grip.
- Escape exits Edit Mode.
- Keyboard:
  - Arrow: move 1 DIP.
  - Shift+Arrow: move 10 DIP.
  - Ctrl+Arrow: resize 1 DIP.
  - Ctrl+Shift+Arrow: resize 10 DIP.
- Clamp widgets so at least 48×48 DIP remains visible on an available work area.
- Hide/delete through accessible settings with confirmation; include Reset Layout.

### Settings navigation

General → Widgets → Pomodoro → Wallpaper → Calendar → Diagnostics/About.

- Use standard native controls for the initial settings UI.
- Use system colors and high-contrast behavior; no theme editor and no undocumented dark-mode API.
- Every error state has one clear primary recovery action.
- Animation is optional and absent by default.

### Required usability script

A new user must, without source docs: locate the tray, enter Edit Mode, move a widget, start/pause/reset focus, select a local folder, recover an off-screen widget, open diagnostics, and quit.

---

## #41 — accessibility requirements

- Target Windows desktop accessibility using UI Automation and the Microsoft accessibility checklist; WCAG 2.2 AA is the content/visual target where applicable.
- Settings uses standard Win32 controls so built-in UIA providers are available.
- Overlay Normal Mode remains read-only/click-through; all core actions have tray/settings/keyboard equivalents.
- Custom overlay UIA provider exposes a minimal fragment:
  - control type Pane/Group;
  - accessible name = widget type;
  - value/help text = current projected visible status;
  - no hidden Calendar title or private detail.
- Interactive Edit Mode keyboard behavior is the #34 map.
- Contrast: 4.5:1 normal text, 3:1 large text and essential non-text UI; never encode meaning by color alone.
- Support high contrast and Windows text scaling at 100%, 150%, and 200%; clamp layout after scale changes.
- Notifications must include text; sound/color are supplemental.
- Test with Inspect, Accessibility Insights/AccChecker where available, Narrator, keyboard only, high contrast, and scaling.
- **MANUAL:** screen-reader usability and assistive-technology review before stable v1.
- Busy-only/private projection must be tested against both rendered text and the UI Automation tree.

---

## #33 — test strategy and Windows evidence

### Test layers

1. Pure core unit tests.
2. Storage integration tests with temporary directories and transactional fixtures.
3. Owned adapter contract tests with fakes.
4. HTTP integration tests with a local mock server.
5. Win32 smoke/system tests that are safe and deterministic.
6. Named physical Windows evidence.
7. Install/upgrade/rollback/uninstall release tests.

### Required injectable seams

`Clock`, `RandomSource`, `CredentialStore`, `CalendarTransport`, `DesktopWallpaper`, `MonitorEnumerator`, `NotificationSink`, and owned filesystem/path services where atomic failure must be simulated.

### Evidence layout

```text
docs/testing/evidence/<issue>/<yyyy-mm-dd>/<environment>/
├── manifest.json
├── commands.txt
├── results.md
├── logs/
└── screenshots/
```

`manifest.json` records source SHA, Windows edition/build, CPU/GPU, monitor geometry/DPI, Rust version, build profile, commands, operator, timestamps, and redaction confirmation.

Keep a manual-debt register with: ID, scenario, issue, environment required, owner/operator, blocking release, status, evidence path, and expiry/retest trigger. Autonomous merges may add debt but may not delete it without linked evidence.

### Windows matrix

- Windows 11 24H2 x64 while supported.
- Windows 11 25H2 x64; owner reference environment.
- Windows 11 26H1 x64 on appropriate hardware when available.
- Single monitor 100%.
- Single monitor 150%.
- Dual monitor 100%/150% mixed DPI.
- Portrait secondary.
- Disconnect/reconnect, reorder, and primary change.
- Explorer restart, Win+D, fullscreen, lock/unlock, sleep/resume, prolonged idle.

Do not run disruptive physical tests while the owner is studying. CI is a compile/test gate, not proof of shell or hardware behavior.

### Flaky policy

No rerun-until-green. Quarantine requires an issue, owner, reason, failure rate, expiry date, and a nonblocking classification. A regression test is required for every fixed reproducible defect unless the missing seam is explicitly recorded.

---

## #35 — non-functional budgets

Initial targets are release-build measurements on named hardware. Tighten only with evidence; weakening a hard release blocker requires recorded approval.

| Area | Initial target |
|---|---|
| Supported OS | Windows 11 x64 24H2, 25H2, 26H1 while Microsoft-supported; baseline build 26100 |
| Unsupported | Windows 10, ARM64, Server, Wine/ReactOS |
| Cold start to tray/surface | p95 ≤ 1.5 s |
| Warm settings open | ≤ 250 ms |
| Cold settings open | ≤ 750 ms |
| Idle CPU, 60 s | median ≤ 0.5%, p95 ≤ 1% |
| Idle working set Alpha 1 | ≤ 60 MiB |
| Idle working set with Calendar | ≤ 100 MiB |
| Idle process handles | ≤ 500 |
| Shutdown/state flush | ≤ 2 s |
| Live timer visible error | ≤ 250 ms |
| Restart/resume deadline recovery | ≤ 2 s after runtime ready |
| Calendar poll | 15 min |
| Calendar stale indicator | after 30 min without successful sync |
| HTTP connect / total timeout | 10 s / 30 s |
| Network backoff | 1, 2, 5, 15 min; cap 15 min |
| Local wallpaper file | ≤ 50 MiB |
| Remote download, if retained | ≤ 30 MiB |
| Decoded image | ≤ 100 megapixels |
| Remote wallpaper cache, if retained | 1 GiB default |
| Logs | five 2-MiB files; 10 MiB total; 14-day cap |
| Duplicate tray/window/notification | zero in acceptance runs |
| Beta soak | no crash/hang in 8 hours on reference environment |

Atomic settings write: write same-directory temporary file → flush → replace target → retain one previous `.bak`. Corrupt config is preserved with timestamped name; safe defaults load and diagnostics show recovery.

---

## #36 — threat model and security architecture

### Trust boundaries/assets

Model: OAuth callback/browser, Google API, optional provider API, remote image decode, local user/filesystem, Credential Manager, installer/release, unsafe Win32/COM, settings/runtime database, logs/diagnostics, and any future activation/IPC channel.

### Required controls

- OAuth: loopback `127.0.0.1` only, pre-bound ephemeral port, PKCE S256, random state, first-valid callback, 8-KiB header limit, 120-s timeout, no callback query logging.
- HTTP: HTTPS only; maximum 3 redirects; redirect target must remain HTTPS and not resolve to loopback/private/link-local ranges for remote content; bounded response and download sizes; bounded retries.
- Paths: canonicalize user-selected local files; never concatenate provider/user strings into executable paths; cache filenames are generated IDs/hashes.
- Images: enforce compressed-size and decoded-pixel limits before full allocation where decoder permits; decode failure leaves current wallpaper unchanged.
- Credentials/private titles: allowlist log fields; redact by construction, not regex only.
- Win32/COM: narrow `unsafe` functions with `# Safety` documentation, owned lifetime types, HRESULT mapping, and thread-affinity assertions.
- No updater and no general local IPC in v1.
- Security-sensitive additions are HIGH risk under governance.

Every externally controlled input must document maximum size, parser/validation, timeout, error category, retry policy, log policy, and user-facing recovery.

---

## #38 — supply-chain, dependency, license, SBOM, provenance

- Commit `Cargo.lock`; CI/release uses `--locked`.
- Retain the workspace MSRV until a deliberate dependency/toolchain PR changes it.
- New runtime dependency requires PR justification: need, alternatives, maintenance/ownership, license, unsafe/native code, default features, transitive cost, and removal boundary.
- Respect governance's one-dependency-per-unit default; prefer zero.
- Add/require:
  - `cargo audit` against RustSec;
  - `cargo deny check advisories bans licenses sources`;
  - GitHub dependency review where plan/repository capability supports it;
  - CycloneDX JSON SBOM through pinned `cargo-cyclonedx` CLI;
  - optional `cargo auditable` for release binaries when compatible.
- Pin third-party GitHub Actions to full commit SHAs.

### License policy

Allowed by default: MIT, Apache-2.0, Apache-2.0 WITH LLVM-exception, BSD-2-Clause, BSD-3-Clause, ISC, Unicode-3.0, Zlib, CC0-1.0, BSL-1.0.

Human review/exception: MPL-2.0 and OFL-1.1. Denied by default: GPL, AGPL, LGPL, SSPL, unknown, unlicensed, and proprietary dependencies/assets.

Only crates.io sources by default. Git dependencies require an immutable commit, separate justification, license evidence, and a plan to return to a release.

Critical advisory blocks release; high advisory blocks release unless a human records an issue-bound, expiring waiver. Every ignored advisory has reason, owner, issue, and expiry.

Release manifest records source SHA, rustc/cargo version, target, Cargo.lock SHA-256, artifact SHA-256, SBOM SHA-256, build workflow/run, and signing state. Signing secrets never enter GitHub, CI logs, agent prompts, or repository files.

---

## #40 — logging, diagnostics, crash recovery, supportability

- Use `tracing` + `tracing-subscriber` when logging implementation begins; this dependency addition is MEDIUM and must be justified in its own unit.
- Components: runtime, tray, surface, layout, pomodoro, wallpaper, calendar, auth, storage, migration, diagnostics.
- Correlation IDs for each startup, Calendar sync, wallpaper cycle, and migration.
- Stable typed error codes; logs include category/code, not private payload.
- Default log allowlist excludes event title, description, location, attendee, OAuth URL/query/code/state/verifier/token, credential target contents, and full user file paths.
- Rotation: five files × 2 MiB, 14-day maximum.
- No telemetry and no remote crash upload in v1.

### Diagnostics UI

Show version, source commit/build, config schema, last startup, last successful Calendar sync, last wallpaper cycle, active error codes, safe mode, data/cache/log locations, and explicit recovery actions.

### Diagnostic bundle

User-initiated only, previewable manifest, zip containing build/system/monitor metadata, redacted settings, error summaries, and bounded log tail. Exclude tokens, event titles/details, OAuth callback data, raw database, screenshots, and full personal paths by default.

### Crash behavior

- Panic hook writes a minimal redacted crash marker.
- No automatic endless restart.
- Three startup crashes within five minutes trigger safe-mode recommendation/launch: no widgets, Calendar, provider, or autostart mutation; settings/diagnostics remain available.
- No watchdog process in v1.

---

## #13 — acceptance matrix

Create `docs/testing/acceptance-matrix.md` with one row per externally observable requirement and these required columns:

`ID | Phase | Blocking | Scenario | Environment | Expected result | Metric/tolerance | Automated command or manual step | Evidence path | Owner | Status | Waiver`

ID prefixes: `SURF`, `POMO`, `WALL`, `CAL`, `STOR`, `SEC`, `PRIV`, `A11Y`, `PERF`, `REL`, `OPS`.

Rules:

- No row uses “works”, “reliable”, “fast”, or “graceful” without an observable result/metric.
- #18 manual debt is copied as open rows, not summarized away.
- A requirement can be `PASS`, `FAIL`, `BLOCKED`, `MANUAL_REQUIRED`, or `WAIVED`; only a human may set a release-blocking waiver.
- Every issue PR adds/updates its rows; #24 executes them, not invents them.

---

## #20 — Alpha 1 implementation

Implement as tracer bullets, each independently reviewed, in this fixed internal order:

1. Runtime/tray/single instance from #7.
2. Production widget host using ADR topology and Normal/Edit Mode behavior.
3. Versioned settings + atomic layout persistence + off-screen clamping.
4. Pomodoro domain/state persistence and tray actions from #19.
5. Pomodoro widget projection and notification dedupe.
6. Local-folder wallpaper source + #5 adapter.
7. Diagnostics/status baseline from #40.
8. Recovery and physical evidence pass.

Do not open one giant Alpha PR. Use the #20 parent with child tracer-bullet issues if the current issue cannot fit one agent context.

Default local-folder behavior:

- User selects one or more folders; non-recursive initially.
- Enumerate supported extensions, canonicalize, sort deterministically, ignore invalid files.
- Assignment policy: shuffled bag with injected deterministic RNG; no repeat until bag exhausted when at least two valid images exist.
- One optional folder set for all monitors in Alpha 1; per-monitor source selection is deferred, while apply remains per-monitor.
- Manual Next advances once; Hold prevents automatic change. Alpha 1 has no schedule unless nearly free; #23 owns scheduling.
- No valid replacement → preserve current wallpaper.

Physical rows for sleep/resume, monitor loss, mixed DPI, Explorer restart, Win+D/fullscreen remain `MANUAL_REQUIRED` until executed.

---

## #37 — privacy, retention, and data lifecycle

- No telemetry and no remote crash reporting in v1.
- Calendar full-title mode is default; events marked private project to `Private`; Busy-only is user-selectable.
- The same privacy projection is applied before rendering, notification, UI Automation, logging, diagnostics, and clipboard/export. Downstream components never receive hidden titles.
- Calendar cache is retained only while the account is connected.
- Disconnect deletes refresh token, in-memory access token, Calendar event cache, sync token, selected calendar IDs, and account metadata. Preserve only generic UI/privacy preferences.
- Pomodoro history remains absent in Alpha 1.
- Layout and settings remain until explicit purge.
- Wallpaper cache/history follows its cap; currently applied files remain pinned.
- Logs: #40 limits and redaction.
- Uninstall preserves user data by default; a separate explicit Purge removes LocalAppData and Credential Manager entries.
- Backup/roaming is unsupported for v1.
- Secure deletion cannot be guaranteed on SSD/filesystem; documentation must say deletion removes references/files through normal Windows APIs.

Create a field-level inventory with purpose, classification, location, retention, deletion trigger, logs/diagnostics eligibility, and owner.

---

## #21 — Calendar Alpha 2

### Sync design

- CalendarList uses its normal list/sync mechanism as researched.
- For each selected calendar, use official Events incremental synchronization:
  - initial full sync with `singleEvents=true`, `showDeleted=true`, and `timeMin = now - 30 days`; no `timeMax`;
  - page until the final `nextSyncToken` and commit results + token transactionally;
  - incremental requests reuse allowed parameters and `syncToken` exactly;
  - HTTP 410 clears only that calendar's event store/token and restarts a full sync;
  - never expose a partially paged sync as committed state.
- Safety cap: 50,000 stored event instances per selected calendar. Exceeding it stops that calendar sync with `CALENDAR_TOO_LARGE`; it does not evict arbitrary records or terminate the runtime.
- UI horizon: Today/current event plus the next 5 events, searching 7 days ahead. Stored sync can exceed display horizon.
- Poll every 15 minutes; manual refresh minimum interval 30 seconds.
- Temporary failure backoff: 1, 2, 5, 15 minutes, cap 15; reset after success.
- Offline/stale after 30 minutes; keep last committed cache.

### Event projection

Normalize into a platform-neutral `AgendaItem` before UI:

- stable event/instance identity;
- calendar identity/color;
- start/end as UTC plus source time-zone metadata;
- all-day local date range;
- status/cancelled;
- privacy-projected display title;
- in-progress flag.

Cancelled/deleted removes the stored instance. All-day end date is exclusive. Recurrences are rendered from expanded instances. Use the user's current Windows time zone for display and recompute projections on time-zone change without rewriting source UTC data.

### Failure isolation

Calendar worker communicates through typed snapshots/events. Auth, network, parsing, storage, or oversized-calendar failures never terminate tray, surface, Pomodoro, or wallpaper work.

### Tests

Mock initial pagination, incremental pagination, changes/deletes, 410 reset, recurrence, all-day exclusive end, time-zone/DST, private projection, Busy-only UIA/log/notification leakage, revoked credentials, offline stale cache, retry cap, transactional failure, and disconnect purge.

---

## #42 — policy/legal research

Use primary sources and record requirements, not legal certainty.

### Google

Before public Calendar testing beyond development/test users, document consent-screen status, sensitive-scope verification, privacy-policy URL, Limited Use compliance, branding, test-user limits, refresh-token behavior, and deletion/revocation flow. No public release while required Google verification/policy items are unresolved.

### Remote wallpaper candidates

- **Unsplash: reject for this product.** Its API rules require hotlinking, attribution, download tracking, and prohibit using the API to replicate the core Unsplash experience, explicitly including wallpaper applications.
- **Bing daily: reject unless an official documented and licensable API/endpoint is found.** Do not rely on reverse-engineered/de facto endpoints.
- **Wallhaven:** technically available through a documented API and SFW guest path, but record API stability, rate limits, image licensing/source responsibility, caching, and distribution risk before selection.

### Assets and dependencies

Every icon/font/sample image has source, author, exact license, modification status, attribution requirement, and redistribution permission. Coordinate crate licenses with #38 and installer notices with #39.

Uncertain material questions remain owner/qualified-advice gates; the agent must not write “legally compliant” without evidence.

---

## #22 — remote-provider selection

### Recommendation — owner gate

**Ship v1 with no remote wallpaper provider. Keep local folders only.**

Reasons: Unsplash's official API rules conflict with a wallpaper app; no suitable official Bing API was established; Wallhaven adds content-rights, policy, network, and maintenance burden to a product already useful with local folders.

This recommendation does not take effect until the owner explicitly approves the v1 scope change and Issue #1 is updated under governance. Until then, #22 remains open and #23 must not start.

### Fallback only when owner insists on remote v1

Choose Wallhaven only, guest/no API key, SFW purity only, general category initially, documented endpoint only, hard application request interval ≥30 seconds, provider cooldown on 429, owned downloaded cache, source URL/ID metadata, and visible source attribution/link. Do not add Unsplash, Bing, NSFW, or a second provider.

---

## #23 — wallpaper Beta, only if retained

Do not execute unless #22 selects a provider with owner approval.

Defaults:

- Schedule presets: 15 min, 30 min, 1 h, 3 h, 6 h, 12 h, daily; default 3 h.
- Minimum provider fetch interval: max(provider requirement, 30 seconds).
- Missed cycle during sleep: perform at most one cycle after resume, then schedule from completion time.
- Manual Next does not shift the automatic schedule.
- Hold pauses automatic cycles until explicitly released.
- Cache cap 1 GiB, LRU among unpinned entries; applied files are pinned.
- Recent history: last 50 provider IDs/hashes.
- Offline/provider error: valid cache → local folder → unchanged current wallpaper.
- Backoff: 1, 2, 5, 15, 60 minutes, cap 60; honor Retry-After when larger.
- Download 30 MiB and decoded 100-MP limits.
- Clear Cache removes unpinned entries only.

Use a deterministic fake provider for rate limit, empty result, corrupt response/download, duplicate avoidance, cache cap, pinning, sleep/resume, and fallback tests.

---

## #39 — release engineering

- Channels: `0.x.y-alpha.N`, `0.x.y-beta.N`, `0.x.y-rc.N`, then `1.0.0`; use SemVer for app version and an independent integer config/schema version.
- Alpha/Beta distribution: portable ZIP artifact.
- Stable candidate: per-user WiX 4 MSI plus portable ZIP.
- No auto-updater in v1.
- MSI uses per-user context and major upgrades with full replacement; preserve LocalAppData by default.
- Installer removes program files, shortcuts, and Solpaper autostart value. Purge is a distinct explicit action that removes local data and credentials.
- Config/data migration is transactional with pre-migration backup; failed migration leaves old data intact and enters recovery UI. Downgrade is unsupported except documented restoration of a compatible backup.
- Dev/test artifacts may be unsigned and clearly labeled.
- Public stable artifact requires a human-selected recognized signing route. Evaluate eligible open-source SignPath Foundation or Microsoft/standard organization validation options. Signing-key generation/import/use is CRITICAL and human-only.
- Do not use MSIX initially; production MSIX distribution requires signing/trust and adds identity/deployment constraints without solving Solpaper's current needs.
- Artifact creation and public publication are separate workflows; agents may build candidates but cannot publish stable.

Every candidate includes ZIP/MSI as applicable, SHA-256 checksums, CycloneDX SBOM, release manifest/provenance, third-party notices, release notes, known limitations, install/upgrade/uninstall/purge instructions.

---

## #43 — product discovery

This work may be structured autonomously but evidence cannot be fabricated.

### Working hypothesis

Primary user: a Windows student/knowledge worker who wants a passive, local-first focus timer and next-agenda view on the desktop without a general widget platform or Solpaper cloud account.

Core jobs:

1. See focus state without opening another app.
2. See upcoming commitments with strong privacy controls.
3. Refresh the desktop from owned local image folders.

Differentiation to test: integrated passive desktop surface + local-first/privacy, not extensibility or provider count.

Activation proxy: within five minutes, the user places/understands a widget and starts one focus session. Seven-day proxy: user starts at least three focus sessions or checks the agenda on three separate days, collected through interview/self-report rather than hidden telemetry.

Plan: 5–8 Windows participants, 30-minute scripted first-use session plus a seven-day follow-up. Remove private participant data from notes.

Reduction criteria presented for owner approval:

- Remote provider leaves v1 when fewer than 2/5 participants value it or policy cost remains material.
- Calendar stays required for public v1 only when at least 4/5 can connect and understand privacy/offline states.
- TUI remains post-v1 regardless.

---

## #45 — maintenance and incident response

- Use GitHub private security advisories as the preferred private reporting path when enabled; `SECURITY.md` must also name an owner-approved private contact. Do not invent an email.
- Suggested response targets for a small project:
  - Critical: acknowledge ≤48 h, mitigation target ≤7 d.
  - High: acknowledge ≤3 d, mitigation target ≤14 d.
  - Medium: acknowledge ≤7 d, target next planned release.
  - Low: backlog/roadmap.
- Dependency review cadence: monthly; critical advisories trigger immediate assessment.
- Latest stable version receives fixes; previous stable minor receives security-only support for 90 days after replacement.
- Bad release: mark/yank the affected release, publish a warning, point users to the previous known-good signed/checksummed artifact, fix forward or produce a rollback candidate, and update known issues.
- Google/provider outage degrades only that subsystem; local Pomodoro/surface remain available. Provider feature has a local disable path.
- Signing-key compromise, malicious dependency, leaked OAuth credential, and repository compromise have separate checklists and are human incident authority.
- Create issue templates for bug, crash, provider/Calendar failure, feature request, and non-sensitive diagnostics. Security reports must not use a public issue template.

---

## #44 — external Beta and independent release review

Minimum 5–8 testers. Seek at least:

- one single-monitor 100% DPI;
- one 125%/150% DPI;
- two dual-monitor users, including mixed DPI;
- one Windows 11 24H2 device while supported;
- one 25H2 device;
- one 26H1 device when practical.

Script: first run, tray discovery, Edit Mode, Pomodoro, layout restart, local wallpaper, Calendar connect/privacy/offline/reconnect, diagnostics, quit, uninstall, and purge. Findings are anonymized and classified blocker/high/medium/low.

Independent review beyond the builder/verifier loop is required where feasible for unsafe Win32 boundaries, OAuth/Credential Manager, migrations, installer/rollback, and release supply chain.

Stable publication requires a human-signed go/no-go record covering known limitations, unresolved risks, accessibility evidence, security/privacy evidence, physical Windows matrix, artifact hashes/SBOM/provenance, rollback plan, and support authority.

---

## #24 — v1 release candidate

#24 performs no new policy design. It:

1. Builds a candidate through #39.
2. Executes every blocking #13 row on named environments.
3. Links evidence, failures, and human waivers.
4. Runs secret/private-data scans and dependency/license/SBOM/provenance checks.
5. Runs clean install, upgrade, failed migration/rollback, uninstall, and purge.
6. Runs the physical Windows debt from #18/#33.
7. Produces the #44 go/no-go package.

No stable publication, signing operation, critical waiver, or destructive migration approval occurs autonomously.

---

# Roadmap-wide human-only register

The agent should not ask routine design questions. It should stop only for these genuine gates:

1. Approve no remote provider in v1 and update Issue #1, or explicitly retain Wallhaven.
2. Create/manage Google Cloud OAuth project, consent screen, test users, verification, and privacy-policy hosting.
3. Approve any material external-service/legal risk.
4. Choose the private security contact.
5. Choose installer/signing route and operate signing credentials.
6. Perform or supervise disruptive physical Windows tests.
7. Recruit external participants and approve anonymized findings.
8. Approve release-blocking waivers, product reduction, and stable publication.

Everything else should follow this blueprint without asking the owner to choose between equivalent options.

# Primary research sources

- Google OAuth for desktop apps: https://developers.google.com/identity/protocols/oauth2/native-app
- Google Calendar scopes: https://developers.google.com/workspace/calendar/api/auth
- Google Calendar Events list: https://developers.google.com/workspace/calendar/api/v3/reference/events/list
- Google Calendar incremental sync: https://developers.google.com/workspace/calendar/api/guides/sync
- Google API Services User Data Policy: https://developers.google.com/terms/api-services-user-data-policy
- IDesktopWallpaper: https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-idesktopwallpaper
- GetMonitorDevicePathAt: https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-idesktopwallpaper-getmonitordevicepathat
- SetWallpaper: https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-idesktopwallpaper-setwallpaper
- Windows taskbar/tray: https://learn.microsoft.com/en-us/windows/win32/shell/taskbar
- Run/RunOnce keys: https://learn.microsoft.com/en-us/windows/win32/setupapi/run-and-runonce-registry-keys
- Windows accessibility checklist: https://learn.microsoft.com/en-us/windows/apps/design/accessibility/accessibility-checklist
- UI Automation overview: https://learn.microsoft.com/en-us/windows/win32/winauto/active-accessibility-and-ui-automation
- Windows 11 release information: https://learn.microsoft.com/en-us/windows/release-health/windows11-release-information
- SmartScreen reputation: https://learn.microsoft.com/en-us/windows/apps/package-and-deploy/smartscreen-reputation
- Windows code-signing options: https://learn.microsoft.com/en-us/windows/apps/package-and-deploy/code-signing-options
- MSI major upgrades: https://learn.microsoft.com/en-us/windows/win32/msi/major-upgrades
- MSI installation context: https://learn.microsoft.com/en-us/windows/win32/msi/installation-context
- MSI rollback: https://learn.microsoft.com/en-us/windows/win32/msi/rollback-installation
- RustSec cargo-audit: https://github.com/rustsec/rustsec/tree/main/cargo-audit
- cargo-deny license checks: https://embarkstudios.github.io/cargo-deny/checks/licenses/index.html
- CycloneDX for Cargo: https://docs.rs/cargo-cyclonedx/latest/cargo_cyclonedx/
- cargo-auditable: https://github.com/rust-secure-code/cargo-auditable
- Unsplash API documentation: https://unsplash.com/documentation
- Unsplash API guidelines: https://help.unsplash.com/en/articles/2511245-unsplash-api-guidelines
- Wallhaven API documentation: https://wallhaven.cc/help/api

## Blueprint completion rule

This document is complete when every open roadmap issue links to the relevant pack, independent standards/spec review confirms no accepted ADR/governance rule was weakened, primary links are validated, and #55 closes. Later implementation may refine a numerical default only through evidence, an issue-linked change, and the required risk/human gate.