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

// --- Local folder catalog + shuffled bag (Issue #20 bullet 6 / pack #5+#20) ---

/// Injectable RNG for deterministic bag shuffles (tests inject fixed sequence).
pub trait RandomSource {
    fn next_u64(&mut self) -> u64;
}

/// Tiny xorshift64* for production host (not crypto).
#[derive(Debug, Clone)]
pub struct XorShift64 {
    state: u64,
}

impl XorShift64 {
    pub fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 {
                0x9E37_79B9_7F4A_7C15
            } else {
                seed
            },
        }
    }

    pub fn from_entropy() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(1);
        Self::new(nanos ^ std::process::id() as u64)
    }
}

impl RandomSource for XorShift64 {
    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
}

/// Non-recursive enumeration of accepted image files under each folder.
///
/// Invalid entries are skipped. Paths are canonicalized when possible and sorted
/// lexicographically for a stable bag source (shuffle is separate).
pub fn list_local_images(folders: &[PathBuf]) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for folder in folders {
        let Ok(rd) = std::fs::read_dir(folder) else {
            continue;
        };
        for entry in rd.flatten() {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            if !is_accepted_extension(&path) {
                continue;
            }
            let canon = std::fs::canonicalize(&path).unwrap_or(path);
            out.push(canon);
        }
    }
    out.sort();
    out.dedup();
    out
}

/// Fisher–Yates shuffled bag: no repeat until exhausted when ≥2 images.
#[derive(Debug, Clone, Default)]
pub struct ShuffledBag {
    source: Vec<PathBuf>,
    remaining: Vec<PathBuf>,
}

impl ShuffledBag {
    pub fn new(source: Vec<PathBuf>) -> Self {
        Self {
            source,
            remaining: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.source.is_empty()
    }

    pub fn source_len(&self) -> usize {
        self.source.len()
    }

    /// Replace catalog (e.g. after folder rescan); clears remaining bag.
    pub fn set_source(&mut self, source: Vec<PathBuf>) {
        self.source = source;
        self.remaining.clear();
    }

    fn refill<R: RandomSource>(&mut self, rng: &mut R) {
        self.remaining = self.source.clone();
        // Fisher–Yates
        let n = self.remaining.len();
        if n < 2 {
            return;
        }
        for i in (1..n).rev() {
            let j = (rng.next_u64() as usize) % (i + 1);
            self.remaining.swap(i, j);
        }
    }

    /// Next path for Manual Next. Refills when exhausted (≥2 sources) or always
    /// returns the sole image when only one exists. `None` when catalog empty.
    pub fn next<R: RandomSource>(&mut self, rng: &mut R) -> Option<PathBuf> {
        if self.source.is_empty() {
            return None;
        }
        if self.source.len() == 1 {
            return self.source.first().cloned();
        }
        if self.remaining.is_empty() {
            self.refill(rng);
        }
        self.remaining.pop()
    }
}

/// Session-facing wallpaper controller state (pure; host owns COM apply).
#[derive(Debug, Clone, Default)]
pub struct LocalWallpaperController {
    pub folders: Vec<PathBuf>,
    /// When true, automatic cycle is suppressed (Alpha 1 has no schedule; flag is still toggled).
    pub hold: bool,
    pub bag: ShuffledBag,
    pub last_applied: Option<PathBuf>,
    pub pins: WallpaperPinSet,
}

impl LocalWallpaperController {
    pub fn from_folders(folders: Vec<PathBuf>, hold: bool) -> Self {
        let images = list_local_images(&folders);
        Self {
            folders,
            hold,
            bag: ShuffledBag::new(images),
            last_applied: None,
            pins: WallpaperPinSet::new(),
        }
    }

    /// Rescan folders and rebuild bag source (keeps hold / pins / last_applied).
    pub fn rescan(&mut self) {
        let images = list_local_images(&self.folders);
        self.bag.set_source(images);
    }

    /// Manual Next: pick next bag image path (does **not** apply; host does).
    /// Hold does not block manual Next (pack: Hold prevents *automatic* change).
    pub fn pick_next<R: RandomSource>(&mut self, rng: &mut R) -> Option<PathBuf> {
        self.bag.next(rng)
    }

    pub fn toggle_hold(&mut self) -> bool {
        self.hold = !self.hold;
        self.hold
    }

    pub fn note_applied(&mut self, owned: PathBuf) {
        self.pins.pin(owned.clone());
        self.last_applied = Some(owned);
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

    #[derive(Default)]
    struct SeqRng {
        seq: Vec<u64>,
        i: usize,
    }
    impl RandomSource for SeqRng {
        fn next_u64(&mut self) -> u64 {
            let v = self.seq.get(self.i).copied().unwrap_or(0);
            self.i += 1;
            v
        }
    }

    #[test]
    fn shuffled_bag_no_repeat_until_exhausted() {
        let paths: Vec<PathBuf> = (0..4)
            .map(|i| PathBuf::from(format!("img{i}.jpg")))
            .collect();
        let mut bag = ShuffledBag::new(paths.clone());
        let mut rng = SeqRng {
            seq: vec![0, 0, 0, 0, 0, 0, 0, 0],
            i: 0,
        };
        let mut seen = Vec::new();
        for _ in 0..4 {
            let p = bag.next(&mut rng).unwrap();
            assert!(!seen.contains(&p), "repeat before exhaust: {p:?}");
            seen.push(p);
        }
        // After exhaust, refill may produce any order; all 4 unique first cycle.
        assert_eq!(seen.len(), 4);
        let fifth = bag.next(&mut rng).unwrap();
        assert!(paths.contains(&fifth));
    }

    #[test]
    fn single_image_always_returns_same() {
        let mut bag = ShuffledBag::new(vec![PathBuf::from("only.png")]);
        let mut rng = SeqRng::default();
        assert_eq!(bag.next(&mut rng).unwrap(), PathBuf::from("only.png"));
        assert_eq!(bag.next(&mut rng).unwrap(), PathBuf::from("only.png"));
    }

    #[test]
    fn empty_bag_returns_none() {
        let mut bag = ShuffledBag::new(vec![]);
        let mut rng = SeqRng::default();
        assert!(bag.next(&mut rng).is_none());
    }

    #[test]
    fn list_local_images_non_recursive_sorted() {
        let root = std::env::temp_dir().join(format!(
            "solpaper-wp-list-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(root.join("nested")).unwrap();
        std::fs::write(root.join("b.PNG"), b"x").unwrap();
        std::fs::write(root.join("a.jpg"), b"x").unwrap();
        std::fs::write(root.join("skip.gif"), b"x").unwrap();
        std::fs::write(root.join("nested").join("deep.jpg"), b"x").unwrap();
        let list = list_local_images(std::slice::from_ref(&root));
        assert_eq!(list.len(), 2, "{list:?}");
        // Sorted by full path; both files present; nested excluded.
        assert!(list.iter().all(|p| is_accepted_extension(p)));
        assert!(!list.iter().any(|p| p.to_string_lossy().contains("deep")));
        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn hold_toggle_does_not_block_pick() {
        let mut ctl = LocalWallpaperController {
            folders: vec![],
            hold: false,
            bag: ShuffledBag::new(vec![PathBuf::from("a.jpg")]),
            last_applied: None,
            pins: WallpaperPinSet::new(),
        };
        assert!(ctl.toggle_hold());
        let mut rng = SeqRng::default();
        assert_eq!(ctl.pick_next(&mut rng).unwrap(), PathBuf::from("a.jpg"));
    }
}
