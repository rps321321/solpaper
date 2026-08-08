//! Autostart adapter: HKCU Run key (Issue #7 / blueprint pack).
//!
//! **HIGH risk:** mutates current-user Run registration. Portable builds must not
//! expose the UI (`portable_allows_autostart_ui() == false`). Default is disabled.

use std::sync::Mutex;

use solpaper_core::{autostart_command_line, AUTOSTART_BACKGROUND_FLAG, AUTOSTART_VALUE_NAME};
use windows::core::PCWSTR;
use windows::Win32::Foundation::{ERROR_FILE_NOT_FOUND, ERROR_SUCCESS};
use windows::Win32::System::Registry::{
    RegCloseKey, RegDeleteValueW, RegOpenKeyExW, RegQueryValueExW, RegSetValueExW, HKEY,
    HKEY_CURRENT_USER, KEY_READ, KEY_SET_VALUE, REG_SZ,
};

const RUN_SUBKEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AutostartError {
    NotInstalledBuild,
    Registry(String),
    InvalidPath,
}

impl std::fmt::Display for AutostartError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotInstalledBuild => write!(f, "autostart only for installed builds"),
            Self::Registry(s) => write!(f, "registry: {s}"),
            Self::InvalidPath => write!(f, "invalid executable path"),
        }
    }
}

impl std::error::Error for AutostartError {}

/// Abstraction for enable/disable/query of logon autostart.
pub trait AutostartStore {
    fn is_enabled(&self) -> Result<bool, AutostartError>;
    fn enable(&self, installed_exe: &str) -> Result<(), AutostartError>;
    fn disable(&self) -> Result<(), AutostartError>;
}

/// In-memory fake for unit tests (optional isolated test key path for integration).
#[derive(Debug, Default)]
pub struct FakeAutostartStore {
    inner: Mutex<FakeState>,
}

#[derive(Debug, Default)]
struct FakeState {
    enabled: bool,
    command: Option<String>,
    /// When set, operations fail with this error once.
    fail_next: Option<AutostartError>,
}

impl FakeAutostartStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn command_line(&self) -> Option<String> {
        self.inner.lock().expect("lock").command.clone()
    }

    pub fn fail_next(&self, err: AutostartError) {
        self.inner.lock().expect("lock").fail_next = Some(err);
    }
}

impl AutostartStore for FakeAutostartStore {
    fn is_enabled(&self) -> Result<bool, AutostartError> {
        let mut g = self.inner.lock().expect("lock");
        if let Some(e) = g.fail_next.take() {
            return Err(e);
        }
        Ok(g.enabled)
    }

    fn enable(&self, installed_exe: &str) -> Result<(), AutostartError> {
        if installed_exe.trim().is_empty() {
            return Err(AutostartError::InvalidPath);
        }
        let mut g = self.inner.lock().expect("lock");
        if let Some(e) = g.fail_next.take() {
            return Err(e);
        }
        g.command = Some(autostart_command_line(installed_exe));
        g.enabled = true;
        Ok(())
    }

    fn disable(&self) -> Result<(), AutostartError> {
        let mut g = self.inner.lock().expect("lock");
        if let Some(e) = g.fail_next.take() {
            return Err(e);
        }
        g.enabled = false;
        g.command = None;
        Ok(())
    }
}

/// Production adapter for `HKCU\...\Run` value `Solpaper`.
///
/// Only mutates the Solpaper value name — never other Run entries.
pub struct WindowsRunKeyAutostart {
    /// When false, enable/disable refuse with [`AutostartError::NotInstalledBuild`].
    pub installed_build: bool,
    /// Override subkey for tests (relative to HKCU); default is the system Run key.
    pub subkey: String,
    /// Value name; default [`AUTOSTART_VALUE_NAME`].
    pub value_name: String,
}

impl Default for WindowsRunKeyAutostart {
    fn default() -> Self {
        Self {
            installed_build: true,
            subkey: RUN_SUBKEY.to_string(),
            value_name: AUTOSTART_VALUE_NAME.to_string(),
        }
    }
}

impl WindowsRunKeyAutostart {
    pub fn production_installed() -> Self {
        Self::default()
    }

    pub fn production_portable() -> Self {
        Self {
            installed_build: false,
            ..Self::default()
        }
    }

    /// Test helper: use a disposable HKCU subkey under Software\Solpaper\...
    pub fn test_key(subkey: impl Into<String>) -> Self {
        Self {
            installed_build: true,
            subkey: subkey.into(),
            value_name: format!("{AUTOSTART_VALUE_NAME}_test"),
        }
    }
}

impl AutostartStore for WindowsRunKeyAutostart {
    fn is_enabled(&self) -> Result<bool, AutostartError> {
        if !self.installed_build {
            return Ok(false);
        }
        match query_run_value(&self.subkey, &self.value_name)? {
            Some(v) => Ok(v.contains(AUTOSTART_BACKGROUND_FLAG)),
            None => Ok(false),
        }
    }

    fn enable(&self, installed_exe: &str) -> Result<(), AutostartError> {
        if !self.installed_build {
            return Err(AutostartError::NotInstalledBuild);
        }
        if installed_exe.trim().is_empty() {
            return Err(AutostartError::InvalidPath);
        }
        let cmd = autostart_command_line(installed_exe);
        set_run_value(&self.subkey, &self.value_name, &cmd)
    }

    fn disable(&self) -> Result<(), AutostartError> {
        if !self.installed_build {
            return Ok(());
        }
        delete_run_value(&self.subkey, &self.value_name)
    }
}

fn open_run_key(subkey: &str, write: bool) -> Result<HKEY, AutostartError> {
    let mut hkey = HKEY::default();
    let access = if write {
        KEY_READ.0 | KEY_SET_VALUE.0
    } else {
        KEY_READ.0
    };
    let wide = wide_z(subkey);
    // SAFETY: RegOpenKeyExW on HKCU with a valid subkey path we own as UTF-16.
    let status = unsafe {
        RegOpenKeyExW(
            HKEY_CURRENT_USER,
            PCWSTR(wide.as_ptr()),
            0,
            windows::Win32::System::Registry::REG_SAM_FLAGS(access),
            &mut hkey,
        )
    };
    if status != ERROR_SUCCESS {
        return Err(AutostartError::Registry(format!(
            "RegOpenKeyExW 0x{:08X}",
            status.0
        )));
    }
    Ok(hkey)
}

fn query_run_value(subkey: &str, name: &str) -> Result<Option<String>, AutostartError> {
    let hkey = open_run_key(subkey, false)?;
    let name_w = wide_z(name);
    let mut ty = REG_SZ;
    let mut size = 0u32;
    // SAFETY: size probe; key opened for read.
    let st = unsafe {
        RegQueryValueExW(
            hkey,
            PCWSTR(name_w.as_ptr()),
            None,
            Some(&mut ty),
            None,
            Some(&mut size),
        )
    };
    if st == ERROR_FILE_NOT_FOUND {
        // SAFETY: close handle.
        unsafe {
            let _ = RegCloseKey(hkey);
        }
        return Ok(None);
    }
    if st != ERROR_SUCCESS {
        unsafe {
            let _ = RegCloseKey(hkey);
        }
        return Err(AutostartError::Registry(format!(
            "RegQueryValueExW size 0x{:08X}",
            st.0
        )));
    }
    let mut buf = vec![0u8; size as usize];
    let st = unsafe {
        RegQueryValueExW(
            hkey,
            PCWSTR(name_w.as_ptr()),
            None,
            Some(&mut ty),
            Some(buf.as_mut_ptr()),
            Some(&mut size),
        )
    };
    unsafe {
        let _ = RegCloseKey(hkey);
    }
    if st == ERROR_FILE_NOT_FOUND {
        return Ok(None);
    }
    if st != ERROR_SUCCESS {
        return Err(AutostartError::Registry(format!(
            "RegQueryValueExW 0x{:08X}",
            st.0
        )));
    }
    // REG_SZ is UTF-16LE bytes.
    let u16s: Vec<u16> = buf
        .chunks_exact(2)
        .map(|c| u16::from_le_bytes([c[0], c[1]]))
        .take_while(|u| *u != 0)
        .collect();
    Ok(Some(String::from_utf16_lossy(&u16s)))
}

fn set_run_value(subkey: &str, name: &str, value: &str) -> Result<(), AutostartError> {
    let hkey = open_run_key(subkey, true)?;
    let name_w = wide_z(name);
    let mut data: Vec<u8> = value
        .encode_utf16()
        .chain(std::iter::once(0))
        .flat_map(|u| u.to_le_bytes())
        .collect();
    // SAFETY: write REG_SZ to key we opened with SET_VALUE; only Solpaper value name.
    let st = unsafe { RegSetValueExW(hkey, PCWSTR(name_w.as_ptr()), 0, REG_SZ, Some(&data)) };
    unsafe {
        let _ = RegCloseKey(hkey);
    }
    // silence unused mut if needed
    let _ = &mut data;
    if st != ERROR_SUCCESS {
        return Err(AutostartError::Registry(format!(
            "RegSetValueExW 0x{:08X}",
            st.0
        )));
    }
    Ok(())
}

fn delete_run_value(subkey: &str, name: &str) -> Result<(), AutostartError> {
    let hkey = match open_run_key(subkey, true) {
        Ok(h) => h,
        Err(_) => return Ok(()),
    };
    let name_w = wide_z(name);
    // SAFETY: delete only our value name.
    let st = unsafe { RegDeleteValueW(hkey, PCWSTR(name_w.as_ptr())) };
    unsafe {
        let _ = RegCloseKey(hkey);
    }
    if st == ERROR_FILE_NOT_FOUND || st == ERROR_SUCCESS {
        return Ok(());
    }
    Err(AutostartError::Registry(format!(
        "RegDeleteValueW 0x{:08X}",
        st.0
    )))
}

fn wide_z(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fake_enable_disable_roundtrip() {
        let fake = FakeAutostartStore::new();
        assert!(!fake.is_enabled().unwrap());
        fake.enable(r"C:\Solpaper\solpaper.exe").unwrap();
        assert!(fake.is_enabled().unwrap());
        let cmd = fake.command_line().unwrap();
        assert!(cmd.contains("--background"));
        assert!(cmd.starts_with('"'));
        fake.disable().unwrap();
        assert!(!fake.is_enabled().unwrap());
        assert!(fake.command_line().is_none());
    }

    #[test]
    fn fake_rejects_empty_path() {
        let fake = FakeAutostartStore::new();
        assert_eq!(fake.enable("").unwrap_err(), AutostartError::InvalidPath);
    }

    #[test]
    fn portable_store_refuses_enable() {
        let s = WindowsRunKeyAutostart::production_portable();
        assert!(!s.is_enabled().unwrap());
        assert_eq!(
            s.enable(r"C:\x\solpaper.exe").unwrap_err(),
            AutostartError::NotInstalledBuild
        );
    }

    /// Integration: write/read/delete under a disposable HKCU test subkey.
    #[test]
    fn windows_test_key_roundtrip() {
        let sub = r"Software\Solpaper\AutostartTest\Run";
        // Ensure parent keys exist via open/create path — RegOpenKeyEx needs existing key.
        // Create by writing via a create helper.
        if create_key_tree(sub).is_err() {
            eprintln!("skip windows_test_key_roundtrip: cannot create test key");
            return;
        }
        let store = WindowsRunKeyAutostart::test_key(sub);
        let _ = store.disable();
        assert!(!store.is_enabled().unwrap_or(true));
        store
            .enable(r"C:\Program Files\Solpaper\solpaper.exe")
            .expect("enable");
        assert!(store.is_enabled().unwrap());
        store.disable().expect("disable");
        assert!(!store.is_enabled().unwrap());
        // Best-effort cleanup of empty keys is optional.
    }

    fn create_key_tree(subkey: &str) -> Result<(), AutostartError> {
        use windows::Win32::System::Registry::{
            RegCreateKeyExW, REG_CREATE_KEY_DISPOSITION, REG_OPTION_NON_VOLATILE,
        };
        let wide = wide_z(subkey);
        let mut hkey = HKEY::default();
        let mut disp = REG_CREATE_KEY_DISPOSITION::default();
        // SAFETY: create HKCU test tree for integration test only.
        let st = unsafe {
            RegCreateKeyExW(
                HKEY_CURRENT_USER,
                PCWSTR(wide.as_ptr()),
                0,
                None,
                REG_OPTION_NON_VOLATILE,
                windows::Win32::System::Registry::REG_SAM_FLAGS(KEY_READ.0 | KEY_SET_VALUE.0),
                None,
                &mut hkey,
                Some(&mut disp as *mut REG_CREATE_KEY_DISPOSITION),
            )
        };
        if st != ERROR_SUCCESS {
            return Err(AutostartError::Registry(format!("create 0x{:08X}", st.0)));
        }
        unsafe {
            let _ = RegCloseKey(hkey);
        }
        Ok(())
    }
}
