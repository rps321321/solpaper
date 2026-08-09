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

- **#20** — Alpha 1 tracer **bullet 3** next: versioned settings/layout persistence + off-screen clamping. Bullets 1–2 complete (PR [#84](https://github.com/rps321321/solpaper/pull/84), PR [#87](https://github.com/rps321321/solpaper/pull/87)).
- Remaining #20 after bullet 3: Pomodoro domain+widget+tray → local wallpaper folders → diagnostics UI → physical evidence (MD-RT-*).
- **#13** — Matrix draft landed (PR #82); human freeze still open.
- Manual evidence: MD-* including MD-RT-01..05, MD-WP-*.

## Active work

- None. Lease `issue-20` released after #87 merge.

## Ready

| Issue | Title | Next action |
|------:|-------|-------------|
| #20 bullet 3 | Settings/layout persistence + clamp | claim lease next fire |
| #20 later bullets | Pomodoro UI, wallpaper | after bullet 3 |
| #13 close | Acceptance freeze | owner v1 boundary |

## Blocked

| Issue | Title | Blocked by / wait for |
|------:|-------|------------------------|
| #13 close | Freeze matrix | owner v1 boundary approval |
| #6 / #21 | Calendar path | #20 complete |
| #22 / #23 | Remote wallpaper | owner gate |
| #24 | v1 RC | release gates |

## Manual evidence required

MD-001..009, MD-A11Y-*, MD-UX-01, MD-PERF-*, MD-WP-01..06, MD-RT-01..05.

## Recently completed

- **#20 bullet 2** — Approach A widget host + Normal/Edit Mode. PR #87 (merged 2026-08-09).
- **#20 bullet 1** — Runtime control HWND + Shell_NotifyIcon tray host. PR #84 (merged 2026-08-08).
- **State PRs** — #85 / #86 after #84; #88 after #87 (bullet 3 frontier).
- **#13 draft** — Acceptance matrix. PR #82.
- **#7** — Tray design + adapters. PR #79.
- **#5** — IDesktopWallpaper. PR #77.
- **#40** — Diagnostics. PR #75.
- **#38** — Supply-chain. PR #74.
- **#36** — Threat model. PR #70.
- **#35** — NFR. PR #68.
- **#34** — UX flows. PR #66.
- **#41** — Accessibility. PR #64.
- **#33** — Test strategy. PR #61.
- **#19** — Pomodoro. PR #58.

## Discovered defects

- None open.

## Last verified repository state

- **Date (UTC):** 2026-08-09
- **Branch:** `main` @ `936345d` (PR #88 state after #87)
- **Product HEAD:** `f2fd52a` (PR #87 widget host); CI all SUCCESS on #87 and #88
- **Open implementation PRs:** none
