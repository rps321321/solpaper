---
name: solpaper-review
description: >
  Run independent standards and spec reviews of a pinned Solpaper diff and
  return both reports for solpaper-verifier final aggregation.
user-invocable: true
disable-model-invocation: false
---

# Solpaper two-axis review

Review a fixed diff, not an ambiguous moving working tree.

This skill **coordinates** the two independent reviewers. It is **not** the sole final merge-facing aggregator. After both reports exist, the builder or loop must invoke `solpaper-verifier` for the authoritative `VERIFIED` / `CHANGES_REQUIRED` / `MANUAL_EVIDENCE_REQUIRED` verdict used by autonomous merge rules.

## Pin the review

Resolve and record:

- repository and branch;
- base ref or merge base;
- head commit;
- exact diff command;
- originating issue or spec and comments;
- risk class and lease;
- tests and evidence claimed by the builder.

Fail early when the base is invalid, the diff is empty, or the originating spec cannot be identified.

## Run independent reviews

Use two fresh subagent contexts in parallel when available. They must receive the same pinned diff but different mandates.

### Standards reviewer

Use `.grok/agents/solpaper-standards-reviewer.md` or an equivalent fresh agent. It evaluates:

- `AGENTS.md`, governance, `CONTEXT.md`, ADRs, and engineering gates;
- architecture and module depth;
- Rust correctness and unsafe Win32 or COM boundaries;
- test quality and seam choice;
- security, privacy, diagnostics, accessibility, and supply-chain implications;
- scope creep and speculative complexity;
- risk class, lease consistency, and merge authority.

### Spec reviewer

Use `.grok/agents/solpaper-spec-reviewer.md` or an equivalent fresh agent. It evaluates:

- every acceptance criterion;
- user or domain behavior delivered;
- non-goals and exclusions;
- missing failure and recovery paths;
- claimed tests and evidence;
- manual Windows evidence honesty;
- documentation and issue-state accuracy.

Neither reviewer receives the other's findings before returning its own result.

## Package both reports

Preserve both full reports. Optionally list provisional material findings and rank by severity, but **do not** issue the autonomous merge-facing final verdict here.

Rank any provisional findings by:

1. correctness, security or privacy, data loss, or dishonest evidence;
2. missing acceptance behavior;
3. architecture or testability defects likely to deepen;
4. maintainability and clarity;
5. nonblocking suggestions.

A builder's test report is not sufficient evidence. Independent re-checks belong primarily to `solpaper-verifier`.

## Output (hand-off to verifier)

Return:

```markdown
# Solpaper two-axis review package

**Base:**
**Head:**
**Issue/spec:**
**Risk:**
**Lease:**
**Provisional status:** READY_FOR_VERIFIER | OBVIOUS_CHANGES_REQUIRED

## Standards report
(full standards-reviewer output)

## Spec report
(full spec-reviewer output)

## Provisional material findings
## Checks observed
## Manual evidence notes
```

Use `OBVIOUS_CHANGES_REQUIRED` only when a report already contains clear material defects; the builder may fix before spending a verifier cycle. Otherwise hand both reports to `solpaper-verifier`.

High-risk work that later receives `VERIFIED` from the verifier still requires human merge approval. Critical work cannot receive autonomous merge authority.