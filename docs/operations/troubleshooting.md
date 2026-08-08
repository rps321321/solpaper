# Troubleshooting (user-facing)

**Issue:** [#40](https://github.com/rps321321/solpaper/issues/40)  
**Full policy:** [diagnostics.md](./diagnostics.md)

Short recovery paths for common failures. Solpaper does not require an online account or cloud diagnostics service.

## Before you report a bug

1. Note the **version** and **build/commit** from Settings → Diagnostics.
2. Copy any **error codes** shown (not event titles or passwords).
3. Prefer exporting a **diagnostic bundle** (preview it first) over screenshots of your calendar.
4. **Never** paste OAuth URLs, tokens, or private event titles into a public GitHub issue.

Security-sensitive reports: see [`SECURITY.md`](../../SECURITY.md).

---

## Widget disappeared or is off-screen

1. Open **Settings → Diagnostics** and check active error codes (`surface` / `layout`).
2. Use **Recreate surfaces** / **Clamp off-screen widgets** if offered.
3. Enter **Edit Mode** from the tray and drag the widget back into view.
4. Quit Solpaper from the tray and start it once more (single-instance should focus the existing app if it is still running).
5. If the Windows shell (Explorer) was restarted, start Solpaper again so the tray icon can re-register.

## Tray icon missing

1. Confirm Solpaper is running in Task Manager (one process).
2. Start Solpaper again — a second launch should activate the existing instance rather than create a duplicate tray.
3. Check Diagnostics for `runtime` / `tray` error codes.
4. Export a bundle if the icon still does not appear after a reboot.

## Calendar not updating

1. Diagnostics → last successful sync and active errors.
2. If category is **auth** → use **Reconnect** in Settings (complete browser sign-in again).
3. If category is **network** → you can stay offline; the last good cache is kept within product limits.
4. If category is **parse** or **provider policy** → note the error code only and file a bug.
5. Do not share calendar event titles in the report.

## Wallpaper not changing

1. Confirm the local folder still exists and is readable.
2. Check Diagnostics for `wallpaper` error codes (decode limits, path, provider).
3. On failure Solpaper keeps the **current** wallpaper; fix the source and retry.
4. Remote provider issues (if enabled) can be disabled locally without quitting the rest of the app.

## App crashes when opening

1. If prompted for **safe mode**, accept it. Widgets, Calendar, remote provider, and autostart changes stay off; Settings and Diagnostics stay on.
2. Open Diagnostics, note crash-marker guidance and build id.
3. Export a diagnostic bundle and file a **crash** issue using the template.
4. Avoid turning autostart on while crash-looping.

## Settings look reset

1. Corrupt configuration is preserved under a timestamped name; safe defaults load.
2. Diagnostics should show a recovery notice (`config` category).
3. Re-apply preferences; report if it recurs every launch.

## Performance or high resource use

1. Note idle time, monitor count, and whether Calendar is connected.
2. Capture Diagnostics counters; avoid multi-hour screen recordings with private data.
3. Beta soak and performance rows are tracked as manual evidence (see testing docs) — still never include secrets.

---

## What a good public report includes

- Steps to reproduce  
- Expected vs actual  
- Windows 11 version (`winver`)  
- Solpaper version + commit/build  
- Error codes / categories from Diagnostics  
- Optional: diagnostic bundle after you have previewed it  

## What to omit

- Access or refresh tokens  
- OAuth callback URLs or query strings  
- Calendar titles, descriptions, attendees, locations  
- Full paths under your user profile  
- Raw database files  
- Screenshots that show private calendar content  
