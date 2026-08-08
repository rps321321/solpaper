# Release manifest schema

**Issue:** [#38](https://github.com/rps321321/solpaper/issues/38)  
**Consumers:** packaging [#39](https://github.com/rps321321/solpaper/issues/39), v1 RC [#24](https://github.com/rps321321/solpaper/issues/24), go/no-go [#44](https://github.com/rps321321/solpaper/issues/44)

Machine-readable file name: `release-manifest.json` (UTF-8 JSON, no secrets).

## Required fields

| Field | Type | Description |
|-------|------|-------------|
| `schema_version` | string | Currently `"1"` |
| `product` | string | `"solpaper"` |
| `version` | string | Semver or tag name |
| `source_sha` | string | Full git commit SHA |
| `target` | string | Rust target triple |
| `profile` | string | e.g. `"release"` |
| `features` | string[] | Enabled Cargo features |
| `rustc_version` | string | `rustc --version` line or semver |
| `cargo_version` | string | `cargo --version` line or semver |
| `cargo_lock_sha256` | string | Lowercase hex SHA-256 of `Cargo.lock` |
| `artifact_path` | string | Relative or logical artifact name |
| `artifact_sha256` | string | Lowercase hex SHA-256 of primary artifact |
| `sbom_path` | string \| null | CycloneDX JSON path if produced |
| `sbom_sha256` | string \| null | SHA-256 of SBOM file |
| `notices_path` | string \| null | Third-party notices path |
| `notices_sha256` | string \| null | SHA-256 of notices |
| `build_workflow` | string \| null | Workflow file or name |
| `build_run_url` | string \| null | CI run URL |
| `signing_state` | string | `"unsigned"` or `"signed"` |
| `built_at_utc` | string | ISO-8601 UTC timestamp |
| `notes` | string \| null | Free-text limitations |

## Example

```json
{
  "schema_version": "1",
  "product": "solpaper",
  "version": "0.0.0-dev",
  "source_sha": "0123456789abcdef0123456789abcdef01234567",
  "target": "x86_64-pc-windows-msvc",
  "profile": "release",
  "features": [],
  "rustc_version": "rustc 1.80.0",
  "cargo_version": "cargo 1.80.0",
  "cargo_lock_sha256": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
  "artifact_path": "solpaper.exe",
  "artifact_sha256": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
  "sbom_path": "solpaper.cdx.json",
  "sbom_sha256": "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
  "notices_path": "THIRD_PARTY_NOTICES.txt",
  "notices_sha256": "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd",
  "build_workflow": "release-build.yml",
  "build_run_url": null,
  "signing_state": "unsigned",
  "built_at_utc": "2026-08-07T00:00:00Z",
  "notes": "Development candidate only; not a public release."
}
```

## Generation

```powershell
powershell -NoProfile -File scripts/write-release-manifest.ps1 `
  -Version '0.0.0-dev' `
  -ArtifactPath 'target/release/solpaper.exe' `
  -SbomPath 'target/sbom/solpaper.cdx.json' `
  -OutPath 'target/release-manifest.json'
```

Agents set `signing_state` to `unsigned` only.
