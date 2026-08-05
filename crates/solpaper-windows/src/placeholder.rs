//! Minimal layered placeholder HWND host (ADR-0001 / ADR-0003).
//!
//! Intentionally small: not a port of the disposable spike architecture.

use std::sync::atomic::{AtomicBool, Ordering};

use solpaper_core::{DipPoint, DipSize};
use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{COLORREF, HINSTANCE, HWND, LPARAM, LRESULT, RECT, WPARAM};
use windows::Win32::Graphics::Gdi::{
    BeginPaint, CreateSolidBrush, DeleteObject, EndPaint, FillRect, InvalidateRect, HBRUSH,
    PAINTSTRUCT,
};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DestroyWindow, DispatchMessageW, GetClientRect, GetMessageW,
    GetSystemMetrics, LoadCursorW, PeekMessageW, PostQuitMessage, RegisterClassW,
    SetLayeredWindowAttributes, ShowWindow, TranslateMessage, CS_HREDRAW, CS_VREDRAW, IDC_ARROW,
    LWA_ALPHA, MSG, PM_REMOVE, SM_CXSCREEN, SM_CYSCREEN, SW_SHOWNOACTIVATE, WM_DESTROY, WM_PAINT,
    WM_QUIT, WNDCLASSW, WS_EX_LAYERED, WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW, WS_POPUP, WS_VISIBLE,
};

static CLASS_REGISTERED: AtomicBool = AtomicBool::new(false);
const CLASS_NAME: PCWSTR = w!("SolpaperPlaceholderHost");

/// Placement for the scaffold placeholder surface (physical pixels at create time).
#[derive(Debug, Clone)]
pub struct PlaceholderConfig {
    pub title: String,
    pub origin: DipPoint,
    pub size: DipSize,
    pub opacity: u8,
}

impl Default for PlaceholderConfig {
    fn default() -> Self {
        Self {
            title: "Solpaper".into(),
            origin: DipPoint { x: 48.0, y: 48.0 },
            size: DipSize {
                width: 280.0,
                height: 160.0,
            },
            opacity: 220,
        }
    }
}

/// Create a single placeholder widget window and run a message loop until closed.
///
/// When `smoke` is true, create the window, pump a few messages with `PeekMessage`,
/// destroy the window, and return (for automated checks).
pub fn run_placeholder_host(config: &PlaceholderConfig, smoke: bool) -> windows::core::Result<()> {
    unsafe {
        let hinstance = HINSTANCE(GetModuleHandleW(None)?.0);
        ensure_class(hinstance)?;

        // Scaffold: treat DIP as physical pixels until full DPI conversion lands with multi-mon.
        let x = config.origin.x as i32;
        let y = config.origin.y as i32;
        let width = config.size.width.max(1.0) as i32;
        let height = config.size.height.max(1.0) as i32;

        let screen_w = GetSystemMetrics(SM_CXSCREEN);
        let screen_h = GetSystemMetrics(SM_CYSCREEN);
        let x = x.clamp(0, (screen_w - 32).max(0));
        let y = y.clamp(0, (screen_h - 32).max(0));

        let title = windows::core::HSTRING::from(config.title.as_str());
        let hwnd = CreateWindowExW(
            WS_EX_LAYERED | WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE,
            CLASS_NAME,
            &title,
            WS_POPUP | WS_VISIBLE,
            x,
            y,
            width,
            height,
            None,
            None,
            hinstance,
            None,
        )?;

        SetLayeredWindowAttributes(hwnd, COLORREF(0), config.opacity, LWA_ALPHA)?;
        let _ = ShowWindow(hwnd, SW_SHOWNOACTIVATE);
        let _ = InvalidateRect(hwnd, None, true);

        if smoke {
            pump_peek(hwnd, 32);
            let _ = DestroyWindow(hwnd);
            pump_peek(hwnd, 16);
            return Ok(());
        }

        let mut msg = MSG::default();
        loop {
            let ok = GetMessageW(&mut msg, None, 0, 0);
            if ok.0 == -1 {
                break;
            }
            if !ok.as_bool() {
                break;
            }
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
    Ok(())
}

unsafe fn pump_peek(hwnd: HWND, max: u32) {
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
        // Avoid spinning forever if the queue stays full of paint noise for this HWND only.
        let _ = hwnd;
    }
}

unsafe fn ensure_class(hinstance: HINSTANCE) -> windows::core::Result<()> {
    if CLASS_REGISTERED.swap(true, Ordering::SeqCst) {
        return Ok(());
    }
    let wc = WNDCLASSW {
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(wnd_proc),
        hInstance: hinstance,
        hCursor: LoadCursorW(None, IDC_ARROW)?,
        lpszClassName: CLASS_NAME,
        ..Default::default()
    };
    if RegisterClassW(&wc) == 0 {
        let _ = windows::core::Error::from_win32();
    }
    Ok(())
}

unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_PAINT => {
            let mut ps = PAINTSTRUCT::default();
            let hdc = BeginPaint(hwnd, &mut ps);
            if !hdc.is_invalid() {
                let mut rc = RECT::default();
                let _ = GetClientRect(hwnd, &mut rc);
                let brush = CreateSolidBrush(COLORREF(0x00C8B4A0));
                if !brush.is_invalid() {
                    let _ = FillRect(hdc, &rc, brush);
                    let _ = DeleteObject(HBRUSH(brush.0));
                }
                let _ = EndPaint(hwnd, &ps);
            }
            LRESULT(0)
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}
