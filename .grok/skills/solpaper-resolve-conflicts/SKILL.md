---
name: solpaper-resolve-conflicts
description: >
  Resolve an in-progress Solpaper merge or rebase conflict by tracing the intent
  of both sides to issues, ADRs, commits, and tests, then validating the result.
user-invocable: true
disable-model-invocation: false
---

# Resolve Solpaper conflicts by intent

Use only when a merge or rebase is already in conflict.

## Inspect state

Record:

- merge or rebase operation;
- current branch and target;
- conflicting files/hunks;
- commits being combined;
- related issues, PRs, ADRs, and tests.

Do not abort merely because resolution is difficult. Do not force-push unless a human explicitly authorizes a history rewrite under the critical-risk policy.

## Trace both intents

For every hunk, identify why each side changed the code or documentation. Use primary repository evidence:

- issue/spec and comments;
- ADR or research finding;
- commit/PR message;
- test added with the change;
- current canonical maps.

Do not choose a side based on recency or formatting alone.

## Resolve

- Preserve both intents when compatible.
- When incompatible, choose the behavior required by the merge/rebase goal and current accepted decision.
- Do not invent a third feature or refactor during conflict resolution.
- Keep product maps, state files, and issue status consistent with actual code.
- Never erase unresolved manual evidence debt during a text merge.

Resolve one hunk at a time and inspect the resulting full file for duplicated or contradictory content.

## Validate

Discover and run the applicable checks, normally:

```powershell
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Also run focused tests covering the conflicting behavior. Review the final diff against both original intents.

Stage the resolved files and continue the merge/rebase until complete. Report trade-offs, tests, and any follow-up issue required.