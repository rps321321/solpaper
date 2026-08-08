//! Desktop wallpaper adapter surface (Issue #5).
//!
//! - [`DesktopWallpaper`] — platform interface (no HWND / overlay types).
//! - [`FakeDesktopWallpaper`] — unit-test seam.
//! - [`ComDesktopWallpaper`] — `IDesktopWallpaper` via the `windows` crate.

mod com;
mod fake;

pub use com::ComDesktopWallpaper;
pub use fake::FakeDesktopWallpaper;

use std::path::{Path, PathBuf};

use solpaper_core::{WallpaperErrorKind, WallpaperMonitor, WallpaperMonitorId, WallpaperPosition};

/// Adapter error: typed kind + optional platform detail (never tokens/paths with secrets).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WallpaperError {
    pub kind: WallpaperErrorKind,
    /// Short platform message or HRESULT hex; must not include private user content.
    pub detail: Option<String>,
}

impl WallpaperError {
    pub fn new(kind: WallpaperErrorKind) -> Self {
        Self { kind, detail: None }
    }

    pub fn with_detail(kind: WallpaperErrorKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: Some(detail.into()),
        }
    }

    pub fn error_code(&self) -> &'static str {
        self.kind.as_error_code()
    }
}

impl std::fmt::Display for WallpaperError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.detail {
            Some(d) => write!(f, "{} ({d})", self.kind.as_error_code()),
            None => write!(f, "{}", self.kind.as_error_code()),
        }
    }
}

impl std::error::Error for WallpaperError {}

impl From<WallpaperErrorKind> for WallpaperError {
    fn from(kind: WallpaperErrorKind) -> Self {
        Self::new(kind)
    }
}

/// Platform-neutral wallpaper control plane.
///
/// Implementations must not accept widget HWND types or WorkerW parenting.
pub trait DesktopWallpaper {
    fn monitors(&self) -> Result<Vec<WallpaperMonitor>, WallpaperError>;
    fn current(&self, monitor: &WallpaperMonitorId) -> Result<Option<PathBuf>, WallpaperError>;
    /// Apply an **owned** cache/local file path. On failure, leave the system wallpaper unchanged.
    fn apply(&self, monitor: &WallpaperMonitorId, owned_file: &Path) -> Result<(), WallpaperError>;
    fn position(&self) -> Result<WallpaperPosition, WallpaperError>;
    fn set_position(&self, position: WallpaperPosition) -> Result<(), WallpaperError>;
}
