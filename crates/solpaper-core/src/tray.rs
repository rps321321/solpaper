//! Tray menu model, command routing, notification dedupe, shutdown policy (Issue #7).
//!
//! Platform tray / registry / HWND code lives in `solpaper-windows`. This module is
//! pure and unit-tested on any host.

use crate::pomodoro::{AvailableActions, TimerStatus};

/// Fixed menu command IDs in blueprint #7 display order (excluding separators).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrayCommand {
    OpenSettings,
    ToggleEditMode,
    PomodoroStartPauseResume,
    PomodoroSkip,
    PomodoroReset,
    WallpaperNext,
    WallpaperHold,
    ToggleAutostart,
    OpenDiagnostics,
    Quit,
}

impl TrayCommand {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OpenSettings => "open_settings",
            Self::ToggleEditMode => "toggle_edit_mode",
            Self::PomodoroStartPauseResume => "pomodoro_start_pause_resume",
            Self::PomodoroSkip => "pomodoro_skip",
            Self::PomodoroReset => "pomodoro_reset",
            Self::WallpaperNext => "wallpaper_next",
            Self::WallpaperHold => "wallpaper_hold",
            Self::ToggleAutostart => "toggle_autostart",
            Self::OpenDiagnostics => "open_diagnostics",
            Self::Quit => "quit",
        }
    }
}

/// One row in the tray context menu (blueprint fixed order).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrayMenuEntry {
    Command {
        command: TrayCommand,
        /// When false, show disabled (not hidden).
        enabled: bool,
        label: &'static str,
    },
    Separator,
}

/// Feature flags that gate tray actions (unavailable = disabled, not hidden).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrayFeatureFlags {
    pub settings: bool,
    pub edit_mode: bool,
    pub pomodoro: bool,
    pub wallpaper: bool,
    /// Installed build only; portable builds keep false.
    pub autostart_ui: bool,
    pub diagnostics: bool,
}

impl Default for TrayFeatureFlags {
    fn default() -> Self {
        Self {
            settings: true,
            edit_mode: true,
            pomodoro: true,
            wallpaper: true,
            autostart_ui: false,
            diagnostics: true,
        }
    }
}

/// Alpha 1 scaffold defaults: settings/diagnostics/quit on; feature surfaces may lag.
pub fn alpha1_scaffold_flags() -> TrayFeatureFlags {
    TrayFeatureFlags {
        settings: true,
        edit_mode: false,
        pomodoro: false,
        wallpaper: false,
        autostart_ui: false,
        diagnostics: true,
    }
}

/// Build the fixed-order tray menu with enablement from flags + Pomodoro legality.
pub fn build_tray_menu(
    flags: TrayFeatureFlags,
    pomodoro: Option<AvailableActions>,
) -> Vec<TrayMenuEntry> {
    let pomo = pomodoro.unwrap_or(AvailableActions {
        start: false,
        pause: false,
        resume: false,
        skip: false,
        reset: false,
    });
    let pomo_spr = flags.pomodoro && (pomo.start || pomo.pause || pomo.resume);
    let spr_label = if pomo.pause {
        "Pause focus"
    } else if pomo.resume {
        "Resume"
    } else {
        "Start focus"
    };

    vec![
        TrayMenuEntry::Command {
            command: TrayCommand::OpenSettings,
            enabled: flags.settings,
            label: "Open Settings",
        },
        TrayMenuEntry::Command {
            command: TrayCommand::ToggleEditMode,
            enabled: flags.edit_mode,
            label: "Edit Mode",
        },
        TrayMenuEntry::Separator,
        TrayMenuEntry::Command {
            command: TrayCommand::PomodoroStartPauseResume,
            enabled: pomo_spr,
            label: spr_label,
        },
        TrayMenuEntry::Command {
            command: TrayCommand::PomodoroSkip,
            enabled: flags.pomodoro && pomo.skip,
            label: "Skip phase",
        },
        TrayMenuEntry::Command {
            command: TrayCommand::PomodoroReset,
            enabled: flags.pomodoro && pomo.reset,
            label: "Reset",
        },
        TrayMenuEntry::Separator,
        TrayMenuEntry::Command {
            command: TrayCommand::WallpaperNext,
            enabled: flags.wallpaper,
            label: "Wallpaper Next",
        },
        TrayMenuEntry::Command {
            command: TrayCommand::WallpaperHold,
            enabled: flags.wallpaper,
            label: "Wallpaper Hold",
        },
        TrayMenuEntry::Separator,
        TrayMenuEntry::Command {
            command: TrayCommand::ToggleAutostart,
            enabled: flags.autostart_ui,
            label: "Start with Windows",
        },
        TrayMenuEntry::Command {
            command: TrayCommand::OpenDiagnostics,
            enabled: flags.diagnostics,
            label: "Diagnostics",
        },
        TrayMenuEntry::Command {
            command: TrayCommand::Quit,
            enabled: true,
            label: "Quit",
        },
    ]
}

/// Whether a command is enabled in a built menu (for routing guards).
pub fn command_enabled(menu: &[TrayMenuEntry], cmd: TrayCommand) -> bool {
    menu.iter().any(|e| match e {
        TrayMenuEntry::Command {
            command, enabled, ..
        } => *command == cmd && *enabled,
        TrayMenuEntry::Separator => false,
    })
}

// --- Notification dedupe (blueprint #7 / #19) --------------------------------

/// Opaque Pomodoro phase instance id used to dedupe tray balloons.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PhaseInstanceId(pub String);

impl PhaseInstanceId {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

/// At most one balloon per phase instance id (PERF-REL-03 / pack #7).
#[derive(Debug, Default, Clone)]
pub struct NotificationDeduper {
    last: Option<PhaseInstanceId>,
}

impl NotificationDeduper {
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns true if this instance id has not yet been notified (and records it).
    pub fn try_notify(&mut self, id: &PhaseInstanceId) -> bool {
        if self.last.as_ref() == Some(id) {
            return false;
        }
        self.last = Some(id.clone());
        true
    }

    pub fn reset(&mut self) {
        self.last = None;
    }
}

// --- Shutdown contract (pack #7; NFR shutdown ≤ 2 s) -------------------------

/// Maximum wait for worker stop during graceful shutdown (NFR PERF-START / pack #7).
pub const SHUTDOWN_WORKER_WAIT_MS: u64 = 2_000;

/// Ordered graceful-shutdown steps (host executes; policy is pure).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShutdownStep {
    StopAcceptingWork,
    StopTimers,
    FlushSettingsAndRuntime,
    StopWorker,
    RemoveTrayIcon,
    DestroyWindows,
    ReleaseMutex,
}

/// Canonical shutdown sequence.
pub const SHUTDOWN_SEQUENCE: &[ShutdownStep] = &[
    ShutdownStep::StopAcceptingWork,
    ShutdownStep::StopTimers,
    ShutdownStep::FlushSettingsAndRuntime,
    ShutdownStep::StopWorker,
    ShutdownStep::RemoveTrayIcon,
    ShutdownStep::DestroyWindows,
    ShutdownStep::ReleaseMutex,
];

/// Second-launch policy when the named mutex is already held.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecondLaunchAction {
    /// Post narrow activation to the existing control window, then exit 0.
    ActivateShowSettings,
}

pub const SECOND_LAUNCH_ACTION: SecondLaunchAction = SecondLaunchAction::ActivateShowSettings;

/// Control window class for narrow activation (not a general IPC protocol).
pub const CONTROL_WINDOW_CLASS: &str = "Solpaper.Runtime.Control.v1";

/// Autostart Run-key value name (pack #7).
pub const AUTOSTART_VALUE_NAME: &str = "Solpaper";

/// CLI flag required on autostart command line (pack #7).
pub const AUTOSTART_BACKGROUND_FLAG: &str = "--background";

/// Build the HKCU Run value: quoted absolute exe + background flag.
pub fn autostart_command_line(installed_exe: &str) -> String {
    format!("\"{installed_exe}\" {AUTOSTART_BACKGROUND_FLAG}")
}

/// Whether portable builds may expose autostart UI (always false per pack).
pub fn portable_allows_autostart_ui() -> bool {
    false
}

/// Map Pomodoro timer status into a coarse tray label fragment (no private data).
pub fn pomodoro_status_label(status: &TimerStatus) -> &'static str {
    match status {
        TimerStatus::Idle => "idle",
        TimerStatus::Running { .. } => "running",
        TimerStatus::Paused { .. } => "paused",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn menu_order_matches_blueprint() {
        let menu = build_tray_menu(TrayFeatureFlags::default(), None);
        let cmds: Vec<_> = menu
            .iter()
            .filter_map(|e| match e {
                TrayMenuEntry::Command { command, .. } => Some(*command),
                TrayMenuEntry::Separator => None,
            })
            .collect();
        assert_eq!(
            cmds,
            vec![
                TrayCommand::OpenSettings,
                TrayCommand::ToggleEditMode,
                TrayCommand::PomodoroStartPauseResume,
                TrayCommand::PomodoroSkip,
                TrayCommand::PomodoroReset,
                TrayCommand::WallpaperNext,
                TrayCommand::WallpaperHold,
                TrayCommand::ToggleAutostart,
                TrayCommand::OpenDiagnostics,
                TrayCommand::Quit,
            ]
        );
        // Separators positions: after edit, after reset, after wallpaper hold.
        assert!(matches!(menu[2], TrayMenuEntry::Separator));
        assert!(matches!(menu[6], TrayMenuEntry::Separator));
        assert!(matches!(menu[9], TrayMenuEntry::Separator));
    }

    #[test]
    fn unavailable_features_disabled_not_omitted() {
        let flags = alpha1_scaffold_flags();
        let menu = build_tray_menu(flags, None);
        assert!(command_enabled(&menu, TrayCommand::OpenSettings));
        assert!(command_enabled(&menu, TrayCommand::Quit));
        assert!(!command_enabled(&menu, TrayCommand::ToggleEditMode));
        assert!(!command_enabled(&menu, TrayCommand::WallpaperNext));
        assert!(!command_enabled(&menu, TrayCommand::ToggleAutostart));
        // Still present:
        assert_eq!(
            menu.iter()
                .filter(|e| matches!(e, TrayMenuEntry::Command { .. }))
                .count(),
            10
        );
    }

    #[test]
    fn pomodoro_actions_gate_menu() {
        let flags = TrayFeatureFlags {
            pomodoro: true,
            ..TrayFeatureFlags::default()
        };
        let actions = AvailableActions {
            start: true,
            pause: false,
            resume: false,
            skip: false,
            reset: true,
        };
        let menu = build_tray_menu(flags, Some(actions));
        assert!(command_enabled(
            &menu,
            TrayCommand::PomodoroStartPauseResume
        ));
        assert!(!command_enabled(&menu, TrayCommand::PomodoroSkip));
        assert!(command_enabled(&menu, TrayCommand::PomodoroReset));
    }

    #[test]
    fn notification_dedupe_one_per_instance() {
        let mut d = NotificationDeduper::new();
        let a = PhaseInstanceId::new("focus-1");
        let b = PhaseInstanceId::new("break-1");
        assert!(d.try_notify(&a));
        assert!(!d.try_notify(&a));
        assert!(d.try_notify(&b));
        assert!(!d.try_notify(&b));
        d.reset();
        assert!(d.try_notify(&a));
    }

    #[test]
    fn shutdown_sequence_order() {
        assert_eq!(SHUTDOWN_SEQUENCE[0], ShutdownStep::StopAcceptingWork);
        assert_eq!(
            *SHUTDOWN_SEQUENCE.last().unwrap(),
            ShutdownStep::ReleaseMutex
        );
        assert_eq!(SHUTDOWN_WORKER_WAIT_MS, 2_000);
    }

    #[test]
    fn autostart_command_line_quoted() {
        let line = autostart_command_line(r"C:\Program Files\Solpaper\solpaper.exe");
        assert_eq!(
            line,
            r#""C:\Program Files\Solpaper\solpaper.exe" --background"#
        );
        assert!(!portable_allows_autostart_ui());
        assert_eq!(CONTROL_WINDOW_CLASS, "Solpaper.Runtime.Control.v1");
        assert_eq!(
            SECOND_LAUNCH_ACTION,
            SecondLaunchAction::ActivateShowSettings
        );
    }
}
