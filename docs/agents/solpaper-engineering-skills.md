# Solpaper engineering skills

This document defines the Grok Build skill system used to develop Solpaper. It is intentionally small, composable, repository-specific, and subordinate to the product and engineering maps in GitHub Issues #1 and #30.

## Why this exists

The first autonomous workflow placed recovery, planning, implementation, testing, review, GitHub operations, and release governance in one large skill. That worked for bootstrapping, but it made the controller expensive to load and difficult to improve safely.

The replacement separates **orchestration** from **engineering discipline**:

- A user-invoked controller decides what kind of work is needed.
- Model-invoked skills provide focused procedures.
- Each implementation turn completes one bounded unit.
- Repository files, issues, ADRs, tests, and evidence carry state across fresh contexts.

## Skill map

| Skill | Invocation | Responsibility |
|---|---|---|
| `solpaper-dev-loop` | user / scheduled | Recover state, select one frontier unit, route it, persist the result, stop. |
| `solpaper-engineering` | user | Route a manually requested task to the right discipline. |
| `solpaper-implement` | model or user | Implement one approved issue or coherent vertical slice. |
| `solpaper-ticketing` | model or user | Convert a plan into independently verifiable tracer-bullet issues with real blocking edges. |
| `solpaper-research` | model or user | Answer one technical question from primary sources and record cited findings in the repository. |
| `solpaper-prototype` | model or user | Build disposable evidence to answer one architecture, state, or interaction question. |
| `solpaper-tdd` | model or user | Run a red-green cycle at an explicit public seam. |
| `solpaper-diagnose` | model or user | Reproduce, minimise, test hypotheses, fix, and retain a regression signal. |
| `solpaper-domain-design` | model or user | Sharpen domain vocabulary, ADRs, seams, and deep-module interfaces. |
| `solpaper-review` | model or user | Review a fixed diff independently against repository standards and the originating spec. |
| `solpaper-resolve-conflicts` | model or user | Resolve merge/rebase conflicts by tracing the intent of both sides. |

## Shared sources of truth

Every skill consumes, rather than restates, these sources:

1. `AGENTS.md` — stable product and safety rules.
2. GitHub Issue #1 — canonical product roadmap.
3. GitHub Issue #30 — engineering-system roadmap.
4. `CONTEXT.md` — domain vocabulary and responsibilities.
5. `docs/adr/` — accepted architectural decisions.
6. `IMPLEMENTATION_PLAN.md` — regeneratable execution ledger.
7. `DEV_STATE.md` — current autonomous-turn state.
8. The originating GitHub issue and its acceptance criteria.

When these disagree, GitHub Issue #1 controls product order, Issue #30 controls engineering gates, and accepted ADRs control implementation choices.

## Work classification

The controller classifies work before routing:

- **Decision or unclear architecture** → `solpaper-domain-design`, `solpaper-research`, or `solpaper-prototype`.
- **Feature or internal implementation** → `solpaper-implement`, which invokes `solpaper-tdd` at agreed seams.
- **Bug or regression** → `solpaper-diagnose`.
- **Large plan requiring decomposition** → `solpaper-ticketing`.
- **Completed diff** → `solpaper-review`.
- **Merge/rebase conflict** → `solpaper-resolve-conflicts`.

A task may use more than one discipline sequentially, but one autonomous firing still ends after one bounded unit or one PR correction cycle.

## Solpaper-specific engineering rules

### Windows and Rust

- Prefer documented Win32 and COM mechanisms.
- Isolate `unsafe` boundaries behind narrow Rust interfaces.
- Do not test private Win32 call sequences when observable behavior can be tested through a stable adapter or domain interface.
- Treat sleep, lock, display hotplug, mixed DPI, Explorer restart, Win+D, and fullscreen behavior as physical evidence unless a reliable automated harness genuinely exercises them.

### Architecture

- Seek deep modules: substantial behavior behind a small, explicit interface.
- Prefer the highest stable test seam that observes user-visible or domain behavior.
- Feature areas begin as modules. New crates require a demonstrated compilation, platform, ownership, or test boundary.
- Prototypes remain disposable. Findings may become ADR inputs; prototype structure does not become production architecture by inertia.

### Autonomous governance

- One active builder and one active implementation PR unless Issue #31 explicitly changes the policy.
- Low- and medium-risk changes may follow the approved autonomous merge path.
- High-risk changes may be implemented and reviewed but require human merge approval.
- Critical actions are human-only: stable release publication, signing-key use, force-push/history rewrite, destructive migration approval, credential-policy weakening, and acceptance of critical security risk.
- No skill may override these limits.

## Review model

Review has two independent axes using fresh contexts:

1. **Standards review** — repository rules, architecture, security/privacy, test quality, risk class, and code smells.
2. **Spec review** — originating issue, acceptance criteria, non-goals, evidence, and unimplemented behavior.

The aggregate verdict is:

- `VERIFIED`
- `CHANGES_REQUIRED`
- `MANUAL_EVIDENCE_REQUIRED`

A builder summary is evidence to inspect, not a fact to trust.

## Attribution

This system is an original Solpaper adaptation inspired by the composable-skill approach in Matt Pocock's MIT-licensed `mattpocock/skills` repository. It adopts general engineering ideas such as narrow skills, tracer-bullet tickets, explicit test seams, disciplined diagnosis, domain vocabulary, deep modules, and independent standards/spec review. The Solpaper files are rewritten for Grok Build, Rust, Win32, the project's GitHub maps, and its autonomous-risk policy; the upstream package is not vendored.

## Validation

After changing skills, run Grok's project inspection and confirm that every `solpaper-*` skill is discovered with the intended invocation mode. A skill-system PR changes no product behavior and does not close a product issue.