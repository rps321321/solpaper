# Operations and supportability

**Issue:** [#40](https://github.com/rps321321/solpaper/issues/40)  
**Pack:** [`deterministic-execution-blueprint.md` § #40](../engineering/deterministic-execution-blueprint.md)

Local-first diagnostics: no Solpaper cloud backend, no v1 telemetry, no remote crash upload.

| Document | Purpose |
|----------|---------|
| [diagnostics.md](./diagnostics.md) | Logging taxonomy, redaction, Diagnostics UI, bundle, crash/safe-mode, consumer requirements |
| [troubleshooting.md](./troubleshooting.md) | User-facing recovery paths |

**Code:** `solpaper_core::diagnostics` (policy + unit tests).  
**Templates:** `.github/ISSUE_TEMPLATE/` (bug, crash, diagnostics).  
**Security contact:** root [`SECURITY.md`](../../SECURITY.md) — not public issue forms.
