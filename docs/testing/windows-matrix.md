# Windows test matrix

**Issue:** [#33](https://github.com/rps321321/solpaper/issues/33)  
**Pack source:** [`deterministic-execution-blueprint.md` § #33](../engineering/deterministic-execution-blueprint.md)  
**Budgets:** [#35 non-functional-requirements.md](../engineering/non-functional-requirements.md) — OS support list aligns with this matrix.

## Purpose

Name the OS builds, display topologies, and disruptive scenarios required before hardware-dependent acceptance rows can pass. CI (`windows-latest`) is **not** a substitute for this matrix.

## OS and architecture

| ID | OS | Arch | Role | Status |
|----|-----|------|------|--------|
| `os-24h2` | Windows 11 24H2 | x64 | Supported while Microsoft-supported; baseline family | Required when available |
| `os-25h2` | Windows 11 25H2 | x64 | Owner reference environment target | Required when available |
| `os-26h1` | Windows 11 26H1 | x64 | On appropriate hardware when available | When available |
| — | Windows 10 | — | **Unsupported** | Do not claim support |
| — | Windows 11 ARM64 | — | **Unsupported for v1** | Out of matrix |
| — | Server / Wine / ReactOS | — | **Unsupported** | Out of matrix |

**Baseline build family (from #35 pack):** build `26100` and successor supported 11 x64 builds. Record exact `winver` build in every evidence `manifest.json`.

### Named environments (operators fill as machines are enrolled)

| Env ID | Operator | OS ID | Build | CPU | GPU | Notes |
|--------|----------|-------|-------|-----|-----|-------|
| `env-owner-primary` | owner | _TBD_ | _TBD_ | _TBD_ | _TBD_ | Default reference; fill on first physical run |
| `env-ci-windows-latest` | GitHub Actions | runner image | dynamic | N/A | N/A | Automated only; not physical proof |

Add rows rather than overwriting; retired machines keep historical evidence paths.

## Monitor and DPI topologies

| Topology ID | Description | Purpose |
|-------------|-------------|---------|
| `topo-single-100` | Single monitor, 100% scale | Baseline layout and Edit Mode |
| `topo-single-150` | Single monitor, 150% scale | DIP vs physical pixels, text/layout |
| `topo-dual-mixed-100-150` | Dual monitor, 100% + 150% mixed DPI | Cross-monitor drag, binding |
| `topo-portrait-secondary` | Landscape primary + portrait secondary | Anchor and off-screen recovery |
| `topo-hotplug` | Disconnect/reconnect, reorder, primary change | No stranded widgets; rebind |
| `topo-offscreen-recovery` | Widget saved off visible work area | Clamp/restore on next start |

Unit/integration tests use `MonitorEnumerator` fakes to cover geometry math for these topologies. Visual and shell correctness still need physical evidence for dual/mixed and hotplug rows.

## Disruptive and recovery scenarios

Do **not** run these while the owner is studying (product lock / #33 pack).

| Scenario ID | Action | Expected theme (detail in #13 rows) | Evidence |
|-------------|--------|-------------------------------------|----------|
| `scn-explorer-restart` | Restart Explorer (`explorer.exe`) | Surfaces recreated without WorkerW-only path; no duplicate Runtime | Manual + logs |
| `scn-win-d` | Win+D then restore | Widgets behave predictably with show-desktop | Manual |
| `scn-fullscreen` | Fullscreen game/video | Normal apps cover Solpaper; not permanent topmost | Manual |
| `scn-lock-unlock` | Lock and unlock session | No duplicate windows; timers consistent with recovery policy | Manual |
| `scn-sleep-resume` | Sleep and resume | No duplicate Runtime/windows; layout intact; Pomodoro recovery | Manual |
| `scn-prolonged-idle` | Idle ≥ 10 minutes (Beta soak target 8 h later) | CPU/memory within #35 budgets; no hang | Manual + counters |
| `scn-process-restart` | Kill/restart Solpaper process | Layout + Pomodoro recovery from storage | Automated core + manual shell |
| `scn-virtual-desktop` | Switch virtual desktops (if claimed in #13) | Documented behavior only; no silent assumption | Manual if in scope |

## Matrix coverage checklist (release phases)

Minimal honest coverage before claiming a phase:

| Phase | Automated | At least one physical env | Topologies | Scenarios |
|-------|-----------|---------------------------|------------|-----------|
| **Alpha 1** | Layers 1–3 for shipped code | `env-owner-primary` or equivalent | `topo-single-100` (+ 150 if claimed) | process restart; Win+D optional note |
| **Alpha 2** | + Calendar mock (layer 4) | same | prior + privacy not shell-dependent | OAuth once on real browser (controlled) |
| **Beta** | + provider mocks if remote retained | dual/mixed if multi-mon claimed | `topo-dual-mixed-100-150` | idle soak start |
| **v1 RC (#24)** | Full automated green | Named OS rows available | All topologies that #13 requires | All disruptive scenarios that #13 requires |

## CI vs physical

| Claim | CI sufficient? |
|-------|----------------|
| `cargo test` pure logic | Yes |
| Adapter contract with fakes | Yes |
| “Works after sleep” | **No** — need `scn-sleep-resume` evidence |
| “Mixed DPI usable” | **No** — need topology evidence |
| “Explorer restart recovers” | **No** — need `scn-explorer-restart` |
| Installer uninstall clean | **No** — release suite |

## How to record a matrix run

1. Choose env ID + topology ID + scenario IDs.
2. Create evidence directory per [evidence/README.md](./evidence/README.md).
3. Fill `manifest.json` (OS build, monitors, DPI, SHA, redaction).
4. Update [manual-debt-register.md](./manual-debt-register.md) status and evidence path.
5. Link evidence from the relevant #13 row when that matrix exists.

## Explicit non-claims

Until evidence exists, documentation and spikes may say **designed** or **partial**, never **passed**, for physical scenarios. Overlay spike language in `docs/research/overlay-feasibility.md` remains the source of #18 debt seeded into the register.
