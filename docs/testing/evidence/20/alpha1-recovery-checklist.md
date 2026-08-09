# Alpha 1 recovery + physical evidence checklist (#20 bullet 8)

**Issue:** [#20](https://github.com/rps321321/solpaper/issues/20)  
**Pack:** Blueprint § #20 bullet 8 — recovery + physical evidence  
**Status:** Procedures ready; **MD-RT-\*** / **MD-WP-\*** remain `open` until a named environment run fills an evidence folder.

This document is the operator guide for completing Alpha 1 recovery claims. Agents must **not** mark debt rows `cleared` without a filled evidence path under `docs/testing/evidence/20/<date>/<env>/`.

## Automated coverage (already in CI)

| Path | Where |
|------|--------|
| Corrupt settings/layout/pomodoro → quarantine + defaults | `solpaper-storage` unit tests |
| Off-screen layout clamp on load | `solpaper-app` unit test |
| Pomodoro `Sync` recovery (≤1 phase) | `solpaper-core` + host restore path |
| Crash markers + safe-mode recommendation | `solpaper-core` diagnostics tests |
| Runtime recovery plan (safe mode skips widgets) | `solpaper-core` `runtime_recovery_plan` |
| Tray TaskbarCreated re-add (code path) | `solpaper-windows` runtime (physical MD-RT-01 still open) |
| Wallpaper prepare/fake keep-previous | `solpaper-windows` wallpaper tests |

## User-facing recovery (dev build)

1. Start: `cargo run -p solpaper-app --release` (or debug).
2. Tray → **Diagnostics** → review redacted status (also written to `%LOCALAPPDATA%\solpaper\logs\diagnostics-status.txt`).
3. When prompted **Run recovery now?**:
   - **Yes** runs, in order when not in safe mode:
     - Recreate widgets from `layout.json` with off-screen clamp
     - Enter **Edit Mode** so the user can re-place widgets
     - Re-scan local wallpaper folders
   - Safe mode: widgets stay off; wallpaper rescan only if errors warrant it.
4. Quit via tray **Quit** (flush layout/pomodoro, remove tray, destroy surfaces).

## Physical scenarios to clear (owner)

Copy templates from [`../manifest.template.json`](../manifest.template.json) and [`../results.template.md`](../results.template.md). Run on an env id from [`../../windows-matrix.md`](../../windows-matrix.md).

### Alpha 1 priority (blocks Alpha 1 claims)

| Debt ID | Scenario | Pass criteria (short) |
|---------|----------|------------------------|
| MD-RT-05 | Second launch | Second `solpaper` activates settings path; **no** second tray icon |
| MD-WP-05 | Invalid wallpaper file | Bad file via Next leaves **previous** desktop wallpaper |
| MD-UX-01 | Usability script | Script in `docs/design/usability-script.md` completable without source docs |
| MD-PERF-01 | Cold start / shutdown budgets | Release profile numbers vs NFR PERF-* |
| MD-A11Y-01 | Keyboard-only core actions | Tray/hotkey Pomodoro without mouse |

### v1 / later (still open; do not clear without run)

| Debt ID | Scenario |
|---------|----------|
| MD-RT-01 | Explorer restart → tray re-add only; widgets not reparented via Explorer |
| MD-RT-02..04 | Autostart installed-build matrix |
| MD-WP-01..04, MD-WP-06 | Multi-monitor / hotplug / position / Explorer wallpaper |
| MD-001..009 | Sleep/resume, lock, multi-mon, DPI, Win+D, fullscreen, idle |
| MD-A11Y-02..05 | UIA, scaling, high contrast, Narrator |
| MD-PERF-02..03 | Calendar working set; Beta soak |

## Suggested `commands.txt` blocks

```powershell
# Build release host
cargo build -p solpaper-app --release

# Smoke (non-interactive create/tear-down)
.\target\release\solpaper.exe --smoke

# Interactive session (record steps in results.md)
.\target\release\solpaper.exe

# Second instance (MD-RT-05)
Start-Process .\target\release\solpaper.exe
# expect: single tray; optional settings activation message
```

## Redaction

- Do not commit screenshots of Calendar (not in Alpha 1) or full `C:\Users\<you>\...` paths.
- Prefer attaching redacted `diagnostics-status.txt` (paths already redacted by the app).
- Confirm `redaction_confirmed: true` in `manifest.json` only after review.

## Outcome of bullet 8 code pass

- Recovery is **reachable** from Diagnostics without a debugger.
- Physical rows remain **open** until owner evidence is filed.
- Issue #20 stays open until Alpha 1 acceptance + required MD rows are cleared or waived by a human.
