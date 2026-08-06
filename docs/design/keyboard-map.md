# Keyboard map

**Issue:** [#34](https://github.com/rps321321/solpaper/issues/34)  
**Pack source:** blueprint § #34 (DEFAULT)  
**A11y:** Edit Mode map is the #41-referenced keyboard behavior.

## Global (Alpha 1)

| Shortcut | Action | Notes |
|----------|--------|-------|
| `Ctrl+Alt+F2` | Toggle Edit Mode | DEFAULT; tray always alternative |
| Escape | Exit Edit Mode | Only when Edit Mode active; no-op in Normal if no dialog |
| — | Pomodoro start/pause | **Tray/settings first** in Alpha 1; optional dedicated hotkeys deferred to avoid conflicts |

Shell tray activation follows Windows (Win+B, arrows, Enter) — not Solpaper-specific.

## Edit Mode (when active)

| Shortcut | Action |
|----------|--------|
| Arrow | Move selection **1 DIP** |
| Shift+Arrow | Move **10 DIP** |
| Ctrl+Arrow | Resize **1 DIP** (edge heuristic: grow right/bottom for Alpha 1) |
| Ctrl+Shift+Arrow | Resize **10 DIP** |
| Tab / Shift+Tab | Cycle widget selection if multiple |
| Escape | Exit Edit Mode |
| `Ctrl+Alt+F2` | Exit Edit Mode (toggle) |
| Delete | **Not** hard-delete; open confirm via settings path or no-op with status “Use Settings to remove” |

After every move/resize: clamp so **≥48×48 DIP** remains visible on an available work area.

## Settings

| Shortcut | Action |
|----------|--------|
| Tab / Shift+Tab | Control order = visual order |
| Enter | Activate default button |
| Escape | Close settings (discard only if no pending confirm; prefer explicit Cancel) |
| Space | Toggle checkboxes |

## Notifications

No required keyboard capture. User dismisses via system toast UI.

## Calendar (Alpha 2)

OAuth uses system browser; return to settings with focus on Calendar page. No overlay keyboard for tokens.

## Conflicts and change policy

- If `Ctrl+Alt+F2` conflicts on a machine, user uses tray; settings may later rebind (post-Alpha 1).
- Do not add Ctrl+Alt combos for every Pomodoro action in Alpha 1 without #7 review.
- Document final shortcuts in Diagnostics/About.
