//! Platform-neutral domain types for Solpaper.
//!
//! No Win32. Unit-testable on any host that can compile the crate.

mod diagnostics;
mod layout;
mod pomodoro;
mod tray;
mod wallpaper;

pub use diagnostics::{
    categorize_error_code, count_crashes_in_window, is_allowed_log_field,
    is_forbidden_bundle_entry_name, is_forbidden_log_field, log_files_to_delete,
    needs_rotation_before_write, redact_user_path, should_recommend_safe_mode, validate_log_fields,
    Component, CorrelationScope, ErrorCategory, LogFileMeta, SafeModePolicy, ALLOWED_LOG_FIELDS,
    AUTO_RESTART_ON_CRASH, BUNDLE_LOG_TAIL_MAX_BYTES, CRASH_LOOP_THRESHOLD, CRASH_LOOP_WINDOW_MS,
    FORBIDDEN_LOG_FIELDS, LOG_FILE_MAX_BYTES, LOG_FILE_MAX_COUNT, LOG_RETENTION_DAYS,
    LOG_TOTAL_MAX_BYTES, REMOTE_CRASH_UPLOAD, TELEMETRY_ENABLED, WATCHDOG_PROCESS,
};
pub use layout::{
    Anchor, DipPoint, DipRect, DipSize, MonitorMatch, WidgetId, WidgetLayoutEntry, WidgetLayoutSet,
};
pub use pomodoro::{
    AvailableActions, Command as PomodoroCommand, DurationMs, Phase, PomodoroConfig, PomodoroEvent,
    PomodoroState, PomodoroView, TimerStatus, UnixMs, DEFAULT_FOCUSES_BEFORE_LONG_BREAK,
    DEFAULT_FOCUS_MS, DEFAULT_LONG_BREAK_MS, DEFAULT_SHORT_BREAK_MS,
};
pub use tray::{
    alpha1_scaffold_flags, autostart_command_line, build_tray_menu, command_enabled,
    pomodoro_status_label, portable_allows_autostart_ui, NotificationDeduper, PhaseInstanceId,
    SecondLaunchAction, ShutdownStep, TrayCommand, TrayFeatureFlags, TrayMenuEntry,
    AUTOSTART_BACKGROUND_FLAG, AUTOSTART_VALUE_NAME, CONTROL_WINDOW_CLASS, SECOND_LAUNCH_ACTION,
    SHUTDOWN_SEQUENCE, SHUTDOWN_WORKER_WAIT_MS,
};
pub use wallpaper::{
    check_decoded_pixels, check_local_file_size, fill_decision, fill_scale_factors,
    is_accepted_extension, require_monitor_id, validate_source_path_shape, FillDecision, FitPolicy,
    ImageRequest, MonitorFingerprint, Orientation, WallpaperErrorKind, WallpaperMonitor,
    WallpaperMonitorId, WallpaperPinSet, WallpaperPosition, ACCEPTED_EXTENSIONS,
    DECODED_MAX_MEGAPIXELS, LOCAL_WALLPAPER_MAX_BYTES, MAX_UPSCALE_FACTOR,
};

/// Crate-level error type for pure domain operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoreError {
    /// Layout entry failed validation (non-positive size, empty id, etc.).
    InvalidLayout(&'static str),
    /// Pomodoro configuration failed validation.
    InvalidPomodoro(&'static str),
    /// Command not legal in the current Pomodoro status.
    IllegalPomodoroTransition(&'static str),
    /// Diagnostics / logging field policy violation.
    InvalidDiagnostics(&'static str),
    /// Wallpaper domain validation failure.
    InvalidWallpaper(&'static str),
}

impl std::fmt::Display for CoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreError::InvalidLayout(msg) => write!(f, "invalid layout: {msg}"),
            CoreError::InvalidPomodoro(msg) => write!(f, "invalid pomodoro: {msg}"),
            CoreError::IllegalPomodoroTransition(msg) => {
                write!(f, "illegal pomodoro transition: {msg}")
            }
            CoreError::InvalidDiagnostics(msg) => write!(f, "invalid diagnostics: {msg}"),
            CoreError::InvalidWallpaper(msg) => write!(f, "invalid wallpaper: {msg}"),
        }
    }
}

impl std::error::Error for CoreError {}
