# Non-functional requirements and quality budgets

**Issue:** [#35](https://github.com/rps321321/solpaper/issues/35)  
**Status:** initial budgets (Alpha 1 gate; tighten only with evidence)  
**Pack source:** [`deterministic-execution-blueprint.md` § #35](./deterministic-execution-blueprint.md)  
**Related:** [#18](https://github.com/rps321321/solpaper/issues/18) baselines · [#33](https://github.com/rps321321/solpaper/issues/33) test strategy · [#41](https://github.com/rps321321/solpaper/issues/41) accessibility · [#13](https://github.com/rps321321/solpaper/issues/13) acceptance matrix · [#24](https://github.com/rps321321/solpaper/issues/24) RC

## Purpose

Centralize **measurable** requirements that guide architecture and become release gates. Vague adjectives (“fast”, “lightweight”, “reliable”) are not requirements here unless paired with a metric, method, and gate class.

[#24](https://github.com/rps321321/solpaper/issues/24) validates previously defined budgets; it must not invent new numeric targets at release time.

## Authority and change rules

| Rule | Policy |
|------|--------|
| Decision store for pack defaults | This file + blueprint § #35 (blueprint LOCKED/DEFAULT remain authoritative for pack content) |
| Initial targets | Release-build measurements on **named** hardware from [windows-matrix.md](../testing/windows-matrix.md) |
| Tighten a target | Allowed with evidence path + PR/issue note |
| Weaken a **Hard** release blocker | Recorded human approval on the issue/PR |
| Measured Alpha/Beta baselines | Use when an honest hard target cannot yet be set; mark gate **Observe** or **Warn** until evidence supports **Hard** |

### Gate classes

| Class | Meaning | Merge / release effect |
|-------|---------|------------------------|
| **Hard** | Release-blocking for the stated phase when that phase claims the feature | FAIL blocks phase ship |
| **Warn** | Threshold that must be reported; human decides waiver | Documented on #13/#24; not silent |
| **Observe** | Telemetry-free manual/counter recording only | No ship block; informs later Hard targets |

## Measurement principles

1. Prefer **release** (`cargo build --release`) profiles for resource and latency claims.
2. Record environment in every evidence `manifest.json` (OS build, CPU, GPU, monitors, DPI, SHA, operator) per [evidence templates](../testing/evidence/).
3. CI (`windows-latest`) may enforce **contract** tests (limits, timeouts as pure logic) but is **not** proof of physical idle CPU/memory on owner hardware.
4. Network and external-service outages have **bounded** retries, timeouts, and offline UX; no unbounded spin.
5. Resource budgets are **not** claimed from debug builds or un-named machines.

## Supported and unsupported boundaries

### Operating system and architecture

| Item | Policy | Gate |
|------|--------|------|
| Supported OS | Windows 11 **x64**, versions **24H2**, **25H2**, **26H1** while Microsoft-supported | Hard (v1 claims) |
| Baseline build family | Build **26100** and successor supported 11 x64 builds; record exact `winver` in evidence | Hard for matrix enrollment |
| Unsupported | Windows 10; Windows 11 **ARM64**; Windows Server; Wine; ReactOS | Hard (do not claim) |

Aligns with [windows-matrix.md](../testing/windows-matrix.md).

### Hardware and display (minimum assumptions)

| Item | Policy | Gate |
|------|--------|------|
| CPU | 64-bit x86-64 capable of running supported Windows 11 desktop session | Observe until named envs enrolled |
| RAM | Enough free working set for budgets below (product idle targets are the binding constraint) | Observe |
| GPU / compositor | Default DWM session; no dedicated GPU required for Alpha 1 widgets | Observe |
| Monitors | **1+**; multi-monitor and mixed DPI are product claims gated by physical matrix | Hard when multi-mon claimed |
| Resolutions | Common desktop work areas; no minimum pixel claim beyond “fits supported Windows 11 desktop” | Observe |
| Orientations | Landscape primary baseline; portrait secondary in matrix (`topo-portrait-secondary`) | Hard when portrait claimed |
| DPI | Per-monitor V2; tested topologies 100% / 150% (+ 200% accessibility path) | Hard for claimed scales |

### Product surface constraints (architecture realism)

Targets assume ADR topology: Approach A widget HWNDs, single process, no WorkerW-only path, no Solpaper cloud backend ([ADR-0001](../adr/0001-desktop-overlay-topology.md), [ADR-0002](../adr/0002-process-model.md)). Spike #18 idle baseline (~7–8 MiB working set, low CPU at 1 Hz) informs that Alpha 1 idle ≤ 60 MiB is realistic for a small widget set.

## Quality budget table

Columns: **ID** · **Scenario** · **Metric** · **Target** · **Method** · **Phase** · **Gate**.

### Platform / lifecycle

| ID | Scenario | Metric | Target | Measurement method | Phase | Gate |
|----|----------|--------|--------|--------------------|-------|------|
| PERF-OS-01 | Supported OS claim | OS / arch | Win11 x64 24H2–26H1 (MS-supported); baseline family 26100 | Evidence `manifest.json` + matrix OS ID | Alpha 1+ | Hard |
| PERF-OS-02 | Unsupported platforms | Support claim | No support claim for Win10, ARM64, Server, Wine/ReactOS | Docs + install path refuse or documented unsupported | v1 | Hard |
| PERF-START-01 | Cold start to tray/surface ready | Wall time p95 | ≤ **1.5 s** | Release build; process start → tray icon present and surface HWND(s) created; n≥20 on named env | Alpha 1 | Hard |
| PERF-SET-01 | Warm settings open | Wall time | ≤ **250 ms** | Settings already loaded once; reopen from tray; stopwatch/perf counter | Alpha 1 | Hard |
| PERF-SET-02 | Cold settings open | Wall time | ≤ **750 ms** | First settings open after process start | Alpha 1 | Hard |
| PERF-STOP-01 | Shutdown / state flush | Wall time | ≤ **2 s** | Graceful quit → process exit after atomic layout/settings flush | Alpha 1 | Hard |

### Resource budgets (idle and active)

| ID | Scenario | Metric | Target | Measurement method | Phase | Gate |
|----|----------|--------|--------|--------------------|-------|------|
| PERF-IDLE-01 | Idle 60 s (Alpha 1: tray + Pomodoro widget, no Calendar) | Process CPU median / p95 | median ≤ **0.5%**, p95 ≤ **1%** | Release build; no user input; sample 60 s after settle; named env | Alpha 1 | Hard |
| PERF-IDLE-02 | Idle working set Alpha 1 | Private working set | ≤ **60 MiB** | Task Manager / `GetProcessMemoryInfo` after 60 s idle settle | Alpha 1 | Hard |
| PERF-IDLE-03 | Idle working set with Calendar connected | Private working set | ≤ **100 MiB** | Same method; account connected, last sync success or stale cache only | Alpha 2 | Hard |
| PERF-IDLE-04 | Idle process handles | Handle count | ≤ **500** | Process Explorer / Win32 handle count after settle | Alpha 1 | Warn → Hard by Beta |
| PERF-IDLE-05 | Prolonged idle smoke | CPU/memory + hang | Within idle budgets; no hang | ≥ **10 min** smoke (MD-009); counters in evidence | Alpha 1 | Hard (smoke) |
| PERF-IDLE-06 | Beta soak | Crash / hang | **Zero** crash or hang in **8 h** on reference env | Unattended soak log + process still responding | Beta | Hard |
| PERF-ACTIVE-01 | Active Edit Mode drag | CPU / jank | No hard number yet; no multi-second freeze | Observe frame/input during drag; record freezes | Alpha 1 | Observe |
| PERF-GPU-01 | Idle GPU | GPU engine use | Prefer near-idle; no continuous full-screen thrash | GPU counters if available | Beta | Observe |

### Timers (Pomodoro)

| ID | Scenario | Metric | Target | Measurement method | Phase | Gate |
|----|----------|--------|--------|--------------------|-------|------|
| PERF-TMR-01 | Live timer visible error | \|displayed remaining − true remaining\| | ≤ **250 ms** while running | Inject/compare clock; or wall vs displayed at second boundary | Alpha 1 | Hard |
| PERF-TMR-02 | Restart / resume deadline recovery | Time after runtime ready until coherent phase | ≤ **2 s** | Kill/restart or sleep/resume; compare stored deadline to display | Alpha 1 | Hard (core auto + Man sleep) |

### Calendar / network

| ID | Scenario | Metric | Target | Measurement method | Phase | Gate |
|----|----------|--------|--------|--------------------|-------|------|
| PERF-CAL-01 | Background poll interval | Period | **15 min** | Config + mock clock tests; logs correlation | Alpha 2 | Hard |
| PERF-CAL-02 | Stale indicator | Time without successful sync | After **30 min** show stale | `Clock` unit/integration + UI observe | Alpha 2 | Hard |
| PERF-CAL-03 | Manual refresh minimum interval | Period | ≥ **30 s** between manual refreshes | Domain policy test | Alpha 2 | Hard |
| PERF-NET-01 | HTTP connect timeout | Duration | **10 s** | Client config constant + mock hang test | Alpha 2 | Hard |
| PERF-NET-02 | HTTP total request timeout | Duration | **30 s** | Client config + mock slow body | Alpha 2 | Hard |
| PERF-NET-03 | Temporary failure backoff | Delays | **1, 2, 5, 15 min**; cap **15 min**; reset on success | Unit with `Clock` | Alpha 2 | Hard |
| PERF-NET-04 | Offline / outage UX | Behavior | Keep last committed cache; bounded retries; no crash loop | Mock offline + integration | Alpha 2 | Hard |
| PERF-CAL-05 | Stored instances safety cap | Count per selected calendar | ≤ **50,000**; excess → `CALENDAR_TOO_LARGE` | Unit/integration | Alpha 2 | Hard |

(HTTP stack and sync details: blueprint § #21; security limits: § #36.)

### Wallpaper / images / cache

| ID | Scenario | Metric | Target | Measurement method | Phase | Gate |
|----|----------|--------|--------|--------------------|-------|------|
| PERF-WALL-01 | Local wallpaper file size | Compressed file | ≤ **50 MiB** | Reject/skip above limit; preserve current wallpaper | Alpha 1 | Hard |
| PERF-WALL-02 | Remote download size (if remote retained) | Response body | ≤ **30 MiB** | Bounded reader; abort + typed error | If #22/#23 retained | Hard |
| PERF-WALL-03 | Decoded image pixels | Width × height | ≤ **100 megapixels** | Pre-check / decode guard | Alpha 1 | Hard |
| PERF-WALL-04 | Remote wallpaper cache (if retained) | On-disk cache | **1 GiB** default cap; pinned applied files never deleted | Cache policy unit + FS inspect | If remote retained | Hard |
| PERF-WALL-05 | Local upscale when filling monitor | Scale factor | ≤ **1.5×** on either edge; else letterbox/pillarbox | Layout/render unit | Alpha 1 | Hard |
| PERF-WALL-06 | Apply failure | Wallpaper state | Keep existing system wallpaper; one typed error; no retry loop | Adapter fake + physical smoke | Alpha 1 | Hard |

### Storage / data loss

| ID | Scenario | Metric | Target | Measurement method | Phase | Gate |
|----|----------|--------|--------|--------------------|-------|------|
| PERF-STOR-01 | Settings / layout write | Durability procedure | Same-dir temp → flush → replace → retain one previous `.bak` | Storage integration tests | Alpha 1 | Hard |
| PERF-STOR-02 | Corrupt config | Recovery | Preserve corrupt file with timestamped name; load safe defaults; diagnostics show recovery | Integration corrupt fixture | Alpha 1 | Hard |
| PERF-STOR-03 | Migration compatibility | Schema | Forward migrations only; document rollback unsupported unless explicit tool | Migration tests + #39 | v1 | Hard |
| PERF-STOR-04 | Data-loss tolerance | Loss window | At most last unflushed in-memory edit on hard kill; graceful quit loses none of committed state | Kill vs quit scenarios | Alpha 1 | Warn |

### Logs / diagnostics

| ID | Scenario | Metric | Target | Measurement method | Phase | Gate |
|----|----------|--------|--------|--------------------|-------|------|
| PERF-LOG-01 | Log volume | Files × size | **Five** files × **2 MiB**; **10 MiB** total cap | Diagnostics / FS after rotation; policy unit tests in `solpaper-core::diagnostics` | When file logging lands | Hard |
| PERF-LOG-02 | Log retention | Age | **14-day** cap | Rotation policy unit tests + FS when writer lands | #40 policy / writer unit | Hard |
| PERF-LOG-03 | Diagnostic bundle | Size / content | User-initiated; redacted; no tokens/titles/raw DB | Bundle name exclusion unit tests + manual | #40 / v1 | Hard |
| PERF-LOG-04 | Log field policy | Privacy | Allowlist excludes private Calendar and OAuth secrets | Unit allowlist tests (`validate_log_fields`) | Alpha 1+ (policy now; full Calendar Alpha 2) | Hard |

### Reliability

| ID | Scenario | Metric | Target | Measurement method | Phase | Gate |
|----|----------|--------|--------|--------------------|-------|------|
| PERF-REL-01 | Duplicate tray icon | Count | **Zero** in acceptance runs | Manual matrix + single-instance tests | Alpha 1 | Hard |
| PERF-REL-02 | Duplicate widget windows | Count | **Zero** after start/restart/sleep/lock | Manual MD-001/002 + single-instance | Alpha 1 / v1 | Hard |
| PERF-REL-03 | Duplicate notifications | Count per completion | **At most one** | `NotificationSink` unit | Alpha 1 | Hard |
| PERF-REL-04 | Crash loops | Startup crashes | ≥ **3** crashes within **5 min** → safe-mode recommendation (no widgets/Calendar/provider/autostart mutation) | #40 design + later evidence | Beta | Hard |
| PERF-REL-05 | Hang | UI/thread | No multi-minute hang in soak; network/disk off UI thread | Soak + design review | Beta | Hard |

### Accessibility conformance target

| ID | Scenario | Metric | Target | Measurement method | Phase | Gate |
|----|----------|--------|--------|--------------------|-------|------|
| PERF-A11Y-01 | Platform a11y | Standard | UIA + Microsoft accessibility checklist; WCAG 2.2 **AA** where applicable | [requirements.md](../accessibility/requirements.md) + A11Y-01..15 | Alpha 1 → v1 | Hard per row phase |

Accessibility detail and rows live under `docs/accessibility/`; this budget only freezes the **conformance target** so #24 does not invent a weaker standard later.

### Upgrade / rollback

| ID | Scenario | Metric | Target | Measurement method | Phase | Gate |
|----|----------|--------|--------|--------------------|-------|------|
| PERF-UPG-01 | Upgrade compatibility | User data | Newer build reads prior schema via migrations; no silent wipe | L7 release suite | v1 RC | Hard |
| PERF-UPG-02 | Rollback | Binary downgrade | Not guaranteed unless documented tool; settings `.bak` may restore last good file | Docs honesty + release notes | v1 | Warn |
| PERF-UPG-03 | Uninstall | Data | Uninstall **preserves** user data by default; explicit Purge removes LocalAppData + CM entries | Release suite | v1 | Hard |

## Acceptance rows for #13 (PERF seed)

Copy structure into `docs/testing/acceptance-matrix.md` when #13 lands. Prefixes match blueprint § #13 (`PERF`, `REL`, `OPS` as needed).

| ID | Phase | Blocking | Scenario | Environment | Expected result | Metric/tolerance | Automated or manual | Evidence / debt | Status |
|----|-------|----------|----------|-------------|-----------------|------------------|---------------------|-----------------|--------|
| PERF-01 | Alpha 1 | Yes | Cold start tray/surface | Named Win11 x64 release | Tray + surface ready | p95 ≤ 1.5 s | Man (+ future harness) | MD-PERF-01 | open |
| PERF-02 | Alpha 1 | Yes | Warm / cold settings open | Named release | Settings visible | ≤ 250 ms warm; ≤ 750 ms cold | Man | MD-PERF-01 | open |
| PERF-03 | Alpha 1 | Yes | Idle 60 s CPU + working set | Named release | Within budget | CPU med≤0.5% p95≤1%; WS≤60 MiB | Man counters | MD-009 | open |
| PERF-04 | Alpha 1 | Yes | Shutdown flush | Named | Clean exit | ≤ 2 s | Man | MD-PERF-01 | open |
| PERF-05 | Alpha 1 | Yes | Timer visible error | CI + Man | Display coherent | ≤ 250 ms | Auto `Clock` + Man | — | open |
| PERF-06 | Alpha 1 | Yes | Local wallpaper limits | CI | Reject oversize/over-MP; keep wallpaper | 50 MiB / 100 MP / 1.5× | Auto adapter/policy | — | open |
| PERF-07 | Alpha 1 | Yes | Atomic settings write + corrupt recovery | CI | `.bak` + timestamped corrupt + defaults | Procedure above | Auto L2 | — | open |
| PERF-08 | Alpha 1 | Yes | No duplicate tray/window/notification | CI + Man | Zero duplicates | Count = 0 | Auto + Man | MD-001 family | open |
| PERF-09 | Alpha 2 | Yes | Calendar poll/stale/timeouts/backoff | CI | Bounded offline behavior | 15 min / 30 min / 10s–30s / 1–15 min | Auto L1/L4 | — | open |
| PERF-10 | Alpha 2 | Yes | Idle WS with Calendar | Named release | ≤ 100 MiB | Working set | Man | MD-PERF-02 | open |
| PERF-11 | Beta | Yes | 8 h soak no crash/hang | Reference env | Still running, budgets held | 8 h | Man soak | MD-PERF-03 | open |
| PERF-12 | v1 | Yes | Log rotation and retention | CI + Man | Five×2 MiB; 14 d | Caps | Auto + Man | #40 | open |
| PERF-13 | v1 | Yes | Upgrade migrations; uninstall preserves data | Release env | Data intact unless Purge | L7 suite | Rel | #39/#24 | open |
| REL-01 | v1 | Yes | Sleep/resume no duplicates | Named physical | No dup Runtime/windows | Count = 0 | Man | MD-001 | open |
| OPS-01 | All | Yes | Evidence names hardware | Any PERF Man row | `manifest.json` complete | Template fields | Process | strategy | open |

## Performance-regression plan (tied to #33 and CI)

### What CI enforces (layers 1–4)

When production code exists for the concern, **required** automated checks (same job names as [ci-policy.md](./ci-policy.md)):

| Concern | Layer | Example assertions |
|---------|------:|--------------------|
| Timer recovery / visible error math | L1 | Deadline math within 250 ms with fake `Clock` |
| Calendar poll, stale, backoff, timeouts as policy constants | L1 + L4 | Intervals and backoff sequence; mock hang respects 10 s / 30 s |
| Wallpaper file/pixel/cache limits | L1/L3 | Reject >50 MiB / >100 MP; cache cap; pin applied |
| Atomic write / corrupt recovery | L2 | Temp→flush→replace→`.bak`; timestamped corrupt |
| Notification dedupe | L1/L3 | At-most-once per completion |
| Log allowlist / size policy (when #40 lands) | L1/L2 | Field allowlist; rotation constants |

CI **must not** claim host-idle CPU% or physical cold-start p95 on GitHub-hosted runners as Alpha 1 product proof. Optional future job: micro-benchmark **contract** only (no flaky wall thresholds on shared runners).

### What named hardware enforces (layer 6)

| Scenario IDs | Debt | Phase |
|--------------|------|-------|
| `scn-prolonged-idle`, cold start, settings open, shutdown | MD-009, MD-PERF-01 | Alpha 1 |
| Calendar-connected idle WS | MD-PERF-02 | Alpha 2 |
| Beta 8 h soak | MD-PERF-03 | Beta |
| Sleep/resume reliability | MD-001 | v1 |

### Regression triggers (re-measure)

Re-run physical PERF rows when any of these change:

- Timer period, poll loops, or render/invalidation path
- Process model, tray, or settings host toolkit
- Calendar sync or HTTP client defaults
- Image decode stack or wallpaper cache policy
- Logging subscriber or diagnostic bundle writer

### Evidence shape

Use [results.template.md](../testing/evidence/results.template.md) columns:

| Metric | Target (#35) | Observed | Notes |
|--------|--------------|----------|-------|
| Cold start p95 | ≤ 1.5 s | | |
| Warm settings | ≤ 250 ms | | |
| Idle working set | ≤ 60 / 100 MiB | | |
| Idle CPU median / p95 | ≤ 0.5% / 1% | | |
| Handle count | ≤ 500 | | |

## Baseline evidence from #18

Spike session (Approach A, release build, single monitor): working set ~**7.7 MiB**, low CPU over ~3 s sample ([overlay-feasibility.md](../research/overlay-feasibility.md)). This supports:

- Alpha 1 idle ≤ 60 MiB as a **realistic Hard** target for a small widget set.
- Prolonged idle and multi-hour soak remain **open manual debt** (MD-009, MD-PERF-03).

## Non-goals (this issue)

- Implementing production timers, HTTP clients, or log rotation code.
- Running the physical performance matrix in this PR.
- Freezing #13 full matrix text (human v1 boundary still open).
- Weakening any Hard budget without human approval.

## Document maintenance

| Event | Update |
|-------|--------|
| New measured baseline | Add row note + evidence path; optionally tighten target |
| Phase ship (#20 / Alpha 2 / #24) | Confirm every Hard row for that phase is PASS, WAIVED (human), or not yet in scope |
| Blueprint pack contradiction | Prefer newer primary-source evidence; record deviation on issue |

## Manual debt seeds (register)

| Suggested ID | Scenario | Blocks |
|--------------|----------|--------|
| MD-PERF-01 | Cold start, settings open, shutdown latency on named release env | Alpha 1 |
| MD-PERF-02 | Idle working set with Calendar connected ≤ 100 MiB | Alpha 2 |
| MD-PERF-03 | Beta 8 h soak no crash/hang | Beta |

Prolonged idle counters remain under existing **MD-009**.
