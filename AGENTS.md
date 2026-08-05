# Agent instructions — solpaper

Stable product and safety rules for autonomous and human development. For workflow details see `docs/agents/`. For roadmap status see GitHub Issue #1, `IMPLEMENTATION_PLAN.md`, and `DEV_STATE.md`.

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

## Change-risk classes (summary)

| Class | Agent merge authority |
|-------|------------------------|
| **LOW** | Docs/format/test-only/plan-state → auto-merge after focused review + applicable checks |
| **MEDIUM** | Ordinary impl + verifier `VERIFIED` + green CI → may auto-merge |
| **HIGH** | Secrets/OAuth/autostart/installer/unsafe Win32/destructive migrations/security policy → verified PR only; **no auto-merge** |
| **CRITICAL** | Public release, signing keys, force-push, credential-policy weakening, fundamental product reduction → **human-only; do not execute** |

Full tables, human-only gates, runaway stops, and kill-switch: `docs/engineering/agent-governance.md`.

## Safety and process

- Do not store secrets in source, config, SQLite, logs, issues, or PRs.
- Do not push directly to `main`.
- Do not force-push.
- Do not destroy unrelated working-tree changes.
- Do not claim a test passed unless it was executed.
- Do not ask the owner routine implementation questions.
- Choose the smallest, safest, and most reversible reasonable option.
- GitHub Issue #1 is the canonical product roadmap when repository mirrors disagree; #30 is the engineering-system map.
- Claim an atomic issue lease before editing; `DEV_STATE.md` alone is not a lease.
- Max one active builder and one active implementation PR.
- Max two verifier cycles per unit; stop after three materially identical failures.
- Declare risk class on every PR (`.github/PULL_REQUEST_TEMPLATE.md`).

## Build and test (production workspace)

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

Use the `solpaper-dev-loop` skill (`/solpaper-dev-loop`) for one bounded development iteration. Persistent memory is GitHub plus `IMPLEMENTATION_PLAN.md` and `DEV_STATE.md` — not conversation history. Governance doc and leases bind unattended behaviour.
