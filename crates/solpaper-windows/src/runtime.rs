//! Runtime control window + Shell_NotifyIcon tray host (Issue #20 / pack #7).
//!
//! Registers `Solpaper.Runtime.Control.v1`, owns the session message loop, tray icon
//! (NIM_ADD + NIM_SETVERSION), TaskbarCreated re-add, fixed-order context menu, and
//! widget host Edit Mode hotkeys (Ctrl+Alt+F2, Escape while editing).
//! Second launch finds this HWND via `FindWindowW`.
//!
//! Tracer bullets 4–7: Pomodoro, wallpaper, diagnostics/status baseline (#40).

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use solpaper_core::{
    alpha1_wallpaper_flags, build_tray_menu, clamp_rect_visible, phase_instance_key,
    pomodoro_command_for_tray, pomodoro_completion_balloon, pomodoro_tray_tip,
    pomodoro_widget_lines, runtime_recovery_plan, ActiveError, ActiveErrorLog, Component,
    CorrelationId, CorrelationScope, DiagnosticsPathDisplay, DiagnosticsSnapshot,
    LocalWallpaperController, NotificationDeduper, PhaseInstanceId, PomodoroCommand, PomodoroEvent,
    PomodoroState, RecoveryAction, StartupRecord, SupportCounters, SurfaceMode, SurfaceRect,
    TrayCommand, TrayMenuEntry, WallpaperCycleKind, WallpaperCycleRecord, WidgetId,
    WidgetLayoutEntry, WidgetLayoutSet, XorShift64, CONTROL_WINDOW_CLASS, REMOTE_CRASH_UPLOAD,
    TELEMETRY_ENABLED,
};
use solpaper_storage::{
    load_layout, save_layout, save_pomodoro, write_diagnostics_status, SettingsDocument,
};
use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, POINT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    RegisterHotKey, UnregisterHotKey, HOT_KEY_MODIFIERS, MOD_ALT, MOD_CONTROL, MOD_NOREPEAT,
    VK_ESCAPE, VK_F2,
};
use windows::Win32::UI::Shell::{
    Shell_NotifyIconW, NIF_ICON, NIF_INFO, NIF_MESSAGE, NIF_TIP, NIIF_INFO, NIM_ADD, NIM_DELETE,
    NIM_MODIFY, NIM_SETVERSION, NOTIFYICONDATAW, NOTIFYICONDATAW_0, NOTIFYICON_VERSION_4,
};
use windows::Win32::UI::WindowsAndMessaging::{
    AppendMenuW, CreatePopupMenu, CreateWindowExW, DefWindowProcW, DestroyMenu, DestroyWindow,
    DispatchMessageW, GetCursorPos, GetMessageW, KillTimer, LoadCursorW, LoadIconW, MessageBoxW,
    PeekMessageW, PostMessageW, PostQuitMessage, RegisterClassW, RegisterWindowMessageW,
    SetForegroundWindow, SetMenuDefaultItem, SetTimer, TrackPopupMenu, TranslateMessage,
    CS_HREDRAW, CS_VREDRAW, HICON, IDC_ARROW, IDI_APPLICATION, IDYES, MB_ICONINFORMATION,
    MB_ICONQUESTION, MB_OK, MB_YESNO, MF_DISABLED, MF_ENABLED, MF_GRAYED, MF_SEPARATOR, MF_STRING,
    MSG, PM_REMOVE, TPM_BOTTOMALIGN, TPM_LEFTALIGN, TPM_RIGHTBUTTON, WM_APP, WM_COMMAND,
    WM_DESTROY, WM_HOTKEY, WM_NULL, WM_QUIT, WM_RBUTTONUP, WM_TIMER, WNDCLASSW, WS_EX_NOACTIVATE,
    WS_EX_TOOLWINDOW, WS_POPUP,
};

use crate::activation::WM_APP_SHOW_SETTINGS;
use crate::wallpaper::{prepare_owned_wallpaper, ComDesktopWallpaper, DesktopWallpaper};
use crate::widget_host::{
    clear_layout_dirty, create_widget_host, destroy_all_widgets, primary_work_area,
    set_pomodoro_projection, set_surface_mode, snapshot_widget_rects, surface_mode,
    toggle_surface_mode, WidgetSurfaceConfig,
};

/// Tray callback message (WM_APP + 2). Control window only.
const WM_TRAYICON: u32 = WM_APP + 2;

/// Base id for tray menu commands (must not collide with system ids).
const MENU_ID_BASE: u16 = 0xA000;

/// Hotkey ids registered on the control HWND.
const HOTKEY_TOGGLE_EDIT: i32 = 1;
const HOTKEY_ESCAPE_EDIT: i32 = 2;

/// Control-window timer for live Pomodoro deadline checks (not a persistence tick).
const TIMER_POMODORO_LIVE: usize = 1;
const POMODORO_LIVE_INTERVAL_MS: u32 = 1_000;

static CONTROL_CLASS_REGISTERED: AtomicBool = AtomicBool::new(false);
static TASKBAR_CREATED_MSG: AtomicU32 = AtomicU32::new(0);

/// True after WM_APP_SHOW_SETTINGS until cleared (settings UI is lazy / later bullet).
static SETTINGS_REQUESTED: AtomicBool = AtomicBool::new(false);

/// Whether the Runtime is accepting new tray work (cleared on shutdown).
static ACCEPTING_WORK: AtomicBool = AtomicBool::new(true);

/// Escape hotkey is registered only while Edit Mode is active.
static ESCAPE_HOTKEY_REGISTERED: AtomicBool = AtomicBool::new(false);

#[derive(Debug)]
pub enum RuntimeError {
    Win32(windows::core::Error),
    Message(String),
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Win32(e) => write!(f, "{e}"),
            Self::Message(s) => write!(f, "{s}"),
        }
    }
}

impl std::error::Error for RuntimeError {}

impl From<windows::core::Error> for RuntimeError {
    fn from(value: windows::core::Error) -> Self {
        Self::Win32(value)
    }
}

/// Host configuration for the Alpha 1 runtime loop.
#[derive(Debug, Clone, Default)]
pub struct RuntimeHostConfig {
    /// When true: create control + tray (+ widgets), pump briefly, tear down.
    pub smoke: bool,
    /// Approach A widget surfaces (empty = tray-only runtime).
    pub widgets: Vec<WidgetSurfaceConfig>,
    /// When set, flush layout JSON on Edit→Normal and shutdown (atomic write).
    pub layout_path: Option<PathBuf>,
    /// When set, flush Pomodoro JSON on semantic transitions and shutdown.
    pub pomodoro_path: Option<PathBuf>,
    /// Initial Pomodoro machine (caller should already have applied recovery `Sync`).
    pub pomodoro: Option<PomodoroState>,
    /// Local wallpaper folders (empty → host uses no catalog until set).
    pub wallpaper_folders: Vec<PathBuf>,
    pub wallpaper_hold: bool,
    /// Cache directory for owned wallpaper files.
    pub wallpaper_cache: Option<PathBuf>,
    /// Settings path for persisting hold + folder prefs.
    pub settings_path: Option<PathBuf>,
    /// Diagnostics baseline (#20 bullet 7 / #40).
    pub diagnostics: Option<DiagnosticsHostConfig>,
}

/// Diagnostics inputs owned by the Runtime for the status surface.
#[derive(Debug, Clone)]
pub struct DiagnosticsHostConfig {
    pub version: String,
    pub build_sha: String,
    pub config_schema_version: u32,
    pub startup: StartupRecord,
    pub safe_mode: bool,
    pub safe_mode_reason: Option<&'static str>,
    /// Absolute paths (will be redacted for display).
    pub data_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub logs_dir: PathBuf,
    /// Where to write the last Diagnostics status text.
    pub status_path: PathBuf,
    /// Seed active errors from startup recovery (settings/layout corrupt, etc.).
    pub initial_errors: Vec<ActiveError>,
    pub initial_counters: SupportCounters,
}

static LAYOUT_PATH: Mutex<Option<PathBuf>> = Mutex::new(None);
static POMODORO_PATH: Mutex<Option<PathBuf>> = Mutex::new(None);
static POMODORO: Mutex<Option<PomodoroState>> = Mutex::new(None);
static NOTIFY_DEDUPER: Mutex<NotificationDeduper> = Mutex::new(NotificationDeduper::empty());
/// Control HWND for tray NIM_MODIFY (tip + balloons).
static CONTROL_HWND: Mutex<Option<UiHwnd>> = Mutex::new(None);
static WALLPAPER_CTL: Mutex<Option<LocalWallpaperController>> = Mutex::new(None);
static WALLPAPER_RNG: Mutex<Option<XorShift64>> = Mutex::new(None);
static WALLPAPER_CACHE: Mutex<Option<PathBuf>> = Mutex::new(None);
static SETTINGS_PATH: Mutex<Option<PathBuf>> = Mutex::new(None);
static DIAGNOSTICS: Mutex<Option<RuntimeDiagnostics>> = Mutex::new(None);

#[derive(Debug)]
struct RuntimeDiagnostics {
    version: String,
    build_sha: String,
    config_schema_version: u32,
    startup: StartupRecord,
    safe_mode: bool,
    safe_mode_reason: Option<&'static str>,
    data_dir: PathBuf,
    cache_dir: PathBuf,
    logs_dir: PathBuf,
    status_path: PathBuf,
    errors: ActiveErrorLog,
    counters: SupportCounters,
    last_wallpaper: Option<WallpaperCycleRecord>,
}

/// HWND is not Send in the windows crate; UI-thread Runtime only.
#[derive(Clone, Copy)]
struct UiHwnd(HWND);
// SAFETY: set and used only on the UI thread that owns the message loop.
unsafe impl Send for UiHwnd {}
unsafe impl Sync for UiHwnd {}

/// Whether a second-launch activation requested Settings (for host / tests).
pub fn take_settings_requested() -> bool {
    SETTINGS_REQUESTED.swap(false, Ordering::SeqCst)
}

/// Run the Runtime: control HWND, tray icon, widget host, message loop.
///
/// # Safety context
///
/// All Win32 calls are confined to this module and `widget_host`. Tray uses a fixed
/// uid; Explorer restart re-adds via `TaskbarCreated` only (does not reparent widgets).
pub fn run_runtime_host(config: &RuntimeHostConfig) -> Result<(), RuntimeError> {
    ACCEPTING_WORK.store(true, Ordering::SeqCst);
    SETTINGS_REQUESTED.store(false, Ordering::SeqCst);
    ESCAPE_HOTKEY_REGISTERED.store(false, Ordering::SeqCst);
    {
        let mut guard = LAYOUT_PATH.lock().expect("layout path");
        *guard = config.layout_path.clone();
    }
    {
        let mut guard = POMODORO_PATH.lock().expect("pomodoro path");
        *guard = config.pomodoro_path.clone();
    }
    {
        let mut guard = WALLPAPER_CACHE.lock().expect("wp cache");
        *guard = config.wallpaper_cache.clone();
    }
    {
        let mut guard = SETTINGS_PATH.lock().expect("settings path");
        *guard = config.settings_path.clone();
    }
    {
        let mut guard = WALLPAPER_CTL.lock().expect("wallpaper ctl");
        *guard = Some(LocalWallpaperController::from_folders(
            config.wallpaper_folders.clone(),
            config.wallpaper_hold,
        ));
    }
    {
        let mut guard = WALLPAPER_RNG.lock().expect("wallpaper rng");
        *guard = Some(XorShift64::from_entropy());
    }
    {
        let mut guard = POMODORO.lock().expect("pomodoro state");
        *guard = config
            .pomodoro
            .clone()
            .or_else(|| Some(PomodoroState::idle_default()));
        let seed = guard.as_ref().and_then(|s| s.last_completion_id);
        let mut dedupe = NOTIFY_DEDUPER.lock().expect("deduper");
        *dedupe = NotificationDeduper::empty();
        dedupe.seed_from_completion_id(seed);
    }
    {
        let mut guard = DIAGNOSTICS.lock().expect("diagnostics");
        *guard = config.diagnostics.as_ref().map(|d| {
            let mut errors = ActiveErrorLog::new();
            for e in &d.initial_errors {
                errors.push(e.clone());
            }
            let mut counters = d.initial_counters.clone();
            if d.safe_mode {
                counters.safe_mode_entries = counters.safe_mode_entries.saturating_add(1);
            }
            RuntimeDiagnostics {
                version: d.version.clone(),
                build_sha: d.build_sha.clone(),
                config_schema_version: d.config_schema_version,
                startup: d.startup.clone(),
                safe_mode: d.safe_mode,
                safe_mode_reason: d.safe_mode_reason,
                data_dir: d.data_dir.clone(),
                cache_dir: d.cache_dir.clone(),
                logs_dir: d.logs_dir.clone(),
                status_path: d.status_path.clone(),
                errors,
                counters,
                last_wallpaper: None,
            }
        });
    }

    let safe_mode = config
        .diagnostics
        .as_ref()
        .map(|d| d.safe_mode)
        .unwrap_or(false);

    unsafe {
        let hinstance = GetModuleHandleW(None)?;
        ensure_control_class(hinstance.into())?;

        let taskbar_msg = RegisterWindowMessageW(w!("TaskbarCreated"));
        if taskbar_msg == 0 {
            return Err(RuntimeError::Message(
                "RegisterWindowMessageW(TaskbarCreated) failed".into(),
            ));
        }
        TASKBAR_CREATED_MSG.store(taskbar_msg, Ordering::SeqCst);

        let class = wide_z(CONTROL_WINDOW_CLASS);
        let title = wide_z("Solpaper Runtime");
        // Hidden top-level window so FindWindowW can locate it (not HWND_MESSAGE).
        let control = CreateWindowExW(
            WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE,
            PCWSTR(class.as_ptr()),
            PCWSTR(title.as_ptr()),
            WS_POPUP,
            0,
            0,
            0,
            0,
            None,
            None,
            hinstance,
            None,
        )?;

        if control.is_invalid() {
            return Err(RuntimeError::Message("control window create failed".into()));
        }
        {
            let mut guard = CONTROL_HWND.lock().expect("control hwnd");
            *guard = Some(UiHwnd(control));
        }

        tray_add(control)?;
        register_toggle_edit_hotkey(control)?;
        // Live deadline checks while the process is continuously running.
        let _ = SetTimer(
            control,
            TIMER_POMODORO_LIVE,
            POMODORO_LIVE_INTERVAL_MS,
            None,
        );

        // Safe mode (#40): no widgets; settings/diagnostics remain via tray.
        let has_widgets = !config.widgets.is_empty() && !safe_mode;
        if has_widgets {
            create_widget_host(&config.widgets)?;
            set_surface_mode(SurfaceMode::Normal);
            record_surface_recreate();
        } else if safe_mode {
            eprintln!("solpaper: safe mode — widgets disabled; open Diagnostics from tray");
        }
        refresh_pomodoro_projection(control);

        if config.smoke {
            // Exercise mode toggle + one pomodoro Start/Reset path for smoke.
            if has_widgets {
                let _ = toggle_surface_mode();
                let _ = toggle_surface_mode();
            }
            apply_pomodoro_tray_command(TrayCommand::PomodoroStartPauseResume);
            apply_pomodoro_tray_command(TrayCommand::PomodoroReset);
            pump_peek(48);
            // Graceful teardown without full GetMessage loop.
            ACCEPTING_WORK.store(false, Ordering::SeqCst);
            flush_layout_to_disk();
            flush_pomodoro_to_disk();
            let _ = KillTimer(control, TIMER_POMODORO_LIVE);
            unregister_all_hotkeys(control);
            let _ = tray_delete(control);
            if has_widgets {
                destroy_all_widgets();
            }
            let _ = DestroyWindow(control);
            pump_peek(16);
            clear_runtime_globals();
            return Ok(());
        }

        let mut msg = MSG::default();
        loop {
            let ok = GetMessageW(&mut msg, None, 0, 0);
            if ok.0 == -1 {
                break;
            }
            if !ok.as_bool() {
                break; // WM_QUIT
            }
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        // Loop exit: flush durable state, then remove tray/hotkeys/widgets if still present.
        flush_layout_to_disk();
        flush_pomodoro_to_disk();
        let _ = KillTimer(control, TIMER_POMODORO_LIVE);
        unregister_all_hotkeys(control);
        let _ = tray_delete(control);
        if has_widgets {
            destroy_all_widgets();
        }
    }

    clear_runtime_globals();
    Ok(())
}

fn clear_runtime_globals() {
    {
        let mut guard = LAYOUT_PATH.lock().expect("layout path");
        *guard = None;
    }
    {
        let mut guard = POMODORO_PATH.lock().expect("pomodoro path");
        *guard = None;
    }
    {
        let mut guard = POMODORO.lock().expect("pomodoro state");
        *guard = None;
    }
    {
        let mut guard = CONTROL_HWND.lock().expect("control hwnd");
        *guard = None;
    }
    {
        let mut guard = NOTIFY_DEDUPER.lock().expect("deduper");
        *guard = NotificationDeduper::empty();
    }
    {
        let mut guard = WALLPAPER_CTL.lock().expect("wallpaper ctl");
        *guard = None;
    }
    {
        let mut guard = WALLPAPER_RNG.lock().expect("wallpaper rng");
        *guard = None;
    }
    {
        let mut guard = WALLPAPER_CACHE.lock().expect("wp cache");
        *guard = None;
    }
    {
        let mut guard = SETTINGS_PATH.lock().expect("settings path");
        *guard = None;
    }
    {
        let mut guard = DIAGNOSTICS.lock().expect("diagnostics");
        *guard = None;
    }
}

fn record_surface_recreate() {
    if let Ok(mut g) = DIAGNOSTICS.lock() {
        if let Some(d) = g.as_mut() {
            d.counters.surface_recreates = d.counters.surface_recreates.saturating_add(1);
        }
    }
}

fn push_active_error(err: ActiveError) {
    if let Ok(mut g) = DIAGNOSTICS.lock() {
        if let Some(d) = g.as_mut() {
            if err.component == Component::Wallpaper {
                d.counters.wallpaper_failures = d.counters.wallpaper_failures.saturating_add(1);
            }
            if err.component == Component::Storage {
                d.counters.storage_recoveries = d.counters.storage_recoveries.saturating_add(1);
            }
            d.errors.push(err);
        }
    }
}

fn note_wallpaper_cycle(ok: bool, error_code: Option<&str>) {
    let now = now_utc_ms();
    let salt = now as u64 ^ 0xA11_FA5E_u64;
    let id = CorrelationId::mint(
        CorrelationScope::WallpaperCycle,
        now,
        std::process::id(),
        salt,
    );
    if let Ok(mut g) = DIAGNOSTICS.lock() {
        if let Some(d) = g.as_mut() {
            d.last_wallpaper = Some(WallpaperCycleRecord {
                at_ms: now,
                kind: WallpaperCycleKind::Local,
                ok,
                error_code: error_code.map(|s| s.to_string()),
                correlation_id: Some(id),
            });
        }
    }
}

fn build_diagnostics_snapshot() -> Option<DiagnosticsSnapshot> {
    let g = DIAGNOSTICS.lock().ok()?;
    let d = g.as_ref()?;
    let has_wp =
        d.errors.has_wallpaper_error() || d.last_wallpaper.as_ref().map(|w| !w.ok).unwrap_or(false);
    Some(DiagnosticsSnapshot {
        version: d.version.clone(),
        build_sha: d.build_sha.clone(),
        config_schema_version: d.config_schema_version,
        last_startup: Some(d.startup.clone()),
        last_calendar_sync_label: "not connected".into(),
        last_wallpaper_cycle: d.last_wallpaper.clone(),
        active_errors: d.errors.as_slice().to_vec(),
        safe_mode: d.safe_mode,
        safe_mode_reason: d.safe_mode_reason,
        paths: DiagnosticsPathDisplay {
            data_dir: solpaper_core::redact_user_path(&d.data_dir.to_string_lossy()),
            cache_dir: solpaper_core::redact_user_path(&d.cache_dir.to_string_lossy()),
            logs_dir: solpaper_core::redact_user_path(&d.logs_dir.to_string_lossy()),
        },
        recovery_actions: DiagnosticsSnapshot::default_recovery_actions(d.safe_mode, has_wp),
        counters: d.counters.clone(),
        telemetry_enabled: TELEMETRY_ENABLED,
        remote_crash_upload: REMOTE_CRASH_UPLOAD,
    })
}

fn open_diagnostics_ui() {
    let Some(snap) = build_diagnostics_snapshot() else {
        eprintln!("solpaper: Diagnostics unavailable (not configured)");
        return;
    };
    let text = snap.format_text();
    if let Ok(g) = DIAGNOSTICS.lock() {
        if let Some(d) = g.as_ref() {
            if let Err(e) = write_diagnostics_status(&d.status_path, &text) {
                eprintln!("solpaper: diagnostics status write failed: {e}");
            } else {
                let redacted = solpaper_core::redact_user_path(&d.status_path.to_string_lossy());
                eprintln!("solpaper: diagnostics status written to {redacted}");
            }
        }
    }
    // MessageBox baseline until Settings → Diagnostics UI lands.
    let body = wide_z(&text);
    let title = wide_z("Solpaper Diagnostics");
    unsafe {
        let _ = MessageBoxW(
            None,
            PCWSTR(body.as_ptr()),
            PCWSTR(title.as_ptr()),
            MB_OK | MB_ICONINFORMATION,
        );
    }

    // Bullet 8: offer runtime recovery after the status view (user consent).
    let has_wp = snap
        .active_errors
        .iter()
        .any(|e| e.component == Component::Wallpaper)
        || snap
            .last_wallpaper_cycle
            .as_ref()
            .map(|w| !w.ok)
            .unwrap_or(false);
    let plan = runtime_recovery_plan(snap.safe_mode, has_wp);
    if plan.is_empty() {
        return;
    }
    let mut prompt = String::from("Run recovery now?\n\n");
    for a in &plan {
        prompt.push_str(&format!("• {}\n", a.label()));
    }
    prompt.push_str("\nYes = run these steps. No = dismiss.");
    let pbody = wide_z(&prompt);
    let ptitle = wide_z("Solpaper Recovery");
    let answer = unsafe {
        MessageBoxW(
            None,
            PCWSTR(pbody.as_ptr()),
            PCWSTR(ptitle.as_ptr()),
            MB_YESNO | MB_ICONQUESTION,
        )
    };
    if answer == IDYES {
        run_recovery_plan(&plan);
    }
}

/// Execute consented recovery steps (widgets + wallpaper + Edit Mode).
fn run_recovery_plan(plan: &[RecoveryAction]) {
    for step in plan {
        match step {
            RecoveryAction::RecreateSurfaces => {
                if let Err(e) = recreate_surfaces_from_disk() {
                    eprintln!("solpaper: recovery recreate failed: {e}");
                    push_active_error(ActiveError::new(
                        "SurfaceRecreateFailed",
                        Component::Surface,
                        Some("recreate recovery failed"),
                    ));
                } else {
                    eprintln!("solpaper: recovery recreated widget surfaces (clamped)");
                }
            }
            RecoveryAction::OpenEditMode => {
                if let Some(hwnd) = control_hwnd() {
                    if !surface_mode().is_edit() {
                        apply_edit_mode(hwnd, SurfaceMode::Edit);
                        eprintln!("solpaper: recovery entered Edit Mode");
                    }
                }
            }
            RecoveryAction::RescanWallpapers => {
                recovery_rescan_wallpapers();
            }
            RecoveryAction::RestartApp
            | RecoveryAction::EnterSafeMode
            | RecoveryAction::ExportBundle => {
                // Not runtime-auto; listed for docs only.
            }
        }
    }
}

fn recovery_rescan_wallpapers() {
    let mut ctl = WALLPAPER_CTL.lock().expect("wallpaper ctl");
    if let Some(ctl) = ctl.as_mut() {
        let before = ctl.bag.source_len();
        ctl.rescan();
        eprintln!(
            "solpaper: recovery wallpaper rescan (catalog {} → {})",
            before,
            ctl.bag.source_len()
        );
    }
}

/// Destroy widgets, reload layout.json, clamp to work area, recreate host.
fn recreate_surfaces_from_disk() -> Result<(), RuntimeError> {
    let path = {
        let g = LAYOUT_PATH.lock().expect("layout path");
        g.clone()
    };
    let Some(path) = path else {
        return Err(RuntimeError::Message(
            "layout path not configured for recovery".into(),
        ));
    };
    let (set, _outcome) =
        load_layout(&path).map_err(|e| RuntimeError::Message(format!("load layout: {e}")))?;
    let configs = layout_set_to_surface_configs(&set);
    destroy_all_widgets();
    if configs.is_empty() {
        eprintln!("solpaper: recovery layout empty — no widgets recreated");
        return Ok(());
    }
    create_widget_host(&configs)
        .map_err(|e| RuntimeError::Message(format!("create widgets: {e}")))?;
    set_surface_mode(SurfaceMode::Normal);
    record_surface_recreate();
    if let Some(hwnd) = control_hwnd() {
        refresh_pomodoro_projection(hwnd);
    }
    Ok(())
}

fn layout_set_to_surface_configs(set: &WidgetLayoutSet) -> Vec<WidgetSurfaceConfig> {
    let work = primary_work_area();
    set.widgets
        .iter()
        .map(|entry| {
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
        })
        .collect()
}

fn wallpaper_next() {
    let cache = {
        let g = WALLPAPER_CACHE.lock().expect("wp cache");
        g.clone()
    };
    let Some(cache_dir) = cache else {
        eprintln!("solpaper: wallpaper cache path not configured");
        push_active_error(ActiveError::new(
            "WallpaperCacheMissing",
            Component::Wallpaper,
            Some("cache path not configured"),
        ));
        note_wallpaper_cycle(false, Some("WallpaperCacheMissing"));
        return;
    };

    let source = {
        let mut ctl = WALLPAPER_CTL.lock().expect("wallpaper ctl");
        let Some(ctl) = ctl.as_mut() else {
            return;
        };
        ctl.rescan();
        if ctl.bag.is_empty() {
            eprintln!("solpaper: no local wallpapers in configured folders");
            push_active_error(ActiveError::new(
                "WallpaperPathInvalid",
                Component::Wallpaper,
                Some("no images in folders"),
            ));
            note_wallpaper_cycle(false, Some("WallpaperPathInvalid"));
            return;
        }
        let mut rng_guard = WALLPAPER_RNG.lock().expect("wallpaper rng");
        let rng = rng_guard.get_or_insert_with(XorShift64::from_entropy);
        ctl.pick_next(rng)
    };
    let Some(source) = source else {
        eprintln!("solpaper: wallpaper bag empty");
        note_wallpaper_cycle(false, Some("WallpaperPathInvalid"));
        return;
    };

    let owned = match prepare_owned_wallpaper(&source, &cache_dir) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("solpaper: wallpaper prepare failed: {e}");
            push_active_error(ActiveError::new(
                "WallpaperFileTooLarge",
                Component::Wallpaper,
                Some("prepare failed"),
            ));
            note_wallpaper_cycle(false, Some("WallpaperFileTooLarge"));
            return;
        }
    };

    let adapter = match ComDesktopWallpaper::new() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("solpaper: wallpaper COM init failed: {e}");
            push_active_error(ActiveError::new(
                "WallpaperPlatform",
                Component::Wallpaper,
                Some("com init failed"),
            ));
            note_wallpaper_cycle(false, Some("WallpaperPlatform"));
            return;
        }
    };
    // Ensure global Fill (pack #5 DEFAULT).
    if let Err(e) = adapter.set_position(solpaper_core::WallpaperPosition::Fill) {
        eprintln!("solpaper: set wallpaper position: {e}");
    }

    let monitors = match adapter.monitors() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("solpaper: enumerate monitors failed: {e}");
            push_active_error(ActiveError::new(
                "WallpaperMonitorUnavailable",
                Component::Wallpaper,
                Some("enumerate failed"),
            ));
            note_wallpaper_cycle(false, Some("WallpaperMonitorUnavailable"));
            return;
        }
    };
    let mut any_ok = false;
    for mon in monitors.into_iter().filter(|m| m.attached) {
        match adapter.apply(&mon.id, &owned) {
            Ok(()) => {
                any_ok = true;
                eprintln!("solpaper: wallpaper applied on {}", mon.id.as_str());
            }
            Err(e) => {
                // Keep existing system wallpaper on failure (adapter contract).
                eprintln!(
                    "solpaper: wallpaper apply failed on {}: {e}",
                    mon.id.as_str()
                );
            }
        }
    }
    if any_ok {
        let mut ctl = WALLPAPER_CTL.lock().expect("wallpaper ctl");
        if let Some(ctl) = ctl.as_mut() {
            ctl.note_applied(owned);
        }
        note_wallpaper_cycle(true, None);
    } else {
        push_active_error(ActiveError::new(
            "WallpaperPlatform",
            Component::Wallpaper,
            Some("apply failed"),
        ));
        note_wallpaper_cycle(false, Some("WallpaperPlatform"));
    }
}

fn wallpaper_toggle_hold() {
    let hold = {
        let mut ctl = WALLPAPER_CTL.lock().expect("wallpaper ctl");
        let Some(ctl) = ctl.as_mut() else {
            return;
        };
        ctl.toggle_hold()
    };
    eprintln!(
        "solpaper: wallpaper hold {}",
        if hold { "ON" } else { "OFF" }
    );
    persist_wallpaper_hold(hold);
}

fn persist_wallpaper_hold(hold: bool) {
    let path = {
        let g = SETTINGS_PATH.lock().expect("settings path");
        g.clone()
    };
    let Some(path) = path else {
        return;
    };
    match SettingsDocument::load_or_default(&path) {
        Ok((mut doc, _)) => {
            doc.wallpaper_hold = hold;
            if let Err(e) = doc.save(&path) {
                eprintln!("solpaper: settings save (hold) failed: {e}");
            }
        }
        Err(e) => eprintln!("solpaper: settings load (hold) failed: {e}"),
    }
}

fn now_utc_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

/// Apply a Pomodoro tray command; persist only on successful semantic transition.
fn apply_pomodoro_tray_command(cmd: TrayCommand) {
    let now = now_utc_ms();
    let mut guard = POMODORO.lock().expect("pomodoro state");
    let Some(state) = guard.as_mut() else {
        return;
    };
    let Some(domain_cmd) = pomodoro_command_for_tray(cmd, &state.status) else {
        return;
    };
    match state.apply(domain_cmd, now) {
        Ok(events) => {
            if !events.is_empty() || matches!(domain_cmd, PomodoroCommand::Reset) {
                // Drop lock before disk I/O by cloning snapshot.
                let snapshot = state.clone();
                drop(guard);
                persist_pomodoro_snapshot(&snapshot);
                log_pomodoro_events(&events, domain_cmd);
                handle_pomodoro_side_effects(&events);
            } else {
                drop(guard);
            }
            if let Some(hwnd) = control_hwnd() {
                refresh_pomodoro_projection(hwnd);
            }
        }
        Err(e) => {
            eprintln!("solpaper: pomodoro {domain_cmd:?} rejected: {e}");
        }
    }
}

fn pomodoro_live_tick() {
    let now = now_utc_ms();
    let mut guard = POMODORO.lock().expect("pomodoro state");
    let Some(state) = guard.as_mut() else {
        return;
    };
    match state.apply(PomodoroCommand::LiveTick, now) {
        Ok(events) => {
            let had_events = !events.is_empty();
            if had_events {
                let snapshot = state.clone();
                drop(guard);
                persist_pomodoro_snapshot(&snapshot);
                log_pomodoro_events(&events, PomodoroCommand::LiveTick);
                handle_pomodoro_side_effects(&events);
            } else {
                drop(guard);
            }
            // Always refresh projection while alive so remaining time ticks.
            if let Some(hwnd) = control_hwnd() {
                refresh_pomodoro_projection(hwnd);
            }
        }
        Err(e) => eprintln!("solpaper: pomodoro LiveTick error: {e}"),
    }
}

fn control_hwnd() -> Option<HWND> {
    CONTROL_HWND.lock().expect("control hwnd").map(|h| h.0)
}

fn log_pomodoro_events(events: &[PomodoroEvent], cmd: PomodoroCommand) {
    if events.is_empty() {
        eprintln!("solpaper: pomodoro {cmd:?} applied");
        return;
    }
    for e in events {
        eprintln!("solpaper: pomodoro event {e:?}");
    }
}

/// Balloon notifications for phase completions (deduped by phase instance id).
fn handle_pomodoro_side_effects(events: &[PomodoroEvent]) {
    let Some(hwnd) = control_hwnd() else {
        return;
    };
    for e in events {
        if let PomodoroEvent::PhaseCompleted {
            phase,
            completion_id,
            ..
        } = e
        {
            let key = PhaseInstanceId::new(phase_instance_key(*completion_id));
            let should = {
                let mut d = NOTIFY_DEDUPER.lock().expect("deduper");
                d.try_notify(&key)
            };
            if should {
                let (title, body) = pomodoro_completion_balloon(*phase);
                tray_balloon(hwnd, title, &body);
            }
        }
    }
}

/// Push `PomodoroView` into the widget host and tray tip.
fn refresh_pomodoro_projection(hwnd: HWND) {
    let now = now_utc_ms();
    let view = {
        let guard = POMODORO.lock().expect("pomodoro state");
        guard.as_ref().map(|s| s.view(now))
    };
    let Some(view) = view else {
        return;
    };
    let lines = pomodoro_widget_lines(&view);
    set_pomodoro_projection(lines, view.progress_0_1);
    let tip = pomodoro_tray_tip(&view);
    tray_update_tip(hwnd, &tip);
}

fn persist_pomodoro_snapshot(state: &PomodoroState) {
    let path = {
        let guard = POMODORO_PATH.lock().expect("pomodoro path");
        guard.clone()
    };
    let Some(path) = path else {
        return;
    };
    match save_pomodoro(&path, state) {
        Ok(()) => {}
        Err(e) => eprintln!("solpaper: pomodoro save failed: {e}"),
    }
}

fn flush_pomodoro_to_disk() {
    let snapshot = {
        let guard = POMODORO.lock().expect("pomodoro state");
        guard.clone()
    };
    if let Some(state) = snapshot {
        persist_pomodoro_snapshot(&state);
    }
}

/// Snapshot live widgets and atomically write `layout.json` when a path is configured.
///
/// Writes last-known geometry on Edit→Normal and shutdown. On failure, logs and keeps
/// in-memory geometry (no crash). Skips when not dirty and widgets already match disk
/// only when there is nothing to snapshot.
fn flush_layout_to_disk() {
    let path = {
        let guard = LAYOUT_PATH.lock().expect("layout path");
        guard.clone()
    };
    let Some(path) = path else {
        return;
    };
    let snaps = snapshot_widget_rects();
    if snaps.is_empty() {
        return;
    }
    let mut set = WidgetLayoutSet::new_empty();
    for (id, rect, opacity) in snaps {
        match WidgetId::new(id.as_str()) {
            Ok(wid) => match WidgetLayoutEntry::from_top_left_rect(
                wid,
                rect.x,
                rect.y,
                rect.width,
                rect.height,
                opacity,
            ) {
                Ok(entry) => set.widgets.push(entry),
                Err(e) => {
                    eprintln!("solpaper: skip layout entry {id}: {e}");
                }
            },
            Err(e) => eprintln!("solpaper: skip layout id {id}: {e}"),
        }
    }
    if set.widgets.is_empty() {
        return;
    }
    match save_layout(&path, &set) {
        Ok(()) => {
            clear_layout_dirty();
            eprintln!("solpaper: layout saved ({} widget(s))", set.widgets.len());
        }
        Err(e) => eprintln!("solpaper: layout save failed: {e}"),
    }
}

unsafe fn register_toggle_edit_hotkey(hwnd: HWND) -> Result<(), RuntimeError> {
    // Pack #34 DEFAULT: Ctrl+Alt+F2 toggles Edit Mode (global, always registered).
    let mods = HOT_KEY_MODIFIERS(MOD_CONTROL.0 | MOD_ALT.0 | MOD_NOREPEAT.0);
    RegisterHotKey(hwnd, HOTKEY_TOGGLE_EDIT, mods, VK_F2.0 as u32).map_err(RuntimeError::from)?;
    Ok(())
}

unsafe fn register_escape_hotkey(hwnd: HWND) {
    if ESCAPE_HOTKEY_REGISTERED.swap(true, Ordering::SeqCst) {
        return;
    }
    // Bare Escape only while Edit Mode is active (keyboard map: exit Edit).
    let mods = HOT_KEY_MODIFIERS(MOD_NOREPEAT.0);
    if RegisterHotKey(hwnd, HOTKEY_ESCAPE_EDIT, mods, VK_ESCAPE.0 as u32).is_err() {
        ESCAPE_HOTKEY_REGISTERED.store(false, Ordering::SeqCst);
        eprintln!("solpaper: Escape hotkey unavailable; use tray Edit Mode to exit");
    }
}

unsafe fn unregister_escape_hotkey(hwnd: HWND) {
    if !ESCAPE_HOTKEY_REGISTERED.swap(false, Ordering::SeqCst) {
        return;
    }
    let _ = UnregisterHotKey(hwnd, HOTKEY_ESCAPE_EDIT);
}

unsafe fn unregister_all_hotkeys(hwnd: HWND) {
    let _ = UnregisterHotKey(hwnd, HOTKEY_TOGGLE_EDIT);
    unregister_escape_hotkey(hwnd);
}

fn apply_edit_mode(hwnd: HWND, mode: SurfaceMode) {
    let prev = surface_mode();
    set_surface_mode(mode);
    unsafe {
        if mode.is_edit() {
            register_escape_hotkey(hwnd);
        } else {
            unregister_escape_hotkey(hwnd);
        }
    }
    // Atomic layout persistence: flush when leaving Edit Mode (tray, hotkey, Escape).
    if prev.is_edit() && !mode.is_edit() {
        flush_layout_to_disk();
    }
}

fn toggle_edit_from_user(hwnd: HWND) {
    let next = surface_mode().toggle();
    apply_edit_mode(hwnd, next);
}

fn wide_z(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

unsafe fn ensure_control_class(
    hinstance: windows::Win32::Foundation::HINSTANCE,
) -> Result<(), RuntimeError> {
    if CONTROL_CLASS_REGISTERED.swap(true, Ordering::SeqCst) {
        return Ok(());
    }
    let class = wide_z(CONTROL_WINDOW_CLASS);
    let wc = WNDCLASSW {
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(control_wnd_proc),
        hInstance: hinstance,
        hCursor: LoadCursorW(None, IDC_ARROW)?,
        lpszClassName: PCWSTR(class.as_ptr()),
        ..Default::default()
    };
    if RegisterClassW(&wc) == 0 {
        // Already registered in this process is OK.
        let err = windows::core::Error::from_win32();
        if err.code().is_err() {
            // Best-effort: continue if class exists.
        }
    }
    Ok(())
}

fn tray_command_id(cmd: TrayCommand) -> u16 {
    MENU_ID_BASE
        + match cmd {
            TrayCommand::OpenSettings => 0,
            TrayCommand::ToggleEditMode => 1,
            TrayCommand::PomodoroStartPauseResume => 2,
            TrayCommand::PomodoroSkip => 3,
            TrayCommand::PomodoroReset => 4,
            TrayCommand::WallpaperNext => 5,
            TrayCommand::WallpaperHold => 6,
            TrayCommand::ToggleAutostart => 7,
            TrayCommand::OpenDiagnostics => 8,
            TrayCommand::Quit => 9,
        }
}

fn command_from_menu_id(id: u16) -> Option<TrayCommand> {
    if id < MENU_ID_BASE {
        return None;
    }
    Some(match id - MENU_ID_BASE {
        0 => TrayCommand::OpenSettings,
        1 => TrayCommand::ToggleEditMode,
        2 => TrayCommand::PomodoroStartPauseResume,
        3 => TrayCommand::PomodoroSkip,
        4 => TrayCommand::PomodoroReset,
        5 => TrayCommand::WallpaperNext,
        6 => TrayCommand::WallpaperHold,
        7 => TrayCommand::ToggleAutostart,
        8 => TrayCommand::OpenDiagnostics,
        9 => TrayCommand::Quit,
        _ => return None,
    })
}

unsafe fn tray_add(hwnd: HWND) -> Result<(), RuntimeError> {
    let mut nid = NOTIFYICONDATAW {
        cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
        hWnd: hwnd,
        uID: 1,
        uFlags: NIF_MESSAGE | NIF_ICON | NIF_TIP,
        uCallbackMessage: WM_TRAYICON,
        Anonymous: NOTIFYICONDATAW_0 {
            uVersion: NOTIFYICON_VERSION_4,
        },
        ..Default::default()
    };

    // Scaffold icon: standard application icon until product resource lands.
    let icon: HICON = LoadIconW(None, IDI_APPLICATION)?;
    nid.hIcon = icon;

    let tip = "Solpaper";
    let tip_wide: Vec<u16> = tip.encode_utf16().chain(std::iter::once(0)).collect();
    let copy_len = (nid.szTip.len() - 1).min(tip_wide.len());
    nid.szTip[..copy_len].copy_from_slice(&tip_wide[..copy_len]);

    if !Shell_NotifyIconW(NIM_ADD, &nid).as_bool() {
        return Err(RuntimeError::Message(
            "Shell_NotifyIconW NIM_ADD failed".into(),
        ));
    }
    // Prefer V4 behavior when available.
    let _ = Shell_NotifyIconW(NIM_SETVERSION, &nid);
    Ok(())
}

unsafe fn tray_delete(hwnd: HWND) -> Result<(), RuntimeError> {
    let nid = NOTIFYICONDATAW {
        cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
        hWnd: hwnd,
        uID: 1,
        ..Default::default()
    };
    let _ = Shell_NotifyIconW(NIM_DELETE, &nid);
    Ok(())
}

fn copy_wide_fixed(dst: &mut [u16], text: &str) {
    let wide: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
    let n = (dst.len() - 1).min(wide.len());
    dst[..n].copy_from_slice(&wide[..n]);
    if n < dst.len() {
        dst[n] = 0;
    }
}

/// Update tray tooltip (NIF_TIP only). Safe to call from UI thread.
fn tray_update_tip(hwnd: HWND, tip: &str) {
    let mut nid = NOTIFYICONDATAW {
        cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
        hWnd: hwnd,
        uID: 1,
        uFlags: NIF_TIP,
        ..Default::default()
    };
    copy_wide_fixed(&mut nid.szTip, tip);
    let _ = unsafe { Shell_NotifyIconW(NIM_MODIFY, &nid) };
}

/// Show a tray balloon for phase completion (`NIF_INFO` / pack #7 DEFAULT).
fn tray_balloon(hwnd: HWND, title: &str, body: &str) {
    let mut nid = NOTIFYICONDATAW {
        cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
        hWnd: hwnd,
        uID: 1,
        uFlags: NIF_INFO,
        ..Default::default()
    };
    // NOTIFYICONDATAW_0 union: uTimeout / uVersion — for balloons set timeout ms.
    nid.Anonymous = NOTIFYICONDATAW_0 { uTimeout: 8_000 };
    nid.dwInfoFlags = NIIF_INFO;
    copy_wide_fixed(&mut nid.szInfoTitle, title);
    copy_wide_fixed(&mut nid.szInfo, body);
    let ok = unsafe { Shell_NotifyIconW(NIM_MODIFY, &nid) };
    if !ok.as_bool() {
        eprintln!("solpaper: tray balloon failed");
    }
}

unsafe fn show_tray_menu(hwnd: HWND) -> Result<(), RuntimeError> {
    if !ACCEPTING_WORK.load(Ordering::SeqCst) {
        return Ok(());
    }
    let available = {
        let guard = POMODORO.lock().expect("pomodoro state");
        guard
            .as_ref()
            .map(|s| s.view(now_utc_ms()).available)
            .unwrap_or(solpaper_core::AvailableActions {
                start: false,
                pause: false,
                resume: false,
                skip: false,
                reset: false,
            })
    };
    let menu = build_tray_menu(alpha1_wallpaper_flags(), Some(available));
    let hmenu = CreatePopupMenu()?;
    for entry in &menu {
        match entry {
            TrayMenuEntry::Separator => {
                let _ = AppendMenuW(hmenu, MF_SEPARATOR, 0, PCWSTR::null());
            }
            TrayMenuEntry::Command {
                command,
                enabled,
                label,
            } => {
                let id = tray_command_id(*command) as usize;
                let wide = wide_z(label);
                let flags = if *enabled {
                    MF_STRING | MF_ENABLED
                } else {
                    MF_STRING | MF_GRAYED | MF_DISABLED
                };
                let _ = AppendMenuW(hmenu, flags, id, PCWSTR(wide.as_ptr()));
            }
        }
    }
    // Quit as default when double-click is not used.
    let _ = SetMenuDefaultItem(hmenu, tray_command_id(TrayCommand::OpenSettings) as u32, 0);

    let mut pt = POINT::default();
    let _ = GetCursorPos(&mut pt);
    let _ = SetForegroundWindow(hwnd);
    let _ = TrackPopupMenu(
        hmenu,
        TPM_LEFTALIGN | TPM_BOTTOMALIGN | TPM_RIGHTBUTTON,
        pt.x,
        pt.y,
        0,
        hwnd,
        None,
    );
    // Required so the menu dismisses correctly on click outside.
    let _ = PostMessageW(hwnd, WM_NULL, WPARAM(0), LPARAM(0));
    let _ = DestroyMenu(hmenu);
    Ok(())
}

fn handle_tray_command(hwnd: HWND, cmd: TrayCommand) {
    if !ACCEPTING_WORK.load(Ordering::SeqCst) && cmd != TrayCommand::Quit {
        return;
    }
    match cmd {
        TrayCommand::Quit => {
            ACCEPTING_WORK.store(false, Ordering::SeqCst);
            flush_layout_to_disk();
            flush_pomodoro_to_disk();
            unsafe {
                let _ = KillTimer(hwnd, TIMER_POMODORO_LIVE);
                unregister_all_hotkeys(hwnd);
                destroy_all_widgets();
                let _ = tray_delete(hwnd);
                let _ = DestroyWindow(hwnd);
            }
        }
        TrayCommand::OpenSettings => {
            // Lazy settings window lands in a later #20 bullet; record request.
            SETTINGS_REQUESTED.store(true, Ordering::SeqCst);
            eprintln!("solpaper: Open Settings (host UI deferred)");
        }
        TrayCommand::OpenDiagnostics => {
            open_diagnostics_ui();
        }
        TrayCommand::ToggleEditMode => {
            toggle_edit_from_user(hwnd);
        }
        TrayCommand::PomodoroStartPauseResume
        | TrayCommand::PomodoroSkip
        | TrayCommand::PomodoroReset => {
            apply_pomodoro_tray_command(cmd);
        }
        TrayCommand::WallpaperNext => wallpaper_next(),
        TrayCommand::WallpaperHold => wallpaper_toggle_hold(),
        // Deferred: autostart tracer / installed-build only.
        TrayCommand::ToggleAutostart => {}
    }
}

unsafe extern "system" fn control_wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    let taskbar_created = TASKBAR_CREATED_MSG.load(Ordering::SeqCst);
    if taskbar_created != 0 && msg == taskbar_created {
        // Explorer restarted: re-add tray only (do not recreate widget HWNDs).
        let _ = tray_delete(hwnd);
        let _ = tray_add(hwnd);
        return LRESULT(0);
    }

    match msg {
        m if m == WM_TRAYICON => {
            let mouse = lparam.0 as u32;
            // NOTIFYICON_VERSION_4 packs differently; handle classic LOWORD for scaffold.
            let event = mouse & 0xffff;
            if event == WM_RBUTTONUP || event == 0x0205 {
                // WM_RBUTTONUP = 0x0205
                let _ = show_tray_menu(hwnd);
            } else if event == 0x0203 {
                // WM_LBUTTONDBLCLK — open settings request
                handle_tray_command(hwnd, TrayCommand::OpenSettings);
            }
            LRESULT(0)
        }
        WM_COMMAND => {
            let id = (wparam.0 as u32) & 0xffff;
            if let Some(cmd) = command_from_menu_id(id as u16) {
                handle_tray_command(hwnd, cmd);
            }
            LRESULT(0)
        }
        m if m == WM_APP_SHOW_SETTINGS => {
            SETTINGS_REQUESTED.store(true, Ordering::SeqCst);
            eprintln!("solpaper: show settings requested (second launch)");
            LRESULT(0)
        }
        WM_HOTKEY => {
            let id = wparam.0 as i32;
            match id {
                HOTKEY_TOGGLE_EDIT => toggle_edit_from_user(hwnd),
                // Escape exits Edit Mode only (does not quit the app).
                HOTKEY_ESCAPE_EDIT if surface_mode().is_edit() => {
                    apply_edit_mode(hwnd, SurfaceMode::Normal);
                }
                _ => {}
            }
            LRESULT(0)
        }
        WM_TIMER => {
            if wparam.0 == TIMER_POMODORO_LIVE && ACCEPTING_WORK.load(Ordering::SeqCst) {
                pomodoro_live_tick();
            }
            LRESULT(0)
        }
        WM_DESTROY => {
            ACCEPTING_WORK.store(false, Ordering::SeqCst);
            flush_layout_to_disk();
            flush_pomodoro_to_disk();
            let _ = KillTimer(hwnd, TIMER_POMODORO_LIVE);
            unregister_all_hotkeys(hwnd);
            destroy_all_widgets();
            let _ = tray_delete(hwnd);
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

unsafe fn pump_peek(max: u32) {
    let mut msg = MSG::default();
    for _ in 0..max {
        if !PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).as_bool() {
            break;
        }
        if msg.message == WM_QUIT {
            break;
        }
        let _ = TranslateMessage(&msg);
        DispatchMessageW(&msg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn menu_ids_round_trip() {
        for cmd in [
            TrayCommand::OpenSettings,
            TrayCommand::Quit,
            TrayCommand::ToggleAutostart,
            TrayCommand::WallpaperNext,
        ] {
            let id = tray_command_id(cmd);
            assert_eq!(command_from_menu_id(id), Some(cmd));
        }
    }

    #[test]
    fn control_class_name_matches_core() {
        assert_eq!(CONTROL_WINDOW_CLASS, "Solpaper.Runtime.Control.v1");
        assert_eq!(WM_TRAYICON, WM_APP + 2);
    }
}
