# Accessibility requirements

**Issue:** [#41](https://github.com/rps321321/solpaper/issues/41)  
**Status:** initial requirements (informs Alpha 1 and toolkit freeze)  
**Pack source:** blueprint § #41 (DEFAULT unless noted)

## Goals

1. Constrain overlay + settings architecture so custom-rendered desktop widgets remain operable and understandable with keyboard and assistive technology.
2. Keep domain privacy (Calendar projection) consistent in **pixels and accessibility trees**.
3. Record unavoidable limits as explicit release decisions, not silent defects.

## Target standards

| Layer | Target | Notes |
|-------|--------|-------|
| Windows desktop | UI Automation (UIA) + [Microsoft accessibility checklist](https://learn.microsoft.com/en-us/windows/apps/design/accessibility/accessibility-checklist) | Primary platform contract |
| Content / visual | WCAG 2.2 Level **AA** where applicable | Contrast, non-color meaning, resize; not every web SC maps 1:1 to desktop |
| Product phases | Alpha 1: keyboard + scaling basics; Beta/v1: full matrix + AT review | See [acceptance-rows.md](./acceptance-rows.md) |

## Surfaces and modes

| Surface | Role | Accessibility posture |
|---------|------|------------------------|
| **Overlay — Normal Mode** | Read-only, click-through desktop widgets | Passive/status only; **no** exclusive mouse-only actions |
| **Overlay — Edit Mode** | Drag/resize/arrange | Keyboard map owned by #34; must not trap focus without Escape |
| **Tray** | Primary always-available entry | Standard shell notify-icon patterns; menu items named |
| **Settings** | Configuration | **Standard Win32 controls** so built-in UIA providers apply (pack DEFAULT) |
| **Notifications** | Phase complete / errors | Text required; sound and color supplemental only |

## Core actions (mouse-free)

Every Alpha 1+ build must expose **non-mouse** paths for:

| Action | Minimum equivalent |
|--------|-------------------|
| Open tray menu / show status | Keyboard focus to tray icon / documented hotkey if product adds one |
| Enter / exit Edit Mode | Tray or settings command; Escape exits Edit Mode (#34 detail) |
| Start / pause / resume / skip / reset Pomodoro | Tray and/or settings and/or keyboard command |
| Open settings | Tray |
| Select local wallpaper folder | Settings (standard file/folder picker) |
| Recover off-screen widget | Settings or tray “reset layout” (#34) |
| Open diagnostics | Settings or tray |
| Quit | Tray |

Overlay Normal Mode **remains** read-only/click-through; it must not be the only place a core action lives.

## UI Automation — settings

- Prefer stock Win32 (or equivalent toolkit controls that expose full UIA): buttons, checkboxes, list views, edit fields, tabs.
- Every interactive control: **Name**, **ControlType**, **Enabled**, and **Value** (where applicable).
- Focus order is logical (reading order); no keyboard traps without Escape/documented exit.
- Dialogs and property pages use standard modality patterns.

## UI Automation — overlay widgets (custom)

Custom-painted Approach A HWNDs (ADR-0001) do **not** get free UIA. Production must supply a **minimal UIA fragment provider** per widget HWND (pack DEFAULT):

| Property | Requirement |
|----------|-------------|
| Control type | `Pane` or `Group` |
| Name | Widget type (e.g. `Pomodoro`, `Calendar agenda`) — stable, not user PII |
| Value / help text | **Current projected visible status only** (see privacy) |
| States | As applicable (`ReadOnly` in Normal Mode; interactive states only in Edit Mode if exposed) |
| Children | Prefer flat minimal tree; avoid dumping internal paint nodes |
| Live region | Optional for timer ticks; if used, announce phase **changes**, not every second |

Implementation detail lives in `solpaper-windows` adapters; domain projection of “what is visible” lives in `solpaper-core` so privacy tests stay pure.

## Privacy in accessibility output

| Mode | Rendered text | UIA Name/Value/Help | Notifications / live regions |
|------|---------------|---------------------|------------------------------|
| Ordinary titles | Ordinary titles | Same projected string | Same |
| Private → `Private` | `Private` | **Must not** contain real title | **Must not** contain real title |
| Busy-only | Busy/free style only | Busy/free only | Busy/free only |

Tests must cover **both** painted strings and the UIA tree (pack DEFAULT). Fixtures remain synthetic ([docs/testing](../testing/)).

## Keyboard

- Full interactive Edit Mode key map is **#34** deliverable; this issue requires that the map **exists** and that Escape/documented exits prevent focus traps.
- Tab order in settings follows visual order.
- Accelerators in tray/settings menus where standard.

## Visual: contrast, color, motion

| Requirement | Value |
|-------------|--------|
| Normal text contrast | ≥ **4.5:1** against adjacent background |
| Large text / essential non-text UI | ≥ **3:1** |
| Meaning by color alone | **Forbidden** — pair with text, icon shape, or pattern |
| Reduced motion | Honor OS “animation effects” / reduced motion when timers or transitions animate; never require motion to convey state |
| High contrast | Usable under Windows high-contrast themes; system colors preferred for settings chrome |

## Text scaling and DPI

| Scale | Support |
|------:|---------|
| 100% | Required |
| 150% | Required |
| 200% | Required |

- Layout must **clamp** after scale changes so widgets are not permanently unusable or fully off-screen (tie to layout recovery).
- Mixed-DPI multi-monitor remains under #33 physical matrix; accessibility scaling tests may start single-monitor.

## Target sizes

Interactive hit targets in Edit Mode and settings:

- Minimum **44×44 DIP** recommended for primary controls (align checklist / touch-adjacent guidance); desktop mouse-only secondary chrome may use **24×24 DIP** minimum with adequate spacing — document any smaller control as a limitation.

Exact Edit Mode grip geometry is #34; do not invent HWND counts here.

## Notifications

- Always include a **text** payload (toast/title/body as applicable).
- Sound and color are optional supplements.
- Deduplication rules from Pomodoro (#19) still apply; accessibility does not justify replaying completions.

## Phase expectations

| Phase | Accessibility bar |
|-------|-------------------|
| **Alpha 1** | Keyboard paths for core actions; settings use standard controls; text scaling 100/150; basic contrast on Pomodoro widget chrome; UIA fragment at least for Pomodoro if overlay ships |
| **Alpha 2** | Calendar privacy in UIA tree; Busy-only/private tests automated |
| **Beta** | High contrast pass; 200% scaling; Inspect/Accessibility Insights smoke |
| **v1** | Full [manual-at-script.md](./manual-at-script.md) + qualified AT feedback or recorded human waiver |

## Unavoidable limitations (process)

If a limitation cannot be fixed before a phase:

1. Record it in the #13 acceptance row and release notes.
2. Classify as **blocking** or **waived** with human owner for waivers.
3. Never leave it only in chat or a closed spike note.

## Non-goals (this issue)

- Implementing production UIA providers (lands with overlay host work).
- Freezing a large UI toolkit (ADR-0003 revisit only after this feasibility).
- Completing physical Narrator sessions in this PR (evidence remains open).
