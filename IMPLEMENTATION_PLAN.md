# Solpaper Implementation Plan

Regeneratable execution ledger. Do not duplicate GitHub issue bodies. Status on GitHub is authoritative; refresh this file each autonomous iteration.

**Canonical roadmap:** [Issue #1](https://github.com/rps321321/solpaper/issues/1)  
**Engineering map:** [Issue #30](https://github.com/rps321321/solpaper/issues/30)  
**Governance:** [docs/engineering/agent-governance.md](docs/engineering/agent-governance.md)  
**ADRs:** [docs/adr/](docs/adr/)  
**CI policy:** [docs/engineering/ci-policy.md](docs/engineering/ci-policy.md)  
**Deterministic packs:** [docs/engineering/deterministic-execution-blueprint.md](docs/engineering/deterministic-execution-blueprint.md)  
**Testing:** [docs/testing/](docs/testing/)  
**Accessibility:** [docs/accessibility/](docs/accessibility/)  
**Design:** [docs/design/](docs/design/)  
**NFR / budgets:** [docs/engineering/non-functional-requirements.md](docs/engineering/non-functional-requirements.md)  
**Security:** [docs/security/](docs/security/) · [SECURITY.md](SECURITY.md)  
**Operations:** [docs/operations/](docs/operations/)  
**Research:** [docs/research/](docs/research/)

**Product order:** #16 → #13 → (#7, #5, #19) → #20 → (#6, #21) → (#22, #23) → #24  
**Blueprint order:** #33 → #41 → #34 → #35 → #36 → #38 → #40 → (#5, #7) → #13 → #20 → …

## Current frontier

- **#20** — Alpha 1 tracer **bullet 8 in progress**: recovery paths + evidence pack (MD-RT-\*, MD-WP-\* remain open). Bullets 1–7 complete (PR [#84](https://github.com/rps321321/solpaper/pull/84)–[#100](https://github.com/rps321321/solpaper/pull/100)).
- After bullet 8 code merge: #20 stays open until owner clears required MD rows / Alpha acceptance.
- **#13** — Matrix draft landed (PR #82); human freeze still open.

## Active work

- **#20 bullet 8** — Recovery + evidence checklist. Lease `issue-20` on `issue-20-recovery-evidence` (MEDIUM).

## Ready

| Issue | Title | Next action |
|------:|-------|-------------|
| #20 close | Alpha 1 complete | owner MD evidence + acceptance |
| #13 close | Acceptance freeze | owner v1 boundary |

## Blocked

| Issue | Title | Blocked by / wait for |
|------:|-------|------------------------|
| #13 close | Freeze matrix | owner v1 boundary approval |
| #6 / #21 | Calendar path | #20 complete (or owner gates) |
| #22 / #23 | Remote wallpaper | owner gate |
| #24 | v1 RC | release gates |

## Manual evidence required

MD-001..009, MD-A11Y-*, MD-UX-01, MD-PERF-*, MD-WP-01..06, MD-RT-01..05. Operator guide: `docs/testing/evidence/20/alpha1-recovery-checklist.md`.

## Recently completed

- **#20 bullet 7** — Diagnostics/status baseline from #40. PR #100 (merged 2026-08-09).
- **#20 bullet 6** — Local-folder wallpaper + tray Next/Hold + #5 adapter. PR #98 (merged 2026-08-09).
- **#20 bullet 5** — Pomodoro widget projection + NIF_INFO notification dedupe. PR #96 (merged 2026-08-09).
- **#20 bullet 4** — Pomodoro state persistence + tray Start/Pause/Resume/Skip/Reset. PR #94 (merged 2026-08-09).
- **#20 bullet 3** — Atomic settings/layout persistence + off-screen clamp. PR #92 (merged 2026-08-09).
- **#20 bullet 2** — Approach A widget host + Normal/Edit Mode. PR #87 (merged 2026-08-09).
- **#20 bullet 1** — Runtime control HWND + Shell_NotifyIcon tray host. PR #84 (merged 2026-08-08).
- **State PRs** — #99 after #98; #101 after #100.
- **#13 draft** — Acceptance matrix. PR #82.
- **#7 / #5 / #40 / #38 / #36** — foundation packs.

## Discovered defects

- None open.

## Last verified repository state

- **Date (UTC):** 2026-08-09
- **Branch:** `issue-20-recovery-evidence` (from `main` @ `318583b`)
- **Open implementation PRs:** pending bullet 8
