# Usability test script

**Issue:** [#34](https://github.com/rps321321/solpaper/issues/34)  
**Pack required script:** A new user must, without source docs: locate the tray, enter Edit Mode, move a widget, start/pause/reset focus, select a local folder, recover an off-screen widget, open diagnostics, and quit.

**Status:** Script ready; **human usability sessions MANUAL** (issue: “small usability review required”). Autonomous agents must not mark usability **passed** without recorded findings.

## Session setup

| Item | Value |
|------|--------|
| Build | Named SHA, Alpha 1+ bits |
| Environment | From `docs/testing/windows-matrix.md` |
| Data | Synthetic only |
| Facilitator notes | No coaching beyond task read-once |
| Evidence | `docs/testing/evidence/34/<date>/<env>/` |

## Tasks (order)

| # | Task (read to participant) | Pass criteria |
|---|----------------------------|---------------|
| 1 | “Solpaper is installed and running. Find it and open its menu.” | Opens tray menu without docs |
| 2 | “Enter the mode where you can move the timer on the desktop.” | Enters Edit Mode (tray or shortcut) |
| 3 | “Move the timer a little, then leave that mode.” | Widget moved; Normal Mode restored; desktop clickable |
| 4 | “Start a focus session, pause it, then reset the timer.” | Domain actions via tray/settings succeed |
| 5 | “Point Solpaper at a folder of pictures on this PC.” | Folder selected or clear skip/error recovery |
| 6 | “Pretend the timer is stuck off-screen. Get it back.” | Reset layout or clamp recovery succeeds |
| 7 | “Open diagnostics or about information.” | Diagnostics/About reachable |
| 8 | “Quit Solpaper completely.” | Process exits |

## Optional tasks (Alpha 2 / a11y)

| # | Task | Pass criteria |
|---|------|---------------|
| A | Keyboard-only repeat of 1–4, 7–8 | Completes without mouse |
| B | 150% text scale smoke | Still completable |
| C | Calendar connect (lab account) | Understands privacy mode |

## Findings template

```markdown
### Session
- Date / env / SHA:
- Participant (anonymous id):
- Duration:

### Task results
| Task | Result (pass/fail/partial) | Notes |
|------|----------------------------|-------|
| 1 | | |

### Discoverability issues
-

### Accidental interaction / focus issues
-

### Severity (blocker / major / minor)
-

### Changes requested
-
```

## Recorded findings (repository)

| Date | Result | Link |
|------|--------|------|
| — | **Not yet run** | — |

When sessions complete, add rows and evidence paths; do not delete failed findings.
