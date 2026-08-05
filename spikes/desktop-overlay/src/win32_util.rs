//! Thin Win32 helpers. All `unsafe` is confined here or in window procedures.

use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{COLORREF, HWND, LPARAM, LRESULT, RECT, WPARAM};
use windows::Win32::Graphics::Gdi::{
    BeginPaint, BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, CreateSolidBrush, DeleteDC,
    DeleteObject, EndPaint, FillRect, SelectObject, HDC, HGDIOBJ, PAINTSTRUCT, SRCCOPY,
};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, GetClientRect, GetWindowLongW, LoadCursorW, RegisterClassW,
    SetLayeredWindowAttributes, SetWindowLongW, SetWindowPos, ShowWindow, CS_HREDRAW, CS_VREDRAW,
    GWL_EXSTYLE, HWND_BOTTOM, IDC_ARROW, LWA_ALPHA, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE,
    SWP_NOZORDER, SWP_SHOWWINDOW, SW_SHOWNOACTIVATE, WINDOW_EX_STYLE, WINDOW_STYLE, WNDCLASSW,
    WS_EX_LAYERED, WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW, WS_EX_TRANSPARENT, WS_POPUP,
};

pub const CLASS_A: PCWSTR = w!("SolpaperSpikeApproachA");
pub const CLASS_B: PCWSTR = w!("SolpaperSpikeApproachB");

/// Global hotkeys use Ctrl+Alt so the spike does not steal bare Esc/S/F2 system-wide.
pub mod hotkeys {
    pub const TOGGLE_EDIT: i32 = 1;
    pub const QUIT: i32 = 2;
    pub const OPACITY_UP: i32 = 3;
    pub const OPACITY_DOWN: i32 = 4;
    pub const SAVE: i32 = 5;
}

pub fn base_ex_style(click_through: bool) -> WINDOW_EX_STYLE {
    let mut style = WS_EX_LAYERED | WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE;
    if click_through {
        style |= WS_EX_TRANSPARENT;
    }
    style
}

pub fn popup_style() -> WINDOW_STYLE {
    WS_POPUP
}

pub unsafe fn register_class(
    class_name: PCWSTR,
    wnd_proc: unsafe extern "system" fn(HWND, u32, WPARAM, LPARAM) -> LRESULT,
) -> windows::core::Result<()> {
    let hinstance = GetModuleHandleW(None)?;
    let wc = WNDCLASSW {
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(wnd_proc),
        hInstance: hinstance.into(),
        lpszClassName: class_name,
        hCursor: LoadCursorW(None, IDC_ARROW)?,
        ..Default::default()
    };
    let atom = RegisterClassW(&wc);
    if atom == 0 {
        // ERROR_CLASS_ALREADY_EXISTS = 1410 — acceptable on re-run in-process.
        let err = windows::core::Error::from_win32();
        if err.code().0 as u32 != 1410 {
            return Err(err);
        }
    }
    Ok(())
}

pub unsafe fn create_layered_window(
    class_name: PCWSTR,
    title: PCWSTR,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    click_through: bool,
) -> windows::core::Result<HWND> {
    let hinstance = GetModuleHandleW(None)?;
    let hwnd = CreateWindowExW(
        base_ex_style(click_through),
        class_name,
        title,
        popup_style(),
        x,
        y,
        width,
        height,
        None,
        None,
        hinstance,
        None,
    )?;
    Ok(hwnd)
}

pub unsafe fn show_no_activate(hwnd: HWND) {
    let _ = ShowWindow(hwnd, SW_SHOWNOACTIVATE);
}

/// Prefer non-topmost placement so ordinary apps can cover widgets.
pub unsafe fn send_toward_desktop(hwnd: HWND) {
    let _ = SetWindowPos(
        hwnd,
        HWND_BOTTOM,
        0,
        0,
        0,
        0,
        SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE | SWP_SHOWWINDOW,
    );
}

pub unsafe fn set_window_alpha(hwnd: HWND, alpha: u8) {
    let _ = SetLayeredWindowAttributes(hwnd, COLORREF(0), alpha, LWA_ALPHA);
}

pub unsafe fn set_click_through(hwnd: HWND, enabled: bool) {
    let mut ex = WINDOW_EX_STYLE(GetWindowLongW(hwnd, GWL_EXSTYLE) as u32);
    if enabled {
        ex |= WS_EX_TRANSPARENT;
    } else {
        ex &= !WS_EX_TRANSPARENT;
    }
    SetWindowLongW(hwnd, GWL_EXSTYLE, ex.0 as i32);
    let _ = SetWindowPos(
        hwnd,
        None,
        0,
        0,
        0,
        0,
        SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE,
    );
}

pub unsafe fn move_resize(hwnd: HWND, x: i32, y: i32, w: i32, h: i32) {
    let _ = SetWindowPos(hwnd, None, x, y, w, h, SWP_NOZORDER | SWP_NOACTIVATE);
}

pub unsafe fn destroy(hwnd: HWND) {
    let _ = windows::Win32::UI::WindowsAndMessaging::DestroyWindow(hwnd);
}

/// Double-buffered paint into the window client area.
pub unsafe fn paint_double_buffered<F>(hwnd: HWND, mut draw: F)
where
    F: FnMut(HDC, i32, i32),
{
    let mut ps = PAINTSTRUCT::default();
    let hdc = BeginPaint(hwnd, &mut ps);
    let mut rc = RECT::default();
    let _ = GetClientRect(hwnd, &mut rc);
    let width = rc.right - rc.left;
    let height = rc.bottom - rc.top;
    if width <= 0 || height <= 0 {
        let _ = EndPaint(hwnd, &ps);
        return;
    }

    let mem_dc = CreateCompatibleDC(hdc);
    let bmp = CreateCompatibleBitmap(hdc, width, height);
    let old = SelectObject(mem_dc, HGDIOBJ(bmp.0));

    let brush = CreateSolidBrush(COLORREF(0x00_12_12_18));
    let _ = FillRect(mem_dc, &rc, brush);
    let _ = DeleteObject(HGDIOBJ(brush.0));

    draw(mem_dc, width, height);

    let _ = BitBlt(hdc, 0, 0, width, height, mem_dc, 0, 0, SRCCOPY);
    let _ = SelectObject(mem_dc, old);
    let _ = DeleteObject(HGDIOBJ(bmp.0));
    let _ = DeleteDC(mem_dc);
    let _ = EndPaint(hwnd, &ps);
}

/// Ctrl+Alt hotkeys so NOACTIVATE windows still receive commands without stealing bare keys.
pub unsafe fn register_spike_hotkeys(hwnd: HWND) -> windows::core::Result<()> {
    use windows::Win32::UI::Input::KeyboardAndMouse::{
        RegisterHotKey, HOT_KEY_MODIFIERS, MOD_ALT, MOD_CONTROL, MOD_NOREPEAT, VK_ESCAPE, VK_F2,
        VK_OEM_MINUS, VK_OEM_PLUS, VK_S,
    };
    let mods = HOT_KEY_MODIFIERS(MOD_CONTROL.0 | MOD_ALT.0 | MOD_NOREPEAT.0);
    RegisterHotKey(hwnd, hotkeys::TOGGLE_EDIT, mods, VK_F2.0 as u32)?;
    RegisterHotKey(hwnd, hotkeys::QUIT, mods, VK_ESCAPE.0 as u32)?;
    RegisterHotKey(hwnd, hotkeys::OPACITY_UP, mods, VK_OEM_PLUS.0 as u32)?;
    RegisterHotKey(hwnd, hotkeys::OPACITY_DOWN, mods, VK_OEM_MINUS.0 as u32)?;
    RegisterHotKey(hwnd, hotkeys::SAVE, mods, VK_S.0 as u32)?;
    Ok(())
}

pub unsafe fn unregister_spike_hotkeys(hwnd: HWND) {
    use windows::Win32::UI::Input::KeyboardAndMouse::UnregisterHotKey;
    for id in [
        hotkeys::TOGGLE_EDIT,
        hotkeys::QUIT,
        hotkeys::OPACITY_UP,
        hotkeys::OPACITY_DOWN,
        hotkeys::SAVE,
    ] {
        let _ = UnregisterHotKey(hwnd, id);
    }
}

pub fn primary_work_area() -> RECT {
    use windows::Win32::UI::WindowsAndMessaging::{SystemParametersInfoW, SPI_GETWORKAREA};
    let mut rc = RECT::default();
    unsafe {
        let _ = SystemParametersInfoW(
            SPI_GETWORKAREA,
            0,
            Some((&mut rc as *mut RECT).cast()),
            Default::default(),
        );
    }
    rc
}
