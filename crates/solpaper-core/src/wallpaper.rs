//! Platform-neutral wallpaper domain types and apply policy (Issue #5).
//!
//! Win32 / COM live in `solpaper-windows`. This module owns monitor request
//! shapes, format/size limits, upscale policy, and pin-set rules so domain
//! tests never link COM.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use crate::CoreError;

/// Maximum compressed local wallpaper file size (NFR PERF-WALL-01).
pub const LOCAL_WALLPAPER_MAX_BYTES: u64 = 50 * 1024 * 1024;
/// Maximum decoded width × height in megapixels (NFR PERF-WALL-03).
pub const DECODED_MAX_MEGAPIXELS: u64 = 100;
/// Maximum acceptable upscale factor when filling a monitor (NFR PERF-WALL-05).
pub const MAX_UPSCALE_FACTOR: f64 = 1.5;

/// Accepted local extensions for Alpha 1 (blueprint #5).
pub const ACCEPTED_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "bmp"];

/// Opaque Windows monitor device path used by `IDesktopWallpaper`.
///
/// This is **not** an HWND and must not be derived from overlay surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WallpaperMonitorId(pub String);

impl WallpaperMonitorId {
    pub fn new(device_path: impl Into<String>) -> Self {
        Self(device_path.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// Best-effort durable fingerprint for layout binding (blueprint #5).
///
/// Prefer normalized device path; then EDID/friendly name; geometry last.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MonitorFingerprint {
    pub device_path: String,
    pub friendly_name: Option<String>,
    pub edid_mfg_product: Option<String>,
    pub width_px: i32,
    pub height_px: i32,
    pub orientation_deg: u16,
}

/// One wallpaper target monitor as observed by the adapter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WallpaperMonitor {
    pub id: WallpaperMonitorId,
    /// False when Windows still lists the path but `GetMonitorRECT` fails or is empty.
    pub attached: bool,
    pub rect_left: i32,
    pub rect_top: i32,
    pub rect_right: i32,
    pub rect_bottom: i32,
    pub fingerprint: MonitorFingerprint,
}

impl WallpaperMonitor {
    pub fn width_px(&self) -> i32 {
        (self.rect_right - self.rect_left).max(0)
    }

    pub fn height_px(&self) -> i32 {
        (self.rect_bottom - self.rect_top).max(0)
    }

    /// Monitor-specific image request for decode/fit (no universal resolution).
    pub fn image_request(&self) -> ImageRequest {
        let w = self.width_px().max(1) as u32;
        let h = self.height_px().max(1) as u32;
        ImageRequest {
            monitor_id: self.id.clone(),
            width_px: w,
            height_px: h,
            aspect: w as f64 / h as f64,
            orientation: if w >= h {
                Orientation::Landscape
            } else {
                Orientation::Portrait
            },
            fit: FitPolicy::Fill,
            max_upscale: MAX_UPSCALE_FACTOR,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Landscape,
    Portrait,
}

/// How Solpaper prepares an image before `SetWallpaper` (position is global on Windows).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FitPolicy {
    /// Prefer Fill framing; letterbox/pillarbox when upscale would exceed max.
    Fill,
}

/// Per-monitor request shape for local (and later remote) image selection.
#[derive(Debug, Clone, PartialEq)]
pub struct ImageRequest {
    pub monitor_id: WallpaperMonitorId,
    pub width_px: u32,
    pub height_px: u32,
    pub aspect: f64,
    pub orientation: Orientation,
    pub fit: FitPolicy,
    pub max_upscale: f64,
}

/// Global desktop wallpaper position (Windows: not per-monitor).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WallpaperPosition {
    Center,
    Tile,
    Stretch,
    Fit,
    #[default]
    Fill,
    Span,
}

impl WallpaperPosition {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Center => "center",
            Self::Tile => "tile",
            Self::Stretch => "stretch",
            Self::Fit => "fit",
            Self::Fill => "fill",
            Self::Span => "span",
        }
    }
}

/// Stable product error codes for wallpaper (compatible with #40 categories).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WallpaperErrorKind {
    /// Path missing, not a file, or failed canonicalize.
    PathInvalid,
    /// Extension outside allowlist.
    FormatUnsupported,
    /// Compressed file too large.
    FileTooLarge,
    /// Decoded pixels exceed cap.
    DecodeTooLarge,
    /// Upscale would exceed max without letterbox path available in caller.
    UpscaleExceeded,
    /// COM / platform failure after policy checks.
    Platform,
    /// Transient COM disconnected; adapter may recreate once.
    PlatformTransient,
    /// Monitor id unknown or detached for apply.
    MonitorUnavailable,
    /// Internal invariant (empty id, etc.).
    Internal,
}

impl WallpaperErrorKind {
    pub fn as_error_code(&self) -> &'static str {
        match self {
            Self::PathInvalid => "WallpaperPathInvalid",
            Self::FormatUnsupported => "WallpaperFormatUnsupported",
            Self::FileTooLarge => "WallpaperFileTooLarge",
            Self::DecodeTooLarge => "WallpaperDecodeTooLarge",
            Self::UpscaleExceeded => "WallpaperUpscaleExceeded",
            Self::Platform => "WallpaperPlatform",
            Self::PlatformTransient => "WallpaperPlatformTransient",
            Self::MonitorUnavailable => "WallpaperMonitorUnavailable",
            Self::Internal => "WallpaperInternal",
        }
    }

    /// Maps into #40 `ErrorCategory` tokens without depending on diagnostics module cycles.
    pub fn error_category_token(&self) -> &'static str {
        match self {
            Self::PathInvalid
            | Self::FormatUnsupported
            | Self::FileTooLarge
            | Self::DecodeTooLarge => "storage",
            Self::UpscaleExceeded => "provider_policy",
            Self::Platform | Self::PlatformTransient | Self::MonitorUnavailable => "surface",
            Self::Internal => "internal",
        }
    }
}

/// Whether a file extension is accepted for local wallpaper.
pub fn is_accepted_extension(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| {
            ACCEPTED_EXTENSIONS
                .iter()
                .any(|a| a.eq_ignore_ascii_case(e))
        })
        .unwrap_or(false)
}

/// Validate file size against PERF-WALL-01. Does not touch the system wallpaper.
pub fn check_local_file_size(size_bytes: u64) -> Result<(), WallpaperErrorKind> {
    if size_bytes > LOCAL_WALLPAPER_MAX_BYTES {
        Err(WallpaperErrorKind::FileTooLarge)
    } else {
        Ok(())
    }
}

/// Validate decoded pixel count against PERF-WALL-03.
pub fn check_decoded_pixels(width: u32, height: u32) -> Result<(), WallpaperErrorKind> {
    let pixels = (width as u64).saturating_mul(height as u64);
    let max = DECODED_MAX_MEGAPIXELS.saturating_mul(1_000_000);
    if pixels > max {
        Err(WallpaperErrorKind::DecodeTooLarge)
    } else {
        Ok(())
    }
}

/// Scale factors when mapping image → monitor (cover/fill).
///
/// Returns `(scale_x, scale_y)` if the image is stretched independently, or the
/// uniform cover scale for Fill. `exceeds_max_upscale` is true when either edge
/// would require scale &gt; [`MAX_UPSCALE_FACTOR`].
pub fn fill_scale_factors(
    image_w: u32,
    image_h: u32,
    monitor_w: u32,
    monitor_h: u32,
) -> (f64, f64, bool) {
    let iw = image_w.max(1) as f64;
    let ih = image_h.max(1) as f64;
    let mw = monitor_w.max(1) as f64;
    let mh = monitor_h.max(1) as f64;
    let sx = mw / iw;
    let sy = mh / ih;
    // Cover scale is the max edge scale.
    let cover = sx.max(sy);
    let exceeds = cover > MAX_UPSCALE_FACTOR + f64::EPSILON;
    (sx, sy, exceeds)
}

/// Decide whether Fill should upscale or letterbox/pillarbox (blueprint #5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FillDecision {
    /// Image large enough or within max upscale — Fill/cover OK.
    Cover,
    /// Would exceed max upscale — prefer letterbox/pillarbox within Fill framing.
    Letterbox,
}

pub fn fill_decision(image_w: u32, image_h: u32, monitor_w: u32, monitor_h: u32) -> FillDecision {
    let (_, _, exceeds) = fill_scale_factors(image_w, image_h, monitor_w, monitor_h);
    if exceeds {
        FillDecision::Letterbox
    } else {
        FillDecision::Cover
    }
}

/// Validate a candidate local source path for apply (extension + non-empty).
///
/// Canonicalization is performed by the adapter on the host filesystem.
pub fn validate_source_path_shape(path: &Path) -> Result<(), WallpaperErrorKind> {
    if path.as_os_str().is_empty() {
        return Err(WallpaperErrorKind::PathInvalid);
    }
    if !is_accepted_extension(path) {
        return Err(WallpaperErrorKind::FormatUnsupported);
    }
    Ok(())
}

/// Tracks cache files currently applied so cleanup must not delete them.
#[derive(Debug, Default, Clone)]
pub struct WallpaperPinSet {
    pinned: HashSet<PathBuf>,
}

impl WallpaperPinSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pin(&mut self, path: PathBuf) {
        self.pinned.insert(path);
    }

    pub fn unpin(&mut self, path: &Path) {
        self.pinned.remove(path);
    }

    pub fn is_pinned(&self, path: &Path) -> bool {
        self.pinned.contains(path)
    }

    pub fn iter(&self) -> impl Iterator<Item = &PathBuf> {
        self.pinned.iter()
    }

    /// Cache cleanup may delete `candidate` only when not pinned.
    pub fn may_delete_cache_file(&self, candidate: &Path) -> bool {
        !self.is_pinned(candidate)
    }
}

/// Domain helper: reject empty monitor id before COM.
pub fn require_monitor_id(id: &WallpaperMonitorId) -> Result<(), CoreError> {
    if id.is_empty() {
        Err(CoreError::InvalidWallpaper("empty monitor id"))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_jpg_png_bmp_case_insensitive() {
        assert!(is_accepted_extension(Path::new("a.JPG")));
        assert!(is_accepted_extension(Path::new("a.jpeg")));
        assert!(is_accepted_extension(Path::new("a.Png")));
        assert!(is_accepted_extension(Path::new("a.bmp")));
        assert!(!is_accepted_extension(Path::new("a.gif")));
        assert!(!is_accepted_extension(Path::new("a.webp")));
        assert!(!is_accepted_extension(Path::new("a")));
    }

    #[test]
    fn file_size_cap() {
        assert!(check_local_file_size(LOCAL_WALLPAPER_MAX_BYTES).is_ok());
        assert_eq!(
            check_local_file_size(LOCAL_WALLPAPER_MAX_BYTES + 1),
            Err(WallpaperErrorKind::FileTooLarge)
        );
    }

    #[test]
    fn decoded_pixel_cap() {
        assert!(check_decoded_pixels(10_000, 10_000).is_ok()); // 100 MP exact
        assert_eq!(
            check_decoded_pixels(10_001, 10_000),
            Err(WallpaperErrorKind::DecodeTooLarge)
        );
    }

    #[test]
    fn upscale_letterbox_when_image_tiny() {
        // 100x100 into 1920x1080 needs >1.5× cover scale.
        assert_eq!(fill_decision(100, 100, 1920, 1080), FillDecision::Letterbox);
        // 1920x1080 into 1920x1080 → cover OK.
        assert_eq!(fill_decision(1920, 1080, 1920, 1080), FillDecision::Cover);
        // 1280x720 into 1920x1080 is 1.5× exactly → cover OK.
        assert_eq!(fill_decision(1280, 720, 1920, 1080), FillDecision::Cover);
    }

    #[test]
    fn pin_set_blocks_delete() {
        let mut pins = WallpaperPinSet::new();
        let p = PathBuf::from("cache/applied-1.png");
        pins.pin(p.clone());
        assert!(pins.is_pinned(&p));
        assert!(!pins.may_delete_cache_file(&p));
        assert!(pins.may_delete_cache_file(Path::new("cache/other.png")));
        pins.unpin(&p);
        assert!(pins.may_delete_cache_file(&p));
    }

    #[test]
    fn image_request_from_monitor() {
        let m = WallpaperMonitor {
            id: WallpaperMonitorId::new(r"\\?\DISPLAY#1"),
            attached: true,
            rect_left: 0,
            rect_top: 0,
            rect_right: 1920,
            rect_bottom: 1080,
            fingerprint: MonitorFingerprint {
                device_path: r"\\?\DISPLAY#1".into(),
                friendly_name: Some("Mock".into()),
                edid_mfg_product: None,
                width_px: 1920,
                height_px: 1080,
                orientation_deg: 0,
            },
        };
        let r = m.image_request();
        assert_eq!(r.width_px, 1920);
        assert_eq!(r.height_px, 1080);
        assert_eq!(r.orientation, Orientation::Landscape);
        assert_eq!(r.fit, FitPolicy::Fill);
        assert!((r.max_upscale - MAX_UPSCALE_FACTOR).abs() < f64::EPSILON);
    }

    #[test]
    fn error_codes_stable() {
        assert_eq!(
            WallpaperErrorKind::FileTooLarge.as_error_code(),
            "WallpaperFileTooLarge"
        );
        assert_eq!(
            WallpaperErrorKind::Platform.error_category_token(),
            "surface"
        );
    }

    #[test]
    fn default_position_is_fill() {
        assert_eq!(WallpaperPosition::default(), WallpaperPosition::Fill);
    }
}
