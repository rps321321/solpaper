---
name: solpaper-prototype
description: >
  Build a disposable Solpaper prototype that answers one architecture, state,
  interaction, or Windows feasibility question and records measured evidence.
user-invocable: true
disable-model-invocation: false
---

# Solpaper prototype

A prototype exists to answer one named question cheaply. It is evidence, not an early production implementation.

## Name the question

Before coding, state:

- the single decision being tested;
- competing approaches;
- pass or fail observations;
- environment and hardware needed;
- what the prototype intentionally omits.

When the question can be answered reliably from primary documentation alone, use `solpaper-research` instead.

## Choose the prototype shape

- **State or domain uncertainty** → small Rust harness that drives scenarios through a public model interface.
- **Win32 or COM feasibility** → isolated spike crate under `spikes/` with minimal dependencies and explicit unsafe boundaries.
- **Interaction or visual uncertainty** → multiple clearly different variations or one focused interaction harness; do not polish a single assumed answer.
- **Performance uncertainty** → reproducible benchmark or harness with named build mode, hardware, workload, and baseline.

## Rules

- Mark the code and directory as disposable.
- Keep it outside the production workspace unless the question explicitly concerns an existing production seam.
- Do not establish production crate boundaries, storage schemas, dependency commitments, or UI-toolkit choices by accident.
- Use synthetic data. Never place secrets, private Calendar data, personal paths, IP addresses, or account identifiers in code or evidence.
- Compare alternatives under the same conditions.
- Separate documented behavior, automated observation, manual observation, and untested expectation.
- Do not automate disruptive Windows actions while the owner is studying; record them as manual evidence.

## Evidence

Record:

- exact commands;
- versions and environment;
- measurements or pass/fail matrix;
- screenshots or recordings only when they add information and contain no private data;
- unresolved risks;
- recommendation and fallback;
- which manual tests remain.

Store the report under `docs/research/` or the convention named by the issue.

## Exit

The prototype ends with one of:

- `RECOMMEND_APPROACH`
- `INCONCLUSIVE`
- `REJECT_GOAL`
- `MANUAL_EVIDENCE_REQUIRED`

The next production issue must record the chosen decision in an ADR. Prototype code may remain temporarily for reproducibility, but it must not be imported into production without normal design, tests, and review.