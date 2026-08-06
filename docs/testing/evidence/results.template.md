# Evidence results

- **Issue:** #
- **Environment:** `env-…`
- **Date (UTC):** YYYY-MM-DD
- **Operator:**
- **Source SHA:**

## Scenarios

| Scenario ID | Debt ID | Result (pass/fail/skip) | Notes |
|-------------|---------|-------------------------|-------|
| scn-… | MD-… | | |

## Measurements (if any)

Targets from [`docs/engineering/non-functional-requirements.md`](../../engineering/non-functional-requirements.md).

| Metric | Target (#35) | Observed | Notes |
|--------|--------------|----------|-------|
| Cold start p95 | ≤ 1.5 s | | |
| Warm settings open | ≤ 250 ms | | |
| Cold settings open | ≤ 750 ms | | |
| Shutdown / state flush | ≤ 2 s | | |
| Idle working set | ≤ 60 MiB (Alpha 1) / ≤ 100 MiB (Calendar) | | |
| Idle CPU (median / p95) | ≤ 0.5% / ≤ 1% | | |
| Idle process handles | ≤ 500 | | |

## Deviations / incidents

- None.

## Follow-ups

- [ ] Update `docs/testing/manual-debt-register.md` if clearing debt
- [ ] Link this path from #13 row when matrix exists
