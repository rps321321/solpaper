# Agent instructions — solpaper

Stable product and safety rules for autonomous and human development. For workflow details see `docs/agents/`. For roadmap status see GitHub Issue #1, `IMPLEMENTATION_PLAN.md`, and `DEV_STATE.md`.

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
- Architecture remains provisional until Issue #18’s overlay spike is complete.
- Local wallpapers precede remote providers.
- At most one remote provider may enter v1.

## Safety and process

- Do not store secrets in source, config, SQLite, logs, issues, or PRs.
- Do not push directly to `main`.
- Do not force-push.
- Do not destroy unrelated working-tree changes.
- Do not claim a test passed unless it was executed.
- Do not ask the owner routine implementation questions.
- Choose the smallest, safest, and most reversible reasonable option.
- GitHub Issue #1 is the canonical roadmap when repository mirrors disagree.

## Build and test (production workspace)

When a production Cargo workspace exists, run:

```powershell
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Spike crates use equivalent checks scoped to that crate. Do not invent a production workspace before Issues #17 and #18 are resolved.

## Autonomous iteration

Use the `solpaper-dev-loop` skill (`/solpaper-dev-loop`) for one bounded development iteration. Persistent memory is GitHub plus `IMPLEMENTATION_PLAN.md` and `DEV_STATE.md` — not conversation history.
