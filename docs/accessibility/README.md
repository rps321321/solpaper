# Accessibility

**Issue:** [#41](https://github.com/rps321321/solpaper/issues/41)  
**Pack:** [`deterministic-execution-blueprint.md` § #41](../engineering/deterministic-execution-blueprint.md)  
**Related:** ADR-0001 topology · ADR-0003 rendering · [#18](https://github.com/rps321321/solpaper/issues/18) overlay · [#33](https://github.com/rps321321/solpaper/issues/33) testing · [#13](https://github.com/rps321321/solpaper/issues/13) acceptance · [#34](https://github.com/rps321321/solpaper/issues/34) UX

Accessibility constrains UI architecture **before** toolkit freeze. It is not release-stage polish.

| Document | Purpose |
|----------|---------|
| [requirements.md](./requirements.md) | Standards, surfaces, keyboard, contrast, scaling, privacy in AT trees |
| [uia-feasibility.md](./uia-feasibility.md) | UI Automation feasibility for Approach A overlays + settings |
| [acceptance-rows.md](./acceptance-rows.md) | Rows to consume in #13 / evidence mapping |
| [automated-checks.md](./automated-checks.md) | What CI/dev can automate vs must stay manual |
| [manual-at-script.md](./manual-at-script.md) | Keyboard, Narrator, high-contrast, scaling script |

## Hard rules

1. Follow blueprint #41 **DEFAULT** decisions unless new primary-source evidence forces a recorded deviation.
2. Core actions must work without a mouse (tray / settings / keyboard equivalents).
3. Private or Busy-only Calendar projection must not leak real titles into the UI Automation tree or live regions.
4. Screen-reader usability before stable v1 is **MANUAL** / external — not closable by docs alone.
5. Do not freeze WebView2/egui/wgpu/etc. without re-running feasibility against this pack.

## Primary references

- [Windows app accessibility checklist](https://learn.microsoft.com/en-us/windows/apps/design/accessibility/accessibility-checklist)
- [UI Automation overview](https://learn.microsoft.com/en-us/windows/win32/winauto/entry-uiauto-win32)
- [WCAG 2.2](https://www.w3.org/TR/WCAG22/) (content/visual target AA where applicable)
