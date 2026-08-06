# Acceptance area → test mapping

**Issue:** [#33](https://github.com/rps321321/solpaper/issues/33)  
**Consumes:** [#13](https://github.com/rps321321/solpaper/issues/13) acceptance areas (product matrix not yet frozen)  
**Strategy layers:** [strategy.md](./strategy.md)

This is the bridge from product acceptance themes to executable tests or explicit manual evidence. When #13 lands a rowed matrix, each row must reference a layer from this file and, if manual, a debt/evidence ID.

Legend:

| Code | Meaning |
|------|---------|
| L1–L7 | Layers from strategy |
| Auto | CI-capable when code exists |
| Man | Named physical/manual evidence required |
| Rel | Release/install suite (#24/#39) |
| Gate | Human-only or external |

## Desktop surface and displays

| Acceptance theme | Layer | Class | Notes / debt |
|------------------|------:|-------|--------------|
| Transparency / opacity | L1 math + L5/L6 visual | Auto + Man | Visual confirmation manual |
| Passive input / desktop usable | L5 design + L6 | Man | Click-through feel |
| Edit Mode drag/resize | L1 hit-test + L6 | Auto + Man | Core hit classification unit-tested |
| Focus / taskbar / Alt+Tab | L6 | Man | Not claimed by CI |
| Normal app coverage (not topmost) | L5/L6 | Man | |
| Win+D / fullscreen | L6 | Man | `scn-win-d`, `scn-fullscreen` |
| Process restart layout restore | L1/L2 + L6 | Auto + Man | Storage round-trip auto |
| Explorer restart | L6 | Man | `MD-003`; never WorkerW-only |
| Sleep/resume | L6 | Man | `MD-001` |
| Lock/unlock | L6 | Man | `MD-002` |
| Single monitor 100%/150% | L1 fake topo + L6 | Auto + Man | |
| Multi-monitor / mixed DPI | L1 + L6 | Auto + Man | `MD-004`, `MD-005` |
| Reorder / hotplug / primary | L1 + L6 | Auto + Man | `MD-006` |
| Off-screen recovery | L1 + L6 | Auto + Man | |

## Pomodoro

| Acceptance theme | Layer | Class | Notes |
|------------------|------:|-------|-------|
| Deadline accuracy | L1 + `Clock` | Auto | |
| Pause / resume / skip / reset | L1 | Auto | |
| Restart / sleep recovery | L1 (+ L6 sleep) | Auto + Man | Core recovery unit; sleep physical |
| At most one missed completion | L1 | Auto | |
| No replay of completed phase | L1 | Auto | |
| Notification deduplication | L1/L3 `NotificationSink` | Auto | |

## Calendar

| Acceptance theme | Layer | Class | Notes |
|------------------|------:|-------|-------|
| Read-only OAuth (system browser) | L4 mock + L6 once | Auto + Man | Real browser once per major change |
| Token protection | L3 fake store + Rel purge | Auto + Rel | No tokens in fixtures |
| Selected calendars | L1/L4 | Auto | |
| Recurring / all-day / cancelled / TZ | L1/L4 fixtures | Auto | Synthetic events only |
| Offline cache / staleness | L1 + `Clock` | Auto | |
| Sync-token recovery | L4 | Auto | |
| Disconnect / purge | L2/L3 + Rel | Auto + Rel | |
| Privacy projection | L1 | Auto | Ordinary / Private / Busy-only |

## Wallpaper

| Acceptance theme | Layer | Class | Notes |
|------------------|------:|-------|-------|
| Local folders | L3 + L6 | Auto + Man | Apply path physical |
| Per-monitor requirements | L3/L6 | Auto + Man | |
| Current-wallpaper preservation on fail | L3 | Auto | |
| Scheduling | L1 + `Clock` | Auto | |
| Cache policy | L1/L2 | Auto | |
| Provider failure / backoff | L4 + `Clock` | Auto | If remote retained |
| Attribution / policy | docs + Rel | Man/Gate | #42 |

## Engineering quality

| Acceptance theme | Layer | Class | Notes |
|------------------|------:|-------|-------|
| CI / branch gates | L1–L4 in CI | Auto | #32 |
| Configuration / migrations | L2 | Auto | |
| Security / privacy tests | L1/L3/L4 | Auto | #36/#37 |
| Dependency / supply-chain evidence | CI tools | Auto/Rel | #38 |
| Diagnostics / recovery design | L2 + L6 | Auto + Man | #40 |
| Accessibility | toolkit + Man | Auto + Man | #41 |
| Install / upgrade / rollback / uninstall | L7 | Rel | #39/#24 |
| Performance / resource budgets | L6 | Man | #35 named hardware |
| Release provenance | Rel | Rel/Gate | |
| Support / incident readiness | process | Gate | #45 |
| External Beta / human approval | — | Gate | #44; human-only |

## How #13 should reference this file

Each future acceptance row should include:

1. **Phase** and whether it blocks that phase.
2. **Layer code(s)** from [strategy.md](./strategy.md).
3. **Topology / scenario IDs** from [windows-matrix.md](./windows-matrix.md) when physical.
4. **Evidence path** or **manual-debt ID** until evidence exists.
5. **Measurement/tolerance** from #35 when quantitative.

Incomplete mapping is allowed while #13 is open; shipping a phase with unmapped **blocking** rows is not.
