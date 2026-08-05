//! Single-instance guard via a named mutex (ADR-0002).

use windows::core::HSTRING;
use windows::Win32::Foundation::{CloseHandle, GetLastError, ERROR_ALREADY_EXISTS, HANDLE};
use windows::Win32::System::Threading::CreateMutexW;

const MUTEX_NAME: &str = "Local\\SolpaperSingleInstance_v1";

#[derive(Debug)]
pub enum SingleInstanceError {
    AlreadyRunning,
    CreateFailed(windows::core::Error),
}

impl std::fmt::Display for SingleInstanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SingleInstanceError::AlreadyRunning => {
                write!(f, "another Solpaper instance is already running")
            }
            SingleInstanceError::CreateFailed(e) => write!(f, "CreateMutexW failed: {e}"),
        }
    }
}

impl std::error::Error for SingleInstanceError {}

/// Holds the named mutex for the process lifetime.
pub struct SingleInstanceGuard {
    handle: HANDLE,
}

impl SingleInstanceGuard {
    pub fn acquire() -> Result<Self, SingleInstanceError> {
        let name = HSTRING::from(MUTEX_NAME);
        unsafe {
            let handle =
                CreateMutexW(None, true, &name).map_err(SingleInstanceError::CreateFailed)?;
            let err = GetLastError();
            if err == ERROR_ALREADY_EXISTS {
                let _ = CloseHandle(handle);
                return Err(SingleInstanceError::AlreadyRunning);
            }
            Ok(Self { handle })
        }
    }
}

impl Drop for SingleInstanceGuard {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseHandle(self.handle);
        }
    }
}
