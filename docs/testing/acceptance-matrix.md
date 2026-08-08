# Solpaper acceptance matrix

**Issue:** [#13](https://github.com/rps321321/solpaper/issues/13)  
**Pack:** [`deterministic-execution-blueprint.md` § #13](../engineering/deterministic-execution-blueprint.md)  
**Strategy:** [strategy.md](./strategy.md) · [acceptance-mapping.md](./acceptance-mapping.md) · [windows-matrix.md](./windows-matrix.md)  
**Manual debt:** [manual-debt-register.md](./manual-debt-register.md)

This is the product-level completion matrix for Prototype 0, Alpha 1, Alpha 2, Beta, and stable v1. It converts architecture, UX, security, privacy, accessibility, reliability, and release requirements into **observable** rows. #24 executes rows; it does not invent weaker requirements.

## Status and authority

| Field | Values |
|-------|--------|
| **Status** | `NOT_RUN` · `PASS` · `FAIL` · `BLOCKED` · `MANUAL_REQUIRED` · `WAIVED` |
| **Blocking** | `Yes` blocks the named **Phase** ship; `Prefer` is non-blocking preferred; `No` is informational |
| **Owner** | `agent` (automation/docs) · `owner` (human operator) · `human-gate` (release/approval only) |
| **Waiver** | Empty unless a **human** records rationale, affected phase, and expiry/review; agents never set release-blocking waivers |

**Freeze rule:** Owner must approve the **v1 boundary summary** (below) and any accepted Windows limitation before treating this matrix as frozen for #24. Until then this is a **draft with enforceable Alpha 1 row IDs** for #20 planning.

**Writing rules (pack):** no “works / reliable / fast / graceful” without metric; #18 debt is rows not summaries; later-phase features must not masquerade as shipped Alpha 1 capability.

## Column legend

| Column | Meaning |
|--------|---------|
| ID | Stable row id (`SURF` `POMO` `WALL` `CAL` `STOR` `SEC` `PRIV` `A11Y` `PERF` `REL` `OPS` `RT` prefixes) |
| Phase | Prototype 0 · Alpha 1 · Alpha 2 · Beta · v1 |
| Blocking | Yes / Prefer / No for that phase |
| Scenario | Observable scenario |
| Environment | CI / named Win11 env / release suite / process |
| Expected result | Observable outcome |
| Metric/tolerance | Numeric or countable bound from #35 or source issue |
| Automated or manual | Command, test module, or manual step + debt id |
| Evidence path | Filled path under `docs/testing/evidence/…` or `—` |
| Owner | Who must clear the row |
| Status | See table above |
| Waiver | Human-only |

---

## Proposed v1 boundary (human review)

| In v1 (blocking unless waived) | Out of Alpha 1 (later or optional) | Explicit non-goals (do not row as shipped) |
|--------------------------------|-------------------------------------|--------------------------------------------|
| Tray runtime, single instance, opt-in autostart (#7) | Google Calendar read-only (#6/#21) | Solpaper cloud backend |
| Approach A widget HWNDs; Normal/Edit Mode (#16/#34) | Remote wallpaper provider (#22/#23 owner gate) | Live widgets baked into wallpaper images |
| Pomodoro domain + tray + widget projection (#19/#20) | Per-monitor wallpaper *source* selection | TUI; general IPC; WorkerW-only surface |
| Local-folder wallpaper via IDesktopWallpaper (#5) | Full packaging/upgrade suite maturity (#39) | Automatic crash restart loops |
| Diagnostics, redaction, safe mode (#40) | External Beta program (#44) | Undocumented shell parenting as sole path |
| Acceptance rows PASS or human-waived (#13/#24) | | |

**Open human decisions:** freeze multi-monitor claim scope; accept any Windows limitation; release-blocking waivers; #22 provider inclusion; #44 go/no-go.

---

## SURF — desktop surface and displays

| ID | Phase | Blocking | Scenario | Environment | Expected result | Metric/tolerance | Automated or manual | Evidence path | Owner | Status | Waiver |
|----|-------|----------|----------|-------------|-----------------|------------------|---------------------|---------------|-------|--------|--------|
| SURF-01 | Alpha 1 | Yes | Passive opacity / transparency | Named physical | Widget content visible; desktop content not permanently obscured by opaque full-screen chrome | Design contract; visual check | Man | — | owner | MANUAL_REQUIRED | |
| SURF-02 | Alpha 1 | Yes | Passive Mode: pointer pass-through to desktop | Named physical | Clicks reach desktop icons/apps under widget chrome when not in Edit Mode | Click-through observed | Man | — | owner | MANUAL_REQUIRED | |
| SURF-03 | Alpha 1 | Yes | Edit Mode: drag/resize Pomodoro widget | CI + named | Hit-test classifies chrome; layout updates; Normal Mode restores pass-through | Unit hit-test + Man | Auto L1 + Man | — | agent/owner | NOT_RUN | |
| SURF-04 | Alpha 1 | Yes | Off-screen widget recovery | CI + named | Layout clamp brings widget fully into virtual screen work area | Unit clamp + Man | Auto L1 + Man | — | agent/owner | NOT_RUN | |
| SURF-05 | Alpha 1 | Yes | Process restart restores layout | CI + named | Last committed layout reloaded; no crash | Storage round-trip | Auto L2 + Man | — | agent/owner | NOT_RUN | |
| SURF-06 | v1 | Yes | Sleep/resume: no duplicate Runtime/windows; layout intact | Named physical | Count of Runtime/widget HWNDs = 1 set; Pomodoro coherent | Count = 0 duplicates | Man MD-001 | — | owner | MANUAL_REQUIRED | |
| SURF-07 | v1 | Yes | Lock/unlock: no duplicate Runtime/windows | Named physical | No second Runtime after unlock | Count = 0 | Man MD-002 | — | owner | MANUAL_REQUIRED | |
| SURF-08 | v1 | Yes | Explorer restart recovery without WorkerW-only path | Named physical | Surface recovers via documented path | Recovery observed; no WorkerW sole path | Man MD-003 | — | owner | MANUAL_REQUIRED | |
| SURF-09 | v1 | Yes | Dual-monitor layout + cross-monitor drag | Dual-monitor physical | Widget can move between monitors; layout persists | Layout coords valid | Man MD-004 | — | owner | MANUAL_REQUIRED | |
| SURF-10 | v1 | Yes | Mixed DPI (100/125/150) usable layout | Mixed-DPI physical | Widgets on-screen and readable | Clamp + no permanent off-screen | Man MD-005 | — | owner | MANUAL_REQUIRED | |
| SURF-11 | v1 | Yes | Monitor disconnect/reconnect/primary change | Hotplug physical | No stranded widgets off virtual screen | Re-clamp after topology change | Man MD-006 | — | owner | MANUAL_REQUIRED | |
| SURF-12 | v1 | Yes | Win+D show desktop then restore | Named physical | Widgets reappear with shell; no permanent hide failure | Restore observed | Man MD-007 | — | owner | MANUAL_REQUIRED | |
| SURF-13 | v1 | Yes | Fullscreen app covers widgets (not permanent topmost) | Named physical | Fullscreen game/video covers widgets | Not always-on-top over exclusive fullscreen | Man MD-008 | — | owner | MANUAL_REQUIRED | |
| SURF-14 | Alpha 1 | Prefer | Taskbar / Alt+Tab do not list widget chrome as normal apps (per ADR) | Named physical | Behavior matches ADR-0001 policy | Policy checklist | Man | — | owner | MANUAL_REQUIRED | |

Sources: #18, #16 ADR-0001, #34, #33 mapping.

---

## RT — tray runtime, single instance, autostart

| ID | Phase | Blocking | Scenario | Environment | Expected result | Metric/tolerance | Automated or manual | Evidence path | Owner | Status | Waiver |
|----|-------|----------|----------|-------------|-----------------|------------------|---------------------|---------------|-------|--------|--------|
| RT-01 | Alpha 1 | Yes | Single-instance mutex | CI | Second process does not start second Runtime | Mutex + `second_launch_outcome` unit | Auto `solpaper-core`/`windows` | — | agent | NOT_RUN | |
| RT-02 | Alpha 1 | Yes | Second launch posts show-settings then exits 0 | CI + named | No second tray; settings activation path | Unit activation + Man MD-RT-05 | Auto + Man MD-RT-05 | — | agent/owner | NOT_RUN | |
| RT-03 | Alpha 1 | Yes | Tray menu fixed order; unavailable disabled not hidden | CI | Menu model matches `build_tray_menu` order | Unit tray menu | Auto | — | agent | NOT_RUN | |
| RT-04 | Alpha 1 | Yes | Notification dedupe by phase instance id | CI | At most one balloon per completion id | Unit `NotificationDeduper` | Auto | — | agent | NOT_RUN | |
| RT-05 | Alpha 1 | Yes | Graceful shutdown removes tray; flush ≤ 2 s | Named release | Process exits; icon gone; state flushed | ≤ 2 s (PERF-04) | Man MD-PERF-01 + design | — | owner | MANUAL_REQUIRED | |
| RT-06 | Alpha 1 | Yes | Autostart default off; enable only installed path | CI | Portable refuse enable; HKCU Run value `Solpaper` + `--background` | Unit Fake + WindowsRunKey | Auto | — | agent | NOT_RUN | |
| RT-07 | v1 | Yes | Explorer restart recreates tray only (not widget reparent via Explorer) | Named physical | Tray re-added; widgets not WorkerW-parented | Man MD-RT-01 | Man MD-RT-01 | — | owner | MANUAL_REQUIRED | |
| RT-08 | v1 | Yes | Logon autostart (installed build) | Named physical | App starts at logon when enabled | Man MD-RT-02 | Man MD-RT-02 | — | owner | MANUAL_REQUIRED | |
| RT-09 | v1 | Yes | Task Manager startup lists Solpaper entry | Named physical | Name matches product policy | Man MD-RT-03 | Man MD-RT-03 | — | owner | MANUAL_REQUIRED | |
| RT-10 | v1 | Yes | Toggle off / uninstall removes only Solpaper Run value | Named physical | Other Run values untouched | Man MD-RT-04 | Man MD-RT-04 | — | owner | MANUAL_REQUIRED | |

Sources: #7, ADR-0008, `docs/design/runtime-tray.md`.

---

## POMO — Pomodoro

| ID | Phase | Blocking | Scenario | Environment | Expected result | Metric/tolerance | Automated or manual | Evidence path | Owner | Status | Waiver |
|----|-------|----------|----------|-------------|-----------------|------------------|---------------------|---------------|-------|--------|--------|
| POMO-01 | Alpha 1 | Yes | Start / pause / resume / skip / reset transitions | CI | Illegal commands rejected; legal transitions match table | Unit state machine | Auto `pomodoro` | — | agent | NOT_RUN | |
| POMO-02 | Alpha 1 | Yes | Deadline-based remaining time | CI | Displayed remaining tracks fake `Clock` | ≤ 250 ms visible error (PERF-05) | Auto | — | agent | NOT_RUN | |
| POMO-03 | Alpha 1 | Yes | Sync after restore completes at most one missed phase | CI | No multi-phase catch-up; no replay of completed instance | Unit Sync/LiveTick | Auto | — | agent | NOT_RUN | |
| POMO-04 | Alpha 1 | Yes | Skip does not credit focus count | CI | Focus count unchanged after Skip Focus | Unit | Auto | — | agent | NOT_RUN | |
| POMO-05 | Alpha 1 | Yes | Phase cadence Focus→break per defaults | CI | 4 focuses → long break path | Unit | Auto | — | agent | NOT_RUN | |
| POMO-06 | Alpha 1 | Yes | Invalid config rejected / clamped | CI | Out-of-range durations not persisted as active illegal | Unit validation | Auto | — | agent | NOT_RUN | |
| POMO-07 | Alpha 1 | Yes | Persistence round-trip of running/paused state | CI | Restart loads coherent phase + deadline/remaining | Storage + domain | Auto | — | agent | NOT_RUN | |
| POMO-08 | Alpha 1 | Yes | Tray actions map to commands | CI + named | Menu enables match state | Unit menu + Man | Auto + Man | — | agent/owner | NOT_RUN | |
| POMO-09 | v1 | Yes | Sleep/resume Pomodoro recovery coherent | Named physical | At most one completion; display matches deadline | Man with MD-001 | Man | — | owner | MANUAL_REQUIRED | |

Sources: #19, `docs/design/pomodoro-state-machine.md`, #35 PERF-TMR-*.

---

## WALL — local wallpaper

| ID | Phase | Blocking | Scenario | Environment | Expected result | Metric/tolerance | Automated or manual | Evidence path | Owner | Status | Waiver |
|----|-------|----------|----------|-------------|-----------------|------------------|---------------------|---------------|-------|--------|--------|
| WALL-01 | Alpha 1 | Yes | Local folder enumerate supported formats only | CI | `.jpg/.jpeg/.png/.bmp`; invalid skipped | Unit source policy | Auto | — | agent | NOT_RUN | |
| WALL-02 | Alpha 1 | Yes | Reject oversize / over-megapixel; keep current wallpaper | CI | ≤ 50 MiB; ≤ 100 MP; fail preserves | Auto adapter/policy PERF-06 | Auto | — | agent | NOT_RUN | |
| WALL-03 | Alpha 1 | Yes | Apply failure keeps system wallpaper; typed error; no retry loop | CI | Fake adapter fail path | Unit | Auto | — | agent | NOT_RUN | |
| WALL-04 | Alpha 1 | Yes | Upscale cap 1.5× else letterbox/pillarbox | CI | Scale policy unit | Auto | Auto | — | agent | NOT_RUN | |
| WALL-05 | Alpha 1 | Yes | Manual Next advances once; Hold blocks auto change | CI | Assignment policy with deterministic RNG bag | Unit | Auto | — | agent | NOT_RUN | |
| WALL-06 | Alpha 1 | Yes | Widgets never baked into wallpaper images | CI + review | Apply path sets desktop wallpaper only | Code review + tests | Auto + review | — | agent | NOT_RUN | |
| WALL-07 | Alpha 1 | Yes | COM adapter apply/enumerate on fake + smoke | CI | Fake covers contracts; COM behind feature | Unit + optional smoke | Auto | — | agent | NOT_RUN | |
| WALL-08 | v1 | Yes | Two distinct images on two monitors | Dual-monitor physical | Per-monitor apply | Man MD-WP-01 | Man MD-WP-01 | — | owner | MANUAL_REQUIRED | |
| WALL-09 | v1 | Yes | Detach/reconnect re-enumerates attached | Hotplug physical | Attached set updates | Man MD-WP-02 | Man MD-WP-02 | — | owner | MANUAL_REQUIRED | |
| WALL-10 | v1 | Prefer | Monitor identity fingerprint stable across rename | Physical | Fingerprint policy observed | Man MD-WP-03 | Man MD-WP-03 | — | owner | MANUAL_REQUIRED | |
| WALL-11 | v1 | Prefer | Global position Fill/Fit/Span | Physical | Position matches adapter set | Man MD-WP-04 | Man MD-WP-04 | — | owner | MANUAL_REQUIRED | |
| WALL-12 | Alpha 1 | Yes | Invalid file keeps previous wallpaper (physical) | Named physical | Desktop unchanged on bad file | Man MD-WP-05 | Man MD-WP-05 | — | owner | MANUAL_REQUIRED | |
| WALL-13 | v1 | Yes | Explorer restart does not require WorkerW for wallpaper | Named physical | COM path still applies | Man MD-WP-06 | Man MD-WP-06 | — | owner | MANUAL_REQUIRED | |

Sources: #5, `docs/research/idesktopwallpaper.md`, #35 PERF-WALL-*, pack #20.

**Not in Alpha 1:** remote provider, schedule (#23), history analytics.

---

## STOR — settings and storage

| ID | Phase | Blocking | Scenario | Environment | Expected result | Metric/tolerance | Automated or manual | Evidence path | Owner | Status | Waiver |
|----|-------|----------|----------|-------------|-----------------|------------------|---------------------|---------------|-------|--------|--------|
| STOR-01 | Alpha 1 | Yes | Atomic settings write | CI | temp → flush → replace → one `.bak` | PERF-STOR-01 | Auto L2 | — | agent | NOT_RUN | |
| STOR-02 | Alpha 1 | Yes | Corrupt config recovery | CI | Timestamped corrupt preserved; safe defaults; diagnostics note | PERF-STOR-02 | Auto L2 | — | agent | NOT_RUN | |
| STOR-03 | Alpha 1 | Yes | Versioned schema migrations (forward) | CI | Prior version loads via migration | Unit migrations | Auto | — | agent | NOT_RUN | |
| STOR-04 | Alpha 1 | Prefer | Hard kill loses at most unflushed edit | Named | Graceful quit loses none of committed state | Kill vs quit | Man + Auto | — | owner | MANUAL_REQUIRED | |

Sources: #35 PERF-STOR-*, #16 storage plan, #40 recovery notes.

---

## CAL — Calendar (Alpha 2+)

| ID | Phase | Blocking | Scenario | Environment | Expected result | Metric/tolerance | Automated or manual | Evidence path | Owner | Status | Waiver |
|----|-------|----------|----------|-------------|-----------------|------------------|---------------------|---------------|-------|--------|--------|
| CAL-01 | Alpha 2 | Yes | Read-only OAuth via system browser + PKCE | CI + one Man | Scopes read-only; PKCE S256; no token in fixtures | SEC-OAUTH-* tests | Auto + Man | — | agent/owner | NOT_RUN | |
| CAL-02 | Alpha 2 | Yes | Refresh token only in Credential Manager target | CI + review | Target `Solpaper/GoogleCalendar/v1/default` | Integration smoke | Auto + review | — | agent | NOT_RUN | |
| CAL-03 | Alpha 2 | Yes | Selected calendars; recurring/all-day/cancelled/TZ fixtures | CI | Synthetic events only | Fixtures L1/L4 | Auto | — | agent | NOT_RUN | |
| CAL-04 | Alpha 2 | Yes | Offline cache; stale after 30 min | CI | Last committed cache kept; stale indicated | Clock unit PERF-09 | Auto | — | agent | NOT_RUN | |
| CAL-05 | Alpha 2 | Yes | Sync timeouts and backoff bounds | CI | 10s connect / 30s total; backoff 1–15 min | PERF-09 | Auto | — | agent | NOT_RUN | |
| CAL-06 | Alpha 2 | Yes | Disconnect/purge removes token + cache + sync tokens | CI | Purge complete | Unit purge | Auto | — | agent | NOT_RUN | |
| CAL-07 | Alpha 2 | Yes | Privacy projection Ordinary/Private/Busy-only everywhere | CI | No private title in UIA/log/notify/pixels | Unit + A11Y-06/07 | Auto | — | agent | NOT_RUN | |
| CAL-08 | Alpha 2 | Yes | Calendar failure does not kill tray/Pomodoro/wallpaper | CI | Isolation | Integration | Auto | — | agent | NOT_RUN | |
| CAL-09 | Alpha 2 | Yes | Idle WS with Calendar ≤ 100 MiB | Named release | Working set bound | Man MD-PERF-02 | Man MD-PERF-02 | — | owner | MANUAL_REQUIRED | |

Sources: #6/#21 packs, #36 SEC-*, #37 PRIV, #35.

---

## SEC — security

| ID | Phase | Blocking | Scenario | Environment | Expected result | Metric/tolerance | Automated or manual | Evidence path | Owner | Status | Waiver |
|----|-------|----------|----------|-------------|-----------------|------------------|---------------------|---------------|-------|--------|--------|
| SEC-01 | Alpha 1 | Yes | No OAuth/Calendar secrets in Alpha 1 paths | CI + review | Grep/policy clean | SEC-A1-01 | Auto + review | — | agent | NOT_RUN | |
| SEC-02 | Alpha 1 | Yes | Wallpaper paths canonicalized under selected roots | CI | Path traversal rejected | SEC-A1-02 unit | Auto | — | agent | NOT_RUN | |
| SEC-03 | Alpha 1 | Yes | Single-instance; no general IPC | CI + review | Mutex only; ADR-0007 | SEC-A1-06 | Auto + review | — | agent | NOT_RUN | |
| SEC-04 | Alpha 1 | Yes | Unsafe Win32 confined; Safety docs | CI + review | windows crate only | SEC-A1-07 clippy/review | Auto + review | — | agent | NOT_RUN | |
| SEC-05 | Alpha 1 | Yes | Autostart is opt-in HIGH surface | PR process | Risk class HIGH when touched | Governance | Review | — | agent | NOT_RUN | |
| SEC-06 | Alpha 2 | Yes | OAuth/Calendar SEC-OAUTH/SEC-CAL rows | CI | See requirements-mapping | Auto | Auto | — | agent | NOT_RUN | |
| SEC-07 | v1 | Yes | If remote provider retained: SEC-REM-* | CI | HTTPS, bounds, no path concat | Auto | Auto | — | agent | NOT_RUN | |
| SEC-08 | v1 | Yes | Residual risks RR-* acknowledged; no silent weaken | Release | Human sign-off | SEC-RC-02 | human-gate | — | human-gate | NOT_RUN | |
| SEC-09 | v1 | Yes | Signing/public release human-only | Process | No agent public release | CRITICAL gate | human-gate | — | human-gate | NOT_RUN | |

Sources: `docs/security/requirements-mapping.md`, threat-model, governance.

---

## PRIV — privacy and diagnostics redaction

| ID | Phase | Blocking | Scenario | Environment | Expected result | Metric/tolerance | Automated or manual | Evidence path | Owner | Status | Waiver |
|----|-------|----------|----------|-------------|-----------------|------------------|---------------------|---------------|-------|--------|--------|
| PRIV-01 | Alpha 1 | Yes | Log field allowlist excludes secrets | CI | `validate_log_fields` style tests | PERF-LOG-04 | Auto | — | agent | NOT_RUN | |
| PRIV-02 | Alpha 2 | Yes | Private Calendar never in logs/UIA/notify/export | CI | Projection before all sinks | SEC-CAL-05 | Auto | — | agent | NOT_RUN | |
| PRIV-03 | Alpha 1 | Yes | Diagnostic bundle excludes secrets and private titles | CI + Man | Bundle preview/exclusion tests | OPS diagnostics | Auto + Man | — | agent/owner | NOT_RUN | |
| PRIV-04 | Alpha 2 | Yes | Disconnect purge user Calendar data | CI | No residual token/cache | Unit | Auto | — | agent | NOT_RUN | |

Sources: #37 pack (maturing), #40 diagnostics, #36.

---

## A11Y — accessibility

| ID | Phase | Blocking | Scenario | Environment | Expected result | Metric/tolerance | Automated or manual | Evidence path | Owner | Status | Waiver |
|----|-------|----------|----------|-------------|-----------------|------------------|---------------------|---------------|-------|--------|--------|
| A11Y-01 | Alpha 1 | Yes | Keyboard start/pause/reset Pomodoro | Named + CI | Action succeeds without mouse | MD-A11Y-01 + script §1 | Auto + Man | — | owner | MANUAL_REQUIRED | |
| A11Y-02 | Alpha 1 | Yes | Keyboard open settings and quit | Named | Settings opens; quit exits | MD-A11Y-01 | Man | — | owner | MANUAL_REQUIRED | |
| A11Y-03 | Alpha 1 | Yes | Settings UIA Name + ControlType | Named + Inspect | Non-empty Name per control | MD-A11Y-02 | Man | — | owner | MANUAL_REQUIRED | |
| A11Y-04 | Alpha 1 | Yes | Overlay Pomodoro UIA Pane/Group + Name | CI + Inspect | Name=widget type; Value=status | Unit + MD-A11Y-02 | Auto + Man | — | agent/owner | NOT_RUN | |
| A11Y-05 | Alpha 1 | Yes | Text scale 100% and 150% | Named | On-screen/usable after clamp | MD-A11Y-03 | Man | — | owner | MANUAL_REQUIRED | |
| A11Y-06 | Alpha 2 | Yes | Private mode absent from pixels and UIA | CI | No real title | Unit | Auto | — | agent | NOT_RUN | |
| A11Y-07 | Alpha 2 | Yes | Busy-only mode UIA strings | CI + Man | Busy/free only | Auto + Man | — | agent/owner | NOT_RUN | |
| A11Y-08 | Beta | Yes | Text scale 200% | Named | Primary actions reachable | MD-A11Y-03 | Man | — | owner | MANUAL_REQUIRED | |
| A11Y-09 | Beta | Yes | High contrast readable | Named | Not color-only state | MD-A11Y-04 | Man | — | owner | MANUAL_REQUIRED | |
| A11Y-10 | Beta | Prefer | Contrast sampling ≥4.5:1 body | Named | Measured ratios | Man measurement | Man | — | owner | MANUAL_REQUIRED | |
| A11Y-11 | Beta | Yes | Notifications carry text | CI + Man | Textual phase/error | Auto + Man | — | agent/owner | NOT_RUN | |
| A11Y-12 | Beta | Prefer | No keyboard trap in Edit Mode | Named | Escape/documented exit | Man script §2 | Man | — | owner | MANUAL_REQUIRED | |
| A11Y-13 | v1 | Yes | Narrator smoke; no private title | Named | Understandable; no leak | MD-A11Y-05 | Man | — | owner | MANUAL_REQUIRED | |
| A11Y-14 | v1 | Yes | AT review or human waiver | Process | Feedback or waiver on #24/#44 | human-gate | human-gate | — | human-gate | NOT_RUN | |
| A11Y-15 | All | Yes | Meaning not by color alone | Design + Man | Text or non-color indicator | Design #34 + Man | Man | — | owner | MANUAL_REQUIRED | |

Sources: `docs/accessibility/acceptance-rows.md`, #41.

---

## PERF — performance and resource budgets

| ID | Phase | Blocking | Scenario | Environment | Expected result | Metric/tolerance | Automated or manual | Evidence path | Owner | Status | Waiver |
|----|-------|----------|----------|-------------|-----------------|------------------|---------------------|---------------|-------|--------|--------|
| PERF-01 | Alpha 1 | Yes | Cold start tray/surface | Named Win11 x64 release | Tray + surface ready | p95 ≤ 1.5 s | Man MD-PERF-01 | — | owner | MANUAL_REQUIRED | |
| PERF-02 | Alpha 1 | Yes | Warm / cold settings open | Named release | Settings visible | ≤ 250 ms warm; ≤ 750 ms cold | Man MD-PERF-01 | — | owner | MANUAL_REQUIRED | |
| PERF-03 | Alpha 1 | Yes | Idle 60 s CPU + working set | Named release | Within budget | CPU med≤0.5% p95≤1%; WS≤60 MiB | Man MD-009 | — | owner | MANUAL_REQUIRED | |
| PERF-04 | Alpha 1 | Yes | Shutdown flush | Named | Clean exit | ≤ 2 s | Man MD-PERF-01 | — | owner | MANUAL_REQUIRED | |
| PERF-05 | Alpha 1 | Yes | Timer visible error | CI + Man | Display coherent | ≤ 250 ms | Auto Clock + Man | — | agent | NOT_RUN | |
| PERF-06 | Alpha 1 | Yes | Local wallpaper limits | CI | Reject oversize; keep wallpaper | 50 MiB / 100 MP / 1.5× | Auto | — | agent | NOT_RUN | |
| PERF-07 | Alpha 1 | Yes | Atomic settings + corrupt recovery | CI | `.bak` + defaults | Procedure #35 | Auto L2 | — | agent | NOT_RUN | |
| PERF-08 | Alpha 1 | Yes | No duplicate tray/window/notification | CI + Man | Zero duplicates | Count = 0 | Auto + Man MD-001 family | — | agent/owner | NOT_RUN | |
| PERF-09 | Alpha 2 | Yes | Calendar poll/stale/timeouts/backoff | CI | Bounded offline | 15m/30m/10s–30s/1–15m | Auto | — | agent | NOT_RUN | |
| PERF-10 | Alpha 2 | Yes | Idle WS with Calendar | Named | ≤ 100 MiB | Working set | Man MD-PERF-02 | — | owner | MANUAL_REQUIRED | |
| PERF-11 | Beta | Yes | 8 h soak no crash/hang | Reference env | Still running; budgets held | 8 h | Man MD-PERF-03 | — | owner | MANUAL_REQUIRED | |
| PERF-12 | v1 | Yes | Log rotation and retention | CI + Man | Five×2 MiB; 14 d | Caps #40 | Auto + Man | — | agent/owner | NOT_RUN | |
| PERF-13 | v1 | Yes | Upgrade migrations; uninstall preserves data | Release env | Data intact unless Purge | L7 suite #39 | Rel | — | owner | NOT_RUN | |

Sources: #35 seed table (copied; do not invent new Hard budgets here).

---

## REL — reliability

| ID | Phase | Blocking | Scenario | Environment | Expected result | Metric/tolerance | Automated or manual | Evidence path | Owner | Status | Waiver |
|----|-------|----------|----------|-------------|-----------------|------------------|---------------------|---------------|-------|--------|--------|
| REL-01 | v1 | Yes | Sleep/resume no duplicates | Named physical | No dup Runtime/windows | Count = 0 | Man MD-001 | — | owner | MANUAL_REQUIRED | |
| REL-02 | Alpha 1 | Yes | Safe mode after crash loop threshold | CI + Man | 3 crashes / 5 min → safe mode; no uncontrolled restart loop | Unit + Man | Auto + Man | — | agent/owner | NOT_RUN | |
| REL-03 | Alpha 1 | Yes | Hang/recovery path documented | Docs + Man | Troubleshooting steps reach Diagnostics | Docs + Man | Process | — | owner | NOT_RUN | |

Sources: #40, #35 PERF-REL-*, MD-001.

---

## OPS — operations, supply chain, release

| ID | Phase | Blocking | Scenario | Environment | Expected result | Metric/tolerance | Automated or manual | Evidence path | Owner | Status | Waiver |
|----|-------|----------|----------|-------------|-----------------|------------------|---------------------|---------------|-------|--------|--------|
| OPS-01 | All | Yes | Evidence manifests name hardware | Any Man PERF/REL | `manifest.json` complete | Template fields | Process | strategy | agent/owner | NOT_RUN | |
| OPS-02 | Alpha 1 | Yes | Required CI gates green on main | GitHub Actions | Windows Rust quality, Governance, CI policy, Supply chain, Dependency review | All SUCCESS | CI | Actions | agent | NOT_RUN | |
| OPS-03 | Alpha 1 | Yes | Supply-chain controls present | CI | lockfile, audit/deny policy, Action pins per #38 | CI supply chain job | Auto | — | agent | NOT_RUN | |
| OPS-04 | Alpha 1 | Yes | Diagnostics categories without payloads | CI | ErrorCategory mapping | Unit | Auto | — | agent | NOT_RUN | |
| OPS-05 | Beta | Yes | Beta soak evidence | Reference env | MD-PERF-03 cleared | 8 h | Man | — | owner | MANUAL_REQUIRED | |
| OPS-06 | v1 | Yes | Release manifest: checksums, source SHA, limitations | Release | Schema valid | #38/#24 | Rel | — | human-gate | NOT_RUN | |
| OPS-07 | v1 | Yes | SECURITY.md reporting path live | Repo | Contact/advisories path | Audit | Process | — | human-gate | NOT_RUN | |
| OPS-08 | v1 | Yes | External Beta / human release approval | Process | #44 recorded | Human only | human-gate | — | human-gate | NOT_RUN | |
| OPS-09 | Alpha 1 | Yes | Human usability script sessions | Named + participant | Script findings recorded | MD-UX-01 | Man MD-UX-01 | — | owner | MANUAL_REQUIRED | |
| OPS-10 | v1 | Prefer | Maintenance / incident readiness | Process | #45 checklist | Gate | human-gate | — | human-gate | NOT_RUN | |

Sources: #32, #33, #38, #40, #44, #45, #34 usability.

---

## Prototype 0 / engineering foundation (reference)

These are not product UX rows; they gate engineering honesty.

| ID | Phase | Blocking | Scenario | Environment | Expected result | Metric/tolerance | Automated or manual | Evidence path | Owner | Status | Waiver |
|----|-------|----------|----------|-------------|-----------------|------------------|---------------------|---------------|-------|--------|--------|
| OPS-P0-01 | Prototype 0 | Yes | Production Cargo workspace builds | CI | `cargo test` workspace green | Exit 0 | CI | Actions | agent | NOT_RUN | |
| OPS-P0-02 | Prototype 0 | Yes | ADRs 0001–0008 recorded | Repo | Architecture decisions readable | Docs present | Review | docs/adr | agent | NOT_RUN | |

---

## Alpha 1 blocking checklist (for #20)

Before #20 claims Alpha 1 complete, every **Alpha 1 + Blocking=Yes** row must be `PASS`, or `MANUAL_REQUIRED` with a **named** evidence plan (debt id), or human `WAIVED`. Minimum automated coverage expected before merge of functional #20 PRs:

- RT-01..04, RT-06 (units exist or land with #20 host)
- POMO-01..08
- WALL-01..07
- STOR-01..03
- PERF-05..08 (auto portions)
- SEC-01..05, PRIV-01, PRIV-03 (policy)
- A11Y-04 (unit portion)
- OPS-02..04

Physical Alpha 1 blockers remain open until run: PERF-01..04, A11Y-01..03/05, OPS-09, WALL-12, RT-05, SURF-01..05 (Man portions), MD-* as linked.

---

## Open manual evidence register (summary)

Do not delete; clear only with evidence paths. Full table: [manual-debt-register.md](./manual-debt-register.md).

| IDs | Blocks |
|-----|--------|
| MD-001..009 | v1 surface/reliability / Alpha 1 idle smoke |
| MD-A11Y-01..05 | Alpha 1 → v1 a11y |
| MD-UX-01 | Alpha 1 usability |
| MD-PERF-01..03 | Alpha 1 / Alpha 2 / Beta |
| MD-WP-01..06 | Wallpaper physical |
| MD-RT-01..05 | Runtime/autostart physical |

---

## Waiver log

| Waiver id | Row IDs | Phase affected | Owner | Rationale | Expiry / review | Issue |
|-----------|---------|----------------|-------|-----------|-----------------|-------|
| *(none)* | | | | | | |

Only humans add rows here.

---

## Maintenance

| Event | Action |
|-------|--------|
| Feature PR | Add/update rows for new observables; never mark Man `PASS` without evidence |
| Physical run | Fill Evidence path; set Status; update debt register |
| #20 / #21 / #23 / #24 | Link PR to row IDs in description |
| Budget change | Edit #35 first; then sync PERF rows here |
| Human freeze | Owner comments on #13 approving v1 boundary summary |

## Related consumer links

| Issue | Uses rows |
|-------|-----------|
| #20 Alpha 1 | SURF (A1), RT, POMO, WALL (A1), STOR, PERF A1, SEC A1, A11Y A1, OPS A1 |
| #21 Calendar | CAL, PRIV, SEC OAuth, PERF-09/10, A11Y-06/07 |
| #23 Remote wallpaper | WALL remote (when retained), SEC-REM, OPS-REM |
| #24 v1 RC | All v1 Blocking=Yes + waiver log + OPS-06..08 |
