# Threat model and security architecture

**Issue:** [#36](https://github.com/rps321321/solpaper/issues/36)  
**Status:** initial architecture (pre-Alpha 1 local-runtime model; Calendar-specific controls must be complete before #6/#21 merges)  
**Pack source:** [`deterministic-execution-blueprint.md` § #36](../engineering/deterministic-execution-blueprint.md)  
**Related:** ADR-0002 process · ADR-0005 storage · ADR-0007 IPC deferred · [#6](https://github.com/rps321321/solpaper/issues/6) OAuth · [#37](https://github.com/rps321321/solpaper/issues/37) privacy · [#38](https://github.com/rps321321/solpaper/issues/38) supply chain · [#40](https://github.com/rps321321/solpaper/issues/40) diagnostics · [#45](https://github.com/rps321321/solpaper/issues/45) maintenance

## Purpose

Identify assets, trust boundaries, actors, abuse cases, and required controls **before** Calendar OAuth, remote content, autostart, and installation expand the attack surface. This document is the decision store for security architecture defaults that implementers and HIGH-risk PR reviewers must follow.

Governance: security-policy changes and residual-risk acceptance are **HIGH** (verified PR, human merge). Accepting a known critical vulnerability or weakening credential policy is **CRITICAL** (human-only).

## Authority

| Rule | Policy |
|------|--------|
| Pack defaults | Blueprint § #36 LOCKED/DEFAULT; this file records them as implementable requirements |
| Storage of secrets | ADR-0005 — Windows Credential Manager only |
| No general local IPC in v1 | ADR-0007 |
| Privacy projection | Blueprint § #37 (same projection before UI, UIA, logs, notifications) |
| HTTP transport (Calendar) | Blueprint § #21 — `reqwest` + rustls, single stack |
| NFR numeric budgets | [`non-functional-requirements.md`](../engineering/non-functional-requirements.md) |

Deviation requires new primary-source or repository evidence, an issue-linked rationale, and the applicable risk/human gate.

## Scope and non-goals

### In scope (v1-oriented model)

- Local Runtime on Windows 11 x64 user session (single process).
- OAuth loopback callback and browser launch for Google Calendar.
- Google Calendar API and optional remote wallpaper provider HTTP.
- Local filesystem (settings, layout, cache, logs, user-selected wallpaper folders).
- Windows Credential Manager for refresh tokens.
- Image decode path (local and remote).
- Unsafe Win32/COM adapters.
- Installer/release artifact integrity (cross-reference #38/#39).
- Future activation/IPC channel **if** ever introduced (must update this model first).

### Non-goals (explicit)

- No Solpaper cloud backend; no remote crash telemetry in v1.
- No updater and no general local IPC protocol in v1 (pack #36).
- Not a formal Common Criteria / STRIDE certification package.
- Not legal certainty for Google Limited Use or third-party image licenses (#42).
- Physical Windows matrix and independent red-team exercises remain MANUAL/external.

---

## Assets

| ID | Asset | Classification | Location (intended) | Notes |
|----|-------|----------------|---------------------|--------|
| A1 | OAuth refresh token | Secret | Windows Credential Manager `Solpaper/GoogleCalendar/v1/default` | Long-lived; delete on disconnect |
| A2 | OAuth access token | Secret (ephemeral) | Process memory only | Discard on exit; never persisted |
| A3 | PKCE verifier / OAuth state / auth code | Secret (ephemeral) | Memory during connect only | Never logged |
| A4 | Calendar event cache (titles, times, metadata) | Sensitive / private | LocalAppData SQLite or equivalent when justified | Privacy-projected before any outbound surface |
| A5 | Selected calendar IDs / account metadata | Sensitive | Settings or runtime DB | Delete on disconnect with A1/A4 |
| A6 | Settings and widget layout | User config | LocalAppData versioned files | No secrets (ADR-0005) |
| A7 | Pomodoro session state | User config | LocalAppData / runtime store | No third-party PII |
| A8 | Wallpaper image cache | User media | LocalAppData cache | Generated IDs; size caps |
| A9 | Logs and diagnostic bundles | Operational | LocalAppData logs | Allowlist fields only |
| A10 | Local wallpaper source paths | Path / privacy | Settings | Full personal paths not logged by default |
| A11 | Desktop widget HWNDs / process integrity | Availability + integrity | User session | Single-instance mutex |
| A12 | Release binary + signing state | Integrity | Distribution channel | Signing keys never in repo/CI logs |
| A13 | OAuth client ID | Public-ish config | Build/config metadata | Not a protected secret for desktop apps |
| A14 | Future IPC channel | Critical if added | N/A in v1 | Forbidden until ADR + model update |

---

## Actors

| ID | Actor | Capability |
|----|-------|------------|
| U | Legitimate user | Owns the Windows session; configures Solpaper; connects Calendar |
| L | Local co-user / other account | Separate Windows profiles; may share machine |
| M | Malware in same user session | Can read process memory, LocalAppData, hijack ports if race allows, inject input |
| N | Network attacker (on-path / evil twin Wi‑Fi) | Observe or MITM non-TLS; cannot forge TLS without trusted CA compromise |
| R | Remote content provider / API | Serves JSON, redirects, or images; may be compromised or malicious |
| B | Browser / local process | Completes OAuth; could open crafted loopback URLs |
| S | Supply-chain / dependency maintainer | Compromised crate, Action, or build tool |
| I | Installer / updater adversary | Replaces binary or drops malicious package (no updater in v1 reduces this) |
| A | Automated agent / CI | Must not place secrets in git, issues, logs, or prompts (governance) |

**Primary local assumption:** Solpaper is not a hardened sandbox against same-session malware. Residual risk from M is accepted at the product level; controls aim to raise cost and prevent accidental leakage, not defeat kernel-level attackers.

---

## Trust boundaries

```text
                         ┌─────────────────────────────────────┐
                         │  External / untrusted               │
                         │  Google API · provider API · CDN    │
                         │  system browser · release channel   │
                         └──────────────┬──────────────────────┘
                                        │ HTTPS only (remote)
                                        │ OAuth redirect → loopback
                         ┌──────────────▼──────────────────────┐
                         │  TB1 OS network / TLS stack         │
                         └──────────────┬──────────────────────┘
                                        │
┌──────────────┐         ┌──────────────▼──────────────────────┐
│ TB2 Credential│◄───────│  Solpaper Runtime (user session)    │
│ Manager       │ tokens │  tray · surfaces · domain · workers │
└──────────────┘         │  TB3: domain vs Win32 unsafe adapters│
                         │  TB4: privacy projection boundary   │
                         └──────────────┬──────────────────────┘
                                        │ paths, files, decode
                         ┌──────────────▼──────────────────────┐
                         │  TB5 Local filesystem (LocalAppData │
                         │  + user-selected wallpaper folders) │
                         └─────────────────────────────────────┘
```

| Boundary | Crossing rules |
|----------|----------------|
| **TB1 Network** | HTTPS only for remote content; max 3 redirects; reject private/loopback/link-local redirect targets for remote fetches; bounded sizes/timeouts/retries |
| **TB2 Credential Manager** | Refresh token only; never settings/SQLite/logs; delete on disconnect; test targets isolated |
| **TB3 Unsafe Win32/COM** | Narrow `unsafe` with `# Safety`, owned lifetimes, HRESULT mapping, thread-affinity checks |
| **TB4 Privacy projection** | Apply Busy/Private rules **before** UI, UIA, notifications, logs, clipboard/export |
| **TB5 Filesystem** | Canonicalize user paths; generated cache names; atomic settings write; no secret material in files |
| **TB6 Loopback OAuth** | `127.0.0.1` only, pre-bound ephemeral port, first valid callback, path `/oauth/callback` |
| **TB7 Installer/release** | Checksums/SBOM/signing (#38/#39); no auto-updater in v1 |
| **TB8 Future IPC** | Closed in v1; opening requires ADR + this model update |

---

## Data-flow summary

Detailed mermaid and per-flow notes: [`data-flow.md`](./data-flow.md).

| Flow | Data | Trust notes |
|------|------|-------------|
| F1 Connect Calendar | Browser ↔ loopback code ↔ token exchange | PKCE S256, state, 120 s, 8 KiB headers; no query logs |
| F2 Calendar sync | HTTPS JSON → normalized `AgendaItem` → store → UI | Privacy projection at boundary; size/page caps |
| F3 Local wallpaper | User folder paths → enumerate → decode → `IDesktopWallpaper` | Path canonicalize; compressed/decode limits; keep current on failure |
| F4 Remote wallpaper (if retained) | HTTPS image → cache ID → decode → apply | Same HTTP + image limits; no provider string in executable paths |
| F5 Settings/layout | Memory ↔ LocalAppData atomic write | No secrets; corrupt → preserve + defaults |
| F6 Logs/diagnostics | Typed errors → rotated files / user bundle | Field allowlist; user-initiated bundle only |
| F7 Credential load/save | Runtime ↔ Credential Manager | A1 only on disk; A2 memory |

---

## Abuse cases and mitigations

Controls below are **DEFAULT** from pack #36 unless labeled otherwise. Implementation issues (#6, #20, #21, #22, #23, #24) must not weaken them without a recorded deviation.

### OAuth and credentials

| ID | Abuse case | Impact | Mitigation |
|----|------------|--------|------------|
| AC-OAuth-01 | CSRF / forged callback | Attacker-bound account | Random 32-byte `state`; reject mismatch |
| AC-OAuth-02 | Auth code interception on loopback | Token theft | PKCE S256; pre-bound `127.0.0.1:0`; first valid callback only |
| AC-OAuth-03 | Port race / hijack before bind | Attacker receives code | Bind listener **before** opening browser |
| AC-OAuth-04 | Oversized or slow callback HTTP | DoS / parser abuse | 8 KiB request line+headers; 120 s timeout |
| AC-OAuth-05 | Logging secrets | Token/code leak in support | Never log callback URL, query, code, verifier, state, tokens |
| AC-OAuth-06 | Refresh token in config/DB | Disk leakage | Credential Manager only; access token memory-only |
| AC-OAuth-07 | Uncontrolled refresh loop | Account lock / noise | `invalid_grant` → `ReconnectRequired`; no tight loop |
| AC-OAuth-08 | Disconnect leaves secrets | Residual account access | Best-effort revoke, then local delete regardless |

### Remote HTTP and images

| ID | Abuse case | Impact | Mitigation |
|----|------------|--------|------------|
| AC-HTTP-01 | Cleartext or downgrade | Token/content MITM | HTTPS only |
| AC-HTTP-02 | Open redirect to internal hosts | SSRF-like fetch | Max 3 redirects; final target HTTPS; reject loopback/private/link-local for remote content |
| AC-HTTP-03 | Huge response body | Disk/memory exhaustion | Bounded response/download (NFR PERF-WALL-02, Calendar JSON bounds in implementers) |
| AC-HTTP-04 | Retry storm | Network/CPU DoS self | Bounded retries/backoff (NFR PERF-NET-*) |
| AC-IMG-01 | Bomb compressed image | Memory exhaustion | Compressed-size + decoded-pixel limits (PERF-WALL-01/03) before full allocation where decoder permits |
| AC-IMG-02 | Malicious decoder crash | Availability | Decode failure keeps current wallpaper; typed error; no retry loop |
| AC-PATH-01 | Path traversal / provider path injection | Read/write unexpected files | Canonicalize user-selected locals; cache names = generated IDs/hashes; never concatenate provider/user strings into executable paths |

### Local storage, logs, multi-user

| ID | Abuse case | Impact | Mitigation |
|----|------------|--------|------------|
| AC-LOG-01 | Event titles/tokens in logs | Privacy breach | Field allowlist by construction; unit tests |
| AC-LOG-02 | Diagnostic zip over-share | Secret/PII export | User-initiated; previewable; exclude tokens, titles, raw DB, full paths |
| AC-FS-01 | Corrupt settings wipe | Data loss | Atomic replace + `.bak`; preserve corrupt with timestamp |
| AC-FS-02 | Other local user reads profile | Cross-account read | Rely on Windows profile ACLs; no world-writable secret paths |
| AC-PRIV-01 | Busy/Private leak via UIA/notification | Title disclosure | Single privacy projection before all surfaces (#37, #41) |

### Win32, process, install

| ID | Abuse case | Impact | Mitigation |
|----|------------|--------|------------|
| AC-WIN-01 | Unsound `unsafe` / COM use-after-free | RCE / crash | Narrow wrappers, `# Safety`, owned types, HRESULT, thread affinity |
| AC-WIN-02 | WorkerW-only shell parenting | Undocumented fragility | ADR-0001: Approach A default; never sole architecture |
| AC-PROC-01 | Multiple instances | Duplicate surfaces/tray | Single-instance mutex |
| AC-INST-01 | Malicious replacement binary | Full compromise | No updater in v1; release checksums/signing (#38/#39); human release authority |
| AC-IPC-01 | Unauthenticated local IPC | Remote control of tray | No general IPC in v1 (ADR-0007) |

### Dependency and build

| ID | Abuse case | Impact | Mitigation |
|----|------------|--------|------------|
| AC-DEP-01 | Malicious crate/Action | Supply-chain RCE | #38: lockfile, audit/deny, pinned Actions SHAs, SBOM |
| AC-DEP-02 | Agent places secrets in tree | Credential leak | Governance + PR checklist; no secrets in fixtures |

---

## Required control summary (pack #36)

Every **externally controlled input** must document maximum size, parser/validation, timeout, error category, retry policy, log policy, and user-facing recovery. Catalog: [`external-input-controls.md`](./external-input-controls.md).

| Area | Control (DEFAULT) |
|------|-------------------|
| OAuth loopback | `127.0.0.1` only; pre-bound ephemeral port; PKCE S256; random state; first valid callback; path `/oauth/callback`; 8 KiB headers; 120 s timeout; no callback query logging |
| Remote HTTP | HTTPS only; ≤3 redirects; no private/loopback/link-local redirect targets; bounded body/time/retries |
| Paths | Canonicalize user-selected files; generated cache IDs; never provider strings as executable paths |
| Images | Compressed + decoded pixel limits; decode failure leaves wallpaper unchanged |
| Credentials / titles | Allowlist log fields; redact by construction |
| Win32/COM | Narrow `unsafe`, Safety docs, owned lifetimes, HRESULT, thread affinity |
| Product surface | No updater; no general local IPC in v1 |
| Risk class | Security-sensitive additions are **HIGH** under governance |

---

## Residual risks

| ID | Risk | Why residual | Treatment |
|----|------|--------------|-----------|
| RR-01 | Same-session malware reads memory/CM | OS shared session model | Accept for desktop app class; document; no false “secure enclave” claim |
| RR-02 | Browser malware completes OAuth as user | Browser is trusted for OAuth UX | Accept; PKCE limits code replay without verifier |
| RR-03 | Compromised Google or provider account/API | Third-party integrity | Scope minimization (read-only Calendar); local disable path |
| RR-04 | Image decoder 0-days | Native codec surface | Bounds + keep current wallpaper; prefer maintained crates; update via #38 |
| RR-05 | LocalAppData readable to same user malware | Profile FS | Accept; secrets only in CM |
| RR-06 | No auto-update → stale vulnerable binary | Product choice (no updater v1) | Document; human release cadence; #45 incident process |
| RR-07 | SSD secure erase incomplete on Purge | Media physics | Document honest deletion semantics (#37) |
| RR-08 | Human accepts high residual without review | Process failure | HIGH residual acceptance requires human approval on issue/PR |

**Human gate:** accepting **high** residual risks (beyond this table’s documented acceptances) requires explicit owner approval. Routine analysis and control implementation may be autonomous under governance.

---

## Security requirements by roadmap issue

Mapping tables: [`requirements-mapping.md`](./requirements-mapping.md).

| Issue | Security obligation |
|------:|---------------------|
| #20 Alpha 1 | Local path canonicalize; image bounds; no network secrets; atomic settings; single-instance |
| #6 OAuth | Full OAuth/CM control set; fake server tests; redaction tests |
| #21 Calendar | HTTP/TLS controls; privacy projection; cache caps; failure isolation |
| #22/#23 Remote wallpaper | HTTP + image + cache controls; provider string hygiene; model update if provider changes |
| #24 v1 RC | Evidence for SEC/PRIV rows; residual risks listed; no silent weaken |
| #38 Supply chain | Audit/deny/SBOM/lockfile (peer pack) |
| #40 Diagnostics | Allowlist logs; redacted bundles |
| #37 Privacy | Field inventory; disconnect purge |
| #45 Maintenance | `SECURITY.md` reporting path; incident authority human |

---

## Acceptance criteria trace (Issue #36)

| Criterion | Where satisfied |
|-----------|-----------------|
| Every externally controlled input has size, validation, failure, logging rules | [`external-input-controls.md`](./external-input-controls.md) |
| OAuth: PKCE, state/CSRF, loopback binding, cancellation, revocation analysis | Abuse cases AC-OAuth-* + input table + #6 mapping |
| Remote images cannot cause unbounded memory/disk | AC-IMG-*, AC-HTTP-03, NFR PERF-WALL-* |
| Secrets excluded from standard logs/diagnostics by testable policy | AC-LOG-*, allowlist rules; unit tests required at implement time |
| Residual risks and non-goals explicit | Sections above |

---

## Review and maintenance

- HIGH-risk PR checklist: [`pr-checklist.md`](./pr-checklist.md).
- Public reporting outline: repository root [`SECURITY.md`](../../SECURITY.md).
- Update this model when adding: updater, IPC, second network stack, new provider, new secret type, or installer privilege change.
- Do not invent private contact emails; owner enables GitHub private security advisories and names contact in `SECURITY.md` (#45).
