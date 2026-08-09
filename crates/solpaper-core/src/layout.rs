//! Widget layout model (DIP + monitor match + anchor). See ADR-0004.

use serde::{Deserialize, Serialize};

use crate::CoreError;

/// Stable identifier for a widget instance in layout state.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WidgetId(String);

impl WidgetId {
    pub fn new(id: impl Into<String>) -> Result<Self, CoreError> {
        let id = id.into();
        if id.trim().is_empty() {
            return Err(CoreError::InvalidLayout("widget id must be non-empty"));
        }
        Ok(Self(id))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Device-independent size (DIP).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DipSize {
    pub width: f32,
    pub height: f32,
}

impl DipSize {
    pub fn new(width: f32, height: f32) -> Result<Self, CoreError> {
        if !(width.is_finite() && height.is_finite()) {
            return Err(CoreError::InvalidLayout("size must be finite"));
        }
        if width <= 0.0 || height <= 0.0 {
            return Err(CoreError::InvalidLayout("size must be positive"));
        }
        Ok(Self { width, height })
    }
}

/// Device-independent offset.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DipPoint {
    pub x: f32,
    pub y: f32,
}

impl DipPoint {
    pub fn new(x: f32, y: f32) -> Result<Self, CoreError> {
        if !(x.is_finite() && y.is_finite()) {
            return Err(CoreError::InvalidLayout("offset must be finite"));
        }
        Ok(Self { x, y })
    }
}

/// Axis-aligned DIP rect (for pure hit tests / clamping helpers).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DipRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl DipRect {
    pub fn contains(&self, px: f32, py: f32) -> bool {
        px >= self.x && py >= self.y && px < self.x + self.width && py < self.y + self.height
    }
}

/// Where a widget anchors within a monitor work area.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Anchor {
    #[default]
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
}

/// Best-effort monitor identity for restore.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MonitorMatch {
    /// Prefer the primary monitor when nothing better is known.
    Primary,
    /// Opaque device name / adapter string from the platform layer.
    DeviceName(String),
}

/// One widget's persisted placement (ADR-0004).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WidgetLayoutEntry {
    pub id: WidgetId,
    pub monitor: MonitorMatch,
    pub anchor: Anchor,
    pub offset_dip: DipPoint,
    pub size_dip: DipSize,
    /// Global window opacity 0–255 (scaffold; per-pixel later).
    pub opacity: u8,
}

impl WidgetLayoutEntry {
    pub fn validate(&self) -> Result<(), CoreError> {
        // Construction of size/id already checks; re-validate for deserialized data.
        if self.id.as_str().trim().is_empty() {
            return Err(CoreError::InvalidLayout("widget id must be non-empty"));
        }
        if self.size_dip.width <= 0.0 || self.size_dip.height <= 0.0 {
            return Err(CoreError::InvalidLayout("size must be positive"));
        }
        if !(self.offset_dip.x.is_finite() && self.offset_dip.y.is_finite()) {
            return Err(CoreError::InvalidLayout("offset must be finite"));
        }
        if !(self.size_dip.width.is_finite() && self.size_dip.height.is_finite()) {
            return Err(CoreError::InvalidLayout("size must be finite"));
        }
        Ok(())
    }

    /// Build a TopLeft-anchored entry from absolute work-area coordinates (scaffold DIP).
    ///
    /// Used when persisting live HWND geometry after Edit Mode. Monitor match stays
    /// Primary until multi-monitor restore deepens.
    pub fn from_top_left_rect(
        id: WidgetId,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        opacity: u8,
    ) -> Result<Self, CoreError> {
        Ok(Self {
            id,
            monitor: MonitorMatch::Primary,
            anchor: Anchor::TopLeft,
            offset_dip: DipPoint::new(x, y)?,
            size_dip: DipSize::new(width, height)?,
            opacity,
        })
    }
}

/// Full layout document for the runtime.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct WidgetLayoutSet {
    pub version: u32,
    pub widgets: Vec<WidgetLayoutEntry>,
}

impl WidgetLayoutSet {
    pub const CURRENT_VERSION: u32 = 1;

    pub fn new_empty() -> Self {
        Self {
            version: Self::CURRENT_VERSION,
            widgets: Vec::new(),
        }
    }

    pub fn validate(&self) -> Result<(), CoreError> {
        if self.version == 0 {
            return Err(CoreError::InvalidLayout("layout version must be >= 1"));
        }
        for w in &self.widgets {
            w.validate()?;
        }
        Ok(())
    }

    /// Resolve a widget's top-left DIP position given work-area size in DIP.
    pub fn resolve_top_left(
        entry: &WidgetLayoutEntry,
        work_width_dip: f32,
        work_height_dip: f32,
    ) -> DipPoint {
        let w = entry.size_dip.width;
        let h = entry.size_dip.height;
        let (base_x, base_y) = match entry.anchor {
            Anchor::TopLeft => (0.0, 0.0),
            Anchor::TopRight => (work_width_dip - w, 0.0),
            Anchor::BottomLeft => (0.0, work_height_dip - h),
            Anchor::BottomRight => (work_width_dip - w, work_height_dip - h),
            Anchor::Center => ((work_width_dip - w) / 2.0, (work_height_dip - h) / 2.0),
        };
        DipPoint {
            x: base_x + entry.offset_dip.x,
            y: base_y + entry.offset_dip.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_widget_id() {
        assert!(WidgetId::new("").is_err());
        assert!(WidgetId::new("   ").is_err());
    }

    #[test]
    fn rejects_non_positive_size() {
        assert!(DipSize::new(0.0, 10.0).is_err());
        assert!(DipSize::new(10.0, -1.0).is_err());
    }

    #[test]
    fn dip_rect_contains() {
        let r = DipRect {
            x: 10.0,
            y: 20.0,
            width: 100.0,
            height: 50.0,
        };
        assert!(r.contains(10.0, 20.0));
        assert!(r.contains(109.0, 69.0));
        assert!(!r.contains(110.0, 20.0));
        assert!(!r.contains(10.0, 70.0));
    }

    #[test]
    fn resolve_top_left_top_left_anchor() {
        let entry = WidgetLayoutEntry {
            id: WidgetId::new("pomodoro").unwrap(),
            monitor: MonitorMatch::Primary,
            anchor: Anchor::TopLeft,
            offset_dip: DipPoint::new(16.0, 24.0).unwrap(),
            size_dip: DipSize::new(200.0, 120.0).unwrap(),
            opacity: 230,
        };
        let p = WidgetLayoutSet::resolve_top_left(&entry, 1920.0, 1080.0);
        assert_eq!(p.x, 16.0);
        assert_eq!(p.y, 24.0);
    }

    #[test]
    fn resolve_top_left_bottom_right_anchor() {
        let entry = WidgetLayoutEntry {
            id: WidgetId::new("cal").unwrap(),
            monitor: MonitorMatch::Primary,
            anchor: Anchor::BottomRight,
            offset_dip: DipPoint::new(-8.0, -8.0).unwrap(),
            size_dip: DipSize::new(100.0, 80.0).unwrap(),
            opacity: 255,
        };
        let p = WidgetLayoutSet::resolve_top_left(&entry, 1000.0, 800.0);
        assert_eq!(p.x, 1000.0 - 100.0 - 8.0);
        assert_eq!(p.y, 800.0 - 80.0 - 8.0);
    }

    #[test]
    fn layout_set_validate_ok() {
        let mut set = WidgetLayoutSet::new_empty();
        set.widgets.push(WidgetLayoutEntry {
            id: WidgetId::new("placeholder").unwrap(),
            monitor: MonitorMatch::Primary,
            anchor: Anchor::TopLeft,
            offset_dip: DipPoint::new(40.0, 40.0).unwrap(),
            size_dip: DipSize::new(280.0, 160.0).unwrap(),
            opacity: 220,
        });
        assert!(set.validate().is_ok());
    }

    #[test]
    fn from_top_left_rect_round_trips_resolve() {
        let entry = WidgetLayoutEntry::from_top_left_rect(
            WidgetId::new("placeholder").unwrap(),
            120.0,
            80.0,
            200.0,
            100.0,
            200,
        )
        .unwrap();
        assert_eq!(entry.anchor, Anchor::TopLeft);
        let p = WidgetLayoutSet::resolve_top_left(&entry, 1920.0, 1080.0);
        assert_eq!(p.x, 120.0);
        assert_eq!(p.y, 80.0);
        assert_eq!(entry.size_dip.width, 200.0);
        assert_eq!(entry.size_dip.height, 100.0);
    }
}
