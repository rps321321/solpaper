---
name: solpaper-review
description: >
  Review a Solpaper diff against a fixed base using independent standards and
  spec reviewers, then aggregate material findings and manual evidence debt.
user-invocable: true
disable-model-invocation: false
---

# Solpaper two-axis review

Review a fixed diff, not an ambiguous moving working tree.

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

## Aggregate

Deduplicate findings, but do not soften disagreement. Rank by:

1. correctness, security or privacy, data loss, or dishonest evidence;
2. missing acceptance behavior;
3. architecture or testability defects likely to deepen;
4. maintainability and clarity;
5. nonblocking suggestions.

Run or rerun relevant checks independently when possible. A builder's test report is not sufficient evidence.

## Verdict

Return exactly one aggregate verdict:

- `VERIFIED` — no material defect; required automated evidence passes; remaining manual debt is nonblocking for the current phase.
- `CHANGES_REQUIRED` — material defect, missing criterion, failed check, scope or risk violation, or unsupported claim.
- `MANUAL_EVIDENCE_REQUIRED` — code and spec review are acceptable, but required physical evidence blocks honest completion.

Use this report:

```markdown
# Solpaper review

**Base:**
**Head:**
**Issue/spec:**
**Risk:**
**Lease:**
**Verdict:**

## Material findings
## Standards findings
## Spec findings
## Checks executed
## Manual evidence
## Merge authority
## Nonblocking notes
```

High-risk work may be `VERIFIED` for code quality while still remaining human-gated for merge. Critical work cannot receive autonomous merge authority.