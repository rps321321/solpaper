//! Win32 adapters for Solpaper (ADR-0001, ADR-0002, ADR-0003).
//!
//! All `unsafe` Win32 boundaries are encapsulated here.

mod activation;
mod autostart;
mod dpi;
mod placeholder;
mod runtime;
mod single_instance;
mod wallpaper;
mod widget_host;

pub use activation::{
    activate_existing_show_settings, post_show_settings, second_launch_outcome, ActivationError,
    SecondLaunchOutcome, WM_APP_SHOW_SETTINGS,
};
pub use autostart::{AutostartError, AutostartStore, FakeAutostartStore, WindowsRunKeyAutostart};
pub use dpi::set_process_dpi_awareness;
pub use placeholder::{
    create_placeholder_window, destroy_placeholder_window, run_placeholder_host, PlaceholderConfig,
};
pub use runtime::{run_runtime_host, take_settings_requested, RuntimeError, RuntimeHostConfig};
pub use single_instance::{SingleInstanceError, SingleInstanceGuard};
pub use wallpaper::{ComDesktopWallpaper, DesktopWallpaper, FakeDesktopWallpaper, WallpaperError};
pub use widget_host::{
    create_widget_host, destroy_all_widgets, set_surface_mode, snapshot_widget_rects, surface_mode,
    toggle_surface_mode, WidgetSurfaceConfig, WIDGET_WINDOW_CLASS,
};
