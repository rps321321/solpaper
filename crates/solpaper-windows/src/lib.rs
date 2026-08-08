//! Win32 adapters for Solpaper (ADR-0001, ADR-0002, ADR-0003).
//!
//! All `unsafe` Win32 boundaries are encapsulated here.

mod activation;
mod autostart;
mod dpi;
mod placeholder;
mod single_instance;
mod wallpaper;

pub use activation::{
    activate_existing_show_settings, post_show_settings, second_launch_outcome, ActivationError,
    SecondLaunchOutcome, WM_APP_SHOW_SETTINGS,
};
pub use autostart::{AutostartError, AutostartStore, FakeAutostartStore, WindowsRunKeyAutostart};
pub use dpi::set_process_dpi_awareness;
pub use placeholder::{run_placeholder_host, PlaceholderConfig};
pub use single_instance::{SingleInstanceError, SingleInstanceGuard};
pub use wallpaper::{ComDesktopWallpaper, DesktopWallpaper, FakeDesktopWallpaper, WallpaperError};
