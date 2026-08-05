//! Platform-neutral domain types for Solpaper.
//!
//! No Win32. Unit-testable on any host that can compile the crate.

mod layout;

pub use layout::{
    Anchor, DipPoint, DipRect, DipSize, MonitorMatch, WidgetId, WidgetLayoutEntry, WidgetLayoutSet,
};

/// Crate-level error type for pure domain operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoreError {
    /// Layout entry failed validation (non-positive size, empty id, etc.).
    InvalidLayout(&'static str),
}

impl std::fmt::Display for CoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreError::InvalidLayout(msg) => write!(f, "invalid layout: {msg}"),
        }
    }
}

impl std::error::Error for CoreError {}
