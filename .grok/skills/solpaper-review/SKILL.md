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
- originating issue/spec and comments;
- risk class;
- tests/evidence claimed by the builder.

Fail early when the base is invalid, the diff is empty, or the originating spec cannot be identified.

## Run independent reviews

Use two fresh subagent contexts in parallel when available. They must receive the same pinned diff but different mandates.

### Standards reviewer

Use `.grok/agents/solpaper-standards-reviewer.md` or an equivalent fresh agent. It evaluates:

- `AGENTS.md`, `CONTEXT.md`, ADRs, and engineering gates;
- architecture and module depth;
- Rust correctness and unsafe Win32/COM boundaries;
- test quality and seam choice;
- security, privacy, diagnostics, accessibility, and supply-chain implications;
- scope creep and speculative complexity;
- risk class and merge authority.

### Spec reviewer

Use `.grok/agents/solpaper-spec-reviewer.md` or an equivalent fresh agent. It evaluates:

- every acceptance criterion;
- user/domain behavior delivered;
- non-goals and exclusions;
- missing failure/recovery paths;
- claimed tests and evidence;
- manual Windows evidence honesty;
- documentation and issue-state accuracy.

Neither reviewer receives the other's findings before returning its own result.

## Aggregate

Deduplicate findings, but do not soften disagreement. Rank by:

1. correctness, security/privacy, data loss, or dishonest evidence;
2. missing acceptance behavior;
3. architecture/testability defects likely to deepen;
4. maintainability and clarity;
5. nonblocking suggestions.

Run or rerun the relevant checks independently when possible. A builder's test report is not sufficient evidence.

## Verdict

Return exactly one aggregate verdict:

- `VERIFIED` — no material defect; required automated evidence passes; remaining manual debt is nonblocking for the current phase.
- `CHANGES_REQUIRED` — material defect, missing criterion, failed check, scope/risk violation, or unsupported claim.
- `MANUAL_EVIDENCE_REQUIRED` — code/spec review is acceptable, but required physical evidence blocks honest completion.

Use this report:

```markdown
# Solpaper review

**Base:**
**Head:**
**Issue/spec:**
**Risk:**
**Verdict:**

## Material findings
## Standards findings
## Spec findings
## Checks executed
## Manual evidence
## Nonblocking notes
```

High-risk work may be `VERIFIED` for code quality while still remaining human-gated for merge. Critical work cannot receive autonomous merge authority.