# Solpaper

**A quiet, local-first desktop companion for Windows 11.**

Solpaper is an open-source Windows app in development. It is meant to keep a small amount of useful information where you can see it, directly on the desktop, without turning the desktop into a control panel or asking you to keep another app open.

The planned experience combines:

- a clear Pomodoro timer;
- a compact, read-only Google Calendar agenda;
- simple local wallpaper controls;
- movable and resizable desktop widgets;
- a passive everyday mode that stays out of the way, plus an Edit Mode for arranging widgets.

Solpaper runs in the signed-in Windows user session. It does not require a Solpaper account or a Solpaper cloud service. Settings, layouts, cached data, and future Calendar credentials are designed to remain on the user's computer.

> [!IMPORTANT]
> **Solpaper is Alpha 1 development-only.** There is no installer, updater, or stable release. The current build is a local-first tray + Pomodoro widget + local wallpaper host for Windows 11 development and manual evidence — not a finished product.

## Why this project exists

Useful information often lives behind several windows, tabs, and notifications. Solpaper explores a calmer alternative: show only a few things that matter, make them glanceable, and let the rest of the system remain quiet.

The project is intentionally narrow. It is not trying to become a general widget marketplace, a cloud dashboard, or a replacement desktop shell.

## Current state

| Area | State |
|---|---|
| Windows desktop-window feasibility | Proven; Approach A widget HWNDs in production host |
| Rust workspace and Windows host | Runtime + tray + single-instance compiling under CI |
| Settings / layout / Pomodoro persistence | Atomic JSON under `%LOCALAPPDATA%\solpaper\` |
| Pomodoro domain + widget projection | Implemented; tray Start/Pause/Skip/Reset |
| Local wallpapers | Folder source + tray Next/Hold via IDesktopWallpaper |
| Diagnostics / recovery baseline | Tray Diagnostics + crash markers + safe mode + recovery prompt |
| CI, test strategy, UX, a11y, NFR, security packs | Defined; threat model and supply-chain docs landed |
| Physical Windows evidence (MD-\*) | **Open** — see [`docs/testing/manual-debt-register.md`](docs/testing/manual-debt-register.md) |
| Public release | Not available |

For the live execution order, see the [roadmap issue](https://github.com/rps321321/solpaper/issues/1) and [`IMPLEMENTATION_PLAN.md`](IMPLEMENTATION_PLAN.md).

## Roadmap

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

The roadmap is deliberately evidence-driven. A planned feature may be simplified or removed when it adds more maintenance or privacy cost than user value.

## Product principles

- **Local first.** Core features should remain useful without a Solpaper server.
- **Calm by default.** Widgets should be readable without competing for attention.
- **Passive until edited.** Normal Mode should not interfere with the desktop; Edit Mode should make arrangement explicit.
- **Least privilege.** Calendar access is read-only, and secrets must not be stored in ordinary configuration or logs.
- **Small, replaceable subsystems.** Pomodoro, Calendar, and wallpaper features should not be able to bring down the whole app.
- **Honest releases.** Manual Windows testing and human approval are required where automation cannot provide trustworthy evidence.

## What is not planned for v1

- Linux or macOS support;
- a Solpaper cloud account or cross-device sync;
- Google Calendar write access;
- a plugin marketplace;
- a required terminal interface;
- more than one remote wallpaper provider;
- live information baked into wallpaper image files.

## Build the current development scaffold

### Requirements

- Windows 11 x64;
- [Rust](https://www.rust-lang.org/tools/install) 1.80 or newer;
- Git and PowerShell.

```powershell
git clone https://github.com/rps321321/solpaper.git
cd solpaper

# Interactive Alpha 1 host (tray + Pomodoro widget + local wallpapers).
cargo run -p solpaper-app

# Create control/tray/widgets, smoke a few paths, tear down.
cargo run -p solpaper-app -- --smoke
```

**What works today (dev build):** single-instance tray host, Edit Mode (Ctrl+Alt+F2), Pomodoro tray actions + widget projection, local wallpaper Next/Hold (drop images under `%LOCALAPPDATA%\solpaper\wallpapers`), Diagnostics status + optional recovery (recreate/clamp widgets, rescan folders), corrupt-config quarantine.

**What does not:** Google Calendar/OAuth, remote wallpaper providers, installer/autostart UX for end users, Settings chrome, diagnostic-bundle zip, or cleared physical evidence rows (MD-\*).

### Run the project checks

```powershell
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings

powershell -NoProfile -File scripts/tests/agent-lease.Tests.ps1
```

## Repository layout

```text
crates/
├── solpaper-app       # Windows user-session executable and composition root
├── solpaper-core      # Platform-neutral product rules and state machines
├── solpaper-windows   # Win32-specific adapters
└── solpaper-storage   # Settings, paths, and layout persistence

docs/
├── adr/               # Accepted architecture decisions
├── design/            # Interaction flows and usability material
├── engineering/       # CI, quality budgets, and development governance
├── security/          # Threat model and security requirements
├── testing/           # Test strategy, evidence format, and manual test debt
└── wayfinder/         # Roadmap mirror and ticket index
```

The disposable overlay experiments live under [`spikes/`](spikes/) and are intentionally excluded from the production Cargo workspace.

## Contributing

Contributions are welcome, but Solpaper is still before its first usable Alpha and not every open issue is ready for implementation.

Before starting a change:

1. Read the [product roadmap](https://github.com/rps321321/solpaper/issues/1) and the [engineering-readiness map](https://github.com/rps321321/solpaper/issues/30).
2. Choose a ticket that is explicitly ready, or discuss the proposed change on its issue first.
3. Keep the pull request focused and link it to the issue it addresses.
4. Run the applicable checks and state clearly which tests were and were not run.
5. Never place credentials, tokens, private Calendar data, or personal diagnostic data in code, fixtures, issues, or pull requests.

AI-assisted contributions are allowed, but the person submitting the change is responsible for understanding it, checking it, and responding to review. Security-sensitive work, public releases, signing-key operations, and destructive migrations require maintainer or human approval.

Project-specific development rules are documented in [`AGENTS.md`](AGENTS.md) and [`docs/engineering/agent-governance.md`](docs/engineering/agent-governance.md). These rules apply to automated contributors and also explain the repository's safety gates to human contributors.

## Security

Please do not report suspected vulnerabilities in a public issue. Follow [`SECURITY.md`](SECURITY.md) for the current private reporting path and supported-version policy.

## License

Solpaper is licensed under the [MIT License](LICENSE). Contributions are inbound=outbound MIT; see [CONTRIBUTING.md](CONTRIBUTING.md). Supply-chain and dependency policy: [docs/security/supply-chain.md](docs/security/supply-chain.md).

## License

See [LICENSE](LICENSE).
