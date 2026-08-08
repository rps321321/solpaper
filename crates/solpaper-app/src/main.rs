//! Solpaper user-session host (ADR-0002 / #7 / #20).
//!
//! Alpha 1 tracer bullet 2: single-instance + control HWND + tray + Approach A
//! widget host with Normal/Edit Mode (tray, Ctrl+Alt+F2, Escape).

use std::env;
use std::process::ExitCode;

use solpaper_core::{
    Anchor, DipPoint, DipSize, MonitorMatch, WidgetId, WidgetLayoutEntry, WidgetLayoutSet,
};
use solpaper_storage::{load_layout, save_layout, AppPaths, SettingsDocument};
use solpaper_windows::{
    activate_existing_show_settings, run_runtime_host, second_launch_outcome,
    set_process_dpi_awareness, RuntimeHostConfig, SecondLaunchOutcome, SingleInstanceGuard,
    WidgetSurfaceConfig,
};

fn main() -> ExitCode {
    if let Err(e) = run() {
        eprintln!("solpaper: {e}");
        return ExitCode::from(1);
    }
    ExitCode::SUCCESS
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let smoke = env::args().any(|a| a == "--smoke");
    let _background = env::args().any(|a| a == "--background");
    set_process_dpi_awareness();

    let _guard = match SingleInstanceGuard::acquire() {
        Ok(g) => g,
        Err(solpaper_windows::SingleInstanceError::AlreadyRunning) => {
            // Narrow activation only (ADR-0007 / pack #7) — never start a second Runtime.
            match second_launch_outcome(activate_existing_show_settings()) {
                SecondLaunchOutcome::Activated => {
                    eprintln!("solpaper: already running; requested Settings");
                }
                SecondLaunchOutcome::AlreadyRunningNoWindow => {
                    eprintln!("solpaper: already running (single-instance)");
                }
            }
            return Ok(());
        }
        Err(e) => return Err(e.into()),
    };

    let paths = AppPaths::from_local_app_data()?;
    paths.ensure_dirs()?;

    let settings = SettingsDocument::load_or_default(&paths.settings)?;
    settings.save(&paths.settings)?;

    let mut layout = load_layout(&paths.layout)?;
    if layout.widgets.is_empty() {
        layout = default_widget_layout(settings.default_opacity)?;
        save_layout(&paths.layout, &layout)?;
    }

    let widgets: Vec<WidgetSurfaceConfig> = layout
        .widgets
        .iter()
        .map(|entry| {
            let origin = WidgetLayoutSet::resolve_top_left(entry, 1920.0, 1080.0);
            WidgetSurfaceConfig {
                id: entry.id.as_str().to_string(),
                title: format!("Solpaper · {}", entry.id.as_str()),
                x: origin.x as i32,
                y: origin.y as i32,
                width: entry.size_dip.width.max(1.0) as i32,
                height: entry.size_dip.height.max(1.0) as i32,
                opacity: entry.opacity,
            }
        })
        .collect();

    // Control window + tray + Approach A widgets. Smoke: create, toggle mode, tear down.
    run_runtime_host(&RuntimeHostConfig { smoke, widgets })?;
    Ok(())
}

fn default_widget_layout(opacity: u8) -> Result<WidgetLayoutSet, solpaper_core::CoreError> {
    let mut set = WidgetLayoutSet::new_empty();
    set.widgets.push(WidgetLayoutEntry {
        id: WidgetId::new("placeholder")?,
        monitor: MonitorMatch::Primary,
        anchor: Anchor::TopLeft,
        offset_dip: DipPoint::new(48.0, 48.0)?,
        size_dip: DipSize::new(280.0, 160.0)?,
        opacity,
    });
    Ok(set)
}
