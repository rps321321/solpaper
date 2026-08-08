# Testing and evidence

**Issue:** [#33](https://github.com/rps321321/solpaper/issues/33)  
**Related:** [#13](https://github.com/rps321321/solpaper/issues/13) acceptance matrix · [#18](https://github.com/rps321321/solpaper/issues/18) overlay debt · [#32](https://github.com/rps321321/solpaper/issues/32) CI · pack in [`deterministic-execution-blueprint.md`](../engineering/deterministic-execution-blueprint.md)

This directory is the single place for Solpaper’s test strategy, Windows matrix, evidence layout, fixtures plan, and manual-debt register. Physical Windows claims are never treated as passed without named evidence.

| Document | Purpose |
|----------|---------|
| [acceptance-matrix.md](./acceptance-matrix.md) | **#13 product acceptance matrix** (phase rows, status, waivers) |
| [strategy.md](./strategy.md) | Test layers, seams, ownership, flaky policy, regression rules |
| [windows-matrix.md](./windows-matrix.md) | Named OS builds, monitor/DPI topologies, disruptive scenarios |
| [acceptance-mapping.md](./acceptance-mapping.md) | #13 acceptance areas → test level and evidence kind |
| [fixtures-and-mocks.md](./fixtures-and-mocks.md) | Deterministic fixtures and local mock-server plan |
| [manual-debt-register.md](./manual-debt-register.md) | Hardware/manual debt that autonomous merges must not delete |
| [evidence/](./evidence/) | Evidence path conventions and templates |

## Hard rules

1. CI is a **compile/test gate**, not proof of shell or hardware behavior.
2. Hardware-dependent claims require a filled evidence tree under `docs/testing/evidence/…` (or an explicit register entry until run).
3. Autonomous merges may **add** manual debt; they must not remove it without linked evidence and an issue trail.
4. No secrets, OAuth tokens, or private Calendar titles in fixtures, logs, screenshots, or evidence manifests.
5. Do not run disruptive physical tests while the owner is studying.

## When to update

- New feature or acceptance row: map it in `acceptance-mapping.md` and, if manual, the debt register.
- Fixed reproducible defect: add a regression test unless the missing seam is recorded.
- Physical run completed: attach evidence path, update register status, do not invent “pass” in CI alone.
