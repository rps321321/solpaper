# UI Automation feasibility — overlays and settings

**Issue:** [#41](https://github.com/rps321321/solpaper/issues/41)  
**Ties to:** [#18](https://github.com/rps321321/solpaper/issues/18) spike · ADR-0001 · ADR-0003 · [#16](https://github.com/rps321321/solpaper/issues/16)  
**Status:** engineering feasibility note (not physical AT sign-off)

## Question

Can Solpaper meet the #41 requirements with:

1. **Approach A** widget-sized top-level HWNDs (ADR-0001 default), and  
2. **Native Win32 + GDI** scaffold painting (ADR-0003 provisional), and  
3. **Standard Win32 settings** controls (pack DEFAULT)?

## Short answer

| Area | Feasible? | Condition |
|------|-----------|-----------|
| Settings accessibility | **Yes** | Use standard Win32 (or toolkit with full UIA); avoid owner-draw-only settings |
| Overlay status exposure | **Yes, with work** | Custom **UIA fragment provider** per widget HWND |
| Normal Mode click-through | **Yes** | Keep interactive paths in tray/settings/keyboard; overlay stays passive |
| Large web/GPU UI stacks | **Not required** | Revisit only if settings complexity forces ADR-0003 change **and** UIA story is proven |
| Screen-reader “delight” | **Unproven until MANUAL** | Docs cannot close Narrator UX |

## Approach A (per-widget HWND)

**Why it helps accessibility**

- Each widget is a distinct top-level window → natural UIA peer boundary (Name = widget type).
- Failure isolation: one provider bug does not blank the whole desktop surface tree.
- Matches product language (Widget ≈ window).

**Costs**

- N custom providers for N widgets (still small for Alpha 1: Pomodoro ± Calendar).
- Z-order / `WS_EX_NOACTIVATE` / toolwindow styles must not remove windows from the UIA tree unexpectedly — verify with Inspect during host implementation.
- Click-through (`WS_EX_TRANSPARENT` / hit-test) must not be confused with “invisible to AT”; windows should remain in the tree when product policy says widgets are visible.

**Fallback Approach B** (monitor-sized surface) remains valid engineering-wise but makes a deeper fragment tree and hit-testing more complex for AT; it is **not** preferred for accessibility.

## Rendering path (ADR-0003)

GDI/layered painting **does not** provide UIA. Feasibility does **not** depend on switching to WebView2/egui for Alpha:

| Path | UIA | Assessment |
|------|-----|------------|
| Win32 + GDI + **custom UIA provider** | Manual provider | **Default path** — sufficient for Pane/Group + Name + Value |
| Win32 common controls (settings) | Built-in | **Required default** for settings |
| WebView2 | Edge-based UIA bridge | Heavy; only if settings need web and a11y is re-proven |
| Immediate-mode UI (egui/etc.) | Often weak UIA | **High risk** for #41 unless proven with tools |

**Revisit trigger (unchanged):** settings complexity, proven inability to meet contrast/scaling with GDI chrome, or non-rectangular needs — always re-check UIA before freeze.

## Minimal overlay provider contract

Align with [requirements.md](./requirements.md):

```text
RawElementProviderSimple / fragment:
  ControlType = Pane | Group
  Name        = widget kind (non-PII)
  Value/Help  = projected visible status from solpaper-core
  No private Calendar strings
```

Prefer implementing against documented Win32 UI Automation provider APIs in `solpaper-windows`, with unit tests feeding projected strings through a pure seam (no HWND required for privacy tests).

## Keyboard vs overlay focus

- Normal Mode: widgets should not steal activation (`WS_EX_NOACTIVATE` pattern from spike).
- Edit Mode: keyboard map is #34; feasibility assumes Edit Mode can take focus **without** permanent topmost and **with** Escape to exit.
- Core actions remain available if overlay focus fails (tray/settings).

## Tooling for verification

| Tool | Use |
|------|-----|
| Inspect (Windows SDK) | Tree, Name, ControlType, Value |
| Accessibility Insights / AccChecker | Automated rule smoke where installable |
| Narrator | MANUAL script before v1 |
| Keyboard only | Automated partial + MANUAL full flows |

## Risks and mitigations

| Risk | Mitigation |
|------|------------|
| Custom provider never shipped | Alpha 1 gate: Pomodoro UIA fragment or explicit phase waiver |
| Privacy leak via UIA | Core projection tests + provider integration test with Busy/Private fixtures |
| High contrast unreadable GDI paints | System color brushes for chrome; evidence at Beta |
| Toolkit lock-in before AT proof | ADR-0003 stays provisional until Alpha UIA smoke + this note |

## Verdict

**Proceed with Approach A + Win32 settings + custom minimal UIA on overlays.**  
Do **not** treat WebView2/egui as accessibility shortcuts.  
**MANUAL** Narrator/AT review remains open through v1 ([manual-at-script.md](./manual-at-script.md)).

This note satisfies #41’s “feasibility before locking production UI toolkit” criterion for architecture decisions; it does **not** clear physical AT evidence.
