//! Normal / Edit Mode surface policy and pure hit/geometry helpers (#20 bullet 2 / #34).
//!
//! Platform HWND code lives in `solpaper-windows`. This module is unit-tested on any host.

use crate::CoreError;

/// Desktop surface interaction mode (ADR-0001 / pack #34).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SurfaceMode {
    /// Widgets are read-only and click-through; actions go through tray/settings/keyboard.
    #[default]
    Normal,
    /// Widgets accept hit-testing for move/resize chrome; Escape / tray / hotkey exit.
    Edit,
}

impl SurfaceMode {
    pub fn toggle(self) -> Self {
        match self {
            Self::Normal => Self::Edit,
            Self::Edit => Self::Normal,
        }
    }

    pub fn is_edit(self) -> bool {
        matches!(self, Self::Edit)
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Edit => "edit",
        }
    }
}

/// Edit Mode hit region inside a widget's local client coordinates (DIP or physical px when 1:1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WidgetHit {
    Body,
    Drag,
    Resize,
}

/// Top drag strip height (pack #34 DEFAULT).
pub const DRAG_STRIP_DIP: f32 = 24.0;

/// Bottom-right resize grip size (pack #34 DEFAULT).
pub const RESIZE_GRIP_DIP: f32 = 12.0;

/// Minimum amount of the widget that must remain visible on a work area (pack #34).
pub const MIN_VISIBLE_DIP: f32 = 48.0;

/// Minimum widget width/height after resize (practical floor; still subject to visibility clamp).
pub const MIN_WIDGET_SIZE_DIP: f32 = 80.0;

/// Classify a local client point relative to a widget of the given size.
///
/// Coordinates are in the same unit as `width`/`height` (DIP at the domain layer;
/// scaffold host may treat DIP ≈ physical px until full multi-mon DPI lands).
pub fn classify_widget_hit(
    width: f32,
    height: f32,
    local_x: f32,
    local_y: f32,
) -> Option<WidgetHit> {
    if !(width.is_finite() && height.is_finite() && local_x.is_finite() && local_y.is_finite()) {
        return None;
    }
    if local_x < 0.0 || local_y < 0.0 || local_x >= width || local_y >= height {
        return None;
    }
    let grip = RESIZE_GRIP_DIP.min(width).min(height);
    if local_x >= width - grip && local_y >= height - grip {
        return Some(WidgetHit::Resize);
    }
    let strip = DRAG_STRIP_DIP.min(height);
    if local_y < strip {
        return Some(WidgetHit::Drag);
    }
    Some(WidgetHit::Body)
}

/// Axis-aligned rect in DIP (or scaffold physical units).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SurfaceRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl SurfaceRect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Result<Self, CoreError> {
        if !(x.is_finite() && y.is_finite() && width.is_finite() && height.is_finite()) {
            return Err(CoreError::InvalidLayout("rect must be finite"));
        }
        if width <= 0.0 || height <= 0.0 {
            return Err(CoreError::InvalidLayout("rect size must be positive"));
        }
        Ok(Self {
            x,
            y,
            width,
            height,
        })
    }

    pub fn right(self) -> f32 {
        self.x + self.width
    }

    pub fn bottom(self) -> f32 {
        self.y + self.height
    }
}

/// Work-area rectangle used for clamping (DIP / scaffold units).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WorkArea {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl WorkArea {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Result<Self, CoreError> {
        if !(x.is_finite() && y.is_finite() && width.is_finite() && height.is_finite()) {
            return Err(CoreError::InvalidLayout("work area must be finite"));
        }
        if width <= 0.0 || height <= 0.0 {
            return Err(CoreError::InvalidLayout("work area size must be positive"));
        }
        Ok(Self {
            x,
            y,
            width,
            height,
        })
    }

    pub fn right(self) -> f32 {
        self.x + self.width
    }

    pub fn bottom(self) -> f32 {
        self.y + self.height
    }
}

/// Clamp so at least [`MIN_VISIBLE_DIP`]×[`MIN_VISIBLE_DIP`] of the widget remains in the work area.
pub fn clamp_rect_visible(rect: SurfaceRect, work: WorkArea) -> SurfaceRect {
    let min_v = MIN_VISIBLE_DIP;
    let mut r = rect;
    r.width = r.width.max(MIN_WIDGET_SIZE_DIP.min(work.width.max(1.0)));
    r.height = r.height.max(MIN_WIDGET_SIZE_DIP.min(work.height.max(1.0)));

    // Keep left edge from going so far right that nothing remains.
    let max_x = work.right() - min_v.min(r.width);
    let min_x = work.x - (r.width - min_v).max(0.0);
    r.x = r.x.clamp(min_x, max_x.max(min_x));

    let max_y = work.bottom() - min_v.min(r.height);
    let min_y = work.y - (r.height - min_v).max(0.0);
    r.y = r.y.clamp(min_y, max_y.max(min_y));

    // Cap size so the widget cannot exceed work area by a huge margin (still allow overhang).
    r.width = r.width.min(work.width.max(min_v) * 2.0).max(1.0);
    r.height = r.height.min(work.height.max(min_v) * 2.0).max(1.0);
    r
}

/// Apply a move delta, then clamp for visibility.
pub fn apply_move(rect: SurfaceRect, dx: f32, dy: f32, work: WorkArea) -> SurfaceRect {
    clamp_rect_visible(
        SurfaceRect {
            x: rect.x + dx,
            y: rect.y + dy,
            width: rect.width,
            height: rect.height,
        },
        work,
    )
}

/// Grow/shrink from the bottom-right (Alpha 1 edge heuristic), then clamp.
pub fn apply_resize(rect: SurfaceRect, dw: f32, dh: f32, work: WorkArea) -> SurfaceRect {
    let width = (rect.width + dw).max(MIN_WIDGET_SIZE_DIP);
    let height = (rect.height + dh).max(MIN_WIDGET_SIZE_DIP);
    clamp_rect_visible(
        SurfaceRect {
            x: rect.x,
            y: rect.y,
            width,
            height,
        },
        work,
    )
}

/// Keyboard nudge step multipliers (pack #34).
pub const NUDGE_STEP_DIP: f32 = 1.0;
pub const NUDGE_STEP_LARGE_DIP: f32 = 10.0;

/// Resolve move/resize delta for an Edit Mode arrow key with modifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditArrow {
    Left,
    Right,
    Up,
    Down,
}

/// Compute (dx, dy) or (dw, dh) for keyboard edit.
///
/// - No Ctrl: move by step
/// - Ctrl: resize by step (right/bottom grow)
/// - Shift: large step (10 DIP)
pub fn edit_arrow_delta(arrow: EditArrow, ctrl: bool, shift: bool) -> (f32, f32, bool) {
    let step = if shift {
        NUDGE_STEP_LARGE_DIP
    } else {
        NUDGE_STEP_DIP
    };
    let (dx, dy) = match arrow {
        EditArrow::Left => (-step, 0.0),
        EditArrow::Right => (step, 0.0),
        EditArrow::Up => (0.0, -step),
        EditArrow::Down => (0.0, step),
    };
    (dx, dy, ctrl)
}

/// Apply keyboard edit to a rect.
pub fn apply_edit_arrow(
    rect: SurfaceRect,
    arrow: EditArrow,
    ctrl: bool,
    shift: bool,
    work: WorkArea,
) -> SurfaceRect {
    let (a, b, is_resize) = edit_arrow_delta(arrow, ctrl, shift);
    if is_resize {
        apply_resize(rect, a, b, work)
    } else {
        apply_move(rect, a, b, work)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn surface_mode_toggle() {
        assert_eq!(SurfaceMode::Normal.toggle(), SurfaceMode::Edit);
        assert_eq!(SurfaceMode::Edit.toggle(), SurfaceMode::Normal);
        assert!(!SurfaceMode::Normal.is_edit());
        assert!(SurfaceMode::Edit.is_edit());
    }

    #[test]
    fn classify_drag_strip_and_grip() {
        let w = 200.0;
        let h = 100.0;
        assert_eq!(classify_widget_hit(w, h, 10.0, 10.0), Some(WidgetHit::Drag));
        assert_eq!(
            classify_widget_hit(w, h, 195.0, 95.0),
            Some(WidgetHit::Resize)
        );
        assert_eq!(classify_widget_hit(w, h, 50.0, 50.0), Some(WidgetHit::Body));
        assert_eq!(classify_widget_hit(w, h, -1.0, 0.0), None);
        assert_eq!(classify_widget_hit(w, h, 200.0, 0.0), None);
    }

    #[test]
    fn pack_constants_match_blueprint() {
        assert_eq!(DRAG_STRIP_DIP, 24.0);
        assert_eq!(RESIZE_GRIP_DIP, 12.0);
        assert_eq!(MIN_VISIBLE_DIP, 48.0);
    }

    #[test]
    fn clamp_keeps_min_visible() {
        let work = WorkArea::new(0.0, 0.0, 1000.0, 800.0).unwrap();
        let off = SurfaceRect::new(-500.0, -500.0, 200.0, 120.0).unwrap();
        let c = clamp_rect_visible(off, work);
        // At least 48 DIP of the widget must intersect the work area.
        assert!(c.right() > work.x + MIN_VISIBLE_DIP - 0.1 || c.x < work.right());
        assert!(c.x + c.width > work.x);
        assert!(c.y + c.height > work.y);
        assert!(c.x < work.right());
        assert!(c.y < work.bottom());
    }

    #[test]
    fn apply_move_and_resize() {
        let work = WorkArea::new(0.0, 0.0, 1920.0, 1080.0).unwrap();
        let r = SurfaceRect::new(100.0, 100.0, 200.0, 120.0).unwrap();
        let moved = apply_move(r, 10.0, -5.0, work);
        assert_eq!(moved.x, 110.0);
        assert_eq!(moved.y, 95.0);
        let resized = apply_resize(r, 40.0, 20.0, work);
        assert_eq!(resized.width, 240.0);
        assert_eq!(resized.height, 140.0);
    }

    #[test]
    fn edit_arrow_move_vs_resize() {
        let work = WorkArea::new(0.0, 0.0, 1000.0, 1000.0).unwrap();
        let r = SurfaceRect::new(50.0, 50.0, 100.0, 100.0).unwrap();
        let moved = apply_edit_arrow(r, EditArrow::Right, false, false, work);
        assert_eq!(moved.x, 51.0);
        let large = apply_edit_arrow(r, EditArrow::Down, false, true, work);
        assert_eq!(large.y, 60.0);
        let resized = apply_edit_arrow(r, EditArrow::Right, true, false, work);
        assert_eq!(resized.width, 101.0);
        assert_eq!(resized.x, 50.0);
    }
}
