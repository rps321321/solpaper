# Agent instructions — solpaper

Stable product and safety rules for autonomous and human development. For workflow details see `docs/agents/`. For roadmap status see GitHub Issues #1 and #30, `IMPLEMENTATION_PLAN.md`, and `DEV_STATE.md`.

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
- Issue #18's spike recommends independent widget-sized HWNDs; production architecture remains provisional until accepted ADRs under Issue #16.
- Local wallpapers precede remote providers.
- At most one remote provider may enter v1.

## Safety and process

- Do not store secrets in source, config, SQLite, logs, issues, PRs, screenshots, fixtures, or evidence.
- Do not push directly to `main`.
- Do not force-push or rewrite shared history.
- Do not destroy unrelated working-tree changes.
- Do not claim a test passed unless it was executed.
- Do not ask the owner routine implementation questions.
- Choose the smallest, safest, and most reversible reasonable option.
- GitHub Issue #1 is the canonical product roadmap.
- GitHub Issue #30 is the canonical engineering-system roadmap.
- Respect the change-risk and human-only gates defined by Issue #31.
- Manual Windows evidence remains open until actually performed on a named environment.

## Engineering skills

The Grok skill map is documented in `docs/agents/solpaper-engineering-skills.md`.

- `solpaper-dev-loop` is a thin scheduled controller: recover, choose one unit, route, persist, stop.
- `solpaper-engineering` routes manual requests.
- Focused skills own implementation, ticketing, research, prototypes, TDD, diagnosis, domain/design, review, and merge-conflict resolution.
- Skills consume this file, `CONTEXT.md`, ADRs, the originating issue/spec, and repository state rather than restating product rules.
- Completed implementation must pass independent standards and spec review through `solpaper-review`.

## Build and test

When a production Cargo workspace exists, run:

```powershell
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Spike crates use equivalent scoped checks. Do not invent a production workspace outside the accepted Issue #16 architecture.

## Autonomous iteration

Use `/solpaper-dev-loop` for one bounded development iteration. Persistent memory is GitHub plus `IMPLEMENTATION_PLAN.md`, `DEV_STATE.md`, `CONTEXT.md`, and ADRs — not conversation history.