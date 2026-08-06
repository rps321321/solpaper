# Wayfinder tickets (index)

Parent map: [solpaper desktop-surface wayfinder map](https://github.com/rps321321/solpaper/issues/1) · in-repo [`map.md`](map.md)

Open/closed state on GitHub is authoritative; refresh this file when tickets are added, blocked, or resolved.

**Product order:** #17 → #18 → #16 → #13 → (#7, #5, #19) → #20 → (#6, #21) → (#22, #23) → #24  
**Bootstrap:** #31 → #16 → #32 (**done**) → then #1/#30. Engineering map: #30 (#31–#45).

## Done

| Issue | Title | Notes |
|------:|-------|-------|
| [#17](https://github.com/rps321321/solpaper/issues/17) | Redefine Solpaper as a Windows desktop-surface application | Product locks recorded; vocabulary in `CONTEXT.md` |
| [#18](https://github.com/rps321321/solpaper/issues/18) | Prototype desktop overlay feasibility on Windows 11 | Spike A/B + `docs/research/overlay-feasibility.md`; recommend Approach A; PR #28 |
| [#31](https://github.com/rps321321/solpaper/issues/31) | Harden autonomous-agent governance and change-risk controls | `docs/engineering/agent-governance.md`, leases, PR #47 |
| [#16](https://github.com/rps321321/solpaper/issues/16) | Record post-spike architecture and scaffold the production workspace | `docs/adr/*`, production workspace under `crates/` |
| [#32](https://github.com/rps321321/solpaper/issues/32) | Establish CI, protected-main policy, and required quality gates | `.github/workflows/*`, `docs/engineering/ci-policy.md`, protected `main`; PR #53 |
| [#55](https://github.com/rps321321/solpaper/issues/55) | Deterministic execution blueprint | `docs/engineering/deterministic-execution-blueprint.md`; PR #57 |
| [#19](https://github.com/rps321321/solpaper/issues/19) | Design the Pomodoro state machine and recovery semantics | `solpaper-core` machine; PR #58 |
| [#33](https://github.com/rps321321/solpaper/issues/33) | Define the test strategy, Windows matrix, and evidence harness | `docs/testing/*`; PR #61 |

## Open — frontier and foundation

| Issue | Title | Blocked by |
|------:|-------|------------|
| [#30](https://github.com/rps321321/solpaper/issues/30) | Raise Solpaper to public-release engineering standards | parent map for #31–#45 |
| [#41](https://github.com/rps321321/solpaper/issues/41) | Accessibility feasibility and requirements | before UI toolkit freeze; pack after #33 |
| [#13](https://github.com/rps321321/solpaper/issues/13) | Define measurable desktop-surface v1 acceptance criteria | human v1 boundary (+ #16, #33 mapping) |
| [#7](https://github.com/rps321321/solpaper/issues/7) | Decide tray runtime, autostart, and single-instance behaviour | #16 + foundation packs |
| [#5](https://github.com/rps321321/solpaper/issues/5) | Research IDesktopWallpaper as wallpaper subsystem adapter | #16 + foundation packs |

## Open — product slices

| Issue | Title | Blocked by |
|------:|-------|------------|
| [#20](https://github.com/rps321321/solpaper/issues/20) | Build Alpha 1: tray, persistent layout, Pomodoro, local wallpapers | #16, #19, #5, #7 |
| [#6](https://github.com/rps321321/solpaper/issues/6) | Research secret storage and Google Calendar desktop OAuth | #20 (+ privacy default from #17) |
| [#21](https://github.com/rps321321/solpaper/issues/21) | Build Alpha 2: read-only Google Calendar agenda widget | #6, #20 |
| [#22](https://github.com/rps321321/solpaper/issues/22) | Research and select the first remote wallpaper provider | #20 |
| [#23](https://github.com/rps321321/solpaper/issues/23) | Build Beta wallpaper scheduling, cache, selected provider | #20, #22 |
| [#24](https://github.com/rps321321/solpaper/issues/24) | Harden, package, and validate Solpaper v1 | #13, #20, #21, #23, #7 |

## Superseded (closed, old wallpaper/TUI product)

| Issue | Title |
|------:|-------|
| [#2](https://github.com/rps321321/solpaper/issues/2) | Wallhaven API research |
| [#3](https://github.com/rps321321/solpaper/issues/3) | Bing fetch research |
| [#4](https://github.com/rps321321/solpaper/issues/4) | Unsplash research |
| [#8](https://github.com/rps321321/solpaper/issues/8) | TUI↔agent IPC |
| [#9](https://github.com/rps321321/solpaper/issues/9) | TUI information architecture |
| [#10](https://github.com/rps321321/solpaper/issues/10) | cron semantics |
| [#11](https://github.com/rps321321/solpaper/issues/11) | cache defaults |
| [#12](https://github.com/rps321321/solpaper/issues/12) | purity/source defaults |
| [#14](https://github.com/rps321321/solpaper/issues/14) | fixed 2560×1440-oriented fit policy |
| [#15](https://github.com/rps321321/solpaper/issues/15) | TUI prototype |

Research write-ups go under [`docs/research/`](../research/) and are linked from the issue when applicable.
