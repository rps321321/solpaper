//! Settings and runtime file locations for Solpaper (ADR-0005).
//!
//! Secrets must never be written here — Credential Manager only.
//!
//! Atomic writes follow the #35 NFR pack: same-directory temp → flush → replace
//! target → retain one previous `.bak`. Corrupt documents are quarantined with a
//! timestamped suffix; callers load safe defaults instead of crashing.

use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use solpaper_core::{PomodoroState, WidgetLayoutSet};
use thiserror::Error;

const APP_FOLDER: &str = "solpaper";
const SETTINGS_FILE: &str = "settings.json";
const LAYOUT_FILE: &str = "layout.json";
const POMODORO_FILE: &str = "pomodoro.json";
const WALLPAPERS_DIR: &str = "wallpapers";

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("LOCALAPPDATA is not set")]
    NoLocalAppData,
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("layout validation failed: {0}")]
    Layout(String),
    #[error("pomodoro validation failed: {0}")]
    Pomodoro(String),
}

/// Outcome of loading a versioned document (settings or layout).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadOutcome {
    /// File missing — defaults used.
    Missing,
    /// File parsed and validated.
    Loaded,
    /// File was corrupt/invalid; quarantined; defaults used.
    RecoveredFromCorrupt,
}

/// Well-known LocalAppData subpaths for the production app.
#[derive(Debug, Clone)]
pub struct AppPaths {
    pub root: PathBuf,
    pub settings: PathBuf,
    pub layout: PathBuf,
    /// Durable Pomodoro machine state (no history / no secrets).
    pub pomodoro: PathBuf,
    /// Default drop-folder for local wallpapers (Alpha 1).
    pub wallpapers: PathBuf,
    pub cache: PathBuf,
    pub logs: PathBuf,
}

impl AppPaths {
    /// Resolve paths under `%LOCALAPPDATA%\solpaper\`.
    pub fn from_local_app_data() -> Result<Self, StorageError> {
        let base = std::env::var_os("LOCALAPPDATA").ok_or(StorageError::NoLocalAppData)?;
        Ok(Self::from_root(PathBuf::from(base).join(APP_FOLDER)))
    }

    pub fn from_root(root: PathBuf) -> Self {
        Self {
            settings: root.join(SETTINGS_FILE),
            layout: root.join(LAYOUT_FILE),
            pomodoro: root.join(POMODORO_FILE),
            wallpapers: root.join(WALLPAPERS_DIR),
            cache: root.join("cache"),
            logs: root.join("logs"),
            root,
        }
    }

    pub fn ensure_dirs(&self) -> Result<(), StorageError> {
        fs::create_dir_all(&self.root)?;
        fs::create_dir_all(&self.cache)?;
        fs::create_dir_all(&self.logs)?;
        fs::create_dir_all(&self.wallpapers)?;
        Ok(())
    }
}

/// Versioned human-readable settings. **No secret fields.**
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SettingsDocument {
    pub version: u32,
    /// Global default widget opacity 0–255.
    pub default_opacity: u8,
    /// Local wallpaper folders (absolute paths). Empty → use default `wallpapers` dir only.
    #[serde(default)]
    pub wallpaper_folders: Vec<String>,
    /// Hold automatic wallpaper changes (pack #20; Alpha 1 has no schedule).
    #[serde(default)]
    pub wallpaper_hold: bool,
}

impl Default for SettingsDocument {
    fn default() -> Self {
        Self {
            version: Self::CURRENT_VERSION,
            default_opacity: 230,
            wallpaper_folders: Vec::new(),
            wallpaper_hold: false,
        }
    }
}

impl SettingsDocument {
    pub const CURRENT_VERSION: u32 = 1;

    pub fn load_or_default(path: &Path) -> Result<(Self, LoadOutcome), StorageError> {
        if !path.exists() {
            return Ok((Self::default(), LoadOutcome::Missing));
        }
        match load_settings_strict(path) {
            Ok(doc) => Ok((doc, LoadOutcome::Loaded)),
            Err(e) => {
                eprintln!("solpaper: settings load failed ({e}); quarantining and using defaults");
                quarantine_corrupt(path)?;
                Ok((Self::default(), LoadOutcome::RecoveredFromCorrupt))
            }
        }
    }

    pub fn save(&self, path: &Path) -> Result<(), StorageError> {
        let text = serde_json::to_string_pretty(self)?;
        atomic_write(path, text.as_bytes())
    }
}

fn load_settings_strict(path: &Path) -> Result<SettingsDocument, StorageError> {
    let text = fs::read_to_string(path)?;
    let doc: SettingsDocument = serde_json::from_str(&text)?;
    if doc.version == 0 {
        return Err(StorageError::Layout("settings version must be >= 1".into()));
    }
    Ok(doc)
}

pub fn load_layout(path: &Path) -> Result<(WidgetLayoutSet, LoadOutcome), StorageError> {
    if !path.exists() {
        return Ok((WidgetLayoutSet::new_empty(), LoadOutcome::Missing));
    }
    match load_layout_strict(path) {
        Ok(set) => Ok((set, LoadOutcome::Loaded)),
        Err(e) => {
            eprintln!("solpaper: layout load failed ({e}); quarantining and using empty layout");
            quarantine_corrupt(path)?;
            Ok((
                WidgetLayoutSet::new_empty(),
                LoadOutcome::RecoveredFromCorrupt,
            ))
        }
    }
}

fn load_layout_strict(path: &Path) -> Result<WidgetLayoutSet, StorageError> {
    let text = fs::read_to_string(path)?;
    let set: WidgetLayoutSet = serde_json::from_str(&text)?;
    set.validate()
        .map_err(|e| StorageError::Layout(e.to_string()))?;
    Ok(set)
}

pub fn save_layout(path: &Path, set: &WidgetLayoutSet) -> Result<(), StorageError> {
    set.validate()
        .map_err(|e| StorageError::Layout(e.to_string()))?;
    let text = serde_json::to_string_pretty(set)?;
    atomic_write(path, text.as_bytes())
}

/// Load durable Pomodoro state; missing/corrupt → idle defaults (quarantine corrupt).
///
/// Callers should apply [`solpaper_core::PomodoroCommand::Sync`] after load for recovery.
pub fn load_pomodoro(path: &Path) -> Result<(PomodoroState, LoadOutcome), StorageError> {
    if !path.exists() {
        return Ok((PomodoroState::idle_default(), LoadOutcome::Missing));
    }
    match load_pomodoro_strict(path) {
        Ok(state) => Ok((state, LoadOutcome::Loaded)),
        Err(e) => {
            eprintln!("solpaper: pomodoro load failed ({e}); quarantining and using idle defaults");
            quarantine_corrupt(path)?;
            Ok((
                PomodoroState::idle_default(),
                LoadOutcome::RecoveredFromCorrupt,
            ))
        }
    }
}

fn load_pomodoro_strict(path: &Path) -> Result<PomodoroState, StorageError> {
    let text = fs::read_to_string(path)?;
    let state: PomodoroState = serde_json::from_str(&text)?;
    state
        .config
        .validate()
        .map_err(|e| StorageError::Pomodoro(e.to_string()))?;
    Ok(state)
}

/// Atomically persist Pomodoro state (semantic transitions only; not every tick).
pub fn save_pomodoro(path: &Path, state: &PomodoroState) -> Result<(), StorageError> {
    state
        .config
        .validate()
        .map_err(|e| StorageError::Pomodoro(e.to_string()))?;
    let text = serde_json::to_string_pretty(state)?;
    atomic_write(path, text.as_bytes())
}

/// Write `data` to `path` atomically: temp → flush → replace; keep one `.bak`.
pub fn atomic_write(path: &Path, data: &[u8]) -> Result<(), StorageError> {
    let parent = path.parent().filter(|p| !p.as_os_str().is_empty());
    if let Some(parent) = parent {
        fs::create_dir_all(parent)?;
    }
    let parent = parent.unwrap_or_else(|| Path::new("."));
    let file_name = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("document");
    let tmp = parent.join(format!(".{file_name}.{}.tmp", std::process::id()));

    {
        let mut f = File::create(&tmp)?;
        f.write_all(data)?;
        f.sync_all()?;
    }

    let bak = backup_path(path);
    if path.exists() {
        // Best-effort single previous backup (replace any older .bak).
        let _ = fs::remove_file(&bak);
        // rename target → bak; if rename fails, still try to install new file.
        if let Err(e) = fs::rename(path, &bak) {
            // Fall back to copy+remove so Windows sharing glitches are less fatal.
            if let Err(copy_err) = fs::copy(path, &bak) {
                let _ = fs::remove_file(&tmp);
                return Err(StorageError::Io(copy_err));
            }
            let _ = fs::remove_file(path);
            let _ = e; // rename failed but copy succeeded
        }
    }

    if let Err(e) = fs::rename(&tmp, path) {
        // Last resort: copy temp into place.
        if let Err(copy_err) = fs::copy(&tmp, path) {
            let _ = fs::remove_file(&tmp);
            return Err(StorageError::Io(copy_err));
        }
        let _ = fs::remove_file(&tmp);
        let _ = e;
    }
    Ok(())
}

fn backup_path(path: &Path) -> PathBuf {
    let mut os = path.as_os_str().to_owned();
    os.push(".bak");
    PathBuf::from(os)
}

fn quarantine_corrupt(path: &Path) -> Result<(), StorageError> {
    if !path.exists() {
        return Ok(());
    }
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let name = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("document");
    let dest = path.with_file_name(format!("{name}.corrupt-{ts}"));
    // Avoid clobbering a previous quarantine in the same second.
    let dest = if dest.exists() {
        path.with_file_name(format!("{name}.corrupt-{ts}-{}", std::process::id()))
    } else {
        dest
    };
    fs::rename(path, dest)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use solpaper_core::{Anchor, DipPoint, DipSize, MonitorMatch, WidgetId, WidgetLayoutEntry};

    fn temp_root() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("solpaper_storage-test-{nanos}"))
    }

    #[test]
    fn settings_roundtrip_has_no_secret_fields() {
        let root = temp_root();
        let paths = AppPaths::from_root(root.clone());
        paths.ensure_dirs().unwrap();
        let doc = SettingsDocument {
            version: 1,
            default_opacity: 200,
            wallpaper_folders: vec![paths.wallpapers.to_string_lossy().into_owned()],
            wallpaper_hold: true,
        };
        doc.save(&paths.settings).unwrap();
        let (loaded, outcome) = SettingsDocument::load_or_default(&paths.settings).unwrap();
        assert_eq!(outcome, LoadOutcome::Loaded);
        assert_eq!(loaded, doc);
        let raw = fs::read_to_string(&paths.settings).unwrap();
        assert!(!raw.to_lowercase().contains("token"));
        assert!(!raw.to_lowercase().contains("secret"));
        assert!(!raw.to_lowercase().contains("password"));
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn settings_missing_wallpaper_fields_default() {
        let root = temp_root();
        let paths = AppPaths::from_root(root.clone());
        paths.ensure_dirs().unwrap();
        fs::write(&paths.settings, br#"{"version":1,"default_opacity":210}"#).unwrap();
        let (doc, _) = SettingsDocument::load_or_default(&paths.settings).unwrap();
        assert!(doc.wallpaper_folders.is_empty());
        assert!(!doc.wallpaper_hold);
        assert_eq!(doc.default_opacity, 210);
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn layout_roundtrip() {
        let root = temp_root();
        let paths = AppPaths::from_root(root.clone());
        let mut set = WidgetLayoutSet::new_empty();
        set.widgets.push(WidgetLayoutEntry {
            id: WidgetId::new("placeholder").unwrap(),
            monitor: MonitorMatch::Primary,
            anchor: Anchor::TopLeft,
            offset_dip: DipPoint::new(32.0, 32.0).unwrap(),
            size_dip: DipSize::new(240.0, 140.0).unwrap(),
            opacity: 220,
        });
        save_layout(&paths.layout, &set).unwrap();
        let (loaded, outcome) = load_layout(&paths.layout).unwrap();
        assert_eq!(outcome, LoadOutcome::Loaded);
        assert_eq!(loaded, set);
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn atomic_write_retains_bak() {
        let root = temp_root();
        fs::create_dir_all(&root).unwrap();
        let path = root.join("settings.json");
        atomic_write(&path, b"{\"version\":1,\"default_opacity\":1}").unwrap();
        atomic_write(&path, b"{\"version\":1,\"default_opacity\":2}").unwrap();
        let bak = backup_path(&path);
        assert!(bak.exists(), "expected .bak after second write");
        let bak_text = fs::read_to_string(&bak).unwrap();
        assert!(
            bak_text.contains("\"default_opacity\": 1")
                || bak_text.contains("\"default_opacity\":1")
        );
        let cur = fs::read_to_string(&path).unwrap();
        assert!(cur.contains("\"default_opacity\": 2") || cur.contains("\"default_opacity\":2"));
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn corrupt_settings_quarantined_and_defaults() {
        let root = temp_root();
        let paths = AppPaths::from_root(root.clone());
        paths.ensure_dirs().unwrap();
        fs::write(&paths.settings, b"{not-json").unwrap();
        let (doc, outcome) = SettingsDocument::load_or_default(&paths.settings).unwrap();
        assert_eq!(outcome, LoadOutcome::RecoveredFromCorrupt);
        assert_eq!(doc, SettingsDocument::default());
        assert!(!paths.settings.exists());
        let quarantined: Vec<_> = fs::read_dir(&paths.root)
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .filter(|n| n.contains("corrupt"))
            .collect();
        assert_eq!(quarantined.len(), 1);
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn corrupt_layout_quarantined_and_empty() {
        let root = temp_root();
        let paths = AppPaths::from_root(root.clone());
        paths.ensure_dirs().unwrap();
        fs::write(&paths.layout, b"{\"version\":0,\"widgets\":[]}").unwrap();
        let (set, outcome) = load_layout(&paths.layout).unwrap();
        assert_eq!(outcome, LoadOutcome::RecoveredFromCorrupt);
        assert!(set.widgets.is_empty());
        assert_eq!(set.version, WidgetLayoutSet::CURRENT_VERSION);
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn missing_files_use_defaults() {
        let root = temp_root();
        let paths = AppPaths::from_root(root.clone());
        let (doc, o1) = SettingsDocument::load_or_default(&paths.settings).unwrap();
        assert_eq!(o1, LoadOutcome::Missing);
        assert_eq!(doc.version, 1);
        let (set, o2) = load_layout(&paths.layout).unwrap();
        assert_eq!(o2, LoadOutcome::Missing);
        assert!(set.widgets.is_empty());
        let (pomo, o3) = load_pomodoro(&paths.pomodoro).unwrap();
        assert_eq!(o3, LoadOutcome::Missing);
        assert!(matches!(pomo.status, solpaper_core::TimerStatus::Idle));
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn pomodoro_roundtrip_running() {
        use solpaper_core::{PomodoroCommand, TimerStatus};
        let root = temp_root();
        let paths = AppPaths::from_root(root.clone());
        paths.ensure_dirs().unwrap();
        let mut state = PomodoroState::idle_default();
        state.apply(PomodoroCommand::Start, 1_000_000).unwrap();
        save_pomodoro(&paths.pomodoro, &state).unwrap();
        let (loaded, outcome) = load_pomodoro(&paths.pomodoro).unwrap();
        assert_eq!(outcome, LoadOutcome::Loaded);
        assert_eq!(loaded, state);
        assert!(matches!(loaded.status, TimerStatus::Running { .. }));
        let raw = fs::read_to_string(&paths.pomodoro).unwrap();
        assert!(!raw.to_lowercase().contains("token"));
        assert!(!raw.to_lowercase().contains("secret"));
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn corrupt_pomodoro_quarantined_and_idle() {
        let root = temp_root();
        let paths = AppPaths::from_root(root.clone());
        paths.ensure_dirs().unwrap();
        fs::write(&paths.pomodoro, b"{not-json").unwrap();
        let (state, outcome) = load_pomodoro(&paths.pomodoro).unwrap();
        assert_eq!(outcome, LoadOutcome::RecoveredFromCorrupt);
        assert_eq!(state, PomodoroState::idle_default());
        assert!(!paths.pomodoro.exists());
        let quarantined: Vec<_> = fs::read_dir(&paths.root)
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .filter(|n| n.contains("corrupt"))
            .collect();
        assert_eq!(quarantined.len(), 1);
        let _ = fs::remove_dir_all(root);
    }
}
