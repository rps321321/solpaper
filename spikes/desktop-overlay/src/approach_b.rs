//! Approach B: one monitor-sized transparent surface hosting multiple widgets.
//!
//! Selective input uses `WM_NCHITTEST` → `HTTRANSPARENT` outside widget bounds
//! (and always in Normal Mode). Empty surface regions never steal desktop clicks.
//!
//! Single-threaded UI; process-static state is intentional for this disposable spike.

#![allow(static_mut_refs)]

use crate::layout::{
    classify_widget_hit, hit_test_widgets, load_layout, save_layout, Approach, SpikeLayout,
    WidgetHit, WidgetLayout,
};
use crate::paint::{calendar_body_lines, paint_widget, timer_body_lines};
use crate::win32_util::{
    create_layered_window, destroy, hotkeys, paint_double_buffered, primary_work_area,
    register_class, register_spike_hotkeys, send_toward_desktop, set_click_through,
    set_window_alpha, show_no_activate, unregister_spike_hotkeys, CLASS_B,
};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicU8, Ordering};
use std::time::Instant;
use windows::core::w;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, POINT, WPARAM};
use windows::Win32::Graphics::Gdi::InvalidateRect;
use windows::Win32::UI::Input::KeyboardAndMouse::{ReleaseCapture, SetCapture};
use windows::Win32::UI::WindowsAndMessaging::{
    DefWindowProcW, DispatchMessageW, GetCursorPos, GetMessageW, TranslateMessage, HTCLIENT,
    HTTRANSPARENT, MSG, WM_DESTROY, WM_HOTKEY, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MOUSEMOVE,
    WM_NCHITTEST, WM_PAINT, WM_QUIT, WM_TIMER,
};

const TIMER_ID: usize = 1;

struct DragState {
    index: usize,
    kind: WidgetHit,
    start_cursor: POINT,
    start_widget: WidgetLayout,
}

struct AppState {
    layout: SpikeLayout,
    started: Instant,
    drag: Option<DragState>,
    origin_x: i32,
    origin_y: i32,
}

static mut STATE: Option<AppState> = None;
static RUNNING: AtomicBool = AtomicBool::new(true);
static OPACITY: AtomicU8 = AtomicU8::new(220);
static EDIT: AtomicBool = AtomicBool::new(false);
static ELAPSED: AtomicU64 = AtomicU64::new(0);

pub fn run() -> windows::core::Result<()> {
    unsafe {
        register_class(CLASS_B, wnd_proc)?;
        let layout = load_layout(Approach::B);
        OPACITY.store(layout.opacity, Ordering::Relaxed);
        EDIT.store(false, Ordering::Relaxed);
        RUNNING.store(true, Ordering::Relaxed);

        let work = primary_work_area();
        let width = (work.right - work.left).max(800);
        let height = (work.bottom - work.top).max(600);

        STATE = Some(AppState {
            layout: layout.clone(),
            started: Instant::now(),
            drag: None,
            origin_x: work.left,
            origin_y: work.top,
        });

        let hwnd = create_layered_window(
            CLASS_B,
            w!("Solpaper Spike Monitor Surface"),
            work.left,
            work.top,
            width,
            height,
            false, // selective hit-test via WM_NCHITTEST
        )?;
        set_window_alpha(hwnd, layout.opacity);
        show_no_activate(hwnd);
        send_toward_desktop(hwnd);

        if let Err(e) = register_spike_hotkeys(hwnd) {
            eprintln!("warning: hotkey registration failed ({e}); use console close to exit");
        }

        let timer_id =
            windows::Win32::UI::WindowsAndMessaging::SetTimer(hwnd, TIMER_ID, 1000, None);
        if timer_id == 0 {
            eprintln!("warning: SetTimer failed");
        }

        println!("Approach B: monitor-sized surface HWND");
        println!("  Ctrl+Alt+F2 Edit/Normal | Ctrl+Alt++/- opacity | Ctrl+Alt+S save | Ctrl+Alt+Esc quit");
        println!("  Empty regions are always click-through (HTTRANSPARENT).");
        println!("  layout: {:?}", crate::layout::layout_path(Approach::B));

        let mut msg = MSG::default();
        while RUNNING.load(Ordering::Relaxed) {
            let ok = GetMessageW(&mut msg, None, 0, 0);
            if !ok.as_bool() || msg.message == WM_QUIT {
                break;
            }
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        unregister_spike_hotkeys(hwnd);
        if let Some(state) = STATE.as_mut() {
            state.layout.opacity = OPACITY.load(Ordering::Relaxed);
            let _ = save_layout(&state.layout);
        }
        destroy(hwnd);
        STATE = None;
    }
    Ok(())
}

unsafe fn local_widgets(state: &AppState) -> Vec<WidgetLayout> {
    state
        .layout
        .widgets
        .iter()
        .map(|w| WidgetLayout {
            id: w.id.clone(),
            x: w.x - state.origin_x,
            y: w.y - state.origin_y,
            width: w.width,
            height: w.height,
        })
        .collect()
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
            paint_double_buffered(hwnd, |hdc, _w, _h| {
                let Some(state) = STATE.as_ref() else {
                    return;
                };
                let locals = local_widgets(state);
                for w in &locals {
                    if w.id == "timer" {
                        let lines = timer_body_lines(elapsed);
                        let refs: Vec<&str> = lines.iter().map(|s| s.as_str()).collect();
                        paint_widget(hdc, w, "Timer", &refs, edit);
                    } else {
                        paint_widget(hdc, w, "Calendar", &calendar_body_lines(), edit);
                    }
                }
            });
            LRESULT(0)
        }
        WM_TIMER => {
            if let Some(state) = STATE.as_ref() {
                ELAPSED.store(state.started.elapsed().as_secs(), Ordering::Relaxed);
            }
            let _ = InvalidateRect(hwnd, None, false);
            LRESULT(0)
        }
        WM_HOTKEY => {
            handle_hotkey(hwnd, wparam.0 as i32);
            LRESULT(0)
        }
        WM_NCHITTEST => {
            let x = (lparam.0 as i32) as i16 as i32;
            let y = ((lparam.0 as u32 >> 16) as i16) as i32;

            let Some(state) = STATE.as_ref() else {
                return LRESULT(HTTRANSPARENT as isize);
            };
            let locals = local_widgets(state);
            let lx = x - state.origin_x;
            let ly = y - state.origin_y;

            if !EDIT.load(Ordering::Relaxed) {
                return LRESULT(HTTRANSPARENT as isize);
            }

            match hit_test_widgets(&locals, lx, ly) {
                Some(_) => LRESULT(HTCLIENT as isize),
                None => LRESULT(HTTRANSPARENT as isize),
            }
        }
        WM_LBUTTONDOWN => {
            if !EDIT.load(Ordering::Relaxed) {
                return LRESULT(0);
            }
            let Some(state) = STATE.as_mut() else {
                return LRESULT(0);
            };
            let mut cursor = POINT::default();
            let _ = GetCursorPos(&mut cursor);
            let locals = local_widgets(state);
            let lx = cursor.x - state.origin_x;
            let ly = cursor.y - state.origin_y;
            if let Some(i) = hit_test_widgets(&locals, lx, ly) {
                let hit = classify_widget_hit(&locals[i], lx, ly).unwrap_or(WidgetHit::Body);
                if matches!(hit, WidgetHit::Drag | WidgetHit::Resize) {
                    state.drag = Some(DragState {
                        index: i,
                        kind: hit,
                        start_cursor: cursor,
                        start_widget: state.layout.widgets[i].clone(),
                    });
                    let _ = SetCapture(hwnd);
                }
            }
            LRESULT(0)
        }
        WM_MOUSEMOVE => {
            if let Some(state) = STATE.as_mut() {
                if let Some(drag) = state.drag.as_ref() {
                    let mut cursor = POINT::default();
                    let _ = GetCursorPos(&mut cursor);
                    let dx = cursor.x - drag.start_cursor.x;
                    let dy = cursor.y - drag.start_cursor.y;
                    let idx = drag.index;
                    let kind = drag.kind;
                    let start = drag.start_widget.clone();
                    if let Some(w) = state.layout.widgets.get_mut(idx) {
                        match kind {
                            WidgetHit::Drag => {
                                w.x = start.x + dx;
                                w.y = start.y + dy;
                            }
                            WidgetHit::Resize => {
                                w.width = (start.width + dx).max(120);
                                w.height = (start.height + dy).max(80);
                            }
                            WidgetHit::Body => {}
                        }
                    }
                    let _ = InvalidateRect(hwnd, None, false);
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
        WM_DESTROY => {
            windows::Win32::UI::WindowsAndMessaging::PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

unsafe fn handle_hotkey(hwnd: HWND, id: i32) {
    match id {
        hotkeys::QUIT => {
            RUNNING.store(false, Ordering::Relaxed);
            windows::Win32::UI::WindowsAndMessaging::PostQuitMessage(0);
        }
        hotkeys::TOGGLE_EDIT => {
            let next = !EDIT.load(Ordering::Relaxed);
            EDIT.store(next, Ordering::Relaxed);
            set_click_through(hwnd, false);
            let _ = InvalidateRect(hwnd, None, false);
            println!(
                "Mode: {}",
                if next {
                    "EDIT (widget regions interactive)"
                } else {
                    "NORMAL (full surface click-through)"
                }
            );
        }
        hotkeys::OPACITY_UP => {
            let o = OPACITY.load(Ordering::Relaxed).saturating_add(10);
            OPACITY.store(o, Ordering::Relaxed);
            set_window_alpha(hwnd, o);
            println!("Opacity: {o}");
        }
        hotkeys::OPACITY_DOWN => {
            let o = OPACITY.load(Ordering::Relaxed).saturating_sub(10).max(40);
            OPACITY.store(o, Ordering::Relaxed);
            set_window_alpha(hwnd, o);
            println!("Opacity: {o}");
        }
        hotkeys::SAVE => {
            if let Some(state) = STATE.as_mut() {
                state.layout.opacity = OPACITY.load(Ordering::Relaxed);
                match save_layout(&state.layout) {
                    Ok(()) => println!(
                        "Saved layout to {:?}",
                        crate::layout::layout_path(Approach::B)
                    ),
                    Err(e) => eprintln!("Save failed: {e}"),
                }
            }
        }
        _ => {}
    }
}
