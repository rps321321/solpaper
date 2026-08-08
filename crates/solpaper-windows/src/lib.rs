//! Win32 adapters for Solpaper (ADR-0001, ADR-0002, ADR-0003).
//!
//! All `unsafe` Win32 boundaries are encapsulated here.

mod dpi;
mod placeholder;
mod single_instance;
mod wallpaper;

pub use dpi::set_process_dpi_awareness;
pub use placeholder::{run_placeholder_host, PlaceholderConfig};
pub use single_instance::{SingleInstanceError, SingleInstanceGuard};
pub use wallpaper::{ComDesktopWallpaper, DesktopWallpaper, FakeDesktopWallpaper, WallpaperError};
