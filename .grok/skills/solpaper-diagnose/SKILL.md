---
name: solpaper-diagnose
description: >
  Diagnose a Solpaper bug or performance regression by constructing a tight
  feedback loop, reproducing, minimising, testing hypotheses, fixing, and retaining regression evidence.
user-invocable: true
disable-model-invocation: false
---

# Diagnose Solpaper failures

Do not begin with a plausible code theory. Begin with a signal that can prove the reported failure is present and later prove it is gone.

## 1. Build the feedback loop

Create the fastest deterministic command that exercises the real failure:

- focused Rust test;
- small CLI/harness with fixture input;
- mock HTTP server and recorded response;
- UI automation where reliable;
- replayed log/event sequence;
- repeated stress or property loop for intermittent failures;
- before/after benchmark for performance regressions;
- structured human-in-the-loop checklist only when physical Windows interaction is unavoidable.

The signal must assert the user's exact symptom, not merely avoid crashing.

If no honest loop can be built, stop and request the missing artifact or environment. Do not guess a fix.

## 2. Reproduce and minimise

Run the loop until the failure is confirmed. Remove inputs, state, timing, dependencies, and steps one by one while preserving the failure.

Capture:

- exact command;
- exact symptom/output;
- reproduction rate;
- smallest load-bearing scenario.

## 3. Form competing hypotheses

Write three to five ranked, falsifiable hypotheses. Each must predict an observation that would support or reject it.

Test one variable at a time. Prefer debugger/inspection and targeted boundary instrumentation over broad logging.

Temporary instrumentation must have a unique marker and be removed before completion.

## 4. Fix through the correct seam

Turn the minimal reproduction into a failing regression test at the highest seam that genuinely reaches the bug. If no suitable seam exists, record the architectural deficiency rather than adding a misleading shallow test.

Apply the smallest root-cause fix. Then:

- run the regression test;
- rerun the original full reproduction;
- run nearby and workspace checks;
- measure the same performance scenario when applicable.

## 5. Close the loop

Before declaring success:

- original symptom no longer reproduces;
- regression evidence is retained or the missing seam is documented;
- temporary diagnostics are removed;
- root cause and why the fix works are recorded in the PR;
- new manual Windows evidence is registered rather than implied;
- follow-up architecture work becomes a separate issue when needed.

After three materially identical failed fixes, stop, persist the failure signature, and return `CHANGES_REQUIRED` or `EXTERNALLY_BLOCKED`.