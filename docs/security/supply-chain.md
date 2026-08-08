# Supply chain, dependency, license, SBOM, and provenance

**Issue:** [#38](https://github.com/rps321321/solpaper/issues/38)  
**Pack:** [`deterministic-execution-blueprint.md` § #38](../engineering/deterministic-execution-blueprint.md)  
**Related:** [ci-policy.md](../engineering/ci-policy.md) · [agent-governance.md](../engineering/agent-governance.md) · [pr-checklist.md](./pr-checklist.md) · root [`LICENSE`](../../LICENSE) · [`SECURITY.md`](../../SECURITY.md)

This document is the project policy for open-source licensing of Solpaper itself, admission of dependencies, automated supply-chain checks, SBOM/third-party notices, release manifests, and emergency dependency response. It is intentionally proportional to a small local-first Windows application with no Solpaper cloud backend.

**Honest scope:** Passing automated scans does **not** prove the application is secure. Scans reduce known classes of risk; residual supply-chain risk always remains (compromised maintainer, novel malware, typosquatting not yet catalogued, Action marketplace compromise).

---

## 1. Project license (Solpaper source)

### Decision (implemented)

| Item | Value |
|------|--------|
| **Project license** | **MIT** |
| **Root grant** | [`LICENSE`](../../LICENSE) (MIT License text) |
| **Cargo workspace `license`** | `MIT` (must match root grant) |
| **Crate metadata** | `license.workspace = true` on all workspace members |
| **Dual MIT OR Apache-2.0** | **Not** claimed. Cargo previously said `MIT OR Apache-2.0` without Apache-2.0 text; that mismatch is **reconciled to MIT**. Dual-license remains an **owner gate** (add `LICENSE-APACHE` + explicit decision before changing metadata). |

Public documentation must describe Solpaper as **MIT-licensed**. Do not imply dual licensing from historical Cargo metadata or comments.

### Contribution licensing

See [`CONTRIBUTING.md`](../../CONTRIBUTING.md).

- **Inbound = outbound:** contributions are offered under the same MIT terms as the project.
- **No CLA** and **no DCO** required by default (smallest mechanism that keeps contribution rights clear for a solo/small project).
- Owner may later require DCO/CLA via a HIGH security/process PR before inviting broad external contribution.

### Non-code assets

Every icon, font, sample image, sound, or bundled third-party document must record:

| Field | Required |
|-------|----------|
| Source URL or provenance | yes |
| Author / rights holder | yes |
| Exact license SPDX (or written permission) | yes |
| Modification status | yes |
| Attribution requirement | yes |
| Redistribution permission | yes |

Current production tree: **no** bundled binary fonts/icons/sample images under `assets/` (register empty). When assets land, add rows to [`docs/security/asset-licenses.md`](./asset-licenses.md) before merge.

Documentation under `docs/` is project content under MIT unless a file states otherwise.

---

## 2. Dependency admission

### Lockfile and toolchain

- **`Cargo.lock` is committed** for the application workspace.
- CI and release builds use **`cargo … --locked`**.
- Workspace **MSRV** (`rust-version` in root `Cargo.toml`) is retained until a deliberate dependency/toolchain PR changes it.
- Prefer **zero** new runtime dependencies per unit; governance default is **at most one** without separate justification ([agent-governance.md](../engineering/agent-governance.md)).

### New runtime dependency justification (PR body)

Every new **direct runtime** dependency (and every substantial new feature enablement on an existing crate) must record:

1. **Need** — what user-visible or security requirement it satisfies  
2. **Alternatives** — stdlib, existing crates, or “none viable”  
3. **Maintenance / ownership** — crates.io owners, recent releases, bus factor  
4. **License** — SPDX; must be on the allow list or have a human exception  
5. **Unsafe / native code** — `unsafe`, C/C++/sys deps, build scripts  
6. **Default features** — which defaults are enabled; disable unused features  
7. **Transitive cost** — approximate new crate count / compile weight  
8. **Removal boundary** — how the dependency can be removed later  

**Security-sensitive dependency classes** (at least **MEDIUM**, often **HIGH** risk class for the PR):

- Network clients, auth/OAuth, cryptography  
- Parsers/decoders for untrusted input (images, JSON from network, installers)  
- Installer / updater / autostart helpers  
- Crates that wrap substantial `unsafe` or Win32 surface  

### Sources

| Source | Policy |
|--------|--------|
| **crates.io** | Allowed by default |
| **git dependency** | Denied by default; requires immutable commit SHA, separate justification, license evidence, and a plan to return to a crates.io release |
| **path** | Workspace members only |
| **unknown registry** | Denied (`cargo deny` sources) |

### Abandoned / yanked / transferred crates

- **Yanked** versions must not appear in `Cargo.lock` for release candidates.  
- **Unmaintained** advisories: treat as high priority to replace; may temporarily remain with documented residual risk.  
- **Unexpected ownership transfer** or typosquat suspicion: freeze upgrades, open a security/maintenance issue (#45 path), prefer removal over silent update.

---

## 3. License policy (dependencies)

### Allowed by default (SPDX)

MIT, Apache-2.0, Apache-2.0 WITH LLVM-exception, BSD-2-Clause, BSD-3-Clause, ISC, Unicode-3.0, Zlib, CC0-1.0, BSL-1.0.

Encoded in root [`deny.toml`](../../deny.toml).

### Human review / exception required

| License | Rule |
|---------|------|
| **MPL-2.0** | Allowed only with crate-scoped `licenses.exceptions` entry + issue-linked rationale |
| **OFL-1.1** | Same (typical for fonts) |

### Denied by default

GPL, AGPL, LGPL, SSPL, unknown, unlicensed, proprietary, and custom licenses without human exception.

### Waivers

License exceptions and advisory ignores require:

- reason,  
- owner (human handle),  
- linked GitHub issue,  
- **expiry date** (ISO date).  

Record them in § 8 of this file when added. Agents must not invent permanent waivers.

---

## 4. Automated checks

Stable job names and failure handling live in [ci-policy.md](../engineering/ci-policy.md). Summary:

| Check | Tool | Gate |
|-------|------|------|
| Advisories + bans + licenses + sources | `cargo deny check` via pinned `cargo-deny-action` | **Hard** fail on policy violation |
| RustSec advisories (second opinion) | `cargo audit` | **Hard** fail on unignored vulnerabilities |
| Manifest/lockfile PR review | GitHub `dependency-review-action` | **Hard** on configured severity; complements cargo-deny |
| SBOM (release candidates) | pinned `cargo-cyclonedx` → CycloneDX JSON | Generated for release candidates; not a merge-blocker for ordinary PRs |
| Optional binary attestation | `cargo auditable` on release binaries | Optional when compatible; not claimed unless demonstrated |

### Local commands

```powershell
# After: cargo install cargo-deny cargo-audit --locked  (or use CI images)
cargo deny check advisories bans licenses sources
cargo audit

# SBOM (release candidate path)
powershell -NoProfile -File scripts/generate-sbom.ps1
```

### Tooling pins

- GitHub Actions third-party steps are pinned to **full commit SHAs** (see workflows).  
- `cargo-deny` / `cargo-audit` / `cargo-cyclonedx` versions are pinned in workflow `with:` or install args and recorded in the release manifest when producing candidates.  
- Do not claim reproducible builds unless a demonstrated bit-for-bit procedure exists; **traceable** builds are the minimum.

### Blind spots (known)

- Advisory databases lag zero-days.  
- License detection confidence is imperfect (`confidence-threshold` in `deny.toml`).  
- `dependency-review` coverage depends on GitHub Advanced Security / plan features.  
- Native code inside sys crates is not fully audited by these tools.  
- Compromised maintainer of an allowed crate is not fully preventable by SPDX allow lists.

---

## 5. SBOM and third-party notices

### SBOM

- Format: **CycloneDX JSON**.  
- Generator: **`cargo-cyclonedx`** CLI, version pinned at generation time.  
- Script: [`scripts/generate-sbom.ps1`](../../scripts/generate-sbom.ps1) writes under `target/sbom/` (gitignored build output) or a path passed by the operator.  
- Release candidates attach the SBOM artifact and record its **SHA-256** in the release manifest.  
- SBOMs must not include secrets, Credential Manager material, or private Calendar data.

### Third-party notices

- Generate a notices file for dependencies (e.g. `cargo deny list` / license texts collection, or `cargo about` if later admitted) before public packaging (#39).  
- Script stub and expected path: [`scripts/generate-third-party-notices.ps1`](../../scripts/generate-third-party-notices.ps1).  
- Notices ship with installers/ZIPs; content is dependency licenses, not Solpaper secrets.

---

## 6. Release manifest and provenance

Every **candidate** artifact (not only stable publications) should be accompanied by a release manifest. Schema and example: [`docs/security/release-manifest.schema.md`](./release-manifest.schema.md).

Required fields:

| Field | Meaning |
|-------|---------|
| `source_sha` | Full git commit SHA of the sources |
| `version` | Package / tag version string |
| `rustc_version` / `cargo_version` | Toolchain used |
| `target` | e.g. `x86_64-pc-windows-msvc` |
| `cargo_lock_sha256` | SHA-256 of `Cargo.lock` |
| `artifact_sha256` | SHA-256 of the primary binary/installer |
| `sbom_sha256` | SHA-256 of the CycloneDX JSON |
| `notices_sha256` | SHA-256 of third-party notices (when present) |
| `build_workflow` / `build_run_url` | CI workflow identity and run URL when built in CI |
| `signing_state` | `unsigned` \| `signed` (agents never perform signing) |
| `features` / `profile` | Build profile and feature set |

### Signing

- **Signing secrets never** enter GitHub repository files, CI logs for autonomous workflows, agent prompts, issues, or diagnostic evidence.  
- Autonomous agents may produce **`unsigned`** candidates only.  
- Public signing-key use is **CRITICAL** / human-only ([agent-governance.md](../engineering/agent-governance.md)).

### Provenance posture

- Prefer GitHub Actions build provenance / attestations when the owner enables them for release workflows (#39).  
- Until enabled, the release manifest + lockfile hash + SBOM hash is the minimum provenance package.  
- Do not claim SLSA levels or full reproducibility without evidence.

---

## 7. Emergency dependency response

Checklist for compromised, malicious, or critically vulnerable dependencies (coordinates with #45 maintenance and #39 packaging):

1. **Stop** autonomous dependency bumps and release publication.  
2. **Identify** affected versions via `Cargo.lock`, SBOM, and release manifests.  
3. **Contain** — yank candidate artifacts from distribution if already published (human); document “do not run” for known-bad SHAs.  
4. **Mitigate** — upgrade, patch, feature-disable, or remove the dependency; prefer remove when trust is broken.  
5. **Verify** — `cargo deny check`, `cargo audit`, full CI, and focused regression tests.  
6. **Communicate** — human owner issues user-facing notes if shipped builds are affected.  
7. **Secrets** — if build secrets could have been exposed, rotate (human-only).  
8. **Record** — issue-linked timeline, residual risk, and expiry for any temporary waiver.

Agents may open fix PRs and draft checklists; they must **not** publish withdrawals, rotate signing keys, or accept critical vulnerabilities without a human.

### Severity → release gate

| Advisory severity | Release candidate |
|-------------------|-------------------|
| **Critical** | **Blocks** release |
| **High** | **Blocks** release unless a human records an issue-bound, **expiring** waiver in § 8 |
| Medium / Low | Track; fix on schedule; do not ignore silently in release notes |

---

## 8. Advisory and license waivers

| ID / crate | Type | Reason | Owner | Issue | Expires |
|------------|------|--------|-------|-------|---------|
| *(none)* | | | | | |

---

## 9. Acceptance mapping (#38)

| Criterion | Mechanism |
|-----------|-----------|
| One unambiguous project-license story | MIT `LICENSE` + Cargo `license = "MIT"` + README/CONTRIBUTING |
| Crate metadata matches license files | workspace license field |
| Contribution licensing explicit | `CONTRIBUTING.md` inbound=outbound MIT |
| New deps visible with reason | PR template + this policy § 2 |
| Denied/unknown licenses fail gate | `deny.toml` + `cargo deny` CI job |
| Policy-blocking vulns fail gate | `cargo deny` advisories + `cargo audit` |
| Release candidate tied to commit + lockfile | release manifest fields |
| Hashes, SBOM, notices | scripts + release-build path |
| No secrets in tooling output | policy + artifact path limits |
| Signing not autonomous | `signing_state: unsigned` default; CRITICAL for keys |
| Scan ≠ proof | § intro + blind spots |

---

## 10. Non-goals

- Legal opinion that the project is “fully compliant” in every jurisdiction.  
- Multi-ecosystem SBOM beyond Cargo for v1.  
- Autonomous stable publication or signing.  
- Replacing human judgment on HIGH/CRITICAL security decisions.
