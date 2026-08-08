# IDesktopWallpaper adapter research — Issue #5

**Status:** research + focused prototype in production crates  
**Date:** 2026-08-08  
**Pack:** [`deterministic-execution-blueprint.md` § #5](../engineering/deterministic-execution-blueprint.md)  
**Code:** `solpaper-core::wallpaper`, `solpaper-windows::wallpaper`  
**Related:** ADR-0002 process · ADR-0006 crates · #18 overlay feasibility · #35 NFR · #40 diagnostics · #20 Alpha 1

## Purpose

Define the smallest reliable Windows adapter for **local** wallpaper files. Wallpaper is a **peer subsystem** of the Runtime — it must not own tray, widget surfaces, scheduling, or remote-provider logic.

## Answers to issue questions

| Question | Decision (DEFAULT unless noted) |
|----------|----------------------------------|
| Rust activation of `IDesktopWallpaper` | `windows` crate only (`Win32_UI_Shell` + `Win32_System_Com`). CLSID `DesktopWallpaper`, interface `IDesktopWallpaper`. No second Windows binding. |
| COM apartment / thread | **STA** via `CoInitializeEx(..., COINIT_APARTMENTTHREADED)` on the UI / Runtime thread that owns the adapter. RAII: uninit only if this guard performed first init (`S_OK`). Compatible with single-process ADR-0002; do not call COM wallpaper from a pure worker MTA without marshaling. |
| Monitor identifiers | `GetMonitorDevicePathAt` strings for all `IDesktopWallpaper` calls. **Not** HWND, **not** GDI `HMONITOR` as the apply key. Overlay/widget layout uses separate DIP/monitor match types (`solpaper-core` layout). |
| Fingerprint for persistence | Best-effort ordered: (1) normalized device path, (2) EDID mfg/product + friendly name + connector when available later, (3) geometry/orientation last. Prototype stores path + RECT geometry. |
| Enumerate / query / apply | `GetMonitorDevicePathCount` + `At` + `GetMonitorRECT` (attached vs detached); `GetWallpaper` / `SetWallpaper`; global `GetPosition` / `SetPosition`. |
| Accepted formats (Alpha 1) | `.jpg`, `.jpeg`, `.png`, `.bmp` only. |
| Position | **Global** for the desktop session (Windows does not expose per-monitor position). Default **Fill** (`DWPOS_FILL`). Per-monitor crop differences → pre-render monitor-sized image into Solpaper cache (decode pipeline in #20 with `image` crate); do not invent per-monitor OS position. |
| Monitor add/remove/rename | Enumerate fresh each cycle; detached paths: `GetMonitorRECT` fails or zero size → `attached: false`; apply → `WallpaperMonitorUnavailable`. Rename changes device path → fingerprint may miss; user re-selects binding. |
| File presence after apply | Windows retains the path string; file should remain available. Solpaper **pins** every applied cache path so cleanup never deletes an active wallpaper. |
| HRESULT / recovery | Map to `WallpaperErrorKind` + optional `0xHRESULT` detail. Transient disconnect (`RPC_E_DISCONNECTED`, `CO_E_OBJNOTCONNECTED`, server unavailable): recreate COM object **once**, then surface error. Invalid path/decode: **keep current wallpaper**, one typed error, **no retry loop**. |

## Image request model (no universal resolution)

Each attached monitor produces an `ImageRequest`:

- `width_px` / `height_px` from `GetMonitorRECT`
- aspect + orientation
- `FitPolicy::Fill` with max upscale **1.5×** (NFR PERF-WALL-05)
- monitor id = wallpaper device path

Above max upscale: **letterbox/pillarbox** within Fill framing rather than further upscale; never a second remote fetch.

Limits (NFR): compressed local ≤ 50 MiB; decoded ≤ 100 megapixels.

## Platform interface

```rust
pub trait DesktopWallpaper {
    fn monitors(&self) -> Result<Vec<WallpaperMonitor>, WallpaperError>;
    fn current(&self, monitor: &WallpaperMonitorId) -> Result<Option<PathBuf>, WallpaperError>;
    fn apply(&self, monitor: &WallpaperMonitorId, owned_file: &Path) -> Result<(), WallpaperError>;
    fn position(&self) -> Result<WallpaperPosition, WallpaperError>;
    fn set_position(&self, position: WallpaperPosition) -> Result<(), WallpaperError>;
}
```

| Implementation | Role |
|----------------|------|
| `FakeDesktopWallpaper` | Unit tests: enumerate/apply/error injection; failure leaves previous path |
| `ComDesktopWallpaper` | Production COM; STA init; optional `apply_with_recover` / `monitors_with_recover` |

## Error codes (diagnostics #40)

| Kind | Code | Category token |
|------|------|----------------|
| Path invalid | `WallpaperPathInvalid` | storage |
| Format | `WallpaperFormatUnsupported` | storage |
| File size | `WallpaperFileTooLarge` | storage |
| Decode size | `WallpaperDecodeTooLarge` | storage |
| Upscale | `WallpaperUpscaleExceeded` | provider_policy |
| Platform | `WallpaperPlatform` | surface |
| Transient COM | `WallpaperPlatformTransient` | surface |
| Monitor | `WallpaperMonitorUnavailable` | surface |

Log allowlist fields only (`error_code`, `error_category`, `component=wallpaper`); redacted paths via `redacted_path`.

## Prototype scope vs deferred

| In this issue | Deferred |
|---------------|----------|
| Research doc | Full `image` crate decode/encode pipeline (#20) |
| Trait + fake contract tests | Slideshow APIs |
| COM enumerate / apply / position | EDID-rich fingerprint |
| Size/extension/upscale policy units | Cache eviction policy (#23) |
| Pin set rules | Remote provider (#22/#23) |
| COM enumerate smoke test | Multi-monitor physical matrix (MANUAL) |

## Tests

Automated:

```text
cargo test -p solpaper-core   # wallpaper policy
cargo test -p solpaper-windows  # fake contract + position maps + COM enumerate smoke
```

COM apply restore smoke (optional owner host; not claimed green without named env):

1. Record `current(primary)`.
2. Write a small solid-color `.png` under `%TEMP%\solpaper-wp-smoke\`.
3. `apply(primary, path)`; confirm desktop changed.
4. Restore previous path if `Some`.

## Manual evidence checklist

| ID | Scenario | Status |
|----|----------|--------|
| MD-WP-01 | Two distinct images on two monitors | open |
| MD-WP-02 | Detach/reconnect monitor; re-enumerate attached | open |
| MD-WP-03 | Monitor rename/identity stability of fingerprint | open |
| MD-WP-04 | Global position Fill/Fit/Span behavior | open |
| MD-WP-05 | Invalid file keeps previous wallpaper | open (fake covered; physical open) |
| MD-WP-06 | Explorer restart does not require WorkerW for wallpaper | open (N/A for COM path) |

Register rows may be copied into `docs/testing/manual-debt-register.md` when #20 claims multi-mon wallpaper.

## Requirements consumed by #20

| Req ID | Requirement |
|--------|-------------|
| WP-A1-01 | Local folder source → canonicalize → size/format checks → owned cache file → `DesktopWallpaper::apply` |
| WP-A1-02 | Use `image` crate (feature-gated codecs) for decode/encode; no second image stack |
| WP-A1-03 | Pin applied cache files; cleanup never deletes pins |
| WP-A1-04 | Default position Fill; global set via adapter |
| WP-A1-05 | Failures: keep system wallpaper; typed error; no retry loop |
| WP-A1-06 | Wallpaper subsystem isolated from tray/surface modules (peer) |
| WP-A1-07 | Correlation id `wallpaper_cycle` on apply attempts (#40) |
| WP-A1-08 | Do not bake widgets into wallpaper images |

## Non-goals (confirmed)

No remote provider, scheduler, cache eviction policy, image recommendation, universal fixed resolution filter, WorkerW wallpaper parenting.

## Residual risks

| Risk | Mitigation |
|------|------------|
| COM apartment mismatch if wallpaper called off UI thread | Document STA ownership; Runtime wires adapter on UI thread (#7/#20) |
| Device path instability across driver updates | Fingerprint cascade; user rebind |
| Solid-color wallpaper → empty `GetWallpaper` path | `current` returns `None` |
| Substantive `unsafe` COM surface | HIGH risk PR; checklist F; human merge |

## Acceptance criteria trace

| Criterion | Evidence |
|-----------|----------|
| Enumerate + apply local file on Win11 | COM adapter + smoke; physical MD-WP-* |
| Per-monitor files where hardware supports | API supports per-monitor `SetWallpaper`; multi-mon MANUAL |
| COM compatible with single-process; no UI block for long work | STA on UI thread; heavy decode off-thread in #20 |
| Invalid apply leaves wallpaper | Fake test + Windows SetWallpaper failure semantics |
| Errors bounded / diagnosable | Typed codes; no process abort |
| Monitor IDs separated from overlay | `WallpaperMonitorId` vs layout types |
| No remote/cache policy/schedule | Non-goals honored |
