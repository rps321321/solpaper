//! Solpaper Issue #18 — disposable desktop overlay feasibility spike.
//!
//! Compares Approach A (per-widget HWND) vs Approach B (monitor surface).
//! Not production code. See README.md and docs/research/overlay-feasibility.md.

mod approach_a;
mod approach_b;
mod layout;
mod paint;
mod win32_util;

use layout::Approach;

fn print_usage() {
    eprintln!(
        "desktop-overlay-spike — Issue #18 disposable prototype\n\n\
         Usage:\n\
           cargo run --release -- --approach a\n\
           cargo run --release -- --approach b\n\n\
         Hotkeys: Ctrl+Alt+F2 edit | Ctrl+Alt++/- opacity | Ctrl+Alt+S save | Ctrl+Alt+Esc quit\n"
    );
}

fn main() {
    // Per-monitor DPI awareness so mixed-DPI movement is measurable.
    unsafe {
        let _ = windows::Win32::UI::HiDpi::SetProcessDpiAwarenessContext(
            windows::Win32::UI::HiDpi::DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2,
        );
    }

    let mut approach: Option<Approach> = None;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--approach" | "-a" => {
                let value = args.next().unwrap_or_default();
                approach = Approach::parse(&value);
                if approach.is_none() {
                    eprintln!("Unknown approach '{value}'. Use 'a' or 'b'.");
                    print_usage();
                    std::process::exit(2);
                }
            }
            "--help" | "-h" => {
                print_usage();
                return;
            }
            other => {
                eprintln!("Unknown argument: {other}");
                print_usage();
                std::process::exit(2);
            }
        }
    }

    let Some(approach) = approach else {
        print_usage();
        std::process::exit(2);
    };

    let result = match approach {
        Approach::A => approach_a::run(),
        Approach::B => approach_b::run(),
    };

    if let Err(err) = result {
        eprintln!("Spike failed: {err}");
        std::process::exit(1);
    }
}
