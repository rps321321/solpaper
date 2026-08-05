//! Platform-neutral domain types for Solpaper.
//!
//! No Win32. Unit-testable on any host that can compile the crate.

mod layout;
mod pomodoro;

pub use layout::{
    Anchor, DipPoint, DipRect, DipSize, MonitorMatch, WidgetId, WidgetLayoutEntry, WidgetLayoutSet,
};
pub use pomodoro::{
    AvailableActions, Command as PomodoroCommand, DurationMs, Phase, PomodoroConfig, PomodoroEvent,
    PomodoroState, PomodoroView, TimerStatus, UnixMs, DEFAULT_FOCUSES_BEFORE_LONG_BREAK,
    DEFAULT_FOCUS_MS, DEFAULT_LONG_BREAK_MS, DEFAULT_SHORT_BREAK_MS,
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
}

impl std::fmt::Display for CoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreError::InvalidLayout(msg) => write!(f, "invalid layout: {msg}"),
            CoreError::InvalidPomodoro(msg) => write!(f, "invalid pomodoro: {msg}"),
            CoreError::IllegalPomodoroTransition(msg) => {
                write!(f, "illegal pomodoro transition: {msg}")
            }
        }
    }
}

impl std::error::Error for CoreError {}
