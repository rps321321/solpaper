---
name: solpaper-research
description: >
  Investigate one Solpaper engineering question against primary sources and record
  a cited, decision-oriented Markdown note in the repository.
user-invocable: true
disable-model-invocation: false
---

# Solpaper engineering research

Research one question whose answer materially affects implementation, security, policy, or architecture.

## Frame the question

Write the question in a falsifiable or decision-oriented form. Examples:

- Which documented Windows mechanism satisfies the required behavior?
- What exact OAuth, scope, storage, or distribution constraint applies?
- Which crate or API boundary has the smallest justified risk?

List the decision this research will unblock and the evidence threshold for answering it.

## Source hierarchy

Prefer, in order:

1. Official Microsoft, Rust, Google, provider, GitHub, or standards documentation.
2. Authoritative source code, SDK headers, specifications, or release notes.
3. Maintainer statements or issue trackers when official documentation is silent.
4. Secondary material only to discover primary sources or represent a clearly labelled practitioner observation.

Do not treat search snippets, generated summaries, de facto endpoints, or an agent's memory as authority.

For unstable facts, record the access date and relevant version.

## Investigate alternatives

Do not research only the favored answer. Compare plausible alternatives on:

- documented support and stability;
- platform/runtime fit;
- security and privacy;
- failure behavior;
- distribution/legal obligations;
- implementation and maintenance cost;
- reversibility.

Mark unknowns explicitly. An honest `insufficient evidence` is preferable to a confident guess.

## Deliverable

Write one focused note under the repository's existing research convention, normally `docs/research/`.

Use this shape:

```markdown
# Question
## Decision unblocked
## Environment and versions
## Findings
## Alternatives considered
## Failure and threat considerations
## Recommendation
## Unknowns / evidence still required
## Sources
```

Cite each material claim close to the claim. Distinguish documented fact, observed experiment, and inference.

## Handoff

Update the originating issue with:

- recommendation;
- rejected alternatives;
- remaining risk;
- exact follow-up ticket or ADR input.

Research does not silently implement the chosen option. Stop after the evidence artifact unless the issue explicitly combines research with a tiny reversible proof.