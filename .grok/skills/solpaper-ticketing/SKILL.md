---
name: solpaper-ticketing
description: >
  Break a Solpaper plan or parent issue into independently verifiable tracer-bullet
  GitHub issues with genuine blocking edges and phase/risk gates.
user-invocable: true
disable-model-invocation: false
---

# Solpaper tracer-bullet ticketing

Turn a plan into agent-grabbable GitHub issues. Do not convert architecture layers into separate tickets merely because they are separate layers.

## Gather context

Read the full source plan or issue and comments, current code, `CONTEXT.md`, relevant ADRs, Issues #1 and #30, governance policy, and existing issues that may overlap.

First remove duplication: extend an existing issue when it already owns the behavior.

## Design slices

Prefer vertical tracer bullets:

- each ticket delivers a narrow but complete behavior or a decisive evidence artifact;
- each ticket fits one fresh agent context;
- each ticket is demonstrable or independently verifiable;
- schema, domain, adapter, UI, tests, docs, and evidence travel together when required for that behavior;
- research and prototypes answer one named question and produce a decision input;
- wide mechanical migrations use expand → migrate batches → contract rather than pretending to be vertical.

## Blocking edges

For every ticket, name only blockers that make starting impossible. Avoid convenient but false linearity.

Distinguish:

- technical blocker;
- evidence blocker;
- human decision gate;
- release-only gate.

A later release gate should not block an earlier private implementation when the work remains reversible and safe.

## Required issue body

```markdown
Part of #<parent>

Type: implementation | research | prototype | design | test | security | release
Risk: LOW | MEDIUM | HIGH | CRITICAL
Status: open

## Outcome
The behavior, decision, or evidence this ticket makes real.

## Acceptance criteria
- Observable criterion
- Test or evidence criterion

## Non-goals
- Explicit exclusions

## Blocked by
- Issue links or `None — ready now`

## Evidence
- Automated checks
- Manual Windows evidence when applicable

## Human gate
- None, or exact decision/approval required
```

Use project vocabulary and avoid brittle file-path prescriptions unless a path itself is contractual.

## Publish safely

Present the draft dependency graph before creating many issues when a human is available. When the owner is AFK and the parent already authorizes decomposition, choose the smallest reasonable graph and document assumptions.

Create issues in blocker-first order, link the parent, and update the canonical map only when the execution path materially changes. Do not close or rewrite the parent merely because children were created.