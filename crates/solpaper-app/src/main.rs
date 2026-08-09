//! Solpaper user-session host (ADR-0002 / #7 / #20).
//!
//! Alpha 1 tracer bullet 7: diagnostics/status baseline from #40.
//! Bullets 1–6: runtime, widgets, layout, Pomodoro, projection, local wallpaper.

use std::env;
use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::Once;
use std::time::{SystemTime, UNIX_EPOCH};

use solpaper_core::{
    clamp_rect_visible, redact_user_path, should_recommend_safe_mode, ActiveError, Anchor,
    Component, CorrelationId, CorrelationScope, CrashMarker, DipPoint, DipSize, MonitorMatch,
    PomodoroCommand, StartupRecord, SupportCounters, SurfaceRect, WidgetId, WidgetLayoutEntry,
    WidgetLayoutSet,
};
use solpaper_storage::{
    append_crash_marker, load_layout, load_pomodoro, AppPaths, CrashMarkerDocument, LoadOutcome,
    SettingsDocument,
};
use solpaper_windows::{
    activate_existing_show_settings, primary_work_area, run_runtime_host, second_launch_outcome,
    set_process_dpi_awareness, DiagnosticsHostConfig, RuntimeHostConfig, SecondLaunchOutcome,
    SingleInstanceGuard, WidgetSurfaceConfig,
};

static PANIC_HOOK: Once = Once::new();

fn main() -> ExitCode {
    if let Err(e) = run() {
        eprintln!("solpaper: {e}");
        return ExitCode::from(1);
    }
    ExitCode::SUCCESS
}

fn build_sha() -> String {
    option_env!("SOLPAPAPER_BUILD_SHA")
        .or(option_env!("GITHUB_SHA"))
        .map(|s| {
            if s.len() > 12 {
                s[..12].to_string()
            } else {
                s.to_string()
            }
        })
        .unwrap_or_else(|| "dev".into())
}

fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

/// Install a panic hook that appends a redacted crash marker (no stacks with paths).
fn install_panic_hook(crash_markers_path: PathBuf, build: String) {
    PANIC_HOOK.call_once(move || {
        let default = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            let marker = CrashMarker::panic_marker(now_ms(), build.clone());
            // Best-effort; never panic inside the hook.
            let _ = append_crash_marker(&crash_markers_path, &marker);
            eprintln!(
                "solpaper: panic recorded as {} (redacted marker; no auto-restart)",
                CrashMarker::INTERNAL_PANIC
            );
            default(info);
        }));
    });
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

    let version = env!("CARGO_PKG_VERSION").to_string();
    let build = build_sha();
    install_panic_hook(paths.crash_markers.clone(), build.clone());

    let startup_ms = now_ms();
    let startup_id = CorrelationId::mint(
        CorrelationScope::Startup,
        startup_ms,
        std::process::id(),
        startup_ms as u64 ^ 0x57A27,
    );
    eprintln!(
        "solpaper: startup correlation_id={} component=runtime event=start",
        startup_id
    );

    let crash_doc = CrashMarkerDocument::load_or_empty(&paths.crash_markers);
    let safe_mode = should_recommend_safe_mode(&crash_doc.times_ms, startup_ms);
    let safe_mode_reason = if safe_mode {
        Some("≥3 startup crash markers within 5 minutes")
    } else {
        None
    };
    if safe_mode {
        eprintln!("solpaper: safe mode recommended (crash markers in window); widgets disabled");
    }

    let mut initial_errors: Vec<ActiveError> = Vec::new();
    let mut counters = SupportCounters::default();

    let (mut settings, settings_outcome) = SettingsDocument::load_or_default(&paths.settings)?;
    if settings_outcome == LoadOutcome::RecoveredFromCorrupt {
        eprintln!("solpaper: recovered settings from corrupt file; writing defaults");
        initial_errors.push(ActiveError::new(
            "SettingsCorruptRecovered",
            Component::Storage,
            Some("settings quarantined; defaults loaded"),
        ));
        counters.storage_recoveries = counters.storage_recoveries.saturating_add(1);
    }
    // Default drop-folder when user has not configured folders yet.
    if settings.wallpaper_folders.is_empty() {
        settings
            .wallpaper_folders
            .push(paths.wallpapers.to_string_lossy().into_owned());
    }
    settings.save(&paths.settings)?;

    let (mut layout, layout_outcome) = load_layout(&paths.layout)?;
    if layout_outcome == LoadOutcome::RecoveredFromCorrupt {
        eprintln!("solpaper: recovered layout from corrupt file");
        initial_errors.push(ActiveError::new(
            "LayoutCorruptRecovered",
            Component::Storage,
            Some("layout quarantined; empty defaults"),
        ));
        counters.storage_recoveries = counters.storage_recoveries.saturating_add(1);
    }
    if layout.widgets.is_empty() && !safe_mode {
        layout = default_widget_layout(settings.default_opacity)?;
        solpaper_storage::save_layout(&paths.layout, &layout)?;
    }

    let work = primary_work_area();
    let widgets: Vec<WidgetSurfaceConfig> = if safe_mode {
        Vec::new()
    } else {
        layout
            .widgets
            .iter()
            .map(|entry| entry_to_surface_config(entry, work))
            .collect()
    };

    let (mut pomodoro, pomodoro_outcome) = load_pomodoro(&paths.pomodoro)?;
    if pomodoro_outcome == LoadOutcome::RecoveredFromCorrupt {
        eprintln!("solpaper: recovered pomodoro from corrupt file");
        initial_errors.push(ActiveError::new(
            "PomodoroCorruptRecovered",
            Component::Storage,
            Some("pomodoro quarantined; idle defaults"),
        ));
        counters.storage_recoveries = counters.storage_recoveries.saturating_add(1);
    }
    // Recovery path: complete at most one expired phase; never auto-start next.
    let now_ms = now_ms();
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

    let wallpaper_folders: Vec<PathBuf> = settings
        .wallpaper_folders
        .iter()
        .map(PathBuf::from)
        .collect();
    if !smoke {
        eprintln!(
            "solpaper: wallpaper folders ({}): drop images or use tray Next/Hold",
            wallpaper_folders.len()
        );
        eprintln!(
            "solpaper: data={} logs={}",
            redact_user_path(&paths.root.to_string_lossy()),
            redact_user_path(&paths.logs.to_string_lossy())
        );
    }

    let diagnostics = DiagnosticsHostConfig {
        version,
        build_sha: build,
        config_schema_version: SettingsDocument::CURRENT_VERSION,
        startup: StartupRecord {
            at_ms: startup_ms,
            ok: true,
            correlation_id: startup_id,
        },
        safe_mode,
        safe_mode_reason,
        data_dir: paths.root.clone(),
        cache_dir: paths.cache.clone(),
        logs_dir: paths.logs.clone(),
        status_path: paths.diagnostics_status.clone(),
        initial_errors,
        initial_counters: counters,
    };

    // Control window + tray + Approach A widgets. Smoke: create, toggle mode, tear down.
    // layout_path / pomodoro_path enable atomic flush on transitions and shutdown.
    run_runtime_host(&RuntimeHostConfig {
        smoke,
        widgets,
        layout_path: Some(paths.layout.clone()),
        pomodoro_path: Some(paths.pomodoro.clone()),
        pomodoro: Some(pomodoro),
        wallpaper_folders,
        wallpaper_hold: settings.wallpaper_hold,
        wallpaper_cache: Some(paths.cache.clone()),
        settings_path: Some(paths.settings.clone()),
        diagnostics: Some(diagnostics),
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
