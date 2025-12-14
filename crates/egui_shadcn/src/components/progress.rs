//! Progress bar component matching shadcn/ui exactly
//!
//! Provides a progress indicator for showing completion status.
//!
//! Reference: <https://ui.shadcn.com/docs/components/progress>

use egui::{Response, Ui, Widget};
use crate::theme::ShadcnTheme;

/// Progress bar widget matching shadcn/ui design
///
/// ## Example
/// ```rust,ignore
/// Progress::new(0.75).ui(ui); // 75% complete
///
/// // Custom height
/// Progress::new(0.5).height(8.0).ui(ui);
/// ```
pub struct Progress {
    value: f32, // 0.0 to 1.0
    height: f32,
    indeterminate: bool,
}

impl Progress {
    /// Create a new progress bar with a value from 0.0 to 1.0
    pub fn new(value: f32) -> Self {
        Self {
            value: value.clamp(0.0, 1.0),
            height: 8.0, // shadcn default height
            indeterminate: false,
        }
    }

    /// Set custom height
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Set indeterminate mode (animated loading)
    pub fn indeterminate(mut self, indeterminate: bool) -> Self {
        self.indeterminate = indeterminate;
        self
    }
}

impl Widget for Progress {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(egui::Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        let desired_width = ui.available_width();
        let desired_size = egui::vec2(desired_width, self.height);

        let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();

            // Draw background track
            painter.rect_filled(
                rect,
                theme.radii.progress(),
                theme.colors.secondary, // Light gray background
            );

            // Draw progress fill
            let fill_width = if self.indeterminate {
                // Animated indeterminate mode (simplified - no animation in this version)
                desired_width * 0.3
            } else {
                desired_width * self.value
            };

            if fill_width > 0.0 {
                let fill_rect = egui::Rect::from_min_size(
                    rect.min,
                    egui::vec2(fill_width, self.height),
                );

                painter.rect_filled(
                    fill_rect,
                    theme.radii.progress(),
                    theme.colors.primary, // Purple fill
                );
            }
        }

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_creation() {
        let progress = Progress::new(0.5);
        assert_eq!(progress.value, 0.5);
        assert_eq!(progress.height, 8.0);
    }

    #[test]
    fn test_progress_clamping() {
        let progress = Progress::new(1.5);
        assert_eq!(progress.value, 1.0);

        let progress = Progress::new(-0.5);
        assert_eq!(progress.value, 0.0);
    }
}
