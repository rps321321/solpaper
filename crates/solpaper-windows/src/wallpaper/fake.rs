//! In-memory [`DesktopWallpaper`] for contract tests (no COM).

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use solpaper_core::{
    check_local_file_size, validate_source_path_shape, MonitorFingerprint, WallpaperErrorKind,
    WallpaperMonitor, WallpaperMonitorId, WallpaperPosition,
};

use super::{DesktopWallpaper, WallpaperError};

/// Configurable fake: enumerate / query / apply / error injection.
#[derive(Debug)]
pub struct FakeDesktopWallpaper {
    inner: Mutex<FakeState>,
}

#[derive(Debug)]
struct FakeState {
    monitors: Vec<WallpaperMonitor>,
    /// monitor id → current wallpaper path
    current: HashMap<String, PathBuf>,
    position: WallpaperPosition,
    /// When set, next `apply` returns this error once (then cleared).
    next_apply_error: Option<WallpaperError>,
    apply_count: u32,
    fail_applies_remaining: u32,
}

impl FakeDesktopWallpaper {
    pub fn new(monitors: Vec<WallpaperMonitor>) -> Self {
        Self {
            inner: Mutex::new(FakeState {
                monitors,
                current: HashMap::new(),
                position: WallpaperPosition::Fill,
                next_apply_error: None,
                apply_count: 0,
                fail_applies_remaining: 0,
            }),
        }
    }

    /// Single attached 1920×1080 mock monitor.
    pub fn single_mock() -> Self {
        let id = WallpaperMonitorId::new(r"\\?\DISPLAY#MOCK1");
        let mon = WallpaperMonitor {
            id: id.clone(),
            attached: true,
            rect_left: 0,
            rect_top: 0,
            rect_right: 1920,
            rect_bottom: 1080,
            fingerprint: MonitorFingerprint {
                device_path: id.as_str().to_string(),
                friendly_name: Some("Mock Monitor".into()),
                edid_mfg_product: None,
                width_px: 1920,
                height_px: 1080,
                orientation_deg: 0,
            },
        };
        Self::new(vec![mon])
    }

    pub fn fail_next_apply(&self, err: WallpaperError) {
        self.inner.lock().expect("fake lock").next_apply_error = Some(err);
    }

    pub fn fail_applies(&self, n: u32) {
        self.inner.lock().expect("fake lock").fail_applies_remaining = n;
    }

    pub fn apply_count(&self) -> u32 {
        self.inner.lock().expect("fake lock").apply_count
    }

    pub fn set_current(&self, monitor: &WallpaperMonitorId, path: PathBuf) {
        self.inner
            .lock()
            .expect("fake lock")
            .current
            .insert(monitor.as_str().to_string(), path);
    }
}

impl DesktopWallpaper for FakeDesktopWallpaper {
    fn monitors(&self) -> Result<Vec<WallpaperMonitor>, WallpaperError> {
        Ok(self.inner.lock().expect("fake lock").monitors.clone())
    }

    fn current(&self, monitor: &WallpaperMonitorId) -> Result<Option<PathBuf>, WallpaperError> {
        let g = self.inner.lock().expect("fake lock");
        if !g.monitors.iter().any(|m| m.id == *monitor) {
            return Err(WallpaperError::new(WallpaperErrorKind::MonitorUnavailable));
        }
        Ok(g.current.get(monitor.as_str()).cloned())
    }

    fn apply(&self, monitor: &WallpaperMonitorId, owned_file: &Path) -> Result<(), WallpaperError> {
        validate_source_path_shape(owned_file).map_err(WallpaperError::from)?;

        let mut g = self.inner.lock().expect("fake lock");
        let mon = g
            .monitors
            .iter()
            .find(|m| m.id == *monitor)
            .cloned()
            .ok_or_else(|| WallpaperError::new(WallpaperErrorKind::MonitorUnavailable))?;
        if !mon.attached {
            return Err(WallpaperError::new(WallpaperErrorKind::MonitorUnavailable));
        }

        if let Some(err) = g.next_apply_error.take() {
            return Err(err);
        }
        if g.fail_applies_remaining > 0 {
            g.fail_applies_remaining -= 1;
            return Err(WallpaperError::with_detail(
                WallpaperErrorKind::Platform,
                "injected apply failure",
            ));
        }

        // Optional size check when the path exists on the test host.
        if owned_file.is_file() {
            let meta = std::fs::metadata(owned_file)
                .map_err(|_| WallpaperError::new(WallpaperErrorKind::PathInvalid))?;
            check_local_file_size(meta.len()).map_err(WallpaperError::from)?;
        }

        g.apply_count += 1;
        g.current
            .insert(monitor.as_str().to_string(), owned_file.to_path_buf());
        Ok(())
    }

    fn position(&self) -> Result<WallpaperPosition, WallpaperError> {
        Ok(self.inner.lock().expect("fake lock").position)
    }

    fn set_position(&self, position: WallpaperPosition) -> Result<(), WallpaperError> {
        self.inner.lock().expect("fake lock").position = position;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn enumerate_and_apply_success() {
        let fake = FakeDesktopWallpaper::single_mock();
        let mons = fake.monitors().unwrap();
        assert_eq!(mons.len(), 1);
        assert!(mons[0].attached);

        let dir = std::env::temp_dir().join("solpaper-fake-wp");
        let _ = std::fs::create_dir_all(&dir);
        let file = dir.join("ok.png");
        let mut f = std::fs::File::create(&file).unwrap();
        f.write_all(b"not-a-real-png-but-extension-ok").unwrap();

        fake.apply(&mons[0].id, &file).unwrap();
        assert_eq!(
            fake.current(&mons[0].id).unwrap().as_deref(),
            Some(file.as_path())
        );
        assert_eq!(fake.apply_count(), 1);
        assert_eq!(fake.position().unwrap(), WallpaperPosition::Fill);
    }

    #[test]
    fn apply_failure_leaves_previous() {
        let fake = FakeDesktopWallpaper::single_mock();
        let id = fake.monitors().unwrap()[0].id.clone();
        let prev = PathBuf::from("cache/previous.png");
        fake.set_current(&id, prev.clone());

        fake.fail_next_apply(WallpaperError::new(WallpaperErrorKind::Platform));
        let err = fake.apply(&id, Path::new("cache/new.png")).unwrap_err();
        assert_eq!(err.kind, WallpaperErrorKind::Platform);
        assert_eq!(fake.current(&id).unwrap(), Some(prev));
        assert_eq!(fake.apply_count(), 0);
    }

    #[test]
    fn rejects_unsupported_extension() {
        let fake = FakeDesktopWallpaper::single_mock();
        let id = fake.monitors().unwrap()[0].id.clone();
        let err = fake.apply(&id, Path::new("x.gif")).unwrap_err();
        assert_eq!(err.kind, WallpaperErrorKind::FormatUnsupported);
    }

    #[test]
    fn unknown_monitor() {
        let fake = FakeDesktopWallpaper::single_mock();
        let err = fake
            .current(&WallpaperMonitorId::new("missing"))
            .unwrap_err();
        assert_eq!(err.kind, WallpaperErrorKind::MonitorUnavailable);
    }

    #[test]
    fn default_position_fill_and_set() {
        let fake = FakeDesktopWallpaper::single_mock();
        assert_eq!(fake.position().unwrap(), WallpaperPosition::Fill);
        fake.set_position(WallpaperPosition::Fit).unwrap();
        assert_eq!(fake.position().unwrap(), WallpaperPosition::Fit);
    }
}
