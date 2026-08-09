//! Observability policy primitives (Issue #40).
//!
//! Pure, platform-neutral rules for structured logging fields, path redaction,
//! log rotation budgets, crash-loop / safe-mode detection, and diagnostic-bundle
//! exclusion. Runtime wiring (`tracing`, file appenders, UI) lives elsewhere.

use crate::CoreError;

/// UTC milliseconds since Unix epoch (injectable for tests). Same unit as Pomodoro.
pub type UnixMs = i64;

// --- Log volume budgets (NFR PERF-LOG-*; blueprint #40) ---------------------

/// Maximum bytes per rotated log file.
pub const LOG_FILE_MAX_BYTES: u64 = 2 * 1024 * 1024;
/// Maximum number of retained log files.
pub const LOG_FILE_MAX_COUNT: usize = 5;
/// Hard cap on total log bytes (files × size).
pub const LOG_TOTAL_MAX_BYTES: u64 = 10 * 1024 * 1024;
/// Maximum age of a retained log file in whole days.
pub const LOG_RETENTION_DAYS: u32 = 14;
/// Maximum log-tail bytes included in a diagnostic bundle (bounded).
pub const BUNDLE_LOG_TAIL_MAX_BYTES: u64 = 512 * 1024;

// --- Crash-loop / safe-mode (PERF-REL-04; blueprint #40) --------------------

/// Sliding window for startup crash counting.
pub const CRASH_LOOP_WINDOW_MS: UnixMs = 5 * 60 * 1000;
/// Number of startup crash markers within the window that recommend safe mode.
pub const CRASH_LOOP_THRESHOLD: usize = 3;

/// Logical product components for structured events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Component {
    Runtime,
    Tray,
    Surface,
    Layout,
    Pomodoro,
    Wallpaper,
    Calendar,
    Auth,
    Storage,
    Migration,
    Diagnostics,
}

impl Component {
    /// Stable snake_case token for log fields and counters.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Runtime => "runtime",
            Self::Tray => "tray",
            Self::Surface => "surface",
            Self::Layout => "layout",
            Self::Pomodoro => "pomodoro",
            Self::Wallpaper => "wallpaper",
            Self::Calendar => "calendar",
            Self::Auth => "auth",
            Self::Storage => "storage",
            Self::Migration => "migration",
            Self::Diagnostics => "diagnostics",
        }
    }

    /// Parse a component token (case-insensitive).
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "runtime" => Some(Self::Runtime),
            "tray" => Some(Self::Tray),
            "surface" => Some(Self::Surface),
            "layout" => Some(Self::Layout),
            "pomodoro" => Some(Self::Pomodoro),
            "wallpaper" => Some(Self::Wallpaper),
            "calendar" => Some(Self::Calendar),
            "auth" => Some(Self::Auth),
            "storage" => Some(Self::Storage),
            "migration" => Some(Self::Migration),
            "diagnostics" => Some(Self::Diagnostics),
            _ => None,
        }
    }
}

/// Stable high-level failure categories (network vs auth vs parse, etc.).
///
/// Specific codes (e.g. `CalendarHttp`) map into one of these for UX and
/// diagnostics without private payloads.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCategory {
    /// Transport, DNS, TLS, timeout, offline.
    Network,
    /// OAuth, tokens, reconnect required.
    Auth,
    /// JSON/schema/normalize failures.
    Parse,
    /// Local filesystem / settings / layout / DB IO.
    Storage,
    /// Provider or API policy (rate limit, too large, feature disabled).
    ProviderPolicy,
    /// Desktop surface / HWND / tray host failures.
    Surface,
    /// Layout geometry / monitor binding.
    Layout,
    /// Pomodoro domain / timer host.
    Pomodoro,
    /// Schema migration.
    Migration,
    /// Corrupt or invalid configuration.
    Config,
    /// Unexpected internal / panic path (redacted).
    Internal,
}

impl ErrorCategory {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Network => "network",
            Self::Auth => "auth",
            Self::Parse => "parse",
            Self::Storage => "storage",
            Self::ProviderPolicy => "provider_policy",
            Self::Surface => "surface",
            Self::Layout => "layout",
            Self::Pomodoro => "pomodoro",
            Self::Migration => "migration",
            Self::Config => "config",
            Self::Internal => "internal",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "network" => Some(Self::Network),
            "auth" => Some(Self::Auth),
            "parse" => Some(Self::Parse),
            "storage" => Some(Self::Storage),
            "provider_policy" | "provider-policy" => Some(Self::ProviderPolicy),
            "surface" => Some(Self::Surface),
            "layout" => Some(Self::Layout),
            "pomodoro" => Some(Self::Pomodoro),
            "migration" => Some(Self::Migration),
            "config" => Some(Self::Config),
            "internal" => Some(Self::Internal),
            _ => None,
        }
    }
}

/// Map a stable typed error code string into a category.
///
/// Codes use `SCREAMING_SNAKE` or `PascalCase` prefixes already used in the
/// security control matrix (e.g. `OAuthStateMismatch`, `CalendarHttp`).
pub fn categorize_error_code(code: &str) -> ErrorCategory {
    let c = code.to_ascii_lowercase();
    if c.contains("oauth") || c.contains("reconnect") || c.contains("credential") {
        return ErrorCategory::Auth;
    }
    if c.contains("calendar") && (c.contains("http") || c.contains("timeout") || c.contains("dns"))
    {
        return ErrorCategory::Network;
    }
    if c.starts_with("provider")
        && (c.contains("http") || c.contains("timeout") || c.contains("download"))
    {
        return ErrorCategory::Network;
    }
    if c.contains("parse") || c.contains("schema") || c.contains("json") {
        return ErrorCategory::Parse;
    }
    if c.contains("too_large")
        || c.contains("toolarge")
        || c.contains("rate")
        || c.contains("quota")
        || c.contains("policy")
        || c.contains("disabled")
        || c.contains("redirect_rejected")
    {
        return ErrorCategory::ProviderPolicy;
    }
    if c.contains("migration") {
        return ErrorCategory::Migration;
    }
    if c.contains("config")
        || c.contains("settings_corrupt")
        || c.contains("settingscorrupt")
        || c.contains("layoutcorrupt")
        || c.contains("pomodorocorrupt")
    {
        return ErrorCategory::Config;
    }
    if c.contains("layout") || c.contains("offscreen") {
        return ErrorCategory::Layout;
    }
    // Wallpaper domain codes (see WallpaperErrorKind::as_error_code).
    if c.starts_with("wallpaper") {
        if c.contains("platform") || c.contains("monitor") {
            return ErrorCategory::Surface;
        }
        if c.contains("upscale") || c.contains("policy") {
            return ErrorCategory::ProviderPolicy;
        }
        if c.contains("internal") {
            return ErrorCategory::Internal;
        }
        // Path/format/size/decode → storage-class local IO.
        return ErrorCategory::Storage;
    }
    if c.contains("monitor") {
        return ErrorCategory::Layout;
    }
    if c.contains("pomodoro") || c.contains("timer") {
        return ErrorCategory::Pomodoro;
    }
    if c.contains("surface")
        || c.contains("hwnd")
        || c.contains("tray")
        || c.contains("dpi")
        || c.contains("compositor")
    {
        return ErrorCategory::Surface;
    }
    if c.contains("storage")
        || c.contains("io")
        || c.contains("cache")
        || c.contains("disk")
        || c.contains("localappdata")
    {
        return ErrorCategory::Storage;
    }
    if c.contains("http") || c.contains("timeout") || c.contains("tls") || c.contains("offline") {
        return ErrorCategory::Network;
    }
    if c.contains("panic") || c.contains("internal") || c.contains("bug") {
        return ErrorCategory::Internal;
    }
    ErrorCategory::Internal
}

// --- Structured field allowlist (PERF-LOG-04; AC-LOG-01) --------------------

/// Field names that may appear on structured log events by default.
///
/// Construction is allowlist-based: unknown keys must not be logged.
pub const ALLOWED_LOG_FIELDS: &[&str] = &[
    "timestamp",
    "level",
    "component",
    "event",
    "correlation_id",
    "error_category",
    "error_code",
    "os_error",
    "http_status",
    "duration_ms",
    "count",
    "retry_attempt",
    "safe_mode",
    "schema_version",
    "build_sha",
    "config_version",
    "monitor_count",
    "widget_count",
    "phase",
    "timer_status",
    "startup_id",
    "sync_id",
    "wallpaper_cycle_id",
    "migration_id",
    "host",
    "path_kind",
    "redacted_path",
    "message",
];

/// Field names that must never appear on structured logs or default diagnostics.
pub const FORBIDDEN_LOG_FIELDS: &[&str] = &[
    "event_title",
    "title",
    "description",
    "location",
    "attendee",
    "attendees",
    "oauth_url",
    "callback_url",
    "authorization_code",
    "code",
    "state",
    "code_verifier",
    "verifier",
    "access_token",
    "refresh_token",
    "token",
    "password",
    "client_secret",
    "credential_target_contents",
    "full_path",
    "user_path",
    "home_path",
    "email",
    "account_email",
];

/// Returns true when `field` is on the default structured-log allowlist.
pub fn is_allowed_log_field(field: &str) -> bool {
    let f = field.trim();
    if f.is_empty() {
        return false;
    }
    ALLOWED_LOG_FIELDS.iter().any(|a| a.eq_ignore_ascii_case(f))
}

/// Returns true when `field` is explicitly forbidden (privacy / secret).
pub fn is_forbidden_log_field(field: &str) -> bool {
    let f = field.trim();
    if f.is_empty() {
        return false;
    }
    FORBIDDEN_LOG_FIELDS
        .iter()
        .any(|a| a.eq_ignore_ascii_case(f))
}

/// Validate a structured field set for logging.
///
/// - Every key must be allowlisted.
/// - No key may be on the forbidden list (defense in depth for aliases).
pub fn validate_log_fields(fields: &[&str]) -> Result<(), CoreError> {
    for f in fields {
        if is_forbidden_log_field(f) {
            return Err(CoreError::InvalidDiagnostics("forbidden log field"));
        }
        if !is_allowed_log_field(f) {
            return Err(CoreError::InvalidDiagnostics("unknown log field"));
        }
    }
    Ok(())
}

// --- Path redaction ---------------------------------------------------------

/// Redact personal directory segments in a path for logs/bundles.
///
/// Replaces the first path segment after `Users` / `home` with `<redacted>`.
/// Does not attempt full OS path canonicalization (platform code does that).
pub fn redact_user_path(path: &str) -> String {
    if path.is_empty() {
        return String::new();
    }
    let normalized = path.replace('\\', "/");
    let mut parts: Vec<&str> = normalized.split('/').collect();
    let mut i = 0;
    while i + 1 < parts.len() {
        let seg = parts[i];
        if seg.eq_ignore_ascii_case("Users")
            || seg.eq_ignore_ascii_case("home")
            || seg.eq_ignore_ascii_case("Documents and Settings")
        {
            parts[i + 1] = "<redacted>";
            break;
        }
        i += 1;
    }
    let joined = parts.join("/");
    // Preserve a Windows drive-style root if the original used backslashes.
    if path.contains('\\') {
        joined.replace('/', "\\")
    } else {
        joined
    }
}

// --- Bundle exclusion (AC-LOG-02; PERF-LOG-03) ------------------------------

/// Basename patterns that must not be packed into a diagnostic zip by default.
pub fn is_forbidden_bundle_entry_name(name: &str) -> bool {
    let n = name.trim().to_ascii_lowercase();
    if n.is_empty() {
        return true;
    }
    // Raw secrets / private stores
    if n.contains("token")
        || n.contains("credential")
        || n.ends_with(".db")
        || n.ends_with(".sqlite")
        || n.ends_with(".sqlite3")
        || n.contains("oauth")
        || n.contains("screenshot")
        || n.ends_with(".png")
        || n.ends_with(".jpg")
        || n.ends_with(".jpeg")
        || n.ends_with(".webp")
    {
        return true;
    }
    // Calendar cache dumps
    if n.contains("calendar")
        && (n.contains("cache") || n.contains("event") || n.contains("agenda"))
    {
        return true;
    }
    false
}

// --- Log rotation policy ----------------------------------------------------

/// Decision helper: whether a candidate log file should be deleted under policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LogFileMeta {
    /// File size in bytes.
    pub size_bytes: u64,
    /// Age in whole days (floor).
    pub age_days: u32,
}

/// Given files newest-first, return how many trailing files to drop so that
/// count ≤ [`LOG_FILE_MAX_COUNT`], total size ≤ [`LOG_TOTAL_MAX_BYTES`], and
/// no file exceeds [`LOG_RETENTION_DAYS`].
///
/// Returns indices (into the input slice) that violate policy and should be removed.
pub fn log_files_to_delete(files_newest_first: &[LogFileMeta]) -> Vec<usize> {
    let mut delete = Vec::new();
    let mut kept: Vec<(usize, LogFileMeta)> = Vec::new();

    for (idx, meta) in files_newest_first.iter().enumerate() {
        if meta.age_days > LOG_RETENTION_DAYS {
            delete.push(idx);
            continue;
        }
        kept.push((idx, *meta));
    }

    // Enforce count: drop oldest first (end of newest-first list).
    while kept.len() > LOG_FILE_MAX_COUNT {
        if let Some((idx, _)) = kept.pop() {
            delete.push(idx);
        }
    }

    // Enforce total size: drop oldest until under cap.
    let mut total: u64 = kept.iter().map(|(_, m)| m.size_bytes).sum();
    while total > LOG_TOTAL_MAX_BYTES {
        if let Some((idx, meta)) = kept.pop() {
            total = total.saturating_sub(meta.size_bytes);
            delete.push(idx);
        } else {
            break;
        }
    }

    delete.sort_unstable();
    delete.dedup();
    delete
}

/// Whether a single write of `additional_bytes` would exceed the per-file cap
/// and therefore requires rotation before append.
pub fn needs_rotation_before_write(current_file_bytes: u64, additional_bytes: u64) -> bool {
    current_file_bytes.saturating_add(additional_bytes) > LOG_FILE_MAX_BYTES
}

// --- Crash loop / safe mode -------------------------------------------------

/// Count crash markers whose timestamps fall in `[now - window, now]`.
pub fn count_crashes_in_window(
    crash_times_ms: &[UnixMs],
    now_ms: UnixMs,
    window_ms: UnixMs,
) -> usize {
    if window_ms < 0 {
        return 0;
    }
    let start = now_ms.saturating_sub(window_ms);
    crash_times_ms
        .iter()
        .filter(|&&t| t >= start && t <= now_ms)
        .count()
}

/// True when ≥ [`CRASH_LOOP_THRESHOLD`] startup crashes occurred within
/// [`CRASH_LOOP_WINDOW_MS`] ending at `now_ms`.
pub fn should_recommend_safe_mode(crash_times_ms: &[UnixMs], now_ms: UnixMs) -> bool {
    count_crashes_in_window(crash_times_ms, now_ms, CRASH_LOOP_WINDOW_MS) >= CRASH_LOOP_THRESHOLD
}

/// Capabilities disabled while safe mode is active (blueprint #40).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SafeModePolicy {
    pub widgets_enabled: bool,
    pub calendar_enabled: bool,
    pub remote_provider_enabled: bool,
    pub autostart_mutation_enabled: bool,
    pub settings_enabled: bool,
    pub diagnostics_enabled: bool,
}

impl SafeModePolicy {
    /// Restricted launch: no widgets/Calendar/provider/autostart mutation;
    /// settings and diagnostics remain available.
    pub const RESTRICTED: Self = Self {
        widgets_enabled: false,
        calendar_enabled: false,
        remote_provider_enabled: false,
        autostart_mutation_enabled: false,
        settings_enabled: true,
        diagnostics_enabled: true,
    };

    /// Normal operation.
    pub const NORMAL: Self = Self {
        widgets_enabled: true,
        calendar_enabled: true,
        remote_provider_enabled: true,
        autostart_mutation_enabled: true,
        settings_enabled: true,
        diagnostics_enabled: true,
    };
}

/// v1: no automatic endless restart after crash (blueprint #40).
pub const AUTO_RESTART_ON_CRASH: bool = false;
/// v1: no separate watchdog process.
pub const WATCHDOG_PROCESS: bool = false;
/// v1: no remote crash upload / telemetry.
pub const REMOTE_CRASH_UPLOAD: bool = false;
/// v1: no product telemetry.
pub const TELEMETRY_ENABLED: bool = false;

// --- Correlation ID kinds ---------------------------------------------------

/// Correlation scopes that must mint a fresh ID at start of the operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CorrelationScope {
    Startup,
    CalendarSync,
    WallpaperCycle,
    Migration,
}

impl CorrelationScope {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Startup => "startup",
            Self::CalendarSync => "calendar_sync",
            Self::WallpaperCycle => "wallpaper_cycle",
            Self::Migration => "migration",
        }
    }
}

// --- Diagnostics / status baseline (Issue #20 bullet 7 / OPS-A1-*) ----------

/// Maximum active error rows retained for the Diagnostics surface.
pub const MAX_ACTIVE_ERRORS: usize = 16;

/// Opaque correlation id (hex, not derived from user content).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CorrelationId(String);

impl CorrelationId {
    /// Mint a fresh opaque id from injectable entropy bits.
    ///
    /// Callers pass `now_ms`, process id, and a counter/random u64. Not a CSPRNG;
    /// only needs opacity for log correlation.
    pub fn mint(scope: CorrelationScope, now_ms: UnixMs, pid: u32, salt: u64) -> Self {
        // Mix scope tag + time + pid + salt into 64-bit then hex (no user payload).
        let mut h = 0xcbf2_9ce4_8422_2325_u64; // FNV-1a offset
        for b in scope.as_str().as_bytes() {
            h ^= u64::from(*b);
            h = h.wrapping_mul(0x100_0000_01b3);
        }
        for b in now_ms.to_le_bytes() {
            h ^= u64::from(b);
            h = h.wrapping_mul(0x100_0000_01b3);
        }
        for b in pid.to_le_bytes() {
            h ^= u64::from(b);
            h = h.wrapping_mul(0x100_0000_01b3);
        }
        for b in salt.to_le_bytes() {
            h ^= u64::from(b);
            h = h.wrapping_mul(0x100_0000_01b3);
        }
        Self(format!("{:016x}", h))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for CorrelationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// One active error for Diagnostics (codes + categories only; no private payload).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActiveError {
    pub code: String,
    pub category: ErrorCategory,
    pub component: Component,
    /// Optional short safe message (must not contain secrets/titles/paths).
    pub message: Option<&'static str>,
}

impl ActiveError {
    pub fn new(
        code: impl Into<String>,
        component: Component,
        message: Option<&'static str>,
    ) -> Self {
        let code = code.into();
        let category = categorize_error_code(&code);
        Self {
            code,
            category,
            component,
            message,
        }
    }
}

/// Last startup record for Diagnostics UI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartupRecord {
    pub at_ms: UnixMs,
    pub ok: bool,
    pub correlation_id: CorrelationId,
}

/// Kind of last wallpaper cycle (no full path).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WallpaperCycleKind {
    None,
    Local,
    Remote,
}

impl WallpaperCycleKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Local => "local",
            Self::Remote => "remote",
        }
    }
}

/// Last wallpaper cycle for Diagnostics (kind + outcome only).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WallpaperCycleRecord {
    pub at_ms: UnixMs,
    pub kind: WallpaperCycleKind,
    pub ok: bool,
    pub error_code: Option<String>,
    pub correlation_id: Option<CorrelationId>,
}

/// User-facing recovery action tokens (Diagnostics UI / status text).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RecoveryAction {
    /// Recreate widget surfaces and clamp off-screen.
    RecreateSurfaces,
    /// Enter Edit Mode for layout recovery.
    OpenEditMode,
    /// Restart the app once (single-instance / tray recovery).
    RestartApp,
    /// Recommend safe mode (crash loop).
    EnterSafeMode,
    /// Re-scan local wallpaper folders.
    RescanWallpapers,
    /// Export a privacy-safe diagnostic bundle (user-initiated; format later).
    ExportBundle,
}

impl RecoveryAction {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RecreateSurfaces => "recreate_surfaces",
            Self::OpenEditMode => "open_edit_mode",
            Self::RestartApp => "restart_app",
            Self::EnterSafeMode => "enter_safe_mode",
            Self::RescanWallpapers => "rescan_wallpapers",
            Self::ExportBundle => "export_bundle",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::RecreateSurfaces => "Recreate widgets / clamp off-screen",
            Self::OpenEditMode => "Open Edit Mode",
            Self::RestartApp => "Restart Solpaper once",
            Self::EnterSafeMode => "Continue in safe mode",
            Self::RescanWallpapers => "Re-scan local wallpaper folders",
            Self::ExportBundle => "Export diagnostic bundle (user-initiated)",
        }
    }

    /// Actions the Runtime may execute automatically after user consent (Alpha 1).
    ///
    /// RestartApp / EnterSafeMode / ExportBundle remain user-driven (quit + relaunch,
    /// already-in-safe-mode, or deferred bundle zip).
    pub fn is_runtime_executable(self) -> bool {
        matches!(
            self,
            Self::RecreateSurfaces | Self::OpenEditMode | Self::RescanWallpapers
        )
    }
}

/// Ordered recovery steps the Runtime will run when the user confirms recovery.
///
/// Safe mode never recreates widgets. Wallpaper rescan remains available when
/// wallpaper errors exist or when a full (non–safe-mode) recovery runs.
pub fn runtime_recovery_plan(safe_mode: bool, has_wallpaper_error: bool) -> Vec<RecoveryAction> {
    let mut plan = Vec::new();
    if !safe_mode {
        plan.push(RecoveryAction::RecreateSurfaces);
        plan.push(RecoveryAction::OpenEditMode);
        plan.push(RecoveryAction::RescanWallpapers);
    } else if has_wallpaper_error {
        plan.push(RecoveryAction::RescanWallpapers);
    }
    plan
}

/// Support counters with no private payloads.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SupportCounters {
    pub failed_syncs: u64,
    pub provider_cooldowns: u64,
    pub duplicate_prevention: u64,
    pub migrations_run: u64,
    pub migrations_failed: u64,
    pub surface_recreates: u64,
    pub safe_mode_entries: u64,
    pub wallpaper_failures: u64,
    pub storage_recoveries: u64,
}

/// Redacted path labels for Diagnostics (never raw personal paths in display helpers).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticsPathDisplay {
    pub data_dir: String,
    pub cache_dir: String,
    pub logs_dir: String,
}

/// Full Diagnostics / About status snapshot (OPS-A1-05).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticsSnapshot {
    pub version: String,
    pub build_sha: String,
    pub config_schema_version: u32,
    pub last_startup: Option<StartupRecord>,
    /// Alpha 1: always "not connected" until Calendar lands.
    pub last_calendar_sync_label: String,
    pub last_wallpaper_cycle: Option<WallpaperCycleRecord>,
    pub active_errors: Vec<ActiveError>,
    pub safe_mode: bool,
    pub safe_mode_reason: Option<&'static str>,
    pub paths: DiagnosticsPathDisplay,
    pub recovery_actions: Vec<RecoveryAction>,
    pub counters: SupportCounters,
    pub telemetry_enabled: bool,
    pub remote_crash_upload: bool,
}

impl DiagnosticsSnapshot {
    /// Build recommended recovery actions from current health.
    pub fn default_recovery_actions(
        safe_mode: bool,
        has_wallpaper_error: bool,
    ) -> Vec<RecoveryAction> {
        let mut actions = vec![
            RecoveryAction::RecreateSurfaces,
            RecoveryAction::OpenEditMode,
            RecoveryAction::RestartApp,
        ];
        if has_wallpaper_error {
            actions.push(RecoveryAction::RescanWallpapers);
        }
        if safe_mode {
            actions.insert(0, RecoveryAction::EnterSafeMode);
        }
        actions.push(RecoveryAction::ExportBundle);
        actions
    }

    /// Human-readable status for tray Diagnostics / status file (no secrets).
    pub fn format_text(&self) -> String {
        let mut out = String::with_capacity(1024);
        out.push_str("Solpaper Diagnostics\n");
        out.push_str("====================\n");
        out.push_str(&format!("Version: {}\n", self.version));
        out.push_str(&format!("Build: {}\n", self.build_sha));
        out.push_str(&format!("Config schema: {}\n", self.config_schema_version));
        match &self.last_startup {
            Some(s) => out.push_str(&format!(
                "Last startup: {} at_ms={} id={}\n",
                if s.ok { "ok" } else { "failed" },
                s.at_ms,
                s.correlation_id
            )),
            None => out.push_str("Last startup: never\n"),
        }
        out.push_str(&format!(
            "Calendar sync: {}\n",
            self.last_calendar_sync_label
        ));
        match &self.last_wallpaper_cycle {
            Some(w) => out.push_str(&format!(
                "Last wallpaper: {} {} at_ms={}{}\n",
                w.kind.as_str(),
                if w.ok { "ok" } else { "failed" },
                w.at_ms,
                w.error_code
                    .as_ref()
                    .map(|c| format!(" code={c}"))
                    .unwrap_or_default()
            )),
            None => out.push_str("Last wallpaper: never\n"),
        }
        out.push_str(&format!(
            "Safe mode: {}{}\n",
            if self.safe_mode { "yes" } else { "no" },
            self.safe_mode_reason
                .map(|r| format!(" ({r})"))
                .unwrap_or_default()
        ));
        if self.active_errors.is_empty() {
            out.push_str("Active errors: none\n");
        } else {
            out.push_str("Active errors:\n");
            for e in &self.active_errors {
                out.push_str(&format!(
                    "  - {} category={} component={}\n",
                    e.code,
                    e.category.as_str(),
                    e.component.as_str()
                ));
            }
        }
        out.push_str(&format!("Data dir: {}\n", self.paths.data_dir));
        out.push_str(&format!("Cache dir: {}\n", self.paths.cache_dir));
        out.push_str(&format!("Logs dir: {}\n", self.paths.logs_dir));
        out.push_str(&format!(
            "Telemetry: {} | Remote crash upload: {}\n",
            self.telemetry_enabled, self.remote_crash_upload
        ));
        out.push_str("Recovery actions:\n");
        for a in &self.recovery_actions {
            out.push_str(&format!("  - {} ({})\n", a.label(), a.as_str()));
        }
        out.push_str(&format!(
            "Counters: storage_recoveries={} wallpaper_failures={} surface_recreates={} safe_mode_entries={} duplicate_prevention={}\n",
            self.counters.storage_recoveries,
            self.counters.wallpaper_failures,
            self.counters.surface_recreates,
            self.counters.safe_mode_entries,
            self.counters.duplicate_prevention
        ));
        out
    }
}

/// Minimal redacted crash marker payload (no stacks with paths/tokens).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrashMarker {
    pub at_ms: UnixMs,
    pub error_code: &'static str,
    pub build_sha: String,
    pub component: Option<Component>,
}

impl CrashMarker {
    pub const INTERNAL_PANIC: &'static str = "InternalPanic";

    pub fn panic_marker(at_ms: UnixMs, build_sha: impl Into<String>) -> Self {
        Self {
            at_ms,
            error_code: Self::INTERNAL_PANIC,
            build_sha: build_sha.into(),
            component: Some(Component::Runtime),
        }
    }
}

/// Ring buffer of active errors (newest last; capped).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ActiveErrorLog {
    errors: Vec<ActiveError>,
}

impl ActiveErrorLog {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn push(&mut self, err: ActiveError) {
        // Dedupe consecutive identical codes.
        if self.errors.last().map(|e| e.code.as_str()) == Some(err.code.as_str()) {
            return;
        }
        self.errors.push(err);
        while self.errors.len() > MAX_ACTIVE_ERRORS {
            self.errors.remove(0);
        }
    }

    pub fn as_slice(&self) -> &[ActiveError] {
        &self.errors
    }

    pub fn clear(&mut self) {
        self.errors.clear();
    }

    pub fn has_wallpaper_error(&self) -> bool {
        self.errors
            .iter()
            .any(|e| e.component == Component::Wallpaper)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_budget_constants_match_nfr() {
        const {
            assert!(LOG_FILE_MAX_BYTES == 2 * 1024 * 1024);
            assert!(LOG_FILE_MAX_COUNT == 5);
            assert!(LOG_TOTAL_MAX_BYTES == 10 * 1024 * 1024);
            assert!(LOG_RETENTION_DAYS == 14);
            assert!(LOG_FILE_MAX_BYTES * LOG_FILE_MAX_COUNT as u64 == LOG_TOTAL_MAX_BYTES);
        }
    }

    #[test]
    fn allowlist_accepts_safe_fields() {
        assert!(is_allowed_log_field("error_code"));
        assert!(is_allowed_log_field("Correlation_Id"));
        assert!(validate_log_fields(&["component", "event", "error_category"]).is_ok());
    }

    #[test]
    fn allowlist_rejects_private_and_unknown() {
        assert!(is_forbidden_log_field("event_title"));
        assert!(is_forbidden_log_field("refresh_token"));
        assert!(is_forbidden_log_field("access_token"));
        assert!(is_forbidden_log_field("code_verifier"));
        assert!(!is_allowed_log_field("event_title"));
        assert!(validate_log_fields(&["event_title"]).is_err());
        assert!(validate_log_fields(&["not_a_real_field"]).is_err());
    }

    #[test]
    fn forbidden_and_allowed_are_disjoint() {
        for f in FORBIDDEN_LOG_FIELDS {
            assert!(
                !is_allowed_log_field(f),
                "forbidden field also allowlisted: {f}"
            );
        }
    }

    #[test]
    fn redact_windows_user_path() {
        let raw = r"C:\Users\alice\AppData\Local\solpaper\logs\app.log";
        let red = redact_user_path(raw);
        assert!(!red.to_ascii_lowercase().contains("alice"));
        assert!(red.contains("<redacted>"));
        assert!(red.contains("solpaper"));
    }

    #[test]
    fn redact_unix_home_path() {
        let raw = "/home/bob/.local/share/solpaper/settings.json";
        let red = redact_user_path(raw);
        assert!(!red.contains("bob"));
        assert!(red.contains("<redacted>"));
    }

    #[test]
    fn bundle_excludes_secrets_and_db() {
        assert!(is_forbidden_bundle_entry_name("refresh_token.bin"));
        assert!(is_forbidden_bundle_entry_name("runtime.sqlite"));
        assert!(is_forbidden_bundle_entry_name("oauth-debug.txt"));
        assert!(is_forbidden_bundle_entry_name("calendar-event-cache.json"));
        assert!(is_forbidden_bundle_entry_name("screen.png"));
        assert!(!is_forbidden_bundle_entry_name("manifest.json"));
        assert!(!is_forbidden_bundle_entry_name("settings.redacted.json"));
        assert!(!is_forbidden_bundle_entry_name("logs-tail.txt"));
    }

    #[test]
    fn crash_loop_three_in_five_minutes() {
        let now = 1_000_000_i64;
        let times = [now - 4 * 60 * 1000, now - 2 * 60 * 1000, now - 30_000];
        assert!(should_recommend_safe_mode(&times, now));
        assert_eq!(
            count_crashes_in_window(&times, now, CRASH_LOOP_WINDOW_MS),
            3
        );
    }

    #[test]
    fn crash_loop_ignores_old_markers() {
        let now = 1_000_000_i64;
        let times = [
            now - 10 * 60 * 1000,
            now - 9 * 60 * 1000,
            now - 8 * 60 * 1000,
            now - 60_000,
        ];
        assert!(!should_recommend_safe_mode(&times, now));
        assert_eq!(
            count_crashes_in_window(&times, now, CRASH_LOOP_WINDOW_MS),
            1
        );
    }

    #[test]
    fn two_crashes_do_not_trigger_safe_mode() {
        let now = 500_000_i64;
        let times = [now - 60_000, now - 10_000];
        assert!(!should_recommend_safe_mode(&times, now));
    }

    #[test]
    fn safe_mode_policy_restricted() {
        let p = SafeModePolicy::RESTRICTED;
        assert!(!p.widgets_enabled);
        assert!(!p.calendar_enabled);
        assert!(!p.remote_provider_enabled);
        assert!(!p.autostart_mutation_enabled);
        assert!(p.settings_enabled);
        assert!(p.diagnostics_enabled);
    }

    #[test]
    fn v1_no_telemetry_or_watchdog() {
        const {
            assert!(!AUTO_RESTART_ON_CRASH);
            assert!(!WATCHDOG_PROCESS);
            assert!(!REMOTE_CRASH_UPLOAD);
            assert!(!TELEMETRY_ENABLED);
        }
    }

    #[test]
    fn rotation_deletes_over_age_and_count() {
        let files = [
            LogFileMeta {
                size_bytes: 100,
                age_days: 1,
            },
            LogFileMeta {
                size_bytes: 100,
                age_days: 2,
            },
            LogFileMeta {
                size_bytes: 100,
                age_days: 3,
            },
            LogFileMeta {
                size_bytes: 100,
                age_days: 4,
            },
            LogFileMeta {
                size_bytes: 100,
                age_days: 5,
            },
            LogFileMeta {
                size_bytes: 100,
                age_days: 6,
            }, // 6th file → count
            LogFileMeta {
                size_bytes: 100,
                age_days: 20,
            }, // over retention
        ];
        let del = log_files_to_delete(&files);
        assert!(del.contains(&5));
        assert!(del.contains(&6));
        assert!(!del.contains(&0));
    }

    #[test]
    fn rotation_enforces_total_size() {
        // Five files that sum over 10 MiB should drop oldest until under cap.
        let big = LOG_FILE_MAX_BYTES;
        let files = [
            LogFileMeta {
                size_bytes: big,
                age_days: 0,
            },
            LogFileMeta {
                size_bytes: big,
                age_days: 0,
            },
            LogFileMeta {
                size_bytes: big,
                age_days: 0,
            },
            LogFileMeta {
                size_bytes: big,
                age_days: 0,
            },
            LogFileMeta {
                size_bytes: big,
                age_days: 0,
            },
        ];
        // Exactly at cap → no delete for size.
        assert!(log_files_to_delete(&files).is_empty());

        let over = [
            LogFileMeta {
                size_bytes: big,
                age_days: 0,
            },
            LogFileMeta {
                size_bytes: big,
                age_days: 0,
            },
            LogFileMeta {
                size_bytes: big,
                age_days: 0,
            },
            LogFileMeta {
                size_bytes: big,
                age_days: 0,
            },
            LogFileMeta {
                size_bytes: big + 1,
                age_days: 0,
            },
        ];
        let del = log_files_to_delete(&over);
        assert!(!del.is_empty());
        // Oldest (last in newest-first) preferred for deletion.
        assert!(del.contains(&(over.len() - 1)) || del.contains(&3) || del.contains(&4));
    }

    #[test]
    fn needs_rotation_at_file_cap() {
        assert!(!needs_rotation_before_write(0, 100));
        assert!(needs_rotation_before_write(LOG_FILE_MAX_BYTES, 1));
        assert!(!needs_rotation_before_write(LOG_FILE_MAX_BYTES - 10, 10));
        assert!(needs_rotation_before_write(LOG_FILE_MAX_BYTES - 9, 10));
    }

    #[test]
    fn categorize_distinguishes_network_auth_parse() {
        assert_eq!(
            categorize_error_code("CalendarHttp"),
            ErrorCategory::Network
        );
        assert_eq!(
            categorize_error_code("OAuthStateMismatch"),
            ErrorCategory::Auth
        );
        assert_eq!(categorize_error_code("CalendarParse"), ErrorCategory::Parse);
        assert_eq!(
            categorize_error_code("CALENDAR_TOO_LARGE"),
            ErrorCategory::ProviderPolicy
        );
        assert_eq!(categorize_error_code("StorageIo"), ErrorCategory::Storage);
        assert_eq!(
            categorize_error_code("SurfaceHwndCreate"),
            ErrorCategory::Surface
        );
    }

    #[test]
    fn component_roundtrip() {
        for c in [
            Component::Runtime,
            Component::Tray,
            Component::Surface,
            Component::Layout,
            Component::Pomodoro,
            Component::Wallpaper,
            Component::Calendar,
            Component::Auth,
            Component::Storage,
            Component::Migration,
            Component::Diagnostics,
        ] {
            assert_eq!(Component::parse(c.as_str()), Some(c));
        }
    }

    #[test]
    fn correlation_id_opaque_and_stable_mix() {
        let a = CorrelationId::mint(CorrelationScope::Startup, 1_000, 42, 7);
        let b = CorrelationId::mint(CorrelationScope::Startup, 1_000, 42, 7);
        let c = CorrelationId::mint(CorrelationScope::Startup, 1_000, 42, 8);
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_eq!(a.as_str().len(), 16);
        assert!(!a.as_str().contains(' '));
    }

    #[test]
    fn active_error_log_caps_and_categorizes() {
        let mut log = ActiveErrorLog::new();
        for i in 0..20 {
            log.push(ActiveError::new(
                format!("StorageIo{i}"),
                Component::Storage,
                None,
            ));
        }
        assert_eq!(log.as_slice().len(), MAX_ACTIVE_ERRORS);
        let path_err = ActiveError::new("WallpaperPathInvalid", Component::Wallpaper, None);
        assert_eq!(path_err.category, ErrorCategory::Storage);
        let platform = ActiveError::new("WallpaperPlatform", Component::Wallpaper, Some("com"));
        assert_eq!(platform.category, ErrorCategory::Surface);
        log.push(platform);
        assert!(log.has_wallpaper_error());
    }

    #[test]
    fn snapshot_format_has_no_private_payloads() {
        let snap = DiagnosticsSnapshot {
            version: "0.1.0".into(),
            build_sha: "deadbeef".into(),
            config_schema_version: 1,
            last_startup: Some(StartupRecord {
                at_ms: 100,
                ok: true,
                correlation_id: CorrelationId::mint(CorrelationScope::Startup, 100, 1, 2),
            }),
            last_calendar_sync_label: "not connected".into(),
            last_wallpaper_cycle: Some(WallpaperCycleRecord {
                at_ms: 200,
                kind: WallpaperCycleKind::Local,
                ok: false,
                error_code: Some("WallpaperApplyFailed".into()),
                correlation_id: None,
            }),
            active_errors: vec![ActiveError::new(
                "SettingsCorruptRecovered",
                Component::Storage,
                Some("recovered"),
            )],
            safe_mode: false,
            safe_mode_reason: None,
            paths: DiagnosticsPathDisplay {
                data_dir: r"C:\Users\<redacted>\AppData\Local\solpaper".into(),
                cache_dir: r"C:\Users\<redacted>\AppData\Local\solpaper\cache".into(),
                logs_dir: r"C:\Users\<redacted>\AppData\Local\solpaper\logs".into(),
            },
            recovery_actions: DiagnosticsSnapshot::default_recovery_actions(false, true),
            counters: SupportCounters {
                storage_recoveries: 1,
                wallpaper_failures: 1,
                ..SupportCounters::default()
            },
            telemetry_enabled: TELEMETRY_ENABLED,
            remote_crash_upload: REMOTE_CRASH_UPLOAD,
        };
        let text = snap.format_text();
        assert!(text.contains("Version: 0.1.0"));
        assert!(text.contains("not connected"));
        assert!(text.contains("SettingsCorruptRecovered"));
        assert!(text.contains("<redacted>"));
        assert!(!text.to_ascii_lowercase().contains("alice"));
        assert!(!text.to_ascii_lowercase().contains("token"));
        assert!(!text.to_ascii_lowercase().contains("refresh"));
        assert!(text.contains("Telemetry: false"));
        assert!(text.contains(&format!("Telemetry: {TELEMETRY_ENABLED}")));
    }

    #[test]
    fn crash_marker_panic_code() {
        let m = CrashMarker::panic_marker(999, "abc");
        assert_eq!(m.error_code, "InternalPanic");
        assert_eq!(categorize_error_code(m.error_code), ErrorCategory::Internal);
    }

    #[test]
    fn runtime_recovery_plan_safe_mode_skips_widgets() {
        let plan = runtime_recovery_plan(true, true);
        assert!(!plan.contains(&RecoveryAction::RecreateSurfaces));
        assert!(!plan.contains(&RecoveryAction::OpenEditMode));
        assert!(plan.contains(&RecoveryAction::RescanWallpapers));
        assert!(plan.iter().all(|a| a.is_runtime_executable()));
    }

    #[test]
    fn runtime_recovery_plan_normal_includes_recreate_and_rescan() {
        let plan = runtime_recovery_plan(false, false);
        assert!(plan.contains(&RecoveryAction::RecreateSurfaces));
        assert!(plan.contains(&RecoveryAction::OpenEditMode));
        assert!(plan.contains(&RecoveryAction::RescanWallpapers));
        assert!(!RecoveryAction::RestartApp.is_runtime_executable());
        assert!(!RecoveryAction::ExportBundle.is_runtime_executable());
    }
}
