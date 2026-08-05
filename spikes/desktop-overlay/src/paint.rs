//! Simple GDI placeholder painting for timer/calendar cards.
//!
//! Intentionally crude — this spike proves window topology, not visual design.

use crate::layout::WidgetLayout;
use windows::Win32::Foundation::{COLORREF, RECT};
use windows::Win32::Graphics::Gdi::{
    CreateSolidBrush, DeleteObject, FillRect, SelectObject, SetBkMode, SetTextColor, TextOutW, HDC,
    HGDIOBJ, TRANSPARENT,
};

pub fn paint_widget(hdc: HDC, widget: &WidgetLayout, title: &str, body_lines: &[&str], edit: bool) {
    let rect = RECT {
        left: widget.x,
        top: widget.y,
        right: widget.x + widget.width,
        bottom: widget.y + widget.height,
    };

    // Card background
    let bg = if edit {
        unsafe { CreateSolidBrush(COLORREF(0x00_2A_3A_4A)) }
    } else {
        unsafe { CreateSolidBrush(COLORREF(0x00_1E_1E_28)) }
    };
    unsafe {
        let _ = FillRect(hdc, &rect, bg);
        let _ = DeleteObject(HGDIOBJ(bg.0));
    }

    // Title bar strip
    let title_rect = RECT {
        left: rect.left,
        top: rect.top,
        right: rect.right,
        bottom: rect.top + 28,
    };
    let title_bg = unsafe { CreateSolidBrush(COLORREF(0x00_3D_5A_80)) };
    unsafe {
        let _ = FillRect(hdc, &title_rect, title_bg);
        let _ = DeleteObject(HGDIOBJ(title_bg.0));
    }

    // Border
    let border = unsafe { CreateSolidBrush(COLORREF(0x00_88_AA_CC)) };
    unsafe {
        // Top
        let r = RECT {
            left: rect.left,
            top: rect.top,
            right: rect.right,
            bottom: rect.top + 1,
        };
        let _ = FillRect(hdc, &r, border);
        // Bottom
        let r = RECT {
            left: rect.left,
            top: rect.bottom - 1,
            right: rect.right,
            bottom: rect.bottom,
        };
        let _ = FillRect(hdc, &r, border);
        // Left
        let r = RECT {
            left: rect.left,
            top: rect.top,
            right: rect.left + 1,
            bottom: rect.bottom,
        };
        let _ = FillRect(hdc, &r, border);
        // Right
        let r = RECT {
            left: rect.right - 1,
            top: rect.top,
            right: rect.right,
            bottom: rect.bottom,
        };
        let _ = FillRect(hdc, &r, border);
        let _ = DeleteObject(HGDIOBJ(border.0));
    }

    // Resize grip indicator in edit mode
    if edit {
        let grip = RECT {
            left: rect.right - 14,
            top: rect.bottom - 14,
            right: rect.right - 2,
            bottom: rect.bottom - 2,
        };
        let gbrush = unsafe { CreateSolidBrush(COLORREF(0x00_CC_DD_EE)) };
        unsafe {
            let _ = FillRect(hdc, &grip, gbrush);
            let _ = DeleteObject(HGDIOBJ(gbrush.0));
        }
    }

    unsafe {
        let _ = SetBkMode(hdc, TRANSPARENT);
        let _ = SetTextColor(hdc, COLORREF(0x00_F0_F0_F0));
        let _ = SelectObject(
            hdc,
            HGDIOBJ(
                windows::Win32::Graphics::Gdi::GetStockObject(
                    windows::Win32::Graphics::Gdi::DEFAULT_GUI_FONT,
                )
                .0,
            ),
        );
    }

    draw_text(hdc, rect.left + 8, rect.top + 6, title);
    let mut y = rect.top + 40;
    for line in body_lines {
        draw_text(hdc, rect.left + 12, y, line);
        y += 22;
    }

    if edit {
        draw_text(hdc, rect.left + 12, rect.bottom - 24, "[EDIT]");
    }
}

fn draw_text(hdc: HDC, x: i32, y: i32, text: &str) {
    let wide: Vec<u16> = text.encode_utf16().collect();
    unsafe {
        let _ = TextOutW(hdc, x, y, &wide);
    }
}

/// System uptime-based fake "timer" text so the 1 Hz refresh is visible.
pub fn timer_body_lines(elapsed_secs: u64) -> [String; 3] {
    let m = elapsed_secs / 60;
    let s = elapsed_secs % 60;
    [
        format!("Focus  {m:02}:{s:02}"),
        "placeholder Pomodoro card".into(),
        "updates @ 1 Hz".into(),
    ]
}

pub fn calendar_body_lines() -> [&'static str; 4] {
    [
        "09:00  Standup",
        "11:30  Deep work",
        "14:00  Private",
        "16:00  Review",
    ]
}
