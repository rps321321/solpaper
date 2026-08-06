# Automated accessibility checks

**Issue:** [#41](https://github.com/rps321321/solpaper/issues/41)  
**Policy:** flaky AT automation must follow [docs/testing/strategy.md](../testing/strategy.md) (no rerun-until-green).

## Feasible automation (implement with features)

| Check | When code exists | How |
|-------|------------------|-----|
| Privacy projection strings | Alpha 2 Calendar | L1 unit tests: ordinary / Private / Busy-only fixtures never emit raw private titles |
| Provider value uses projection | Overlay host | Unit/integration: mock projection → provider Value/Help equals projected text only |
| Notification text present | Notifications | `NotificationSink` fake asserts non-empty text body |
| Contrast tokens (design tokens) | If theme constants exist | Unit test ratio helper on declared fg/bg pairs (not full desktop screenshot CI) |
| Keyboard command routing | Tray/commands | Unit: command enum handles start/pause/reset without UI |

## Partially automatable (optional later)

| Check | Notes |
|-------|-------|
| UIA tree smoke | Optional HWND test spawning a throwaway window + UIA client query; **must** be non-disruptive and stable on CI `windows-latest` or stay `#[ignore]` with issue |
| Accessibility Insights CLI | Only if tool is pinned and licensed for CI; not required for Alpha 1 |

## Not automated (mandatory manual)

| Check | Why |
|-------|-----|
| Narrator understandability | Subjective AT UX; pack marks MANUAL before v1 |
| High contrast “usable” | Theme + GPU/driver variance |
| Full keyboard Edit Mode feel | Depends on #34 map and real focus |
| Mixed-DPI AT traversal | Hardware matrix #33 |

## CI policy interaction

- Production CI remains fmt/check/test/clippy/build ([ci-policy](../engineering/ci-policy.md)).
- Do **not** fail CI on missing Narrator.
- Privacy and projection tests **do** fail CI once Calendar/overlay providers exist.

## This PR

Documents the plan only. No new CI jobs or dependencies.
