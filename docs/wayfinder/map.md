# Solpaper roadmap mirror

**Canonical roadmap:** [GitHub Issue #1](https://github.com/rps321321/solpaper/issues/1)  
**Engineering-readiness map:** [GitHub Issue #30](https://github.com/rps321321/solpaper/issues/30)  
**Detailed execution ledger:** [`IMPLEMENTATION_PLAN.md`](../../IMPLEMENTATION_PLAN.md)

This file is the in-repository mirror of the public roadmap. GitHub issues remain authoritative for live state. Last refreshed: **2026-08-07**.

## Product destination

Solpaper is a local-first Windows 11 desktop companion written in Rust. It is intended to show a few useful things directly on the desktop without behaving like a full dashboard or replacement shell.

The planned v1 experience includes:

- a Pomodoro widget;
- a read-only Google Calendar agenda;
- simple wallpaper management, beginning with local folders;
- movable, resizable widgets with a passive Normal Mode and an explicit Edit Mode;
- tray and settings controls;
- local storage, bounded diagnostics, and no Solpaper cloud backend.

Wallpaper management is one part of the product, not the product's central architecture.

## Current verified state

| Work | State |
|---|---|
| Product definition (#17) | Complete |
| Windows overlay feasibility (#18) | Complete; per-widget top-level windows are the default direction |
| Agent governance and human-only risk gates (#31) | Complete |
| Production Rust workspace and ADRs (#16) | Complete |
| Windows CI and protected `main` (#32) | Complete |
| Pomodoro state machine (#19) | Complete |
| Test and Windows evidence strategy (#33) | Complete; physical evidence remains open |
| Accessibility requirements (#41) | Complete; physical assistive-technology sessions remain open |
| UX flows and interaction specification (#34) | Complete; human usability session remains open |
| Non-functional requirements and budgets (#35) | Complete; named-hardware performance evidence remains open |
| Threat model and security architecture (#36) | In review in PR #70; HIGH-risk and human-merge only |
| Usable Alpha 1 application (#20) | Not started |

The repository currently contains a compilable Windows scaffold, basic settings and layout foundations, and a tested platform-neutral Pomodoro engine. It does **not** yet contain the finished tray, widget UI, Calendar connection, wallpaper workflow, installer, or public release.

## Current execution order

The deterministic foundation sequence is:

1. **#36** — merge the reviewed threat model and security architecture.
2. **#38** — establish dependency, license, SBOM, and supply-chain controls.
3. **#40** — define bounded logging, diagnostics, and crash recovery.
4. **#5 and #7** — complete the Windows wallpaper adapter and tray/runtime decisions.
5. **#13** — freeze measurable acceptance rows and the human-approved v1 boundary.
6. **#20** — integrate the first useful offline Alpha.

Work after Alpha 1 follows the product stages below.

## Product stages

### Alpha 1 — useful offline foundation

Issue: [#20](https://github.com/rps321321/solpaper/issues/20)

- one Windows user-session runtime and tray;
- passive desktop widgets plus Edit Mode;
- persistent size, position, monitor binding, and opacity;
- Pomodoro widget and recovery behaviour;
- local-folder wallpaper application;
- safe settings and basic diagnostics.

### Alpha 2 — read-only Calendar

Issues: [#6](https://github.com/rps321321/solpaper/issues/6), [#37](https://github.com/rps321321/solpaper/issues/37), [#42](https://github.com/rps321321/solpaper/issues/42), and [#21](https://github.com/rps321321/solpaper/issues/21)

- system-browser OAuth with least-privilege read-only scopes;
- refresh tokens protected by Windows Credential Manager;
- selected calendars, recurring and all-day event handling;
- offline cache with clear stale state;
- full-title, private-detail, and Busy-only privacy modes.

### Beta — reliability and optional remote wallpaper source

Issues: [#22](https://github.com/rps321321/solpaper/issues/22) and [#23](https://github.com/rps321321/solpaper/issues/23)

A remote wallpaper provider is optional, not assumed. At most one provider may enter v1, and only after its API stability, terms, attribution, content defaults, caching rules, and maintenance cost are acceptable.

### v1 — packaged and evidenced release

Issues: [#24](https://github.com/rps321321/solpaper/issues/24), [#44](https://github.com/rps321321/solpaper/issues/44), and [#45](https://github.com/rps321321/solpaper/issues/45)

A stable release requires an installable candidate, completed acceptance evidence, documented privacy and security behaviour, accessibility checks, upgrade and rollback rules, external testing, and explicit human approval.

## Locked principles

- Windows 11 x64 and Rust for v1.
- Local-first operation with no Solpaper cloud backend.
- A normal user-session application, not a Windows service.
- Live information is rendered as UI and is never baked into wallpaper images.
- Documented Win32 APIs are preferred; undocumented Explorer techniques cannot be the only supported path.
- Tray, Edit Mode, and visual settings are the primary interface; a TUI is post-v1.
- Calendar access is read-only and least-privilege.
- Credentials and private Calendar content must not enter ordinary configuration, logs, issues, pull requests, screenshots, or test evidence.
- Local wallpapers come before remote-provider complexity.
- No hidden telemetry or remote crash upload in v1 without a separate human-approved decision.
- Public release, signing-key use, destructive migration approval, and acceptance of critical security risk are human-only actions.

## Manual evidence that remains open

Automation cannot close these items by assertion:

- sleep/resume and lock/unlock;
- Explorer restart;
- single and multiple monitors;
- mixed DPI, monitor hotplug, reorder, and primary-monitor changes;
- Win+D and fullscreen behaviour;
- prolonged idle and named-hardware performance measurements;
- Narrator, keyboard, high-contrast, and scaling sessions;
- human usability sessions.

Evidence is tracked in [`docs/testing/manual-debt-register.md`](../testing/manual-debt-register.md).

## Out of scope for v1

- Linux or macOS;
- Windows service deployment;
- Solpaper accounts, cloud sync, or cross-machine profiles;
- mobile companion or remote control;
- Google Calendar write access;
- AI-generated wallpapers;
- a plugin marketplace;
- a required terminal interface;
- more than one remote wallpaper provider;
- automatic screen-sharing detection without a reliable documented Windows mechanism.

## Completion rule

The roadmap closes only when #24 demonstrates the required #13 acceptance criteria with a packaged candidate, #44 records external validation and explicit human release approval, #45 establishes post-release ownership, and all release-blocking engineering work in #30 is complete or explicitly waived by a human with a recorded reason.
