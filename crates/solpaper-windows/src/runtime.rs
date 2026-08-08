//! Runtime control window + Shell_NotifyIcon tray host (Issue #20 / pack #7).
//!
//! Registers `Solpaper.Runtime.Control.v1`, owns the session message loop, tray icon
//! (NIM_ADD + NIM_SETVERSION), TaskbarCreated re-add, and fixed-order context menu
//! from `solpaper_core::tray`. Second launch finds this HWND via `FindWindowW`.

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use solpaper_core::{
    alpha1_scaffold_flags, build_tray_menu, TrayCommand, TrayMenuEntry, CONTROL_WINDOW_CLASS,
};
use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, POINT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::Shell::{
    Shell_NotifyIconW, NIF_ICON, NIF_MESSAGE, NIF_TIP, NIM_ADD, NIM_DELETE, NIM_SETVERSION,
    NOTIFYICONDATAW, NOTIFYICONDATAW_0, NOTIFYICON_VERSION_4,
};
use windows::Win32::UI::WindowsAndMessaging::{
    AppendMenuW, CreatePopupMenu, CreateWindowExW, DefWindowProcW, DestroyMenu, DestroyWindow,
    DispatchMessageW, GetCursorPos, GetMessageW, LoadCursorW, LoadIconW, PeekMessageW,
    PostMessageW, PostQuitMessage, RegisterClassW, RegisterWindowMessageW, SetForegroundWindow,
    SetMenuDefaultItem, TrackPopupMenu, TranslateMessage, CS_HREDRAW, CS_VREDRAW, HICON,
    IDC_ARROW, IDI_APPLICATION, MF_DISABLED, MF_ENABLED, MF_GRAYED, MF_SEPARATOR, MF_STRING, MSG,
    PM_REMOVE, TPM_BOTTOMALIGN, TPM_LEFTALIGN, TPM_RIGHTBUTTON, WM_APP, WM_COMMAND, WM_DESTROY,
    WM_NULL, WM_QUIT, WM_RBUTTONUP, WNDCLASSW, WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW, WS_POPUP,
};

use crate::activation::WM_APP_SHOW_SETTINGS;
use crate::placeholder::{create_placeholder_window, destroy_placeholder_window, PlaceholderConfig};

/// Tray callback message (WM_APP + 2). Control window only.
const WM_TRAYICON: u32 = WM_APP + 2;

/// Base id for tray menu commands (must not collide with system ids).
const MENU_ID_BASE: u16 = 0xA000;

static CONTROL_CLASS_REGISTERED: AtomicBool = AtomicBool::new(false);
static TASKBAR_CREATED_MSG: AtomicU32 = AtomicU32::new(0);

/// True after WM_APP_SHOW_SETTINGS until cleared (settings UI is lazy / later bullet).
static SETTINGS_REQUESTED: AtomicBool = AtomicBool::new(false);

/// Whether the Runtime is accepting new tray work (cleared on shutdown).
static ACCEPTING_WORK: AtomicBool = AtomicBool::new(true);

#[derive(Debug)]
pub enum RuntimeError {
    Win32(windows::core::Error),
    Message(String),
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Win32(e) => write!(f, "{e}"),
            Self::Message(s) => write!(f, "{s}"),
        }
    }
}

impl std::error::Error for RuntimeError {}

impl From<windows::core::Error> for RuntimeError {
    fn from(value: windows::core::Error) -> Self {
        Self::Win32(value)
    }
}

/// Host configuration for the Alpha 1 runtime loop.
#[derive(Debug, Clone, Default)]
pub struct RuntimeHostConfig {
    /// When true: create control + tray (+ optional placeholder), pump briefly, tear down.
    pub smoke: bool,
    /// Optional scaffold placeholder surface (widget host lands in later #20 bullets).
    pub placeholder: Option<PlaceholderConfig>,
}

/// Whether a second-launch activation requested Settings (for host / tests).
pub fn take_settings_requested() -> bool {
    SETTINGS_REQUESTED.swap(false, Ordering::SeqCst)
}

/// Run the Runtime: control HWND, tray icon, message loop.
///
/// # Safety context
///
/// All Win32 calls are confined to this module. Tray uses a fixed uid; Explorer
/// restart re-adds via `TaskbarCreated` only (does not reparent widgets).
pub fn run_runtime_host(config: &RuntimeHostConfig) -> Result<(), RuntimeError> {
    ACCEPTING_WORK.store(true, Ordering::SeqCst);
    SETTINGS_REQUESTED.store(false, Ordering::SeqCst);

    unsafe {
        let hinstance = GetModuleHandleW(None)?;
        ensure_control_class(hinstance.into())?;

        let taskbar_msg = RegisterWindowMessageW(w!("TaskbarCreated"));
        if taskbar_msg == 0 {
            return Err(RuntimeError::Message(
                "RegisterWindowMessageW(TaskbarCreated) failed".into(),
            ));
        }
        TASKBAR_CREATED_MSG.store(taskbar_msg, Ordering::SeqCst);

        let class = wide_z(CONTROL_WINDOW_CLASS);
        let title = wide_z("Solpaper Runtime");
        // Hidden top-level window so FindWindowW can locate it (not HWND_MESSAGE).
        let control = CreateWindowExW(
            WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE,
            PCWSTR(class.as_ptr()),
            PCWSTR(title.as_ptr()),
            WS_POPUP,
            0,
            0,
            0,
            0,
            None,
            None,
            hinstance,
            None,
        )?;

        if control.is_invalid() {
            return Err(RuntimeError::Message("control window create failed".into()));
        }

        tray_add(control)?;

        let placeholder = if let Some(ref pc) = config.placeholder {
            Some(create_placeholder_window(pc)?)
        } else {
            None
        };

        if config.smoke {
            pump_peek(48);
            // Graceful teardown without full GetMessage loop.
            ACCEPTING_WORK.store(false, Ordering::SeqCst);
            let _ = tray_delete(control);
            if let Some(ph) = placeholder {
                destroy_placeholder_window(ph);
            }
            let _ = DestroyWindow(control);
            pump_peek(16);
            return Ok(());
        }

        let mut msg = MSG::default();
        loop {
            let ok = GetMessageW(&mut msg, None, 0, 0);
            if ok.0 == -1 {
                break;
            }
            if !ok.as_bool() {
                break; // WM_QUIT
            }
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        // Loop exit: ensure tray removed if still present (WM_DESTROY path also removes).
        let _ = tray_delete(control);
        if let Some(ph) = placeholder {
            // Placeholder may already be destroyed; destroy is idempotent via IsWindow check.
            destroy_placeholder_window(ph);
        }
    }

    Ok(())
}

fn wide_z(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

unsafe fn ensure_control_class(
    hinstance: windows::Win32::Foundation::HINSTANCE,
) -> Result<(), RuntimeError> {
    if CONTROL_CLASS_REGISTERED.swap(true, Ordering::SeqCst) {
        return Ok(());
    }
    let class = wide_z(CONTROL_WINDOW_CLASS);
    let wc = WNDCLASSW {
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(control_wnd_proc),
        hInstance: hinstance,
        hCursor: LoadCursorW(None, IDC_ARROW)?,
        lpszClassName: PCWSTR(class.as_ptr()),
        ..Default::default()
    };
    if RegisterClassW(&wc) == 0 {
        // Already registered in this process is OK.
        let err = windows::core::Error::from_win32();
        if err.code().is_err() {
            // Best-effort: continue if class exists.
        }
    }
    Ok(())
}

fn tray_command_id(cmd: TrayCommand) -> u16 {
    MENU_ID_BASE
        + match cmd {
            TrayCommand::OpenSettings => 0,
            TrayCommand::ToggleEditMode => 1,
            TrayCommand::PomodoroStartPauseResume => 2,
            TrayCommand::PomodoroSkip => 3,
            TrayCommand::PomodoroReset => 4,
            TrayCommand::WallpaperNext => 5,
            TrayCommand::WallpaperHold => 6,
            TrayCommand::ToggleAutostart => 7,
            TrayCommand::OpenDiagnostics => 8,
            TrayCommand::Quit => 9,
        }
}

fn command_from_menu_id(id: u16) -> Option<TrayCommand> {
    if id < MENU_ID_BASE {
        return None;
    }
    Some(match id - MENU_ID_BASE {
        0 => TrayCommand::OpenSettings,
        1 => TrayCommand::ToggleEditMode,
        2 => TrayCommand::PomodoroStartPauseResume,
        3 => TrayCommand::PomodoroSkip,
        4 => TrayCommand::PomodoroReset,
        5 => TrayCommand::WallpaperNext,
        6 => TrayCommand::WallpaperHold,
        7 => TrayCommand::ToggleAutostart,
        8 => TrayCommand::OpenDiagnostics,
        9 => TrayCommand::Quit,
        _ => return None,
    })
}

unsafe fn tray_add(hwnd: HWND) -> Result<(), RuntimeError> {
    let mut nid = NOTIFYICONDATAW {
        cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
        hWnd: hwnd,
        uID: 1,
        uFlags: NIF_MESSAGE | NIF_ICON | NIF_TIP,
        uCallbackMessage: WM_TRAYICON,
        Anonymous: NOTIFYICONDATAW_0 {
            uVersion: NOTIFYICON_VERSION_4,
        },
        ..Default::default()
    };

    // Scaffold icon: standard application icon until product resource lands.
    let icon: HICON = LoadIconW(None, IDI_APPLICATION)?;
    nid.hIcon = icon;

    let tip = "Solpaper";
    let tip_wide: Vec<u16> = tip.encode_utf16().chain(std::iter::once(0)).collect();
    let copy_len = (nid.szTip.len() - 1).min(tip_wide.len());
    nid.szTip[..copy_len].copy_from_slice(&tip_wide[..copy_len]);

    if !Shell_NotifyIconW(NIM_ADD, &nid).as_bool() {
        return Err(RuntimeError::Message("Shell_NotifyIconW NIM_ADD failed".into()));
    }
    // Prefer V4 behavior when available.
    let _ = Shell_NotifyIconW(NIM_SETVERSION, &nid);
    Ok(())
}

unsafe fn tray_delete(hwnd: HWND) -> Result<(), RuntimeError> {
    let nid = NOTIFYICONDATAW {
        cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
        hWnd: hwnd,
        uID: 1,
        ..Default::default()
    };
    let _ = Shell_NotifyIconW(NIM_DELETE, &nid);
    Ok(())
}

unsafe fn show_tray_menu(hwnd: HWND) -> Result<(), RuntimeError> {
    if !ACCEPTING_WORK.load(Ordering::SeqCst) {
        return Ok(());
    }
    let menu = build_tray_menu(alpha1_scaffold_flags(), None);
    let hmenu = CreatePopupMenu()?;
    for entry in &menu {
        match entry {
            TrayMenuEntry::Separator => {
                let _ = AppendMenuW(hmenu, MF_SEPARATOR, 0, PCWSTR::null());
            }
            TrayMenuEntry::Command {
                command,
                enabled,
                label,
            } => {
                let id = tray_command_id(*command) as usize;
                let wide = wide_z(label);
                let flags = if *enabled {
                    MF_STRING | MF_ENABLED
                } else {
                    MF_STRING | MF_GRAYED | MF_DISABLED
                };
                let _ = AppendMenuW(hmenu, flags, id, PCWSTR(wide.as_ptr()));
            }
        }
    }
    // Quit as default when double-click is not used.
    let _ = SetMenuDefaultItem(hmenu, tray_command_id(TrayCommand::OpenSettings) as u32, 0);

    let mut pt = POINT::default();
    let _ = GetCursorPos(&mut pt);
    let _ = SetForegroundWindow(hwnd);
    let _ = TrackPopupMenu(
        hmenu,
        TPM_LEFTALIGN | TPM_BOTTOMALIGN | TPM_RIGHTBUTTON,
        pt.x,
        pt.y,
        0,
        hwnd,
        None,
    );
    // Required so the menu dismisses correctly on click outside.
    let _ = PostMessageW(hwnd, WM_NULL, WPARAM(0), LPARAM(0));
    let _ = DestroyMenu(hmenu);
    Ok(())
}

fn handle_tray_command(hwnd: HWND, cmd: TrayCommand) {
    if !ACCEPTING_WORK.load(Ordering::SeqCst) && cmd != TrayCommand::Quit {
        return;
    }
    match cmd {
        TrayCommand::Quit => {
            ACCEPTING_WORK.store(false, Ordering::SeqCst);
            unsafe {
                let _ = tray_delete(hwnd);
                let _ = DestroyWindow(hwnd);
            }
        }
        TrayCommand::OpenSettings => {
            // Lazy settings window lands in a later #20 bullet; record request.
            SETTINGS_REQUESTED.store(true, Ordering::SeqCst);
            eprintln!("solpaper: Open Settings (host UI deferred)");
        }
        TrayCommand::OpenDiagnostics => {
            eprintln!("solpaper: Diagnostics (host UI deferred)");
        }
        // Disabled in alpha1_scaffold_flags or deferred to later tracer bullets.
        TrayCommand::ToggleEditMode
        | TrayCommand::PomodoroStartPauseResume
        | TrayCommand::PomodoroSkip
        | TrayCommand::PomodoroReset
        | TrayCommand::WallpaperNext
        | TrayCommand::WallpaperHold
        | TrayCommand::ToggleAutostart => {}
    }
}

unsafe extern "system" fn control_wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    let taskbar_created = TASKBAR_CREATED_MSG.load(Ordering::SeqCst);
    if taskbar_created != 0 && msg == taskbar_created {
        // Explorer restarted: re-add tray only (do not recreate widget HWNDs).
        let _ = tray_delete(hwnd);
        let _ = tray_add(hwnd);
        return LRESULT(0);
    }

    match msg {
        m if m == WM_TRAYICON => {
            let mouse = lparam.0 as u32;
            // NOTIFYICON_VERSION_4 packs differently; handle classic LOWORD for scaffold.
            let event = mouse & 0xffff;
            if event == WM_RBUTTONUP || event == 0x0205 {
                // WM_RBUTTONUP = 0x0205
                let _ = show_tray_menu(hwnd);
            } else if event == 0x0203 {
                // WM_LBUTTONDBLCLK — open settings request
                handle_tray_command(hwnd, TrayCommand::OpenSettings);
            }
            LRESULT(0)
        }
        WM_COMMAND => {
            let id = (wparam.0 as u32) & 0xffff;
            if let Some(cmd) = command_from_menu_id(id as u16) {
                handle_tray_command(hwnd, cmd);
            }
            LRESULT(0)
        }
        m if m == WM_APP_SHOW_SETTINGS => {
            SETTINGS_REQUESTED.store(true, Ordering::SeqCst);
            eprintln!("solpaper: show settings requested (second launch)");
            LRESULT(0)
        }
        WM_DESTROY => {
            ACCEPTING_WORK.store(false, Ordering::SeqCst);
            let _ = tray_delete(hwnd);
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

unsafe fn pump_peek(max: u32) {
    let mut msg = MSG::default();
    for _ in 0..max {
        if !PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).as_bool() {
            break;
        }
        if msg.message == WM_QUIT {
            break;
        }
        let _ = TranslateMessage(&msg);
        DispatchMessageW(&msg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn menu_ids_round_trip() {
        for cmd in [
            TrayCommand::OpenSettings,
            TrayCommand::Quit,
            TrayCommand::ToggleAutostart,
            TrayCommand::WallpaperNext,
        ] {
            let id = tray_command_id(cmd);
            assert_eq!(command_from_menu_id(id), Some(cmd));
        }
    }

    #[test]
    fn control_class_name_matches_core() {
        assert_eq!(CONTROL_WINDOW_CLASS, "Solpaper.Runtime.Control.v1");
        assert_eq!(WM_TRAYICON, WM_APP + 2);
    }
}
