---
name: solpaper-domain-design
description: >
  Sharpen Solpaper's domain vocabulary, responsibilities, seams, and deep-module
  interfaces; record durable decisions in CONTEXT.md and ADRs.
user-invocable: true
disable-model-invocation: false
---

# Solpaper domain and codebase design

Use this skill when terms, ownership, interfaces, or architecture are unclear or when a change risks making the codebase harder to understand.

## Consume the existing model

Read `CONTEXT.md`, relevant ADRs, the originating issue, and surrounding code. Use canonical terms consistently. Call out a term that conflicts with the glossary instead of silently creating a synonym.

## Sharpen the domain

For each fuzzy concept, determine:

- what it is;
- what it owns;
- what it does not own;
- its invariants;
- commands/events or inputs/outputs;
- failure and recovery semantics;
- how it differs from nearby concepts.

Stress the model with edge scenarios: restart, sleep, time changes, monitor loss, stale Calendar data, revoked credentials, missing wallpapers, corrupt state, partial migration, and multiple user actions close together.

Update `CONTEXT.md` when a durable term or responsibility changes.

## Design deep modules

A good module hides substantial behavior behind a small interface. Evaluate:

- interface size and conceptual weight;
- invariants callers must understand;
- duplicated policy leaking across callers;
- platform details escaping into domain code;
- whether one public seam can exercise most important behavior;
- whether errors preserve useful domain meaning;
- whether the module remains replaceable and testable.

Prefer placing complexity behind a narrow owned seam rather than distributing it across many shallow helpers, crates, or coordinators.

For Solpaper, keep these boundaries explicit:

- domain state and policies;
- desktop/window adapter;
- persistence;
- credentials;
- Calendar protocol/sync;
- wallpaper selection/application;
- composition/runtime UI.

Do not create a crate merely to name a concept. Split only for a demonstrated ownership, platform, compilation, or test boundary.

## Decide and record

An ADR is required when a choice is durable, constrains future work, has meaningful alternatives, or changes a previously accepted decision.

Use:

```markdown
# Title
Status: proposed | accepted | superseded

## Context
## Decision
## Alternatives
## Consequences
## Evidence and unresolved risk
```

Cite research/prototype evidence. Keep manual Windows debt visible. Do not promote an unresolved prototype assumption to accepted architecture without the approval required by its issue.

## Output

Return:

- terms clarified;
- interface/seam selected;
- decisions recorded;
- alternatives rejected;
- follow-up evidence or implementation issue;
- any human gate still required.