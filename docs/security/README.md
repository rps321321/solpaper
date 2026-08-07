# Security

**Issue:** [#36](https://github.com/rps321321/solpaper/issues/36)  
**Pack:** [`deterministic-execution-blueprint.md` § #36](../engineering/deterministic-execution-blueprint.md)  
**Related:** ADR-0005 storage · ADR-0007 IPC · [#6](https://github.com/rps321321/solpaper/issues/6) OAuth · [#37](https://github.com/rps321321/solpaper/issues/37) privacy · [#38](https://github.com/rps321321/solpaper/issues/38) supply chain · [#40](https://github.com/rps321321/solpaper/issues/40) diagnostics · [#45](https://github.com/rps321321/solpaper/issues/45) maintenance · root [`SECURITY.md`](../../SECURITY.md)

Security architecture is defined **before** Calendar, remote content, autostart, and install expand the attack surface. It is not release-stage polish.

| Document | Purpose |
|----------|---------|
| [threat-model.md](./threat-model.md) | Assets, actors, boundaries, abuse cases, mitigations, residual risks |
| [data-flow.md](./data-flow.md) | Data-flow and trust-boundary diagrams |
| [external-input-controls.md](./external-input-controls.md) | Size, validation, timeout, error, retry, log, recovery per external input |
| [requirements-mapping.md](./requirements-mapping.md) | Requirements mapped to #6, #20, #21, #22, #23, #24 |
| [pr-checklist.md](./pr-checklist.md) | HIGH-risk PR security review checklist |
| [supply-chain.md](./supply-chain.md) | License, dependency admission, cargo-deny/audit, SBOM, provenance (#38) |
| [asset-licenses.md](./asset-licenses.md) | Non-code asset license register |
| [release-manifest.schema.md](./release-manifest.schema.md) | Candidate release-manifest.json fields |

## Hard rules

1. Follow blueprint #36 **DEFAULT** controls unless new primary-source evidence forces a recorded deviation.
2. Secrets live only in Windows Credential Manager (ADR-0005)—never settings, SQLite, logs, git, issues, or prompts.
3. Every externally controlled input has documented bounds and failure behavior.
4. No updater and no general local IPC in v1 without ADR + model update.
5. Security-policy and residual-risk acceptance are **HIGH** (human merge). Weakening credential policy or accepting critical vulns is **CRITICAL** (human-only).
6. Do not invent private security contact emails; owner configures reporting in `SECURITY.md`.

## Product locks that constrain security

- Local-first Windows 11 x64 Runtime; no Solpaper cloud backend.
- Read-only Google Calendar when connected; privacy projection before all surfaces.
- Local wallpapers first; at most one remote provider in v1.
- Documented Win32 preferred; WorkerW/Progman never sole architecture.
