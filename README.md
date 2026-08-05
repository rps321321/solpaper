# solpaper

Lightweight, **local-first** Windows 11 x64 **desktop-surface** application in Rust.

A user-session **Runtime** owns desktop widget **Surfaces**, productivity state, tray/settings interaction, and a peer **wallpaper subsystem**. **Pomodoro** and a read-only **Google Calendar** agenda are first-class widget use cases. Wallpaper fetching/cycling is one subsystem—not the product root.

## Status

Product destination locked (Issue #17). Next technical frontier: overlay feasibility spike (Issue #18). No production Cargo workspace yet.

| Artifact | Where |
|----------|--------|
| **Wayfinder map (tracker)** | [solpaper desktop-surface wayfinder map](https://github.com/rps321321/solpaper/issues/1) |
| **Wayfinder map (in-repo mirror)** | [`docs/wayfinder/map.md`](docs/wayfinder/map.md) |
| **Ticket index** | [`docs/wayfinder/tickets.md`](docs/wayfinder/tickets.md) |
| **Domain glossary** | [`CONTEXT.md`](CONTEXT.md) |
| **Agent rules** | [`AGENTS.md`](AGENTS.md) |
| **Implementation ledger** | [`IMPLEMENTATION_PLAN.md`](IMPLEMENTATION_PLAN.md) |
| **Research notes** | [`docs/research/`](docs/research/) |

## Product locks

- **Platform:** Windows 11 x64, Rust, local-first; no Solpaper cloud backend.
- **UI:** tray + Edit Mode + visual settings surface (TUI not primary v1).
- **Pomodoro:** required for Alpha 1 / v1.
- **Calendar:** read-only Google Calendar, Alpha 2, intended for v1; privacy default shows titles with private details as `Private`; Busy-only mode required.
- **Wallpaper:** peer subsystem; local folders first; at most one remote provider in v1.
- **Win32:** prefer documented APIs; WorkerW/Progman must never be the sole architecture.
- **Live widgets:** rendered as UI, never baked into wallpaper images.

Window topology, renderer, and Cargo boundaries remain provisional until the overlay spike (#18).

## Planned slices

| Slice | Scope |
|-------|--------|
| Prototype 0 | Overlay feasibility only |
| Alpha 1 | Tray, layout, Pomodoro, local wallpapers |
| Alpha 2 | Read-only Calendar agenda widget |
| Beta | One remote wallpaper provider + scheduling |
| v1 | Packaged, validated Windows 11 build |

## Platform

Windows 11 x64 only for v1. Linux/macOS are out of scope for this effort.

## License

See [LICENSE](LICENSE).
