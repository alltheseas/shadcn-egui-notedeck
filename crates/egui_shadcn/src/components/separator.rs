//! Separator component ported from shadcn/ui
//!
//! A visual divider for separating content.
//!
//! Reference: <https://ui.shadcn.com/docs/components/separator>

use egui::{Response, Ui, Vec2};
use crate::theme::ShadcnTheme;

/// Orientation for separator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeparatorOrientation {
    /// Horizontal separator (default)
    Horizontal,
    /// Vertical separator
    Vertical,
}

/// Separator component for dividing content
///
/// ## Example
/// ```rust,ignore
/// Separator::horizontal(ui);
/// Separator::vertical(ui).with_height(100.0);
/// ```
pub struct Separator {
    orientation: SeparatorOrientation,
    size: Option<f32>,
}

impl Separator {
    /// Create a horizontal separator
    pub fn horizontal() -> Self {
        Self {
            orientation: SeparatorOrientation::Horizontal,
            size: None,
        }
    }

    /// Create a vertical separator
    pub fn vertical() -> Self {
        Self {
            orientation: SeparatorOrientation::Vertical,
            size: None,
        }
    }

    /// Set the size (width for horizontal, height for vertical)
    pub fn with_size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }

    /// Show the separator
    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(egui::Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        match self.orientation {
            SeparatorOrientation::Horizontal => {
                ui.add_space(theme.spacing.sm);
                let response = ui.separator();
                ui.add_space(theme.spacing.sm);
                response
            }
            SeparatorOrientation::Vertical => {
                let height = self.size.unwrap_or(ui.available_height());
                ui.allocate_ui_with_layout(
                    Vec2::new(1.0, height),
                    egui::Layout::top_down(egui::Align::Center),
                    |ui| {
                        ui.separator()
                    },
                ).inner
            }
        }
    }
}
