//! `IDesktopWallpaper` COM adapter (blueprint #5).
//!
//! # Safety
//!
//! All COM calls are confined to this module. Callers must use the adapter from
//! an STA-compatible thread (typically the UI thread that called
//! [`ComDesktopWallpaper::new`]). The adapter initializes COM as STA if the
//! thread is not already in an apartment.

use std::path::{Path, PathBuf};

use solpaper_core::{
    check_local_file_size, validate_source_path_shape, MonitorFingerprint, WallpaperErrorKind,
    WallpaperMonitor, WallpaperMonitorId, WallpaperPosition,
};
use windows::core::{HSTRING, PWSTR};
use windows::Win32::Foundation::RECT;
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CoTaskMemFree, CoUninitialize, CLSCTX_ALL,
    COINIT_APARTMENTTHREADED,
};
use windows::Win32::UI::Shell::{
    DesktopWallpaper, IDesktopWallpaper, DESKTOP_WALLPAPER_POSITION, DWPOS_CENTER, DWPOS_FILL,
    DWPOS_FIT, DWPOS_SPAN, DWPOS_STRETCH, DWPOS_TILE,
};

use super::{DesktopWallpaper, WallpaperError};

/// HRESULT values treated as transient COM server disconnect (recreate once).
const RPC_E_DISCONNECTED: i32 = -2147417848; // 0x80010108
const CO_E_OBJNOTCONNECTED: i32 = -2147220995; // 0x800401FD
const RPC_S_SERVER_UNAVAILABLE: i32 = -2147023174; // 0x800706BA

/// RAII COM apartment: uninitializes only if this guard performed the init.
struct ComApartment {
    should_uninit: bool,
}

impl ComApartment {
    fn init_sta() -> Result<Self, WallpaperError> {
        // SAFETY: CoInitializeEx on the calling thread; paired with CoUninitialize in Drop
        // when this call returned S_OK (first init on the thread).
        let hr = unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED) };
        if hr.is_ok() {
            // S_OK — we own uninit.
            return Ok(Self {
                should_uninit: true,
            });
        }
        // S_FALSE (1): already initialized on this thread — do not uninit.
        if hr.0 == 1 {
            return Ok(Self {
                should_uninit: false,
            });
        }
        Err(WallpaperError::with_detail(
            WallpaperErrorKind::Platform,
            format!("CoInitializeEx failed: 0x{:08X}", hr.0 as u32),
        ))
    }
}

impl Drop for ComApartment {
    fn drop(&mut self) {
        if self.should_uninit {
            // SAFETY: balances a successful CoInitializeEx from this guard.
            unsafe { CoUninitialize() };
        }
    }
}

/// Production adapter owning one `IDesktopWallpaper` instance.
pub struct ComDesktopWallpaper {
    _apartment: ComApartment,
    inner: IDesktopWallpaper,
    /// After a transient COM failure we recreate at most once per operation.
    recreated: bool,
}

impl ComDesktopWallpaper {
    /// Activate `IDesktopWallpaper` on the current thread (STA).
    pub fn new() -> Result<Self, WallpaperError> {
        let apartment = ComApartment::init_sta()?;
        let inner = create_desktop_wallpaper()?;
        Ok(Self {
            _apartment: apartment,
            inner,
            recreated: false,
        })
    }

    fn recreate_inner(&mut self) -> Result<(), WallpaperError> {
        self.inner = create_desktop_wallpaper()?;
        self.recreated = true;
        Ok(())
    }

    fn with_retry<T>(
        &mut self,
        mut op: impl FnMut(&IDesktopWallpaper) -> Result<T, WallpaperError>,
    ) -> Result<T, WallpaperError> {
        match op(&self.inner) {
            Ok(v) => Ok(v),
            Err(e) if e.kind == WallpaperErrorKind::PlatformTransient && !self.recreated => {
                self.recreate_inner()?;
                op(&self.inner)
            }
            Err(e) => Err(e),
        }
    }
}

fn create_desktop_wallpaper() -> Result<IDesktopWallpaper, WallpaperError> {
    // SAFETY: CoCreateInstance for the documented DesktopWallpaper CLSID; result is an
    // owned IDesktopWallpaper COM pointer managed by the windows crate.
    unsafe { CoCreateInstance(&DesktopWallpaper, None, CLSCTX_ALL) }.map_err(map_com_error)
}

fn map_com_error(e: windows::core::Error) -> WallpaperError {
    let code = e.code().0;
    if code == RPC_E_DISCONNECTED
        || code == CO_E_OBJNOTCONNECTED
        || code == RPC_S_SERVER_UNAVAILABLE
    {
        return WallpaperError::with_detail(
            WallpaperErrorKind::PlatformTransient,
            format!("0x{:08X}", code as u32),
        );
    }
    WallpaperError::with_detail(
        WallpaperErrorKind::Platform,
        format!("0x{:08X}", code as u32),
    )
}

/// Convert COM `PWSTR` (caller-owned, CoTaskMem) into `String` and free.
///
/// # Safety
/// `p` must be null or a CoTaskMem-allocated wide string from `IDesktopWallpaper`.
unsafe fn take_pwstr(p: PWSTR) -> String {
    if p.is_null() {
        return String::new();
    }
    let s = p.to_string().unwrap_or_default();
    CoTaskMemFree(Some(p.0 as *const _));
    s
}

fn map_position_from_win(p: DESKTOP_WALLPAPER_POSITION) -> WallpaperPosition {
    match p {
        DWPOS_CENTER => WallpaperPosition::Center,
        DWPOS_TILE => WallpaperPosition::Tile,
        DWPOS_STRETCH => WallpaperPosition::Stretch,
        DWPOS_FIT => WallpaperPosition::Fit,
        DWPOS_FILL => WallpaperPosition::Fill,
        DWPOS_SPAN => WallpaperPosition::Span,
        _ => WallpaperPosition::Fill,
    }
}

fn map_position_to_win(p: WallpaperPosition) -> DESKTOP_WALLPAPER_POSITION {
    match p {
        WallpaperPosition::Center => DWPOS_CENTER,
        WallpaperPosition::Tile => DWPOS_TILE,
        WallpaperPosition::Stretch => DWPOS_STRETCH,
        WallpaperPosition::Fit => DWPOS_FIT,
        WallpaperPosition::Fill => DWPOS_FILL,
        WallpaperPosition::Span => DWPOS_SPAN,
    }
}

fn rect_attached(r: RECT) -> bool {
    (r.right - r.left) > 0 && (r.bottom - r.top) > 0
}

impl DesktopWallpaper for ComDesktopWallpaper {
    fn monitors(&self) -> Result<Vec<WallpaperMonitor>, WallpaperError> {
        // Note: monitors() uses &self; retry requires &mut. Expose via helper on &self
        // without recreate for enumerate — recreate is handled in mut methods.
        enumerate_monitors(&self.inner)
    }

    fn current(&self, monitor: &WallpaperMonitorId) -> Result<Option<PathBuf>, WallpaperError> {
        if monitor.is_empty() {
            return Err(WallpaperError::new(WallpaperErrorKind::Internal));
        }
        let id = HSTRING::from(monitor.as_str());
        // SAFETY: GetWallpaper with monitor device path; returns CoTaskMem string.
        let pw = unsafe { self.inner.GetWallpaper(&id) }.map_err(map_com_error)?;
        let s = unsafe { take_pwstr(pw) };
        if s.is_empty() {
            Ok(None)
        } else {
            Ok(Some(PathBuf::from(s)))
        }
    }

    fn apply(&self, monitor: &WallpaperMonitorId, owned_file: &Path) -> Result<(), WallpaperError> {
        validate_source_path_shape(owned_file).map_err(WallpaperError::from)?;
        if monitor.is_empty() {
            return Err(WallpaperError::new(WallpaperErrorKind::Internal));
        }

        let canon = owned_file
            .canonicalize()
            .map_err(|_| WallpaperError::new(WallpaperErrorKind::PathInvalid))?;
        let meta = std::fs::metadata(&canon)
            .map_err(|_| WallpaperError::new(WallpaperErrorKind::PathInvalid))?;
        if !meta.is_file() {
            return Err(WallpaperError::new(WallpaperErrorKind::PathInvalid));
        }
        check_local_file_size(meta.len()).map_err(WallpaperError::from)?;

        let id = HSTRING::from(monitor.as_str());
        let path = HSTRING::from(canon.as_os_str());
        // SAFETY: SetWallpaper with monitor device path and absolute owned file path.
        // On failure the system wallpaper is left unchanged by Windows.
        unsafe { self.inner.SetWallpaper(&id, &path) }.map_err(map_com_error)
    }

    fn position(&self) -> Result<WallpaperPosition, WallpaperError> {
        // SAFETY: GetPosition has no string ownership.
        let p = unsafe { self.inner.GetPosition() }.map_err(map_com_error)?;
        Ok(map_position_from_win(p))
    }

    fn set_position(&self, position: WallpaperPosition) -> Result<(), WallpaperError> {
        let p = map_position_to_win(position);
        // SAFETY: SetPosition with documented enum.
        unsafe { self.inner.SetPosition(p) }.map_err(map_com_error)
    }
}

fn enumerate_monitors(inner: &IDesktopWallpaper) -> Result<Vec<WallpaperMonitor>, WallpaperError> {
    // SAFETY: GetMonitorDevicePathCount is a simple out-u32 COM call.
    let count = unsafe { inner.GetMonitorDevicePathCount() }.map_err(map_com_error)?;
    let mut out = Vec::with_capacity(count as usize);
    for i in 0..count {
        // SAFETY: index in range; returns CoTaskMem device path string.
        let pw = unsafe { inner.GetMonitorDevicePathAt(i) }.map_err(map_com_error)?;
        let device_path = unsafe { take_pwstr(pw) };
        if device_path.is_empty() {
            continue;
        }
        let id_h = HSTRING::from(device_path.as_str());
        // SAFETY: GetMonitorRECT for this device path.
        let rect = unsafe { inner.GetMonitorRECT(&id_h) };
        let (attached, rect) = match rect {
            Ok(r) if rect_attached(r) => (true, r),
            Ok(r) => (false, r),
            Err(_) => (
                false,
                RECT {
                    left: 0,
                    top: 0,
                    right: 0,
                    bottom: 0,
                },
            ),
        };
        let id = WallpaperMonitorId::new(device_path.clone());
        out.push(WallpaperMonitor {
            id,
            attached,
            rect_left: rect.left,
            rect_top: rect.top,
            rect_right: rect.right,
            rect_bottom: rect.bottom,
            fingerprint: MonitorFingerprint {
                device_path,
                friendly_name: None,
                edid_mfg_product: None,
                width_px: (rect.right - rect.left).max(0),
                height_px: (rect.bottom - rect.top).max(0),
                orientation_deg: 0,
            },
        });
    }
    Ok(out)
}

/// Mutable helper for operations that may recreate the COM object once.
impl ComDesktopWallpaper {
    /// Like [`DesktopWallpaper::apply`] but recreates the COM object once on transient failure.
    pub fn apply_with_recover(
        &mut self,
        monitor: &WallpaperMonitorId,
        owned_file: &Path,
    ) -> Result<(), WallpaperError> {
        let monitor = monitor.clone();
        let owned = owned_file.to_path_buf();
        self.with_retry(|inner| apply_on(inner, &monitor, &owned))
    }

    pub fn monitors_with_recover(&mut self) -> Result<Vec<WallpaperMonitor>, WallpaperError> {
        self.with_retry(enumerate_monitors)
    }
}

fn apply_on(
    inner: &IDesktopWallpaper,
    monitor: &WallpaperMonitorId,
    owned_file: &Path,
) -> Result<(), WallpaperError> {
    validate_source_path_shape(owned_file).map_err(WallpaperError::from)?;
    if monitor.is_empty() {
        return Err(WallpaperError::new(WallpaperErrorKind::Internal));
    }
    let canon = owned_file
        .canonicalize()
        .map_err(|_| WallpaperError::new(WallpaperErrorKind::PathInvalid))?;
    let meta = std::fs::metadata(&canon)
        .map_err(|_| WallpaperError::new(WallpaperErrorKind::PathInvalid))?;
    if !meta.is_file() {
        return Err(WallpaperError::new(WallpaperErrorKind::PathInvalid));
    }
    check_local_file_size(meta.len()).map_err(WallpaperError::from)?;
    let id = HSTRING::from(monitor.as_str());
    let path = HSTRING::from(canon.as_os_str());
    // SAFETY: SetWallpaper; failure leaves prior wallpaper.
    unsafe { inner.SetWallpaper(&id, &path) }.map_err(map_com_error)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_roundtrip_maps() {
        assert_eq!(
            map_position_from_win(map_position_to_win(WallpaperPosition::Fill)),
            WallpaperPosition::Fill
        );
        assert_eq!(
            map_position_from_win(map_position_to_win(WallpaperPosition::Fit)),
            WallpaperPosition::Fit
        );
    }

    /// Integration smoke: enumerate monitors when COM is available (Windows CI / owner host).
    /// Does not change wallpaper.
    #[test]
    fn com_enumerate_smoke() {
        let adapter = match ComDesktopWallpaper::new() {
            Ok(a) => a,
            Err(e) => {
                // Allow non-interactive environments that refuse COM.
                eprintln!("skip com_enumerate_smoke: {e}");
                return;
            }
        };
        let mons = adapter.monitors().expect("enumerate monitors");
        // At least one entry is typical on a desktop session; empty is not a hard fail in CI.
        for m in &mons {
            assert!(!m.id.is_empty());
            assert_eq!(m.fingerprint.device_path, m.id.as_str());
        }
    }
}
