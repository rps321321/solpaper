# solpaper

Rust wallpaper cycler for **Windows 11 x64**: a tray agent fetches high-res images (target **2560×1440**), applies them per monitor, and a TUI configures sources, schedule, and API keys.

## Status

v1 is being wayfound and built under a living map. Implementation is not complete yet.

| Artifact | Where |
|----------|--------|
| **Wayfinder map (tracker)** | [solpaper v1 wayfinder map](https://github.com/rps321321/solpaper/issues/1) |
| **Wayfinder map (in-repo mirror)** | [`docs/wayfinder/map.md`](docs/wayfinder/map.md) |
| **Ticket index** | [`docs/wayfinder/tickets.md`](docs/wayfinder/tickets.md) |
| **Domain glossary** | [`CONTEXT.md`](CONTEXT.md) |
| **Research notes** | [`docs/research/`](docs/research/) |

## Product locks (charting)

- **Sources:** Wallhaven → Bing → Unsplash (priority fallback; Unsplash only if key is set)
- **Apply:** `IDesktopWallpaper`, different image per monitor
- **Schedule:** cron + friendly presets
- **Process:** user-session tray agent + TUI control plane
- **Cache:** persistent; cycle from cache if all sources fail
- **Secrets:** Windows Credential Manager

## Platform

Windows 11 x64 only for v1. Linux/macOS are out of scope for this effort.

## License

See [LICENSE](LICENSE).
