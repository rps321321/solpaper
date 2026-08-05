//! Approach A: one transparent top-level HWND per sample widget.
//!
//! Single-threaded UI; process-static state is intentional for this disposable spike.

#![allow(static_mut_refs)]

use crate::layout::{
    classify_widget_hit, load_layout, save_layout, Approach, SpikeLayout, WidgetHit, WidgetLayout,
};
use crate::paint::{calendar_body_lines, paint_widget, timer_body_lines};
use crate::win32_util::{
    create_layered_window, destroy, hotkeys, move_resize, paint_double_buffered, register_class,
    register_spike_hotkeys, send_toward_desktop, set_click_through, set_window_alpha,
    show_no_activate, unregister_spike_hotkeys, CLASS_A,
};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicU8, Ordering};
use std::time::Instant;
use windows::core::w;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, POINT, RECT, WPARAM};
use windows::Win32::Graphics::Gdi::InvalidateRect;
use windows::Win32::UI::Input::KeyboardAndMouse::{ReleaseCapture, SetCapture};
use windows::Win32::UI::WindowsAndMessaging::{
    DefWindowProcW, DispatchMessageW, GetCursorPos, GetMessageW, GetWindowRect, TranslateMessage,
    HTCLIENT, HTTRANSPARENT, MSG, WM_DESTROY, WM_HOTKEY, WM_LBUTTONDOWN, WM_LBUTTONUP,
    WM_MOUSEMOVE, WM_NCHITTEST, WM_PAINT, WM_QUIT, WM_TIMER,
};

const TIMER_ID: usize = 1;

struct WidgetWindow {
    hwnd: HWND,
    layout: WidgetLayout,
}

struct AppState {
    layout: SpikeLayout,
    started: Instant,
    drag: Option<DragState>,
}

#[derive(Clone, Copy)]
struct DragState {
    index: usize,
    kind: WidgetHit,
    start_cursor: POINT,
    start_rect: RECT,
}

static mut STATE: Option<AppState> = None;
static mut WINDOWS: Vec<WidgetWindow> = Vec::new();
static RUNNING: AtomicBool = AtomicBool::new(true);
static OPACITY: AtomicU8 = AtomicU8::new(220);
static EDIT: AtomicBool = AtomicBool::new(false);
static ELAPSED: AtomicU64 = AtomicU64::new(0);

pub fn run() -> windows::core::Result<()> {
    unsafe {
        register_class(CLASS_A, wnd_proc)?;
        let layout = load_layout(Approach::A);
        OPACITY.store(layout.opacity, Ordering::Relaxed);
        EDIT.store(false, Ordering::Relaxed);
        RUNNING.store(true, Ordering::Relaxed);

        STATE = Some(AppState {
            layout: layout.clone(),
            started: Instant::now(),
            drag: None,
        });
        WINDOWS.clear();

        for w in &layout.widgets {
            let title = match w.id.as_str() {
                "timer" => w!("Solpaper Spike Timer"),
                _ => w!("Solpaper Spike Calendar"),
            };
            let hwnd = create_layered_window(
                CLASS_A, title, w.x, w.y, w.width, w.height,
                true, // Normal Mode: whole-window click-through
            )?;
            set_window_alpha(hwnd, layout.opacity);
            show_no_activate(hwnd);
            send_toward_desktop(hwnd);
            WINDOWS.push(WidgetWindow {
                hwnd,
                layout: w.clone(),
            });
        }

        let hotkey_hwnd = WINDOWS
            .first()
            .map(|w| w.hwnd)
            .ok_or_else(windows::core::Error::from_win32)?;
        if let Err(e) = register_spike_hotkeys(hotkey_hwnd) {
            eprintln!("warning: hotkey registration failed ({e}); use console close to exit");
        }

        if let Some(first) = WINDOWS.first() {
            let id =
                windows::Win32::UI::WindowsAndMessaging::SetTimer(first.hwnd, TIMER_ID, 1000, None);
            if id == 0 {
                eprintln!("warning: SetTimer failed");
            }
        }

        println!("Approach A: independent widget HWNDs");
        println!("  Ctrl+Alt+F2 Edit/Normal | Ctrl+Alt++/- opacity | Ctrl+Alt+S save | Ctrl+Alt+Esc quit");
        println!("  layout: {:?}", crate::layout::layout_path(Approach::A));

        let mut msg = MSG::default();
        while RUNNING.load(Ordering::Relaxed) {
            let ok = GetMessageW(&mut msg, None, 0, 0);
            if !ok.as_bool() || msg.message == WM_QUIT {
                break;
            }
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        unregister_spike_hotkeys(hotkey_hwnd);
        if let Some(state) = STATE.as_mut() {
            sync_layout_from_windows(state);
            state.layout.opacity = OPACITY.load(Ordering::Relaxed);
            let _ = save_layout(&state.layout);
        }
        for w in WINDOWS.drain(..) {
            destroy(w.hwnd);
        }
        STATE = None;
    }
    Ok(())
}

unsafe fn sync_layout_from_windows(state: &mut AppState) {
    for (i, win) in WINDOWS.iter().enumerate() {
        let mut rc = RECT::default();
        let _ = GetWindowRect(win.hwnd, &mut rc);
        if let Some(slot) = state.layout.widgets.get_mut(i) {
            slot.x = rc.left;
            slot.y = rc.top;
            slot.width = rc.right - rc.left;
            slot.height = rc.bottom - rc.top;
        }
    }
}

unsafe fn widget_index_for(hwnd: HWND) -> Option<usize> {
    WINDOWS.iter().position(|w| w.hwnd == hwnd)
}

unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_PAINT => {
            let edit = EDIT.load(Ordering::Relaxed);
            let elapsed = ELAPSED.load(Ordering::Relaxed);
            let idx = widget_index_for(hwnd);
            paint_double_buffered(hwnd, |hdc, width, height| {
                let Some(i) = idx else { return };
                let id = WINDOWS
                    .get(i)
                    .map(|w| w.layout.id.clone())
                    .unwrap_or_default();
                let local = WidgetLayout {
                    id: id.clone(),
                    x: 0,
                    y: 0,
                    width,
                    height,
                };
                if id == "timer" {
                    let lines = timer_body_lines(elapsed);
                    let refs: Vec<&str> = lines.iter().map(|s| s.as_str()).collect();
                    paint_widget(hdc, &local, "Timer", &refs, edit);
                } else {
                    paint_widget(hdc, &local, "Calendar", &calendar_body_lines(), edit);
                }
            });
            LRESULT(0)
        }
        WM_TIMER => {
            if let Some(state) = STATE.as_ref() {
                ELAPSED.store(state.started.elapsed().as_secs(), Ordering::Relaxed);
            }
            for w in &WINDOWS {
                let _ = InvalidateRect(w.hwnd, None, false);
            }
            LRESULT(0)
        }
        WM_HOTKEY => {
            handle_hotkey(wparam.0 as i32);
            LRESULT(0)
        }
        WM_NCHITTEST => {
            if !EDIT.load(Ordering::Relaxed) {
                return LRESULT(HTTRANSPARENT as isize);
            }
            LRESULT(HTCLIENT as isize)
        }
        WM_LBUTTONDOWN => {
            if !EDIT.load(Ordering::Relaxed) {
                return LRESULT(0);
            }
            let Some(i) = widget_index_for(hwnd) else {
                return LRESULT(0);
            };
            let mut cursor = POINT::default();
            let _ = GetCursorPos(&mut cursor);
            let mut rc = RECT::default();
            let _ = GetWindowRect(hwnd, &mut rc);
            let local_x = cursor.x - rc.left;
            let local_y = cursor.y - rc.top;
            let temp = WidgetLayout {
                id: String::new(),
                x: 0,
                y: 0,
                width: rc.right - rc.left,
                height: rc.bottom - rc.top,
            };
            let hit = classify_widget_hit(&temp, local_x, local_y).unwrap_or(WidgetHit::Body);
            if matches!(hit, WidgetHit::Drag | WidgetHit::Resize) {
                if let Some(state) = STATE.as_mut() {
                    state.drag = Some(DragState {
                        index: i,
                        kind: hit,
                        start_cursor: cursor,
                        start_rect: rc,
                    });
                }
                let _ = SetCapture(hwnd);
            }
            LRESULT(0)
        }
        WM_MOUSEMOVE => {
            if let Some(state) = STATE.as_mut() {
                if let Some(drag) = state.drag {
                    let mut cursor = POINT::default();
                    let _ = GetCursorPos(&mut cursor);
                    let dx = cursor.x - drag.start_cursor.x;
                    let dy = cursor.y - drag.start_cursor.y;
                    let r = drag.start_rect;
                    match drag.kind {
                        WidgetHit::Drag => {
                            move_resize(
                                WINDOWS[drag.index].hwnd,
                                r.left + dx,
                                r.top + dy,
                                r.right - r.left,
                                r.bottom - r.top,
                            );
                        }
                        WidgetHit::Resize => {
                            let nw = (r.right - r.left + dx).max(120);
                            let nh = (r.bottom - r.top + dy).max(80);
                            move_resize(WINDOWS[drag.index].hwnd, r.left, r.top, nw, nh);
                        }
                        WidgetHit::Body => {}
                    }
                }
            }
            LRESULT(0)
        }
        WM_LBUTTONUP => {
            if let Some(state) = STATE.as_mut() {
                state.drag = None;
            }
            let _ = ReleaseCapture();
            LRESULT(0)
        }
        WM_DESTROY => LRESULT(0),
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

unsafe fn handle_hotkey(id: i32) {
    match id {
        hotkeys::QUIT => {
            RUNNING.store(false, Ordering::Relaxed);
            windows::Win32::UI::WindowsAndMessaging::PostQuitMessage(0);
        }
        hotkeys::TOGGLE_EDIT => {
            let next = !EDIT.load(Ordering::Relaxed);
            EDIT.store(next, Ordering::Relaxed);
            for w in &WINDOWS {
                set_click_through(w.hwnd, !next);
                let _ = InvalidateRect(w.hwnd, None, false);
            }
            println!(
                "Mode: {}",
                if next {
                    "EDIT"
                } else {
                    "NORMAL (click-through)"
                }
            );
        }
        hotkeys::OPACITY_UP => {
            let o = OPACITY.load(Ordering::Relaxed).saturating_add(10);
            OPACITY.store(o, Ordering::Relaxed);
            for w in &WINDOWS {
                set_window_alpha(w.hwnd, o);
            }
            println!("Opacity: {o}");
        }
        hotkeys::OPACITY_DOWN => {
            let o = OPACITY.load(Ordering::Relaxed).saturating_sub(10).max(40);
            OPACITY.store(o, Ordering::Relaxed);
            for w in &WINDOWS {
                set_window_alpha(w.hwnd, o);
            }
            println!("Opacity: {o}");
        }
        hotkeys::SAVE => {
            if let Some(state) = STATE.as_mut() {
                sync_layout_from_windows(state);
                state.layout.opacity = OPACITY.load(Ordering::Relaxed);
                match save_layout(&state.layout) {
                    Ok(()) => {
                        println!(
                            "Saved layout to {:?}",
                            crate::layout::layout_path(Approach::A)
                        )
                    }
                    Err(e) => eprintln!("Save failed: {e}"),
                }
            }
        }
        _ => {}
    }
}
