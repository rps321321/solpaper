# Evidence directory conventions

**Issue:** [#33](https://github.com/rps321321/solpaper/issues/33)

Physical and release evidence is stored under this tree so runs are reproducible and reviewable. Automated CI logs are **not** stored here by default.

## Path layout

```text
docs/testing/evidence/<issue>/<yyyy-mm-dd>/<environment>/
├── manifest.json      # required
├── commands.txt       # commands run, in order
├── results.md         # outcomes vs expected
├── logs/              # redacted logs only
└── screenshots/       # optional; no secrets on screen
```

| Segment | Rules |
|---------|--------|
| `<issue>` | GitHub issue number that owns the claim (e.g. `18`, `24`, `33`) |
| `<yyyy-mm-dd>` | UTC date of the run |
| `<environment>` | Env ID from [windows-matrix.md](../windows-matrix.md) (e.g. `env-owner-primary`) |

Example:

```text
docs/testing/evidence/18/2026-08-12/env-owner-primary/
```

## Required files

### `manifest.json`

Copy from [manifest.template.json](./manifest.template.json). Must include:

- Source git SHA (full)
- Windows edition and build (`winver`)
- CPU and GPU (human-readable)
- Monitor geometry and DPI scale per display
- Rust version and build profile (`debug` / `release`)
- Commands summary (or pointer to `commands.txt`)
- Operator identity (handle, not email required)
- Start/end timestamps (UTC, ISO-8601)
- `redaction_confirmed`: `true` only after operator verifies no secrets/private Calendar data

### `commands.txt`

Exact commands, one logical step per block. Prefer copy-pasteable PowerShell.

### `results.md`

Copy structure from [results.template.md](./results.template.md). Mark each scenario pass/fail/skip with short notes.

## Screenshots and logs

- Prefer window-style dumps and short log excerpts over long videos.
- Crop or avoid lock screens, notification toasts with real event titles, and credential UI.
- If a screenshot might contain private data, **do not commit it**; note “captured locally, not committed” in `results.md`.

## What not to commit

- OAuth tokens, refresh tokens, client secrets
- Real Calendar event titles or attendee lists
- Absolute paths that reveal unrelated personal files when avoidable
- Crash dumps with unknown contents (summarize instead)

## Empty tree policy

Until the first physical run, this directory may contain only templates and this README. Absence of run folders means **no physical claim is proven**.
