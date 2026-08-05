# solpaper v1 wayfinder map

**Canonical tracker issue:** https://github.com/rps321321/solpaper/issues/1  
**Label:** `wayfinder:map`

This file is the **in-repo mirror**. Update it whenever the issue map is updated (especially Decisions so far).

---

## Destination

Working **v1** of solpaper on Windows 11 x64: a Rust **tray agent** keeps cycling high-res wallpapers (target **2560×1440**) via **IDesktopWallpaper** with a **different image per monitor**; a **TUI control plane** configures sources, schedule, API keys, purity, and status. Cycling continues when the TUI is closed.

## Notes

- **Domain:** desktop wallpaper cycling (fetch → cache → apply → schedule).
- **Execution is in-scope** for this map (charting chose destination mode C). Tickets may research, decide, prototype, and implement until v1 runs on the machine. Still **one ticket resolved per session** when working the map.
- **Skills every session should consult:** grilling, domain-modeling; Win32/COM docs for wallpaper; Credential Manager docs; research/find-docs for external APIs and crates.
- **Platform:** Windows 11 x64 only for this effort.
- **Language:** Rust.
- **Dual home:** GitHub Issues (workflow) + `docs/wayfinder/` on `main` (durable docs). Refer to tickets **by name** (link + title), not bare numbers alone.
- **Product locks from charting** (tickets go deeper; do not restate as ticket answers):
  - Sources: Wallhaven + Bing + Unsplash (Unsplash only when key present).
  - Source selection: **priority fallback** default order **Wallhaven → Bing → Unsplash** (skip Unsplash if no key).
  - Apply: **IDesktopWallpaper** (COM), **different image per monitor**.
  - Schedule: **cron expression + friendly presets** (presets write cron).
  - Process: **user-session tray agent** + TUI as remote control plane (not Windows SCM service).
  - Cache: **persistent** under LocalAppData-style cache; **on total source failure, cycle from cache**.
  - Content: **all Wallhaven purity levels configurable** from day one (safe defaults TBD on a ticket).
  - Secrets: **Windows Credential Manager** for API keys.
  - Skip-now: treat as **in v1** unless a later ticket drops it.

## Decisions so far

_(none yet — closed tickets append here)_

## Not yet specified

- Exact Rust crates (TUI, HTTP, image decode, Windows bindings, tray, cron parser, IPC).
- Install / distribution (cargo install vs scoop vs portable zip vs installer).
- Autostart registration UX (first-run wizard vs explicit “Install autostart”).
- Single-instance locking and “TUI when agent missing” recovery.
- Wallpaper positioning policy per monitor (fill / fit / span) — may graduate from grill ticket.
- Wallhaven sort/query defaults (random, toplist, tags).
- Whether “Bing” means official daily wallpaper endpoint vs search scrape (research will force this).
- Offline-only mode as a first-class setting vs emergent cache fallback.
- Logging location, log levels, and whether tray shows balloon errors.
- Image format support matrix (JPEG/PNG/WebP) and downscale-when-larger behavior.
- Config file format and migration story.
- Pause/resume and “hold this wallpaper” if not covered by schedule grill.
- Implementation module boundaries and Cargo workspace layout details beyond scaffold.
- End-to-end implementation tickets (agent loop, sources, TUI wiring) — graduate after research + DoD.

## Out of scope

- Linux / macOS / non-Windows targets for this effort.
- Windows Service (SCM) based agent — charting chose user-session tray instead.
- Cloud sync of settings or cross-machine profiles.
- AI / generative wallpaper sources.
- Mobile companions, web dashboard, or remote control over the network.
- Guaranteeing exact 2560×1440 only as the sole accepted size (target filter; larger OK if we downscale — exact policy TBD in fog above).
