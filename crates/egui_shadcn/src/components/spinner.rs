//! Spinner component ported from shadcn/ui
//!
//! A loading spinner indicator with animation.
//!
//! Reference: <https://ui.shadcn.com/docs/components/spinner>

use egui::{Response, Ui, Vec2, Widget};
use crate::theme::ShadcnTheme;

/// Size variants for Spinner component
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpinnerSize {
    /// Small spinner: 16px
    Small,
    /// Medium spinner: 24px (default)
    Medium,
    /// Large spinner: 32px
    Large,
    /// Extra large spinner: 40px
    XLarge,
    /// Extra extra large spinner: 48px
    XXLarge,
    /// Extra extra extra large spinner: 56px
    XXXLarge,
}

impl SpinnerSize {
    /// Get the pixel size for this variant
    pub const fn pixels(&self) -> f32 {
        match self {
            SpinnerSize::Small => 16.0,
            SpinnerSize::Medium => 24.0,
            SpinnerSize::Large => 32.0,
            SpinnerSize::XLarge => 40.0,
            SpinnerSize::XXLarge => 48.0,
            SpinnerSize::XXXLarge => 56.0,
        }
    }

    /// Get the stroke width for this size
    pub const fn stroke_width(&self) -> f32 {
        match self {
            SpinnerSize::Small => 2.0,
            SpinnerSize::Medium => 2.5,
            SpinnerSize::Large => 3.0,
            SpinnerSize::XLarge => 3.5,
            SpinnerSize::XXLarge => 4.0,
            SpinnerSize::XXXLarge => 4.5,
        }
    }
}

/// Spinner component for loading states
///
/// A circular animated spinner matching shadcn/ui design.
///
/// ## Example
/// ```rust,ignore
/// use egui_shadcn::components::{Spinner, SpinnerSize};
///
/// // Default spinner
/// ui.add(Spinner::new());
///
/// // Large spinner
/// ui.add(Spinner::new().size(SpinnerSize::Large));
/// ```
#[derive(Debug, Clone)]
pub struct Spinner {
    size: SpinnerSize,
}

impl Spinner {
    /// Create a new spinner with default size
    pub fn new() -> Self {
        Self {
            size: SpinnerSize::Medium,
        }
    }

    /// Set the size of the spinner
    pub fn size(mut self, size: SpinnerSize) -> Self {
        self.size = size;
        self
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for Spinner {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(egui::Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        let size = self.size.pixels();
        let stroke_width = self.size.stroke_width();

        // Allocate space for the spinner
        let (rect, response) = ui.allocate_exact_size(
            Vec2::splat(size),
            egui::Sense::hover(),
        );

        if ui.is_rect_visible(rect) {
            let time = ui.input(|i| i.time);

            // Rotation speed: full rotation every 0.8 seconds
            let angle = (time * std::f64::consts::TAU * 1.25) as f32;

            let center = rect.center();
            let radius = size / 2.0 - stroke_width;

            // Notedeck style: light primary track + solid primary indicator
            let track_color = theme.colors.primary.linear_multiply(0.3);
            let indicator_color = theme.colors.primary;

            // Draw full circle track (background ring)
            let num_points = 48;
            for i in 0..num_points {
                let t1 = i as f32 / num_points as f32 * std::f32::consts::TAU;
                let t2 = (i + 1) as f32 / num_points as f32 * std::f32::consts::TAU;
                let p1 = egui::pos2(center.x + radius * t1.cos(), center.y + radius * t1.sin());
                let p2 = egui::pos2(center.x + radius * t2.cos(), center.y + radius * t2.sin());
                ui.painter().line_segment([p1, p2], egui::Stroke::new(stroke_width, track_color));
            }

            // Draw rotating indicator arc (1/4 of circle)
            let arc_length = std::f32::consts::TAU * 0.25; // 90 degrees
            let arc_points = 12;
            for i in 0..arc_points {
                let t1 = angle + (i as f32 / arc_points as f32) * arc_length;
                let t2 = angle + ((i + 1) as f32 / arc_points as f32) * arc_length;
                let p1 = egui::pos2(center.x + radius * t1.cos(), center.y + radius * t1.sin());
                let p2 = egui::pos2(center.x + radius * t2.cos(), center.y + radius * t2.sin());
                ui.painter().line_segment([p1, p2], egui::Stroke::new(stroke_width, indicator_color));
            }

            // Request continuous repaint for animation
            ui.ctx().request_repaint();
        }

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spinner_creation() {
        let spinner = Spinner::new();
        assert_eq!(spinner.size, SpinnerSize::Medium);
    }

    #[test]
    fn test_spinner_size() {
        let spinner = Spinner::new().size(SpinnerSize::Large);
        assert_eq!(spinner.size, SpinnerSize::Large);

        assert_eq!(SpinnerSize::Small.pixels(), 16.0);
        assert_eq!(SpinnerSize::Medium.pixels(), 24.0);
        assert_eq!(SpinnerSize::Large.pixels(), 32.0);
    }

    #[test]
    fn test_spinner_stroke() {
        assert_eq!(SpinnerSize::Small.stroke_width(), 2.0);
        assert_eq!(SpinnerSize::Medium.stroke_width(), 2.5);
        assert_eq!(SpinnerSize::Large.stroke_width(), 3.0);
    }
}
