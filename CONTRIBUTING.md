# Contributing to Solpaper

Thanks for your interest. Solpaper is a **local-first Windows 11** desktop-surface application in Rust. Product locks and agent rules live in [`AGENTS.md`](./AGENTS.md); the roadmap is [Issue #1](https://github.com/rps321321/solpaper/issues/1).

## License of contributions

Solpaper is licensed under the **MIT License** (see [`LICENSE`](./LICENSE)).

**Inbound = outbound:** by submitting a contribution (pull request, patch, or other submission), you offer it under the **MIT License** on the same terms as the project, and you confirm you have the right to do so.

- **No Contributor License Agreement (CLA)** is required at this time.
- **No Developer Certificate of Origin (DCO)** sign-off is required at this time.

The owner may introduce DCO or CLA later via an explicit repository change before inviting broad external contribution. Dual-licensing the project (for example MIT OR Apache-2.0) is an **owner decision** and is **not** claimed until Apache-2.0 license text and metadata are deliberately added.

## Dependency and supply-chain expectations

Read [`docs/security/supply-chain.md`](./docs/security/supply-chain.md) before adding crates or changing CI.

- Keep `Cargo.lock` committed; use `cargo … --locked` in CI-facing changes.
- Justify every **new runtime dependency** in the PR (need, alternatives, maintenance, license, unsafe/native, features, transitive cost, removal boundary).
- Prefer zero new dependencies; governance allows at most one per unit without separate justification.
- Allowed dependency licenses and denied licenses are enforced with `cargo deny` (`deny.toml`).

## Security and privacy

- Do not commit secrets, OAuth tokens, or private Calendar data.
- Report vulnerabilities per [`SECURITY.md`](./SECURITY.md) (not public issues for exploits).
- High-risk areas (Credential Manager, OAuth, autostart, installer, unsafe Win32, security policy) require careful review and human merge approval.

## Development checks

On Windows 11 with the workspace MSRV toolchain:

```powershell
cargo fmt --all -- --check
cargo check --workspace --all-targets --locked
cargo test --workspace --all-targets --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo deny check advisories bans licenses sources
cargo audit
```

Governance tooling:

```powershell
powershell -NoProfile -File scripts/tests/agent-lease.Tests.ps1
```

## Pull requests

Use [`.github/PULL_REQUEST_TEMPLATE.md`](./.github/PULL_REQUEST_TEMPLATE.md). Declare change-risk class (LOW / MEDIUM / HIGH / CRITICAL). Do not open CRITICAL work autonomously.

## Code of collaboration

Be precise, keep diffs small, and prefer reversible changes. Physical Windows evidence claims need named-environment evidence or explicit manual debt.
