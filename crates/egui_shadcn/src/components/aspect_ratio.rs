//! Aspect Ratio component ported from shadcn/ui
//!
//! A container that maintains a specific aspect ratio for its content,
//! useful for responsive layouts where content should scale proportionally.
//!
//! ## When to use AspectRatio
//!
//! - **Images & thumbnails**: Ensure images maintain consistent proportions
//! - **Video embeds**: Create 16:9 containers for video players
//! - **Cards & tiles**: Create uniform grid layouts with consistent item shapes
//! - **Responsive design**: Content that should scale while preserving proportions
//!
//! ## How it works
//!
//! The component takes the available width (or a constrained width via `max_width`)
//! and calculates the height based on the specified ratio. Content is rendered
//! within this fixed-proportion container with clipping enabled.
//!
//! Reference: <https://ui.shadcn.com/docs/components/aspect-ratio>

use egui::{Response, Ui, Vec2};

/// Common aspect ratios
#[derive(Debug, Clone, Copy)]
pub enum AspectRatioPreset {
    /// 1:1 square
    Square,
    /// 16:9 widescreen video
    Video,
    /// 4:3 standard
    Standard,
    /// 3:2 photo
    Photo,
    /// 21:9 ultrawide
    Ultrawide,
    /// Custom ratio (width / height)
    Custom(f32),
}

impl AspectRatioPreset {
    /// Get the ratio value (width / height)
    pub fn value(&self) -> f32 {
        match self {
            AspectRatioPreset::Square => 1.0,
            AspectRatioPreset::Video => 16.0 / 9.0,
            AspectRatioPreset::Standard => 4.0 / 3.0,
            AspectRatioPreset::Photo => 3.0 / 2.0,
            AspectRatioPreset::Ultrawide => 21.0 / 9.0,
            AspectRatioPreset::Custom(ratio) => *ratio,
        }
    }
}

/// Aspect Ratio container component
///
/// ## Example
/// ```rust,ignore
/// // 16:9 video container
/// AspectRatio::new(AspectRatioPreset::Video)
///     .show(ui, |ui| {
///         ui.label("Video content here");
///     });
///
/// // Custom 2:1 ratio
/// AspectRatio::ratio(2.0)
///     .show(ui, |ui| {
///         ui.label("Wide content");
///     });
/// ```
pub struct AspectRatio {
    ratio: f32,
    max_width: Option<f32>,
    min_width: Option<f32>,
}

impl AspectRatio {
    /// Create a new aspect ratio container with a preset
    pub fn new(preset: AspectRatioPreset) -> Self {
        Self {
            ratio: preset.value(),
            max_width: None,
            min_width: None,
        }
    }

    /// Create with a custom ratio (width / height)
    ///
    /// For example, `ratio(16.0 / 9.0)` creates a 16:9 container.
    pub fn ratio(ratio: f32) -> Self {
        Self {
            ratio,
            max_width: None,
            min_width: None,
        }
    }

    /// Set maximum width constraint
    pub fn max_width(mut self, width: f32) -> Self {
        self.max_width = Some(width);
        self
    }

    /// Set minimum width constraint
    pub fn min_width(mut self, width: f32) -> Self {
        self.min_width = Some(width);
        self
    }

    /// Show the aspect ratio container with content
    pub fn show<R>(self, ui: &mut Ui, content: impl FnOnce(&mut Ui) -> R) -> AspectRatioResponse<R> {
        // Calculate width based on available space and constraints
        let mut width = ui.available_width();

        if let Some(max) = self.max_width {
            width = width.min(max);
        }
        if let Some(min) = self.min_width {
            width = width.max(min);
        }

        // Calculate height based on ratio
        let height = width / self.ratio;
        let size = Vec2::new(width, height);

        // Allocate the space (with click sense for interactivity)
        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

        // Create a child UI constrained to the rect
        let mut inner = None;
        if ui.is_rect_visible(rect) {
            #[allow(deprecated)]
            ui.allocate_ui_at_rect(rect, |ui| {
                ui.set_clip_rect(rect);
                inner = Some(content(ui));
            });
        }

        AspectRatioResponse {
            inner,
            response,
            rect,
        }
    }
}

/// Response from showing an aspect ratio container
pub struct AspectRatioResponse<R> {
    /// The return value from the content closure (None if not visible)
    pub inner: Option<R>,
    /// The response from the container
    pub response: Response,
    /// The actual rect used for the container
    pub rect: egui::Rect,
}

impl<R> AspectRatioResponse<R> {
    /// Get the actual size used
    pub fn size(&self) -> Vec2 {
        self.rect.size()
    }

    /// Get the width
    pub fn width(&self) -> f32 {
        self.rect.width()
    }

    /// Get the height
    pub fn height(&self) -> f32 {
        self.rect.height()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aspect_ratio_presets() {
        assert_eq!(AspectRatioPreset::Square.value(), 1.0);
        assert!((AspectRatioPreset::Video.value() - 16.0 / 9.0).abs() < 0.001);
        assert!((AspectRatioPreset::Standard.value() - 4.0 / 3.0).abs() < 0.001);
    }

    #[test]
    fn test_aspect_ratio_custom() {
        let custom = AspectRatioPreset::Custom(2.5);
        assert_eq!(custom.value(), 2.5);
    }

    #[test]
    fn test_aspect_ratio_creation() {
        let ar = AspectRatio::ratio(2.0)
            .max_width(800.0)
            .min_width(200.0);

        assert_eq!(ar.ratio, 2.0);
        assert_eq!(ar.max_width, Some(800.0));
        assert_eq!(ar.min_width, Some(200.0));
    }
}
