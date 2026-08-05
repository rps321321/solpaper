//! Settings and runtime file locations for Solpaper (ADR-0005).
//!
//! Secrets must never be written here — Credential Manager only.

use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use solpaper_core::WidgetLayoutSet;
use thiserror::Error;

const APP_FOLDER: &str = "solpaper";
const SETTINGS_FILE: &str = "settings.json";
const LAYOUT_FILE: &str = "layout.json";

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
}

/// Well-known LocalAppData subpaths for the production app.
#[derive(Debug, Clone)]
pub struct AppPaths {
    pub root: PathBuf,
    pub settings: PathBuf,
    pub layout: PathBuf,
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
            cache: root.join("cache"),
            logs: root.join("logs"),
            root,
        }
    }

    pub fn ensure_dirs(&self) -> Result<(), StorageError> {
        fs::create_dir_all(&self.root)?;
        fs::create_dir_all(&self.cache)?;
        fs::create_dir_all(&self.logs)?;
        Ok(())
    }
}

/// Versioned human-readable settings. **No secret fields.**
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SettingsDocument {
    pub version: u32,
    /// Global default widget opacity 0–255.
    pub default_opacity: u8,
}

impl Default for SettingsDocument {
    fn default() -> Self {
        Self {
            version: 1,
            default_opacity: 230,
        }
    }
}

impl SettingsDocument {
    pub fn load_or_default(path: &Path) -> Result<Self, StorageError> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let text = fs::read_to_string(path)?;
        let doc: Self = serde_json::from_str(&text)?;
        Ok(doc)
    }

    pub fn save(&self, path: &Path) -> Result<(), StorageError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let text = serde_json::to_string_pretty(self)?;
        fs::write(path, text)?;
        Ok(())
    }
}

pub fn load_layout(path: &Path) -> Result<WidgetLayoutSet, StorageError> {
    if !path.exists() {
        return Ok(WidgetLayoutSet::new_empty());
    }
    let text = fs::read_to_string(path)?;
    let set: WidgetLayoutSet = serde_json::from_str(&text)?;
    set.validate()
        .map_err(|e| StorageError::Layout(e.to_string()))?;
    Ok(set)
}

pub fn save_layout(path: &Path, set: &WidgetLayoutSet) -> Result<(), StorageError> {
    set.validate()
        .map_err(|e| StorageError::Layout(e.to_string()))?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let text = serde_json::to_string_pretty(set)?;
    fs::write(path, text)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use solpaper_core::{Anchor, DipPoint, DipSize, MonitorMatch, WidgetId, WidgetLayoutEntry};
    use std::time::{SystemTime, UNIX_EPOCH};

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
        };
        doc.save(&paths.settings).unwrap();
        let loaded = SettingsDocument::load_or_default(&paths.settings).unwrap();
        assert_eq!(loaded, doc);
        let raw = fs::read_to_string(&paths.settings).unwrap();
        assert!(!raw.to_lowercase().contains("token"));
        assert!(!raw.to_lowercase().contains("secret"));
        assert!(!raw.to_lowercase().contains("password"));
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
        let loaded = load_layout(&paths.layout).unwrap();
        assert_eq!(loaded, set);
        let _ = fs::remove_dir_all(root);
    }
}
