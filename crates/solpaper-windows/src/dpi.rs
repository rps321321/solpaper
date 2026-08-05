//! Per-monitor DPI awareness setup.

use windows::core::HRESULT;
use windows::Win32::UI::HiDpi::{SetProcessDpiAwarenessContext, DPI_AWARENESS_CONTEXT};

/// Best-effort Per-Monitor V2. Failures are ignored so older hosts still run.
pub fn set_process_dpi_awareness() {
    // DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2 = -4 as HANDLE-like value
    const PER_MONITOR_V2: DPI_AWARENESS_CONTEXT = DPI_AWARENESS_CONTEXT(-4isize as _);
    unsafe {
        let _ = SetProcessDpiAwarenessContext(PER_MONITOR_V2);
    }
    let _ = HRESULT(0);
}
