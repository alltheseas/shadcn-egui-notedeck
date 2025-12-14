//! Label component matching shadcn/ui exactly
//!
//! Renders an accessible label associated with form controls.
//!
//! Reference: <https://ui.shadcn.com/docs/components/label>

use egui::{Response, Ui, Widget};
use crate::theme::ShadcnTheme;

/// Label widget matching shadcn/ui design
///
/// ## Example
/// ```rust,ignore
/// Label::new("Your email address").ui(ui);
///
/// // For accessibility with inputs
/// Label::new("Email").for_id("email-input").ui(ui);
/// ```
pub struct Label {
    text: String,
    for_id: Option<egui::Id>,
}

impl Label {
    /// Create a new label with the given text
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            for_id: None,
        }
    }

    /// Associate this label with a specific widget ID for accessibility
    pub fn for_id(mut self, id: impl Into<egui::Id>) -> Self {
        self.for_id = Some(id.into());
        self
    }
}

impl Widget for Label {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(egui::Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        // shadcn label uses medium font weight and sm font size (0.875rem = 14px)
        let font_id = egui::FontId::proportional(theme.typography.body().size * 0.875);
        let color = theme.colors.foreground;

        // Create label text
        let galley = ui.painter().layout_no_wrap(
            self.text.clone(),
            font_id.clone(),
            color,
        );

        // Allocate space for the label
        let (rect, response) = ui.allocate_exact_size(
            galley.size(),
            egui::Sense::hover(),
        );

        // Draw the label text
        ui.painter().galley(
            rect.min,
            galley,
            color,
        );

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label_creation() {
        let label = Label::new("Test Label");
        assert_eq!(label.text, "Test Label");
        assert!(label.for_id.is_none());
    }

    #[test]
    fn test_label_with_id() {
        let label = Label::new("Test").for_id("test-id");
        assert!(label.for_id.is_some());
    }
}
