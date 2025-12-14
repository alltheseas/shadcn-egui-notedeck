//! Kbd component ported from shadcn/ui
//!
//! Displays keyboard shortcuts in a styled format.
//!
//! Reference: <https://ui.shadcn.com/docs/components/kbd>

use egui::{Response, Ui, Widget};
use crate::theme::ShadcnTheme;

/// Kbd component for displaying keyboard shortcuts
///
/// ## Example
/// ```rust,ignore
/// ui.add(Kbd::new("Ctrl"));
/// ui.add(Kbd::new("K"));
/// ```
pub struct Kbd {
    text: String,
}

impl Kbd {
    /// Create a new keyboard shortcut display
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
        }
    }
}

impl Widget for Kbd {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(egui::Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        // shadcn kbd style: subtle background with clear border
        let bg_color = theme.colors.muted;
        let border_color = theme.colors.foreground.linear_multiply(0.2);

        let frame = egui::Frame::NONE
            .fill(bg_color)
            .stroke(egui::Stroke::new(1.0, border_color))
            .inner_margin(5.0) // Compact padding
            .corner_radius(4.0) // Small, consistent radius
            .shadow(theme.shadows.xs2); // Subtle bottom shadow for "key" effect

        frame.show(ui, |ui| {
            ui.label(
                egui::RichText::new(&self.text)
                    .size(12.0) // Slightly smaller for compact look
                    .family(egui::FontFamily::Monospace)
                    .color(theme.colors.foreground.linear_multiply(0.8)),
            );
        }).response
    }
}
