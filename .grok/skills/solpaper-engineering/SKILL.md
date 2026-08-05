---
name: solpaper-engineering
description: >
  Route a Solpaper engineering request to the smallest appropriate discipline:
  ticketing, research, prototype, TDD implementation, diagnosis, domain/design,
  review, or conflict resolution.
user-invocable: true
disable-model-invocation: true
---

# Solpaper engineering router

Route the current request. Do not implement every phase yourself.

## Read first

Read only what is necessary from:

- `AGENTS.md`
- `CONTEXT.md`
- relevant `docs/adr/`
- the originating GitHub issue/spec
- Issue #1 for product order
- Issue #30 for engineering gates

## Choose one primary discipline

- Work is too large or tickets are horizontal/unclear → `solpaper-ticketing`
- A technical fact or external API must be established → `solpaper-research`
- A design assumption needs cheap executable evidence → `solpaper-prototype`
- One approved feature/slice is ready to build → `solpaper-implement`
- Something is broken, flaky, failing, or slow → `solpaper-diagnose`
- Vocabulary, responsibilities, seams, or architecture are unclear → `solpaper-domain-design`
- A branch/PR/diff needs judgment → `solpaper-review`
- Git reports merge or rebase conflicts → `solpaper-resolve-conflicts`

When multiple disciplines are needed, run them in the dependency order that produces the earliest useful evidence. Stop after the bounded request is satisfied; do not broaden scope.

## Never bypass

- Change-risk and human-only gates from Issue #31
- Required CI and evidence
- Manual Windows evidence debt
- Git branch/PR rules
- Product ordering from Issue #1
- Engineering gates from Issue #30

State which discipline was selected and why, then invoke or follow that skill.