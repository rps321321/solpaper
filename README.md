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
> **Solpaper is pre-Alpha.** There is no installer or everyday-use release yet. The current executable is a development scaffold with a placeholder window, not a finished product preview.

## Why this project exists

Useful information often lives behind several windows, tabs, and notifications. Solpaper explores a calmer alternative: show only a few things that matter, make them glanceable, and let the rest of the system remain quiet.

The project is intentionally narrow. It is not trying to become a general widget marketplace, a cloud dashboard, or a replacement desktop shell.

## Current state

| Area | State |
|---|---|
| Windows desktop-window feasibility | Proven through a disposable prototype |
| Rust workspace and Windows host | Scaffolded and compiling |
| Settings and widget-layout foundations | Implemented at a basic level |
| Pomodoro state machine | Implemented and unit-tested |
| CI, test strategy, UX, accessibility, and performance budgets | Defined |
| Threat model and security architecture | In review in [PR #70](https://github.com/rps321321/solpaper/pull/70) |
| Usable Alpha 1 application | Not built yet |
| Public release | Not available |

For the live execution order, see the [roadmap issue](https://github.com/rps321321/solpaper/issues/1) and [`IMPLEMENTATION_PLAN.md`](IMPLEMENTATION_PLAN.md).

## Roadmap

| Stage | Intended result |
|---|---|
| **Alpha 1** | Tray app, movable desktop widgets, Pomodoro UI, persistent layout, and local-folder wallpapers |
| **Alpha 2** | Read-only Google Calendar connection, privacy modes, offline cache, and agenda widget |
| **Beta** | Reliability work and, only if it is worth the policy and maintenance cost, one remote wallpaper provider |
| **v1** | Installable and documented Windows 11 release with security, privacy, accessibility, recovery, and release evidence |

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

# Open the current placeholder host.
cargo run -p solpaper-app

# Create the host, pump briefly, and exit.
cargo run -p solpaper-app -- --smoke
```

The interactive command currently opens a placeholder surface. It does not yet provide the tray, Pomodoro widget, Calendar, or wallpaper workflow described above.

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

Solpaper is available under the [MIT License](LICENSE).
