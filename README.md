# solpaper

Lightweight, **local-first** Windows 11 x64 **desktop-surface** application in Rust.

A user-session **Runtime** owns desktop widget **Surfaces**, productivity state, tray/settings interaction, and a peer **wallpaper subsystem**. **Pomodoro** and a read-only **Google Calendar** agenda are first-class widget use cases. Wallpaper fetching/cycling is one subsystem—not the product root.

## Status

| Milestone | State |
|-----------|--------|
| Product destination (#17) | Complete |
| Overlay spike (#18) | Complete — Approach A recommended; manual evidence debt open |
| Architecture + workspace (#16) | ADRs + production crates scaffold |
| CI / protected main (#32) | Next engineering bootstrap |
| Alpha 1 application features | Not started |

Production workspace (ADR-0006):

```text
crates/
├── solpaper-app       # `solpaper` binary / composition root
├── solpaper-core      # platform-neutral domain
├── solpaper-windows   # Win32 adapters
└── solpaper-storage   # paths / settings / layout files
```

ADRs: [`docs/adr/`](docs/adr/). Spike (disposable): [`spikes/desktop-overlay/`](spikes/desktop-overlay/).

| Artifact | Where |
|----------|--------|
| **Wayfinder map (tracker)** | [Issue #1](https://github.com/rps321321/solpaper/issues/1) |
| **Engineering map** | [Issue #30](https://github.com/rps321321/solpaper/issues/30) |
| **Wayfinder map (in-repo)** | [`docs/wayfinder/map.md`](docs/wayfinder/map.md) |
| **Ticket index** | [`docs/wayfinder/tickets.md`](docs/wayfinder/tickets.md) |
| **Domain glossary** | [`CONTEXT.md`](CONTEXT.md) |
| **Agent rules** | [`AGENTS.md`](AGENTS.md) |
| **Governance** | [`docs/engineering/agent-governance.md`](docs/engineering/agent-governance.md) |
| **Implementation ledger** | [`IMPLEMENTATION_PLAN.md`](IMPLEMENTATION_PLAN.md) |
| **Research notes** | [`docs/research/`](docs/research/) |
| **Security / threat model** | [`docs/security/`](docs/security/) · [`SECURITY.md`](SECURITY.md) |

## Develop

```powershell
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Placeholder host (interactive). Use --smoke for a short non-interactive pump.
cargo run -p solpaper-app
cargo run -p solpaper-app -- --smoke
```

## Product locks

- **Platform:** Windows 11 x64, Rust, local-first; no Solpaper cloud backend.
- **UI:** tray + Edit Mode + visual settings surface (TUI not primary v1).
- **Pomodoro:** required for Alpha 1 / v1.
- **Calendar:** read-only Google Calendar, Alpha 2, intended for v1; privacy default shows titles with private details as `Private`; Busy-only mode required.
- **Wallpaper:** peer subsystem; local folders first; at most one remote provider in v1.
- **Win32:** prefer documented APIs; WorkerW/Progman must never be the sole architecture.
- **Live widgets:** rendered as UI, never baked into wallpaper images.
- **Topology (ADR-0001):** one top-level widget-sized HWND per widget by default.
- **Secrets (ADR-0005):** Windows Credential Manager only—never in config/SQLite/logs.

## Planned slices

| Slice | Scope |
|-------|--------|
| Prototype 0 | Overlay feasibility (done) |
| Scaffold | ADRs + workspace host (this tree) |
| Alpha 1 | Tray, layout, Pomodoro, local wallpapers |
| Alpha 2 | Read-only Calendar agenda widget |
| Beta | One remote wallpaper provider + scheduling |
| v1 | Packaged, validated Windows 11 build |

## Platform

Windows 11 x64 only for v1. Linux/macOS are out of scope for this effort.

## License

Solpaper is licensed under the [MIT License](LICENSE). Contributions are inbound=outbound MIT; see [CONTRIBUTING.md](CONTRIBUTING.md). Supply-chain and dependency policy: [docs/security/supply-chain.md](docs/security/supply-chain.md).

## License

See [LICENSE](LICENSE).
