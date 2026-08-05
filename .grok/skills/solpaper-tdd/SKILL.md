---
name: solpaper-tdd
description: >
  Develop one Solpaper behavior test-first at an explicit public seam using a
  strict red-green cycle and refactor-resistant assertions.
user-invocable: true
disable-model-invocation: false
---

# Solpaper test-driven development

Use TDD for behavior with a stable public seam. The purpose is fast, trustworthy feedback, not maximizing test count.

## Select the seam

Before writing a test, name:

- the public interface exercised;
- the behavior observed;
- why this is the highest stable seam that remains fast and deterministic;
- what lower-level implementation details the test will ignore.

Prefer seams such as:

- Pomodoro/domain commands and resulting views/events;
- widget-layout policy given monitor snapshots;
- persistence interfaces through temporary storage;
- Calendar projection/sync interfaces against fixtures or a mock server;
- wallpaper selection/application adapters through fakes;
- narrow Windows adapter contracts where real system behavior cannot be automated safely.

Do not test private functions, Win32 call order, internal SQL rows, or mock choreography unless that detail is itself contractual.

## Cycle

For one behavior at a time:

1. Write the smallest test that expresses the behavior.
2. Run it and confirm it fails for the expected reason.
3. Add only enough implementation to pass.
4. Run the focused test until green.
5. Run nearby tests for the same seam.
6. Continue with the next behavior only when the current slice is stable.

Do not write every anticipated test first. Do not add speculative implementation for future cycles.

## Test quality

A retained test must:

- read in Solpaper domain language;
- derive expected values independently from the production algorithm;
- observe behavior through a public interface;
- remain deterministic by controlling time, randomness, filesystem, network, and locale where relevant;
- fail with a useful message;
- avoid secrets and private real-world data;
- prove a meaningful acceptance criterion or regression.

Use fakes at owned seams. Use mocks sparingly for external protocols where interaction order is contractual.

## Windows boundary

Physical Windows behavior that needs Explorer restart, sleep, lock, monitor hotplug, mixed DPI, Win+D, or fullscreen interaction cannot be replaced by a unit test that merely asserts intended flags. Test the policy and adapter shape automatically, then retain the physical step under manual evidence.

## Completion evidence

Record:

- seam selected;
- red command and expected failure;
- green command and result;
- broader checks run;
- behavior intentionally not covered;
- manual evidence still required.

Refactoring may follow once behavior is green, but it must preserve the same public tests and pass normal review.