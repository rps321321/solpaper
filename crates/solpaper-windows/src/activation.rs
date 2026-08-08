//! Narrow second-launch activation (Issue #7).
//!
//! Not a general IPC protocol (ADR-0007). When the named mutex is already held,
//! the second process posts [`WM_APP_SHOW_SETTINGS`] to the control window and exits 0.

use solpaper_core::CONTROL_WINDOW_CLASS;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{FindWindowW, IsWindow, PostMessageW, WM_APP};

/// Application-defined message: show or create the in-process Settings window.
///
/// `WM_APP + 1` — reserved for Solpaper Runtime control surface only.
pub const WM_APP_SHOW_SETTINGS: u32 = WM_APP + 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActivationError {
    ControlWindowNotFound,
    PostFailed(String),
}

impl std::fmt::Display for ActivationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ControlWindowNotFound => {
                write!(f, "runtime control window not found")
            }
            Self::PostFailed(s) => write!(f, "PostMessage failed: {s}"),
        }
    }
}

impl std::error::Error for ActivationError {}

/// Find the Runtime control window and post show-settings.
///
/// # Safety context
///
/// Uses `FindWindowW` / `PostMessageW` only. Does not send arbitrary payloads.
pub fn activate_existing_show_settings() -> Result<(), ActivationError> {
    let class = wide_z(CONTROL_WINDOW_CLASS);
    // SAFETY: FindWindowW with a well-known class name we register; no title match.
    let hwnd = unsafe { FindWindowW(PCWSTR(class.as_ptr()), PCWSTR::null()) }
        .map_err(|_| ActivationError::ControlWindowNotFound)?;
    if hwnd.is_invalid() || unsafe { !IsWindow(hwnd).as_bool() } {
        return Err(ActivationError::ControlWindowNotFound);
    }
    post_show_settings(hwnd)
}

/// Post show-settings to a known HWND (unit / host tests).
pub fn post_show_settings(hwnd: HWND) -> Result<(), ActivationError> {
    // SAFETY: PostMessageW of WM_APP_SHOW_SETTINGS with zero wParam/lParam only.
    let ok = unsafe { PostMessageW(hwnd, WM_APP_SHOW_SETTINGS, WPARAM(0), LPARAM(0)) };
    if ok.is_err() {
        return Err(ActivationError::PostFailed(
            "PostMessageW returned FALSE".into(),
        ));
    }
    Ok(())
}

fn wide_z(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

/// Policy helper: second-launch outcome for the host `main`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecondLaunchOutcome {
    /// Posted activation; process should exit 0.
    Activated,
    /// Control window missing (race/crash); exit 0 without starting a second runtime.
    AlreadyRunningNoWindow,
}

/// Map activation attempt for second process (never starts a second Runtime).
pub fn second_launch_outcome(activate: Result<(), ActivationError>) -> SecondLaunchOutcome {
    match activate {
        Ok(()) => SecondLaunchOutcome::Activated,
        Err(ActivationError::ControlWindowNotFound) => SecondLaunchOutcome::AlreadyRunningNoWindow,
        Err(ActivationError::PostFailed(_)) => SecondLaunchOutcome::AlreadyRunningNoWindow,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wm_app_show_settings_is_wm_app_plus_one() {
        assert_eq!(WM_APP_SHOW_SETTINGS, WM_APP + 1);
        assert_eq!(CONTROL_WINDOW_CLASS, "Solpaper.Runtime.Control.v1");
    }

    #[test]
    fn second_launch_outcomes() {
        assert_eq!(
            second_launch_outcome(Ok(())),
            SecondLaunchOutcome::Activated
        );
        assert_eq!(
            second_launch_outcome(Err(ActivationError::ControlWindowNotFound)),
            SecondLaunchOutcome::AlreadyRunningNoWindow
        );
    }
}
