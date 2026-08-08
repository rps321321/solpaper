# Security review checklist for high-risk PRs

**Issue:** [#36](https://github.com/rps321321/solpaper/issues/36)  
**Use when:** PR risk class is **HIGH** (or reviewer upgrades to HIGH), especially OAuth/tokens, Credential Manager, Calendar privacy storage, autostart, installer/updater, substantive `unsafe` Win32, destructive-capable migrations, or security policy docs.

**Authority:** [`agent-governance.md`](../engineering/agent-governance.md) — HIGH may open a verified PR but **must not auto-merge**. CRITICAL work is human-only.

## How to use

1. Author completes the checklist in the PR body or a linked comment.
2. Standards/spec reviewers and verifier confirm unchecked material items.
3. Human merger re-checks residual-risk and secret-handling items before merge.

Mark each item: `[x]` done · `[ ]` not done · `N/A` with one-line reason.

---

## A. Classification and process

- [ ] Risk class is **HIGH** (or CRITICAL stopped and not executed autonomously)
- [ ] Issue lease owner/branch/PR metadata match governance
- [ ] Scope limited to the leased unit; no drive-by secret-policy weaken
- [ ] No force-push; no direct push to `main`
- [ ] Execution-pack defaults followed; deviations recorded with evidence

## B. Secrets and credentials

- [ ] No secrets, tokens, private Calendar payloads, or live credentials in source, fixtures, screenshots, logs, issue text, or PR
- [ ] Refresh tokens only via Credential Manager (or test fake); never settings/SQLite/git
- [ ] Access tokens memory-only if OAuth touched
- [ ] Credential target names documented; test targets isolated and cleaned up
- [ ] Disconnect/purge path deletes credentials when account lifecycle changes

## C. OAuth / network (if touched)

- [ ] Loopback `127.0.0.1` only; listener bound before browser open
- [ ] PKCE S256 + state validation; first valid callback only
- [ ] Header/size/time limits enforced (8 KiB callback headers; 120 s connect; HTTP 10 s/30 s)
- [ ] HTTPS only for remote; redirect count and private/loopback/link-local rejection
- [ ] Bounded retries/backoff; no tight refresh or download loops
- [ ] Callback URL/query/code/state/verifier/tokens never logged

## D. Privacy and logging

- [ ] Privacy projection applied before UI, UIA, notifications, logs, export (Calendar)
- [ ] Log fields allowlisted; redaction by construction
- [ ] Diagnostic bundle exclusions respected if diagnostics touched
- [ ] Automated redaction or allowlist tests added/updated when logging or OAuth changes

## E. Filesystem and images

- [ ] User paths canonicalized; traversal rejected
- [ ] Cache names are generated IDs/hashes
- [ ] Compressed and decoded size limits enforced
- [ ] Decode/apply failure leaves prior wallpaper/settings intact as specified
- [ ] Atomic settings write / corrupt preserve when storage touched

## F. Win32 / unsafe

- [ ] `unsafe` minimized and confined to adapter modules
- [ ] Each unsafe block has `# Safety` documenting invariants
- [ ] COM/Windows lifetimes owned; HRESULT mapped; thread affinity checked where required
- [ ] No new WorkerW/Progman-only architecture dependency

## G. Installer, autostart, IPC, updater

- [ ] Autostart changes are explicit UX + HIGH justification
- [ ] No general local IPC introduced without ADR + threat-model update
- [ ] No updater introduced in v1 without human architecture approval
- [ ] Installer/migration destructive capability called out; human gate if destructive

## H. Supply chain (if deps or CI touched)

- [ ] New runtime dependency justified (need, license, unsafe, features, removal)
- [ ] At most one dependency addition per unit without separate justification
- [ ] `Cargo.lock` updated; Actions pinned to full SHAs when workflows change
- [ ] `cargo deny check` and `cargo audit` considered; deny.toml exceptions only with issue-bound expiring waiver ([supply-chain.md](./supply-chain.md))
- [ ] Project license story unchanged or deliberately updated (MIT `LICENSE` + Cargo `license`)

## I. Threat model maintenance

- [ ] New external inputs added to [`external-input-controls.md`](./external-input-controls.md)
- [ ] New assets/boundaries/abuse cases reflected in [`threat-model.md`](./threat-model.md) when surface expands
- [ ] Residual risk acceptance (beyond documented RR-*) has human approval note

## J. Tests and honesty

- [ ] Focused automated tests cover new failure modes listed in the input matrix
- [ ] Claims of physical Windows behavior backed by evidence paths or listed as MANUAL debt
- [ ] PR template lists tests run, tests not run, security/privacy impact, limitations

---

## Minimal paste block for PR bodies

```markdown
### Security checklist (#36)
- Risk HIGH declared; no secrets in tree
- OAuth/CM/network/privacy/unsafe items: (summary or N/A)
- Threat-model / input-matrix updated: yes/no/N/A
- Residual risks: (none new | link approval)
```
