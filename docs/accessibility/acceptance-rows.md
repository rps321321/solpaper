# Accessibility acceptance rows (for #13)

**Issue:** [#41](https://github.com/rps321321/solpaper/issues/41) → consumed by [#13](https://github.com/rps321321/solpaper/issues/13)  
**Evidence:** [#33](https://github.com/rps321321/solpaper/issues/33) layers + [manual-at-script.md](./manual-at-script.md)

Each row is written so #13 can copy structure: phase, scenario, expected, measurement, test/evidence ref.

Legend: **A** = automated when code exists · **M** = manual/AT · **R** = release evidence

| ID | Phase | Blocks | Scenario | Expected (observable) | Test / evidence |
|----|-------|--------|----------|----------------------|-----------------|
| A11Y-01 | Alpha 1 | Yes | Keyboard: start/pause/reset Pomodoro without mouse | Action succeeds via tray/settings/keyboard only | A + M script §1 |
| A11Y-02 | Alpha 1 | Yes | Keyboard: open settings and quit | Settings opens; quit exits runtime | A (where possible) + M §1 |
| A11Y-03 | Alpha 1 | Yes | Settings controls expose UIA Name + ControlType | Inspect shows non-empty Name for each interactive control | M Inspect / Insights |
| A11Y-04 | Alpha 1 | Yes | Overlay Pomodoro exposes UIA Pane/Group + Name | Name is widget type; Value is projected status | A provider unit + M Inspect |
| A11Y-05 | Alpha 1 | Yes | Text scale 100% and 150% | Widgets remain on-screen/usable after clamp | M scaling §3 |
| A11Y-06 | Alpha 2 | Yes | Private Calendar mode | Real title absent from pixels **and** UIA Value/Name/Help | A projection + provider tests |
| A11Y-07 | Alpha 2 | Yes | Busy-only mode | Only busy/free style strings in UIA tree | A + M |
| A11Y-08 | Beta | Yes | Text scale 200% | Layout clamped; primary actions reachable | M §3 |
| A11Y-09 | Beta | Yes | High contrast theme | Settings + widget status still readable; not color-only state | M §4 |
| A11Y-10 | Beta | Prefer | Contrast sampling | Body text ≥4.5:1; large/essential non-text ≥3:1 on default theme | M measurement |
| A11Y-11 | Beta | Yes | Notifications carry text | Toast/notification shows textual phase/error; color/sound optional | A notification sink + M |
| A11Y-12 | Beta | Prefer | No keyboard trap in Edit Mode | Escape or documented exit returns to Normal/tray | M §2 (#34 map) |
| A11Y-13 | v1 | Yes | Narrator smoke on Pomodoro + settings | Status changes understandable; no private title spoken | M §5 **MANUAL** |
| A11Y-14 | v1 | Yes | AT review or human waiver | Qualified feedback or recorded waiver on #24/#44 | Gate **human** |
| A11Y-15 | All | Yes | Meaning not by color alone | State has text or non-color indicator | M + design review #34 |

## Mapping to #33 layers

| Rows | Layers |
|------|--------|
| A11Y-06, A11Y-07, privacy unit | L1 core projection |
| A11Y-04 provider contract | L3/L5 with fakes + HWND smoke when safe |
| A11Y-01–03, 05, 08–15 | L6 physical / manual AT |
| A11Y-14 | Human gate (not CI) |

## Manual debt seeds (register)

When implementation starts, add or clear via `docs/testing/manual-debt-register.md`:

| Suggested ID | Scenario |
|--------------|----------|
| MD-A11Y-01 | Keyboard-only Alpha 1 path |
| MD-A11Y-02 | Inspect UIA overlay + settings |
| MD-A11Y-03 | Scaling 100/150/200 |
| MD-A11Y-04 | High contrast |
| MD-A11Y-05 | Narrator v1 smoke |

Do not mark these cleared without evidence paths under `docs/testing/evidence/`.
