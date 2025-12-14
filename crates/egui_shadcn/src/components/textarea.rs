//! Textarea component matching shadcn/ui exactly
//!
//! Multi-line text input component.
//!
//! Reference: <https://ui.shadcn.com/docs/components/textarea>

use egui::{Response, TextEdit, Ui, Widget};
use crate::theme::ShadcnTheme;

/// Textarea widget matching shadcn/ui design
///
/// ## Example
/// ```rust,ignore
/// let mut text = String::new();
/// Textarea::new(&mut text)
///     .placeholder("Type your message here...")
///     .rows(4)
///     .ui(ui);
/// ```
pub struct Textarea<'a> {
    text: &'a mut String,
    placeholder: Option<String>,
    rows: usize,
    enabled: bool,
}

impl<'a> Textarea<'a> {
    /// Create a new textarea bound to a string
    pub fn new(text: &'a mut String) -> Self {
        Self {
            text,
            placeholder: None,
            rows: 3,
            enabled: true,
        }
    }

    /// Set placeholder text
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// Set the number of visible rows
    pub fn rows(mut self, rows: usize) -> Self {
        self.rows = rows;
        self
    }

    /// Set whether the textarea is enabled
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

impl<'a> Widget for Textarea<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(egui::Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        // Calculate height based on rows
        let line_height = theme.typography.body().size * 1.5;
        let height = line_height * self.rows as f32;

        // shadcn textarea styling
        let bg_color = theme.colors.background;
        let border_color = theme.colors.input;
        let text_color = if self.enabled {
            theme.colors.foreground
        } else {
            theme.colors.muted_foreground
        };

        // Create the frame
        let frame = egui::Frame::NONE
            .fill(bg_color)
            .stroke(egui::Stroke::new(1.0, border_color))
            .corner_radius(theme.radii.md)
            .inner_margin(theme.spacing.vec2_xy(3, 2)); // 12px x 8px padding

        frame.show(ui, |ui| {
            ui.set_min_height(height);

            let mut text_edit = TextEdit::multiline(self.text)
                .font(egui::FontId::proportional(theme.typography.body().size))
                .text_color(text_color)
                .desired_width(f32::INFINITY)
                .desired_rows(self.rows);

            if let Some(placeholder) = self.placeholder {
                text_edit = text_edit.hint_text(placeholder);
            }

            let response = ui.add_enabled(self.enabled, text_edit);

            // Draw focus ring on focus
            if response.has_focus() && self.enabled {
                let rect = response.rect.expand(2.0);
                theme.draw_focus_ring(ui.painter(), rect, theme.radii.md, false);
            }

            response
        }).inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_textarea_creation() {
        let mut text = String::new();
        let textarea = Textarea::new(&mut text);
        assert_eq!(textarea.rows, 3);
        assert!(textarea.enabled);
    }

    #[test]
    fn test_textarea_with_options() {
        let mut text = String::new();
        let textarea = Textarea::new(&mut text)
            .placeholder("Enter text")
            .rows(5)
            .enabled(false);
        assert_eq!(textarea.rows, 5);
        assert!(!textarea.enabled);
        assert_eq!(textarea.placeholder, Some("Enter text".to_string()));
    }
}
