---
name: solpaper-engineering
description: >
  Route a Solpaper engineering request to the smallest appropriate discipline:
  ticketing, research, prototype, implementation, diagnosis, domain/design,
  review, or conflict resolution.
user-invocable: true
disable-model-invocation: true
---

# Solpaper engineering router

Route the current request. Do not implement every phase yourself.

## Read first

Read only what is necessary from:

- `AGENTS.md`
- `docs/engineering/agent-governance.md`
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
- A branch, PR, or fixed diff needs judgment → `solpaper-review`
- Git reports merge or rebase conflicts → `solpaper-resolve-conflicts`

When multiple disciplines are needed, run them in the dependency order that produces the earliest useful evidence. Stop after the bounded request is satisfied; do not broaden scope.

## Never bypass

- kill switch, issue lease, concurrency limits, retry limits, or verifier-cycle limits;
- change-risk classes and human-only gates;
- required CI and evidence;
- manual Windows evidence debt;
- Git branch and PR rules;
- product ordering from Issue #1;
- engineering gates from Issue #30.

State which discipline was selected and why, then invoke or follow that skill.