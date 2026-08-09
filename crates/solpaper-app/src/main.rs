//! Solpaper user-session host (ADR-0002 / #7 / #20).
//!
//! Alpha 1 tracer bullet 5: Pomodoro widget projection + tray balloon dedupe.
//! Bullets 1–4: runtime/tray, widget host, settings/layout, domain persist + tray.

use std::env;
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

use solpaper_core::{
    clamp_rect_visible, Anchor, DipPoint, DipSize, MonitorMatch, PomodoroCommand, SurfaceRect,
    WidgetId, WidgetLayoutEntry, WidgetLayoutSet,
};
use solpaper_storage::{load_layout, load_pomodoro, AppPaths, LoadOutcome, SettingsDocument};
use solpaper_windows::{
    activate_existing_show_settings, primary_work_area, run_runtime_host, second_launch_outcome,
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

    let (settings, settings_outcome) = SettingsDocument::load_or_default(&paths.settings)?;
    if settings_outcome == LoadOutcome::RecoveredFromCorrupt {
        eprintln!("solpaper: recovered settings from corrupt file; writing defaults");
    }
    settings.save(&paths.settings)?;

    let (mut layout, layout_outcome) = load_layout(&paths.layout)?;
    if layout_outcome == LoadOutcome::RecoveredFromCorrupt {
        eprintln!("solpaper: recovered layout from corrupt file");
    }
    if layout.widgets.is_empty() {
        layout = default_widget_layout(settings.default_opacity)?;
        solpaper_storage::save_layout(&paths.layout, &layout)?;
    }

    let work = primary_work_area();
    let widgets: Vec<WidgetSurfaceConfig> = layout
        .widgets
        .iter()
        .map(|entry| entry_to_surface_config(entry, work))
        .collect();

    let (mut pomodoro, pomodoro_outcome) = load_pomodoro(&paths.pomodoro)?;
    if pomodoro_outcome == LoadOutcome::RecoveredFromCorrupt {
        eprintln!("solpaper: recovered pomodoro from corrupt file");
    }
    // Recovery path: complete at most one expired phase; never auto-start next.
    let now_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0);
    match pomodoro.apply(PomodoroCommand::Sync, now_ms) {
        Ok(events) if !events.is_empty() => {
            eprintln!(
                "solpaper: pomodoro recovery completed {} event(s)",
                events.len()
            );
            solpaper_storage::save_pomodoro(&paths.pomodoro, &pomodoro)?;
        }
        Ok(_) => {
            // Ensure a durable file exists after first run / missing.
            if pomodoro_outcome != LoadOutcome::Loaded {
                solpaper_storage::save_pomodoro(&paths.pomodoro, &pomodoro)?;
            }
        }
        Err(e) => eprintln!("solpaper: pomodoro Sync on restore failed: {e}"),
    }

    // Control window + tray + Approach A widgets. Smoke: create, toggle mode, tear down.
    // layout_path / pomodoro_path enable atomic flush on transitions and shutdown.
    run_runtime_host(&RuntimeHostConfig {
        smoke,
        widgets,
        layout_path: Some(paths.layout.clone()),
        pomodoro_path: Some(paths.pomodoro.clone()),
        pomodoro: Some(pomodoro),
    })?;
    Ok(())
}

fn entry_to_surface_config(
    entry: &WidgetLayoutEntry,
    work: solpaper_core::WorkArea,
) -> WidgetSurfaceConfig {
    let origin = WidgetLayoutSet::resolve_top_left(entry, work.width, work.height);
    let raw = SurfaceRect::new(
        origin.x,
        origin.y,
        entry.size_dip.width.max(1.0),
        entry.size_dip.height.max(1.0),
    )
    .unwrap_or(SurfaceRect {
        x: work.x + 48.0,
        y: work.y + 48.0,
        width: 280.0,
        height: 160.0,
    });
    // Off-screen / missing-monitor recovery: keep at least MIN_VISIBLE_DIP in work area.
    let placed = clamp_rect_visible(raw, work);
    WidgetSurfaceConfig {
        id: entry.id.as_str().to_string(),
        title: format!("Solpaper · {}", entry.id.as_str()),
        x: placed.x as i32,
        y: placed.y as i32,
        width: placed.width.max(1.0) as i32,
        height: placed.height.max(1.0) as i32,
        opacity: entry.opacity,
    }
}

fn default_widget_layout(opacity: u8) -> Result<WidgetLayoutSet, solpaper_core::CoreError> {
    let mut set = WidgetLayoutSet::new_empty();
    set.widgets.push(WidgetLayoutEntry {
        // Prefer `pomodoro`; host also projects onto legacy `placeholder` ids.
        id: WidgetId::new("pomodoro")?,
        monitor: MonitorMatch::Primary,
        anchor: Anchor::TopLeft,
        offset_dip: DipPoint::new(48.0, 48.0)?,
        size_dip: DipSize::new(280.0, 160.0)?,
        opacity,
    });
    Ok(set)
}

#[cfg(test)]
mod tests {
    use super::*;
    use solpaper_core::WorkArea;

    #[test]
    fn offscreen_entry_is_clamped_into_work_area() {
        let work = WorkArea::new(0.0, 0.0, 800.0, 600.0).unwrap();
        let entry = WidgetLayoutEntry {
            id: WidgetId::new("placeholder").unwrap(),
            monitor: MonitorMatch::Primary,
            anchor: Anchor::TopLeft,
            offset_dip: DipPoint::new(5000.0, 5000.0).unwrap(),
            size_dip: DipSize::new(200.0, 100.0).unwrap(),
            opacity: 200,
        };
        let cfg = entry_to_surface_config(&entry, work);
        // Must not remain at 5000,5000 — clamp keeps min visible region in work area.
        assert!(cfg.x < 800, "x={} should be inside/near work area", cfg.x);
        assert!(cfg.y < 600, "y={} should be inside/near work area", cfg.y);
        assert!(cfg.width > 0 && cfg.height > 0);
    }
}
