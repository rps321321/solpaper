# Security requirements mapped to roadmap issues

**Issue:** [#36](https://github.com/rps321321/solpaper/issues/36)  
**Sources:** [`threat-model.md`](./threat-model.md), [`external-input-controls.md`](./external-input-controls.md), blueprint packs

These rows are **implementation gates** for later issues. #13 acceptance matrix should import SEC/PRIV IDs; #24 validates evidence, it does not invent weaker controls.

## #20 — Alpha 1 (local runtime)

| Req ID | Requirement | Control refs | Evidence intent |
|--------|-------------|--------------|-----------------|
| SEC-A1-01 | No OAuth tokens or Calendar private data in Alpha 1 paths | Non-goal for #20 network secrets | Code review + grep policy |
| SEC-A1-02 | User wallpaper folders canonicalized; files under selected roots | AC-PATH-01, local path table | Unit path tests |
| SEC-A1-03 | Local image compressed ≤ 50 MiB; decode ≤ 100 MP; fail keeps wallpaper | PERF-WALL-01/03/06 | Unit + adapter fake |
| SEC-A1-04 | Cache/generated names never raw user path concatenation into exec paths | AC-PATH-01 | Unit |
| SEC-A1-05 | Settings atomic write + corrupt preserve | AC-FS-01, PERF-STOR-* | Storage integration |
| SEC-A1-06 | Single-instance mutex; no general IPC | AC-PROC-01, ADR-0007 | Unit + manual tray |
| SEC-A1-07 | Narrow unsafe Win32 only in windows crate with Safety docs | AC-WIN-01, TB3 | Clippy + review checklist |
| SEC-A1-08 | Autostart (if landed under #7) is HIGH risk; no silent enable without UX | Governance HIGH | PR risk class |

## #6 — Google OAuth and credential storage

| Req ID | Requirement | Control refs | Evidence intent |
|--------|-------------|--------------|-----------------|
| SEC-OAUTH-01 | Desktop app OAuth, system browser, read-only scopes only | Blueprint #6 LOCKED | Code + config |
| SEC-OAUTH-02 | Pre-bind `127.0.0.1:0`; callback `/oauth/callback` | AC-OAuth-02/03 | Fake server tests |
| SEC-OAUTH-03 | PKCE S256 (32-byte verifier) + independent 32-byte state | AC-OAuth-01/02 | Unit crypto params |
| SEC-OAUTH-04 | First valid callback only; 8 KiB headers; 120 s timeout | AC-OAuth-04 | Fake server |
| SEC-OAUTH-05 | Never log callback URL/query/code/state/verifier/tokens | AC-OAuth-05 | Redaction tests |
| SEC-OAUTH-06 | Refresh token only in Credential Manager target `Solpaper/GoogleCalendar/v1/default` | AC-OAuth-06, ADR-0005 | Integration smoke + review |
| SEC-OAUTH-07 | Access token memory-only; discard on exit | A2 | Design + review |
| SEC-OAUTH-08 | Disconnect: best-effort revoke then local delete always | AC-OAuth-08 | Unit sequence |
| SEC-OAUTH-09 | `invalid_grant`/missing → `ReconnectRequired`; no refresh storm | AC-OAuth-07 | Unit |
| SEC-OAUTH-10 | No real credentials in git, CI logs, agent prompts, fixtures | Governance | PR checklist |

**Gate:** Calendar-specific model controls above must be complete before #6/#21 implementation merges.

## #21 — Calendar Alpha 2

| Req ID | Requirement | Control refs | Evidence intent |
|--------|-------------|--------------|-----------------|
| SEC-CAL-01 | Single HTTPS stack (`reqwest` rustls); no second HTTP client family | TB1, #21 pack | Dependency + code |
| SEC-CAL-02 | HTTPS only; timeouts 10 s connect / 30 s total | PERF-NET-01/02 | Config tests |
| SEC-CAL-03 | Redirect policy for any URL fetch: ≤3, HTTPS, no private/loopback/link-local | AC-HTTP-02 | Mock redirect tests |
| SEC-CAL-04 | Page/body bounds; 50k instance cap → `CALENDAR_TOO_LARGE` | Input matrix | Unit/integration |
| SEC-CAL-05 | Privacy projection before UI, UIA, notifications, logs, export | TB4, AC-PRIV-01 | Unit + a11y fixtures |
| SEC-CAL-06 | Failure isolation: Calendar errors never kill tray/Pomodoro/wallpaper | F2 diagram | Integration |
| SEC-CAL-07 | Disconnect purge: token, cache, sync tokens, calendar IDs, account metadata | #37 | Unit purge |
| SEC-CAL-08 | Stale after 30 min; keep last committed cache offline | PERF-CAL-02/NET-04 | Unit clock |

## #22 / #23 — Remote wallpaper provider and scheduling

| Req ID | Requirement | Control refs | Evidence intent |
|--------|-------------|--------------|-----------------|
| SEC-REM-01 | HTTPS-only downloads; redirect + SSRF-ish rejection | AC-HTTP-* | Mock tests |
| SEC-REM-02 | Download ≤ 30 MiB; decode limits shared with local | PERF-WALL-02/03 | Bounded reader tests |
| SEC-REM-03 | Cache files use generated IDs; 1 GiB cache cap; pin applied | PERF-WALL-04 | Cache policy tests |
| SEC-REM-04 | Provider/user strings never concatenated into executable paths | AC-PATH-01 | Unit |
| SEC-REM-05 | Decode/apply failure keeps current wallpaper; no retry loop | AC-IMG-02 | Adapter fake |
| SEC-REM-06 | Threat model updated if provider choice or privileges change | Maintenance rule | Docs PR |
| SEC-REM-07 | License/API policy from #42 before selection; Unsplash rejected | #42 pack | Research record |

**Owner gate:** #22 remains recommendation-class for product inclusion; security controls apply if retained.

## #24 — v1 release candidate

| Req ID | Requirement | Control refs | Evidence intent |
|--------|-------------|--------------|-----------------|
| SEC-RC-01 | All SEC/PRIV acceptance rows executed or waived by human | #13 matrix | Evidence pack |
| SEC-RC-02 | Residual risks RR-* acknowledged; no silent control weaken | threat-model residual table | Release notes / go-no-go |
| SEC-RC-03 | Supply-chain: lockfile, audit/deny, SBOM, Action SHA pins | #38 | Release manifest |
| SEC-RC-04 | Signing/release human-only; no agent public release | Governance CRITICAL | Human checklist |
| SEC-RC-05 | `SECURITY.md` reporting path live (owner contact + advisories) | #45 | Repo root file |
| SEC-RC-06 | Installer/uninstall: data preserve default; Purge explicit | PERF-UPG-03 | Release suite |
| SEC-RC-07 | No updater/IPC surprise in v1 binary surface | AC-INST-01, AC-IPC-01 | Feature audit |

## #40 — logging / diagnostics (supportability)

Full OPS rows: [`../operations/diagnostics.md`](../operations/diagnostics.md). Security-relevant gates:

| Req ID | Requirement | Control refs | Evidence intent |
|--------|-------------|--------------|-----------------|
| SEC-OPS-01 | Field allowlist; never tokens/titles in default logs | AC-LOG-01, PERF-LOG-04 | Unit allowlist tests |
| SEC-OPS-02 | Diagnostic bundle user-initiated; exclude secrets/DB/titles | AC-LOG-02, PERF-LOG-03 | Unit + manual preview |
| SEC-OPS-03 | No remote crash upload / telemetry in v1 | Pack #40 owner gate | Feature audit |
| SEC-OPS-04 | Crash markers redacted; no auto-restart loop | PERF-REL-04 | Unit safe-mode policy |

## Cross-cutting (any issue)

| Req ID | Requirement | When |
|--------|-------------|------|
| SEC-X-01 | Security-sensitive PRs declare **HIGH** risk; no auto-merge | Always |
| SEC-X-02 | Use [`pr-checklist.md`](./pr-checklist.md) for OAuth, CM, unsafe Win32, installer, migrations | HIGH PRs |
| SEC-X-03 | New external input → add row to external-input-controls matrix | Before merge |
| SEC-X-04 | Log allowlist tests when logging lands (#40) and for Calendar (#21) | #40 / #21 |

## Seed acceptance IDs for #13

Suggested prefixes `SEC` / `PRIV` (blueprint § #13). Status starts `open` until implemented and evidenced.

| ID | Phase | Scenario (short) | Blocking |
|----|-------|------------------|----------|
| SEC-01 | Alpha 2 | OAuth PKCE + state reject forged callback | Yes |
| SEC-02 | Alpha 2 | Refresh token only in Credential Manager | Yes |
| SEC-03 | Alpha 2 | Logs/diagnostics contain no token/code/state fixtures | Yes |
| SEC-04 | Alpha 1 | Oversized local image rejected; wallpaper unchanged | Yes |
| SEC-05 | Beta | Remote redirect to private IP rejected | Yes if remote |
| SEC-06 | Alpha 1 | Settings corrupt recovery preserves file | Yes |
| PRIV-01 | Alpha 2 | Busy/Private titles not in UIA/notifications/logs | Yes |
| PRIV-02 | Alpha 2 | Disconnect deletes token + calendar cache | Yes |
