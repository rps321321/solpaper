//! Minimal layout persistence for the #18 spike.
//!
//! Stored under `%LOCALAPPDATA%\solpaper-overlay-spike\`. Not production storage.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Approach {
    A,
    B,
}

impl Approach {
    pub fn as_str(self) -> &'static str {
        match self {
            Approach::A => "a",
            Approach::B => "b",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "a" | "approach-a" | "widget" | "independent" => Some(Approach::A),
            "b" | "approach-b" | "surface" | "monitor" => Some(Approach::B),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetLayout {
    pub id: String,
    /// Logical pixels relative to the primary monitor origin (virtual screen).
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpikeLayout {
    pub approach: Approach,
    /// 0–255 window alpha applied via `SetLayeredWindowAttributes`.
    pub opacity: u8,
    pub widgets: Vec<WidgetLayout>,
}

impl SpikeLayout {
    pub fn default_for(approach: Approach) -> Self {
        Self {
            approach,
            opacity: 220,
            widgets: vec![
                WidgetLayout {
                    id: "timer".into(),
                    x: 80,
                    y: 80,
                    width: 280,
                    height: 140,
                },
                WidgetLayout {
                    id: "calendar".into(),
                    x: 80,
                    y: 250,
                    width: 320,
                    height: 200,
                },
            ],
        }
    }
}

pub fn layout_dir() -> PathBuf {
    let base = std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    base.join("solpaper-overlay-spike")
}

pub fn layout_path(approach: Approach) -> PathBuf {
    layout_dir().join(format!("layout-{}.json", approach.as_str()))
}

pub fn load_layout(approach: Approach) -> SpikeLayout {
    let path = layout_path(approach);
    match fs::read_to_string(&path) {
        Ok(text) => match serde_json::from_str::<SpikeLayout>(&text) {
            Ok(mut layout) => {
                layout.approach = approach;
                sanitize_layout(layout)
            }
            Err(_) => SpikeLayout::default_for(approach),
        },
        Err(_) => SpikeLayout::default_for(approach),
    }
}

pub fn save_layout(layout: &SpikeLayout) -> std::io::Result<()> {
    let dir = layout_dir();
    fs::create_dir_all(&dir)?;
    let path = layout_path(layout.approach);
    let text = serde_json::to_string_pretty(layout)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    fs::write(path, text)
}

/// Clamp sizes/opacity so a corrupted file cannot create unusable windows.
fn sanitize_layout(mut layout: SpikeLayout) -> SpikeLayout {
    if layout.opacity < 40 {
        layout.opacity = 40;
    }
    for w in &mut layout.widgets {
        w.width = w.width.clamp(120, 800);
        w.height = w.height.clamp(80, 600);
    }
    if layout.widgets.is_empty() {
        return SpikeLayout::default_for(layout.approach);
    }
    layout
}

/// Axis-aligned hit test used by Approach B selective input.
pub fn hit_test_widgets(widgets: &[WidgetLayout], x: i32, y: i32) -> Option<usize> {
    // Top-most in list order last (later widgets paint above earlier ones).
    widgets
        .iter()
        .enumerate()
        .rev()
        .find(|(_, w)| x >= w.x && y >= w.y && x < w.x + w.width && y < w.y + w.height)
        .map(|(i, _)| i)
}

/// Title-bar drag region (top 28 px) and bottom-right resize grip (16×16).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WidgetHit {
    Body,
    Drag,
    Resize,
}

pub fn classify_widget_hit(widget: &WidgetLayout, x: i32, y: i32) -> Option<WidgetHit> {
    if x < widget.x || y < widget.y || x >= widget.x + widget.width || y >= widget.y + widget.height
    {
        return None;
    }
    let lx = x - widget.x;
    let ly = y - widget.y;
    if lx >= widget.width - 16 && ly >= widget.height - 16 {
        return Some(WidgetHit::Resize);
    }
    if ly < 28 {
        return Some(WidgetHit::Drag);
    }
    Some(WidgetHit::Body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn approach_parse() {
        assert_eq!(Approach::parse("a"), Some(Approach::A));
        assert_eq!(Approach::parse("B"), Some(Approach::B));
        assert_eq!(Approach::parse("monitor"), Some(Approach::B));
        assert_eq!(Approach::parse("nope"), None);
    }

    #[test]
    fn default_layout_has_two_widgets() {
        let l = SpikeLayout::default_for(Approach::A);
        assert_eq!(l.widgets.len(), 2);
        assert!(l.widgets.iter().any(|w| w.id == "timer"));
        assert!(l.widgets.iter().any(|w| w.id == "calendar"));
    }

    #[test]
    fn roundtrip_json() {
        let original = SpikeLayout::default_for(Approach::B);
        let text = serde_json::to_string(&original).unwrap();
        let back: SpikeLayout = serde_json::from_str(&text).unwrap();
        assert_eq!(back.approach, Approach::B);
        assert_eq!(back.opacity, original.opacity);
        assert_eq!(back.widgets.len(), 2);
    }

    #[test]
    fn hit_test_prefers_later_widget() {
        let widgets = vec![
            WidgetLayout {
                id: "a".into(),
                x: 0,
                y: 0,
                width: 100,
                height: 100,
            },
            WidgetLayout {
                id: "b".into(),
                x: 50,
                y: 50,
                width: 100,
                height: 100,
            },
        ];
        assert_eq!(hit_test_widgets(&widgets, 10, 10), Some(0));
        assert_eq!(hit_test_widgets(&widgets, 60, 60), Some(1));
        assert_eq!(hit_test_widgets(&widgets, 200, 200), None);
    }

    #[test]
    fn classify_drag_and_resize() {
        let w = WidgetLayout {
            id: "t".into(),
            x: 100,
            y: 100,
            width: 200,
            height: 100,
        };
        assert_eq!(classify_widget_hit(&w, 110, 110), Some(WidgetHit::Drag));
        assert_eq!(classify_widget_hit(&w, 290, 190), Some(WidgetHit::Resize));
        assert_eq!(classify_widget_hit(&w, 150, 150), Some(WidgetHit::Body));
        assert_eq!(classify_widget_hit(&w, 0, 0), None);
    }

    #[test]
    fn sanitize_clamps_tiny_opacity_and_size() {
        let mut l = SpikeLayout::default_for(Approach::A);
        l.opacity = 1;
        l.widgets[0].width = 10;
        l.widgets[0].height = 10;
        let s = sanitize_layout(l);
        assert!(s.opacity >= 40);
        assert!(s.widgets[0].width >= 120);
        assert!(s.widgets[0].height >= 80);
    }
}
