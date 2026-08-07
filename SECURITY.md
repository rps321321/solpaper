# Security policy

Solpaper is a local-first Windows 11 desktop-surface application. This file is the public security reporting and support outline required by Issue [#36](https://github.com/rps321321/solpaper/issues/36) and maintenance pack [#45](https://github.com/rps321321/solpaper/issues/45).

**Architecture and controls:** [`docs/security/`](docs/security/) (threat model, input bounds, HIGH-risk PR checklist).

## Supported versions

| Version | Support |
|---------|---------|
| Latest stable release | Full fixes |
| Previous stable minor | Security-only fixes for **90 days** after replacement |
| Pre-release / development builds | Best effort; not production support |
| Unreleased `main` | No stability guarantee |

Exact version numbers will be filled when public releases exist. Until then, treat Git tags and GitHub Releases as the only candidates for “stable.”

## Reporting a vulnerability

**Do not** file public GitHub issues for security vulnerabilities or include exploit details, tokens, or private Calendar data in public trackers.

### Preferred path

1. Use **GitHub private security advisories** for this repository when the owner has enabled them:  
   `https://github.com/rps321321/solpaper/security/advisories/new`
2. If private advisories are not yet available, use an **owner-approved private contact** recorded below.

### Owner-approved private contact

| Field | Value |
|-------|--------|
| Status | **Pending owner configuration** |
| Contact | *Not published yet — do not invent or guess an email* |
| Notes | Owner must enable private advisories and/or name a contact before public release claims (#45, #24) |

Agents and contributors **must not** invent email addresses or alternative contacts.

### What to include

- Solpaper version or commit SHA, Windows build, and install channel if known  
- Description of the issue and impact (confidentiality, integrity, availability)  
- Reproduction steps **without** real OAuth tokens or private calendar contents  
- Whether the issue is already public  

### Response targets (small-project guidance)

| Severity | Acknowledge | Mitigation target |
|----------|-------------|-------------------|
| Critical | ≤ 48 hours | ≤ 7 days |
| High | ≤ 3 days | ≤ 14 days |
| Medium | ≤ 7 days | Next planned release |
| Low | Backlog / roadmap | As scheduled |

These are **targets**, not contractual SLAs. Incident authority for signing-key compromise, malicious dependency, leaked OAuth credential, and repository compromise is **human** (#45).

## Security-sensitive product rules (summary)

- **No Solpaper cloud backend** in v1; network use is limited to user-configured integrations (e.g. Google Calendar, optional remote wallpaper).
- **Secrets:** OAuth refresh tokens in **Windows Credential Manager** only—not config files, SQLite, logs, or git ([ADR-0005](docs/adr/0005-storage-split.md)).
- **OAuth:** desktop installed-app flow, system browser, PKCE, loopback `127.0.0.1`, no callback secret logging (see threat model).
- **No general local IPC** and **no auto-updater** in v1 without a new ADR and threat-model update.
- **Privacy:** Calendar titles are privacy-projected before UI, accessibility trees, notifications, and logs.

## Dependency and release integrity

Supply-chain policy: [`docs/security/supply-chain.md`](docs/security/supply-chain.md) (Issue [#38](https://github.com/rps321321/solpaper/issues/38)).

- Project license: **MIT** ([`LICENSE`](LICENSE)); Cargo metadata matches.
- CI runs `cargo deny` and `cargo audit`; `Cargo.lock` is committed; builds use `--locked`.
- Release candidates carry hashes, CycloneDX SBOM, third-party notices, and a release manifest (`signing_state: unsigned` unless a human signed).
- Public release, signing keys, and credential-policy weakening are **human-only** gates ([agent governance](docs/engineering/agent-governance.md)).

## Safe harbor for good-faith research

Good-faith security research that follows this policy and avoids privacy harm, service abuse, and data destruction is welcome. Do not access other users’ data or disrupt production systems you do not own.

## Updates

Material changes to this policy or to the threat model should land via pull request with risk class **HIGH** (security policy) and human merge approval.
