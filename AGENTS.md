# Agent instructions — solpaper

Stable product and safety rules for autonomous and human development. For workflow details see `docs/agents/`. For roadmap status see GitHub Issues #1 and #30, `IMPLEMENTATION_PLAN.md`, and `DEV_STATE.md`.

**Governance (enforceable):** [`docs/engineering/agent-governance.md`](docs/engineering/agent-governance.md)  
**Lease tooling:** `scripts/agent-lease.ps1` · store: `.agent/leases/`

## Product locks

- Solpaper is a local-first Windows 11 desktop-surface application written in Rust.
- Wallpaper management is one subsystem, not the product root.
- Pomodoro is required for Alpha 1.
- Read-only Google Calendar is Alpha 2 and intended for v1.
- TUI is deferred until after v1.
- No Solpaper cloud backend.
- Live widgets are not baked into wallpaper images.
- Documented Win32 APIs are preferred.
- WorkerW/Progman must never be the sole supported architecture.
- Architecture ADRs live in `docs/adr/` (Issue #16); Approach A widget HWNDs are default. Manual evidence debt from #18 and accessibility toolkit review remain open.
- Local wallpapers precede remote providers.
- At most one remote provider may enter v1.

## Change-risk classes

| Class | Agent merge authority |
|-------|------------------------|
| **LOW** | Docs, format, test-only, plan/state → auto-merge after focused review and applicable checks |
| **MEDIUM** | Ordinary implementation + verifier `VERIFIED` + green CI → may auto-merge |
| **HIGH** | Secrets, OAuth, autostart, installer, unsafe Win32, destructive migrations, security policy → verified PR only; **no auto-merge** |
| **CRITICAL** | Public release, signing keys, force-push, credential-policy weakening, fundamental product reduction → **human-only; do not execute** |

Full tables, human-only gates, runaway stops, and kill switch: `docs/engineering/agent-governance.md`.

## Safety and process

- Do not store secrets or private Calendar data in source, config, SQLite, logs, issues, PRs, screenshots, fixtures, or evidence.
- Do not push directly to `main`.
- Do not force-push or rewrite shared history.
- Do not destroy unrelated working-tree changes.
- Do not claim a test passed unless it was executed.
- Do not ask the owner routine implementation questions.
- Choose the smallest, safest, and most reversible reasonable option.
- GitHub Issue #1 is the canonical product roadmap; Issue #30 is the engineering-system map.
- Claim an atomic issue lease before editing; `DEV_STATE.md` alone is not a lease.
- Maximum one active builder and one active implementation PR.
- Maximum two verifier cycles per unit; stop after three materially identical failures.
- Declare risk class and lease on every PR through `.github/PULL_REQUEST_TEMPLATE.md`.
- Manual Windows evidence remains open until performed on a named environment.

## Engineering skills

The Grok skill system is documented in `docs/agents/solpaper-engineering-skills.md`.

- `solpaper-dev-loop` is a thin scheduled controller: recover, govern, choose one unit, route, persist, stop.
- `solpaper-engineering` routes manually requested work.
- Focused skills own implementation, ticketing, research, prototypes, TDD, diagnosis, domain/design, review, and conflict resolution.
- Skills consume this file, governance, `CONTEXT.md`, ADRs, the originating issue/spec, leases, and repository state rather than restating product rules.
- Completed implementation uses independent standards and spec review through `solpaper-review`; `solpaper-verifier` aggregates the two reports.
- No skill may bypass governance, lease ownership, the kill switch, CI, manual evidence, or human-only gates.

## Build and test

When a production Cargo workspace exists, run:

```powershell
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Spike crates use equivalent checks scoped to that crate (`spikes/` is excluded from the production workspace).

Governance tooling:

```powershell
powershell -NoProfile -File scripts/tests/agent-lease.Tests.ps1
```

## Autonomous iteration

Use `/solpaper-dev-loop` for one bounded development iteration. Persistent memory is GitHub plus governance, leases, `IMPLEMENTATION_PLAN.md`, `DEV_STATE.md`, `CONTEXT.md`, and ADRs — not conversation history.
