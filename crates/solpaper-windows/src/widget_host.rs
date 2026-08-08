//! Production Approach A widget host (ADR-0001 / #20 tracer bullet 2).
//!
//! One top-level layered HWND per widget. Normal Mode is click-through; Edit Mode
//! exposes drag strip + resize grip. Escape and Ctrl+Alt+F2 are owned by the Runtime
//! control window; this module applies mode and input to widget HWNDs only.
//!
//! Scaffold: DIP treated as physical pixels until multi-monitor DPI conversion lands.

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Mutex;

use solpaper_core::{
    apply_edit_arrow, apply_move, apply_resize, clamp_rect_visible, classify_widget_hit, EditArrow,
    SurfaceMode, SurfaceRect, WidgetHit, WorkArea, DRAG_STRIP_DIP, MIN_WIDGET_SIZE_DIP,
    RESIZE_GRIP_DIP,
};
use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{COLORREF, HWND, LPARAM, LRESULT, POINT, RECT, WPARAM};
use windows::Win32::Graphics::Gdi::{
    BeginPaint, CreateSolidBrush, DeleteObject, EndPaint, FillRect, FrameRect, InvalidateRect,
    HBRUSH, PAINTSTRUCT,
};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::Input::KeyboardAndMouse::{ReleaseCapture, SetCapture};
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DestroyWindow, GetClientRect, GetCursorPos, GetSystemMetrics,
    GetWindowLongW, GetWindowRect, IsWindow, LoadCursorW, RegisterClassW,
    SetLayeredWindowAttributes, SetWindowLongW, SetWindowPos, ShowWindow, CS_HREDRAW, CS_VREDRAW,
    GWL_EXSTYLE, HTCLIENT, HTTRANSPARENT, HWND_BOTTOM, IDC_ARROW, LWA_ALPHA, SM_CXSCREEN,
    SM_CYSCREEN, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SWP_NOZORDER, SWP_SHOWWINDOW,
    SW_SHOWNOACTIVATE, WINDOW_EX_STYLE, WM_DESTROY, WM_KEYDOWN, WM_LBUTTONDOWN, WM_LBUTTONUP,
    WM_MOUSEMOVE, WM_NCHITTEST, WM_PAINT, WNDCLASSW, WS_EX_LAYERED, WS_EX_NOACTIVATE,
    WS_EX_TOOLWINDOW, WS_EX_TRANSPARENT, WS_POPUP, WS_VISIBLE,
};

static CLASS_REGISTERED: AtomicBool = AtomicBool::new(false);
const CLASS_NAME: PCWSTR = w!("Solpaper.Widget.Host.v1");

/// Class atom name for FindWindow / diagnostics.
pub const WIDGET_WINDOW_CLASS: &str = "Solpaper.Widget.Host.v1";

/// Placement for one widget surface (scaffold units ≈ DIP).
#[derive(Debug, Clone)]
pub struct WidgetSurfaceConfig {
    pub id: String,
    pub title: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub opacity: u8,
}

impl Default for WidgetSurfaceConfig {
    fn default() -> Self {
        Self {
            id: "placeholder".into(),
            title: "Solpaper".into(),
            x: 48,
            y: 48,
            width: 280,
            height: 160,
            opacity: 220,
        }
    }
}

/// HWND is not `Send` in the `windows` crate; UI-thread-only host owns these handles.
#[derive(Clone, Copy)]
struct UiHwnd(HWND);

// SAFETY: Widget host state is mutated only on the UI thread that owns the message loop.
unsafe impl Send for UiHwnd {}
unsafe impl Sync for UiHwnd {}

#[derive(Clone, Copy)]
struct DragState {
    index: usize,
    kind: WidgetHit,
    start_cursor: POINT,
    start_rect: RECT,
}

struct HostState {
    mode: SurfaceMode,
    widgets: Vec<WidgetSlot>,
    drag: Option<DragState>,
    selected: usize,
}

struct WidgetSlot {
    id: String,
    hwnd: UiHwnd,
}

static STATE: Mutex<Option<HostState>> = Mutex::new(None);
/// Mirrors STATE.mode for fast hit-test path without lock contention on every NCHITTEST if needed.
static EDIT_ACTIVE: AtomicBool = AtomicBool::new(false);
static SELECTED: AtomicUsize = AtomicUsize::new(0);

/// Create all widget HWNDs (Approach A). Caller must pump messages; destroy with [`destroy_all_widgets`].
pub fn create_widget_host(configs: &[WidgetSurfaceConfig]) -> windows::core::Result<Vec<HWND>> {
    unsafe {
        let hinstance = GetModuleHandleW(None)?;
        ensure_class(hinstance.into())?;

        let mut slots = Vec::with_capacity(configs.len());
        let mut hwnds = Vec::with_capacity(configs.len());
        for cfg in configs {
            let hwnd = create_one(hinstance.into(), cfg)?;
            // Normal Mode default: click-through.
            set_click_through(hwnd, true);
            send_toward_desktop(hwnd);
            slots.push(WidgetSlot {
                id: cfg.id.clone(),
                hwnd: UiHwnd(hwnd),
            });
            hwnds.push(hwnd);
        }

        let mut guard = STATE.lock().expect("widget host state");
        *guard = Some(HostState {
            mode: SurfaceMode::Normal,
            widgets: slots,
            drag: None,
            selected: 0,
        });
        EDIT_ACTIVE.store(false, Ordering::SeqCst);
        SELECTED.store(0, Ordering::SeqCst);
        Ok(hwnds)
    }
}

/// Destroy every widget HWND and clear host state.
pub fn destroy_all_widgets() {
    let mut guard = STATE.lock().expect("widget host state");
    if let Some(state) = guard.take() {
        for slot in state.widgets {
            unsafe {
                let hwnd = slot.hwnd.0;
                if !hwnd.is_invalid() && IsWindow(hwnd).as_bool() {
                    let _ = DestroyWindow(hwnd);
                }
            }
        }
    }
    EDIT_ACTIVE.store(false, Ordering::SeqCst);
}

/// Current surface mode (Normal / Edit).
pub fn surface_mode() -> SurfaceMode {
    STATE
        .lock()
        .expect("widget host state")
        .as_ref()
        .map(|s| s.mode)
        .unwrap_or(SurfaceMode::Normal)
}

/// Enter or leave Edit Mode; updates click-through and chrome on all widgets.
pub fn set_surface_mode(mode: SurfaceMode) {
    let mut guard = STATE.lock().expect("widget host state");
    let Some(state) = guard.as_mut() else {
        return;
    };
    if state.mode == mode {
        return;
    }
    state.mode = mode;
    state.drag = None;
    EDIT_ACTIVE.store(mode.is_edit(), Ordering::SeqCst);
    let click_through = !mode.is_edit();
    for slot in &state.widgets {
        unsafe {
            set_click_through(slot.hwnd.0, click_through);
            let _ = InvalidateRect(slot.hwnd.0, None, true);
        }
    }
    eprintln!(
        "solpaper: surface mode → {} ({})",
        mode.as_str(),
        if mode.is_edit() {
            "interactive"
        } else {
            "click-through"
        }
    );
}

/// Toggle Normal ↔ Edit.
pub fn toggle_surface_mode() -> SurfaceMode {
    let next = surface_mode().toggle();
    set_surface_mode(next);
    next
}

/// Live geometries after drag/resize (for tests / later layout persistence).
pub fn snapshot_widget_rects() -> Vec<(String, SurfaceRect)> {
    let guard = STATE.lock().expect("widget host state");
    let Some(state) = guard.as_ref() else {
        return Vec::new();
    };
    let mut out = Vec::with_capacity(state.widgets.len());
    for slot in &state.widgets {
        unsafe {
            let hwnd = slot.hwnd.0;
            if hwnd.is_invalid() || !IsWindow(hwnd).as_bool() {
                continue;
            }
            let mut rc = RECT::default();
            let _ = GetWindowRect(hwnd, &mut rc);
            if let Ok(rect) = SurfaceRect::new(
                rc.left as f32,
                rc.top as f32,
                (rc.right - rc.left) as f32,
                (rc.bottom - rc.top) as f32,
            ) {
                out.push((slot.id.clone(), rect));
            }
        }
    }
    out
}

/// Primary work area in scaffold units (full primary screen metrics).
pub fn primary_work_area() -> WorkArea {
    unsafe {
        let w = GetSystemMetrics(SM_CXSCREEN) as f32;
        let h = GetSystemMetrics(SM_CYSCREEN) as f32;
        WorkArea::new(0.0, 0.0, w.max(1.0), h.max(1.0)).unwrap_or(WorkArea {
            x: 0.0,
            y: 0.0,
            width: 1920.0,
            height: 1080.0,
        })
    }
}

unsafe fn create_one(
    hinstance: windows::Win32::Foundation::HINSTANCE,
    config: &WidgetSurfaceConfig,
) -> windows::core::Result<HWND> {
    let work = primary_work_area();
    let raw = SurfaceRect::new(
        config.x as f32,
        config.y as f32,
        config.width.max(1) as f32,
        config.height.max(1) as f32,
    )
    .unwrap_or(SurfaceRect {
        x: 48.0,
        y: 48.0,
        width: 280.0,
        height: 160.0,
    });
    let placed = clamp_rect_visible(raw, work);
    let x = placed.x as i32;
    let y = placed.y as i32;
    let width = placed.width.max(1.0) as i32;
    let height = placed.height.max(1.0) as i32;

    let title = windows::core::HSTRING::from(config.title.as_str());
    // Start without WS_EX_TRANSPARENT; set_click_through applies Normal Mode after create.
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
    Ok(hwnd)
}

unsafe fn set_click_through(hwnd: HWND, enabled: bool) {
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

unsafe fn send_toward_desktop(hwnd: HWND) {
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

unsafe fn move_resize_hwnd(hwnd: HWND, rect: SurfaceRect) {
    let _ = SetWindowPos(
        hwnd,
        None,
        rect.x as i32,
        rect.y as i32,
        rect.width.max(1.0) as i32,
        rect.height.max(1.0) as i32,
        SWP_NOZORDER | SWP_NOACTIVATE,
    );
}

unsafe fn ensure_class(
    hinstance: windows::Win32::Foundation::HINSTANCE,
) -> windows::core::Result<()> {
    if CLASS_REGISTERED.swap(true, Ordering::SeqCst) {
        return Ok(());
    }
    let wc = WNDCLASSW {
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(widget_wnd_proc),
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

fn index_for_hwnd(state: &HostState, hwnd: HWND) -> Option<usize> {
    state.widgets.iter().position(|w| w.hwnd.0 == hwnd)
}

unsafe extern "system" fn widget_wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_NCHITTEST => {
            if !EDIT_ACTIVE.load(Ordering::SeqCst) {
                return LRESULT(HTTRANSPARENT as isize);
            }
            LRESULT(HTCLIENT as isize)
        }
        WM_PAINT => {
            paint_widget(hwnd);
            LRESULT(0)
        }
        WM_LBUTTONDOWN => {
            if !EDIT_ACTIVE.load(Ordering::SeqCst) {
                return LRESULT(0);
            }
            on_lbutton_down(hwnd);
            LRESULT(0)
        }
        WM_MOUSEMOVE => {
            on_mouse_move();
            LRESULT(0)
        }
        WM_LBUTTONUP => {
            on_lbutton_up();
            LRESULT(0)
        }
        WM_KEYDOWN => {
            if EDIT_ACTIVE.load(Ordering::SeqCst) {
                on_key_down(wparam);
            }
            LRESULT(0)
        }
        WM_DESTROY => LRESULT(0),
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

unsafe fn paint_widget(hwnd: HWND) {
    let edit = EDIT_ACTIVE.load(Ordering::SeqCst);
    let mut ps = PAINTSTRUCT::default();
    let hdc = BeginPaint(hwnd, &mut ps);
    if hdc.is_invalid() {
        return;
    }
    let mut rc = RECT::default();
    let _ = GetClientRect(hwnd, &mut rc);
    // Body fill (cool slate in Normal; slightly warmer in Edit).
    let body = if edit {
        COLORREF(0x00_3A_4A_5A)
    } else {
        COLORREF(0x00_C8_B4_A0)
    };
    let brush = CreateSolidBrush(body);
    if !brush.is_invalid() {
        let _ = FillRect(hdc, &rc, brush);
        let _ = DeleteObject(HBRUSH(brush.0));
    }

    if edit {
        // Clear border (not color-alone meaning: also draw drag strip + grip).
        let border = CreateSolidBrush(COLORREF(0x00_F0_F0_F0));
        if !border.is_invalid() {
            let _ = FrameRect(hdc, &rc, border);
            let _ = DeleteObject(HBRUSH(border.0));
        }
        // Drag strip (top 24).
        let strip_h = (DRAG_STRIP_DIP as i32).min(rc.bottom - rc.top).max(1);
        let strip = RECT {
            left: rc.left,
            top: rc.top,
            right: rc.right,
            bottom: rc.top + strip_h,
        };
        let strip_brush = CreateSolidBrush(COLORREF(0x00_20_80_C0));
        if !strip_brush.is_invalid() {
            let _ = FillRect(hdc, &strip, strip_brush);
            let _ = DeleteObject(HBRUSH(strip_brush.0));
        }
        // Resize grip (bottom-right 12).
        let grip = (RESIZE_GRIP_DIP as i32)
            .min(rc.right - rc.left)
            .min(rc.bottom - rc.top)
            .max(1);
        let grip_rc = RECT {
            left: rc.right - grip,
            top: rc.bottom - grip,
            right: rc.right,
            bottom: rc.bottom,
        };
        let grip_brush = CreateSolidBrush(COLORREF(0x00_E0_E0_E0));
        if !grip_brush.is_invalid() {
            let _ = FillRect(hdc, &grip_rc, grip_brush);
            let _ = DeleteObject(HBRUSH(grip_brush.0));
        }
    }

    let _ = EndPaint(hwnd, &ps);
}

unsafe fn on_lbutton_down(hwnd: HWND) {
    let mut guard = STATE.lock().expect("widget host state");
    let Some(state) = guard.as_mut() else {
        return;
    };
    let Some(i) = index_for_hwnd(state, hwnd) else {
        return;
    };
    state.selected = i;
    SELECTED.store(i, Ordering::SeqCst);

    let mut cursor = POINT::default();
    let _ = GetCursorPos(&mut cursor);
    let mut rc = RECT::default();
    let _ = GetWindowRect(hwnd, &mut rc);
    let local_x = (cursor.x - rc.left) as f32;
    let local_y = (cursor.y - rc.top) as f32;
    let width = (rc.right - rc.left) as f32;
    let height = (rc.bottom - rc.top) as f32;
    let hit = classify_widget_hit(width, height, local_x, local_y).unwrap_or(WidgetHit::Body);
    if matches!(hit, WidgetHit::Drag | WidgetHit::Resize) {
        state.drag = Some(DragState {
            index: i,
            kind: hit,
            start_cursor: cursor,
            start_rect: rc,
        });
        let _ = SetCapture(hwnd);
    }
}

unsafe fn on_mouse_move() {
    let mut guard = STATE.lock().expect("widget host state");
    let Some(state) = guard.as_mut() else {
        return;
    };
    let Some(drag) = state.drag else {
        return;
    };
    let Some(slot) = state.widgets.get(drag.index) else {
        return;
    };
    let mut cursor = POINT::default();
    let _ = GetCursorPos(&mut cursor);
    let dx = (cursor.x - drag.start_cursor.x) as f32;
    let dy = (cursor.y - drag.start_cursor.y) as f32;
    let r = drag.start_rect;
    let base = SurfaceRect {
        x: r.left as f32,
        y: r.top as f32,
        width: (r.right - r.left) as f32,
        height: (r.bottom - r.top) as f32,
    };
    let work = primary_work_area();
    let next = match drag.kind {
        WidgetHit::Drag => apply_move(base, dx, dy, work),
        WidgetHit::Resize => apply_resize(base, dx, dy, work),
        WidgetHit::Body => return,
    };
    move_resize_hwnd(slot.hwnd.0, next);
}

unsafe fn on_lbutton_up() {
    let mut guard = STATE.lock().expect("widget host state");
    if let Some(state) = guard.as_mut() {
        state.drag = None;
    }
    let _ = ReleaseCapture();
}

unsafe fn on_key_down(wparam: WPARAM) {
    use windows::Win32::UI::Input::KeyboardAndMouse::{
        GetKeyState, VK_CONTROL, VK_DOWN, VK_LEFT, VK_RIGHT, VK_SHIFT, VK_UP,
    };
    let vk = wparam.0 as i32;
    let arrow = match vk {
        x if x == VK_LEFT.0 as i32 => EditArrow::Left,
        x if x == VK_RIGHT.0 as i32 => EditArrow::Right,
        x if x == VK_UP.0 as i32 => EditArrow::Up,
        x if x == VK_DOWN.0 as i32 => EditArrow::Down,
        _ => return,
    };
    let ctrl = GetKeyState(VK_CONTROL.0 as i32) < 0;
    let shift = GetKeyState(VK_SHIFT.0 as i32) < 0;

    let mut guard = STATE.lock().expect("widget host state");
    let Some(state) = guard.as_mut() else {
        return;
    };
    let i = state.selected.min(state.widgets.len().saturating_sub(1));
    let Some(slot) = state.widgets.get(i) else {
        return;
    };
    let mut rc = RECT::default();
    let _ = GetWindowRect(slot.hwnd.0, &mut rc);
    let base = SurfaceRect {
        x: rc.left as f32,
        y: rc.top as f32,
        width: (rc.right - rc.left).max(MIN_WIDGET_SIZE_DIP as i32) as f32,
        height: (rc.bottom - rc.top).max(MIN_WIDGET_SIZE_DIP as i32) as f32,
    };
    let work = primary_work_area();
    let next = apply_edit_arrow(base, arrow, ctrl, shift, work);
    move_resize_hwnd(slot.hwnd.0, next);
    let _ = InvalidateRect(slot.hwnd.0, None, true);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn widget_class_name_stable() {
        assert_eq!(WIDGET_WINDOW_CLASS, "Solpaper.Widget.Host.v1");
    }

    #[test]
    fn default_surface_config_positive() {
        let d = WidgetSurfaceConfig::default();
        assert!(d.width > 0 && d.height > 0);
    }
}
