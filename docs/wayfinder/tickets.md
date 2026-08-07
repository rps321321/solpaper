# Solpaper ticket index

**Parent roadmap:** [Issue #1](https://github.com/rps321321/solpaper/issues/1)  
**Engineering-readiness map:** [Issue #30](https://github.com/rps321321/solpaper/issues/30)  
**Roadmap mirror:** [`map.md`](map.md)

GitHub issue and pull-request state is authoritative. This page is a compact navigation aid, not a second issue tracker. Last refreshed: **2026-08-07**.

## Current order

```text
#36 review/merge
  → #38 supply chain and licensing
  → #40 diagnostics and recovery
  → #5 wallpaper adapter + #7 tray/runtime
  → #13 acceptance matrix
  → #20 Alpha 1
  → #6/#37/#42/#21 Calendar Alpha 2
  → #22/#23 optional remote-wallpaper Beta
  → #24/#44/#45 v1 validation and release approval
```

## Completed foundation

| Issue | Work | Result |
|---:|---|---|
| [#17](https://github.com/rps321321/solpaper/issues/17) | Product definition | Solpaper redefined as a local-first Windows desktop companion |
| [#18](https://github.com/rps321321/solpaper/issues/18) | Desktop-window feasibility | Per-widget top-level windows recommended; manual hardware evidence retained |
| [#31](https://github.com/rps321321/solpaper/issues/31) | Development governance | Risk classes, leases, bounded autonomous work, and human-only gates |
| [#16](https://github.com/rps321321/solpaper/issues/16) | Architecture and workspace | ADRs plus the four-crate production Rust workspace |
| [#32](https://github.com/rps321321/solpaper/issues/32) | CI and protected main | Required Windows quality checks and branch protection |
| [#55](https://github.com/rps321321/solpaper/issues/55) | Deterministic execution blueprint | Default decisions and execution packs for remaining roadmap work |
| [#19](https://github.com/rps321321/solpaper/issues/19) | Pomodoro domain | Tested state machine and recovery semantics in `solpaper-core` |
| [#33](https://github.com/rps321321/solpaper/issues/33) | Test strategy | Windows matrix, evidence format, seams, and manual-debt register |
| [#41](https://github.com/rps321321/solpaper/issues/41) | Accessibility | UI Automation, keyboard, contrast, scaling, and manual AT requirements |
| [#34](https://github.com/rps321321/solpaper/issues/34) | UX design | First run, Normal/Edit Mode, settings structure, keyboard map, and usability script |
| [#35](https://github.com/rps321321/solpaper/issues/35) | Quality budgets | Startup, resource, timing, network, cache, and logging limits |

## In review

| Issue | Work | State |
|---:|---|---|
| [#36](https://github.com/rps321321/solpaper/issues/36) | Threat model and security architecture | [PR #70](https://github.com/rps321321/solpaper/pull/70), HIGH-risk, human merge required |

## Next foundation work

| Issue | Work | Starts when |
|---:|---|---|
| [#38](https://github.com/rps321321/solpaper/issues/38) | Dependency, license, SBOM, and supply-chain controls | After #36 |
| [#40](https://github.com/rps321321/solpaper/issues/40) | Logging, diagnostics, crash recovery, and supportability | After #38 |
| [#5](https://github.com/rps321321/solpaper/issues/5) | `IDesktopWallpaper` adapter for local files | After the foundation packs |
| [#7](https://github.com/rps321321/solpaper/issues/7) | Tray runtime, autostart, and single-instance behaviour | After the foundation packs |
| [#13](https://github.com/rps321321/solpaper/issues/13) | Measurable product acceptance matrix | After #5/#7 and the remaining foundation inputs; human v1 boundary required |

## Product slices

| Issue | Stage | Depends on |
|---:|---|---|
| [#20](https://github.com/rps321321/solpaper/issues/20) | Alpha 1: tray, layout, Pomodoro UI, local wallpapers | #5, #7, #13, #38, #40, and merged #36 |
| [#6](https://github.com/rps321321/solpaper/issues/6) | Calendar auth and Windows Credential Manager research | Alpha 1 plus #36/#37/#42 requirements |
| [#21](https://github.com/rps321321/solpaper/issues/21) | Alpha 2: read-only Calendar agenda | #6 and #20; privacy and policy gates complete |
| [#22](https://github.com/rps321321/solpaper/issues/22) | Decide whether one remote wallpaper provider belongs in v1 | Stable Alpha 1 and provider-policy evidence |
| [#23](https://github.com/rps321321/solpaper/issues/23) | Beta wallpaper schedule, cache, fallback, and selected provider | #20 and an approved #22 recommendation |
| [#24](https://github.com/rps321321/solpaper/issues/24) | Package and validate the v1 candidate | Product slices, acceptance matrix, and release-readiness gates |

## Remaining release-readiness work

| Issue | Work | Main gate |
|---:|---|---|
| [#37](https://github.com/rps321321/solpaper/issues/37) | Privacy, retention, and Calendar data lifecycle | Before Calendar implementation merges |
| [#39](https://github.com/rps321321/solpaper/issues/39) | Packaging, versioning, update, signing, and rollback design | Before Beta packaging |
| [#42](https://github.com/rps321321/solpaper/issues/42) | Google, provider, asset, and distribution policy review | Before public integration testing and distribution |
| [#43](https://github.com/rps321321/solpaper/issues/43) | Product positioning and external discovery | Before public Beta messaging and final scope |
| [#44](https://github.com/rps321321/solpaper/issues/44) | External Beta, independent review, and human approval | Blocks stable publication |
| [#45](https://github.com/rps321321/solpaper/issues/45) | Maintenance, incident response, security reporting, and support lifecycle | Required before stable v1 |

## Superseded issues

Issues #2–#4, #8–#12, and #14–#15 described the earlier wallpaper/TUI-first product or selected details before the desktop-companion architecture was validated. They remain closed as not planned. Relevant questions may return only through the current subsystem issues above.
