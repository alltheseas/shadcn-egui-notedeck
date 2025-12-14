//! Checkbox component matching shadcn/ui exactly
//!
//! Provides checkbox with checked, unchecked, and indeterminate states.
//!
//! Reference: <https://ui.shadcn.com/docs/components/checkbox>

use egui::{Response, Sense, Ui, Widget};
use crate::theme::ShadcnTheme;

/// Checkbox widget matching shadcn/ui design
///
/// ## Example
/// ```rust,ignore
/// let mut checked = false;
/// if Checkbox::new(&mut checked).ui(ui).changed() {
///     // Handle change
/// }
///
/// // With label
/// if Checkbox::new(&mut checked).label("Accept terms").ui(ui).changed() {
///     // Handle change
/// }
/// ```
pub struct Checkbox<'a> {
    checked: &'a mut bool,
    label: Option<String>,
    description: Option<String>,
    indeterminate: bool,
    enabled: bool,
}

impl<'a> Checkbox<'a> {
    /// Create a new checkbox bound to a boolean value
    pub fn new(checked: &'a mut bool) -> Self {
        Self {
            checked,
            label: None,
            description: None,
            indeterminate: false,
            enabled: true,
        }
    }

    /// Add a label to the checkbox
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Add a description below the label
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the checkbox to indeterminate state
    pub fn indeterminate(mut self, indeterminate: bool) -> Self {
        self.indeterminate = indeterminate;
        self
    }

    /// Set whether the checkbox is enabled
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

impl<'a> Widget for Checkbox<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(egui::Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        let visual_size = 18.0; // Slightly larger for better visibility
        let touch_target = 44.0; // Apple HIG minimum touch target
        let spacing = 8.0; // 8px spacing between checkbox and label

        let sense = if self.enabled {
            Sense::click()
        } else {
            Sense::hover()
        };

        // Calculate label width if present
        let label_width = if let Some(ref label_text) = self.label {
            let font_id = egui::FontId::proportional(theme.typography.body().size);
            ui.painter().layout_no_wrap(
                label_text.clone(),
                font_id,
                theme.colors.foreground,
            ).size().x
        } else {
            0.0
        };

        // Calculate description dimensions if present
        let desc_width = if let Some(ref desc_text) = self.description {
            let font_id = egui::FontId::proportional(theme.typography.small().size);
            ui.painter().layout_no_wrap(
                desc_text.clone(),
                font_id,
                theme.colors.muted_foreground,
            ).size().x
        } else {
            0.0
        };

        let desc_height = if self.description.is_some() {
            theme.typography.small().size + 8.0 // small text + spacing
        } else {
            0.0
        };

        // Height increases if we have description
        let total_height = if self.description.is_some() {
            touch_target + desc_height
        } else {
            touch_target
        };

        // Width should accommodate both label and description (whichever is wider)
        let text_width = label_width.max(desc_width);
        let total_width = touch_target + if self.label.is_some() || self.description.is_some() { spacing + text_width } else { 0.0 };
        let (mut response, painter) = ui.allocate_painter(
            egui::vec2(total_width.max(ui.available_width()), total_height),
            sense,
        );

        if response.clicked() && self.enabled {
            *self.checked = !*self.checked;
            response.mark_changed();
        }

        if ui.is_rect_visible(response.rect) {
            // Center the visual checkbox vertically within the touch target area (not including description)
            let visual_offset_x = (touch_target - visual_size) / 2.0;
            let visual_offset_y = if self.description.is_some() {
                // Align checkbox with top of label when description present
                (touch_target - visual_size) / 2.0
            } else {
                (touch_target - visual_size) / 2.0
            };
            let rect = egui::Rect::from_min_size(
                response.rect.min + egui::vec2(visual_offset_x, visual_offset_y),
                egui::vec2(visual_size, visual_size)
            );

            let hovered = response.hovered() && self.enabled;
            let pressed = response.is_pointer_button_down_on() && self.enabled;

            // shadcn checkbox: square with very small corner radius (2px)
            let rounding = 3.0; // Near-square corners like shadcn

            // Colors based on state - matching shadcn exactly
            let (bg_color, border_color, border_width) = if *self.checked || self.indeterminate {
                // Checked state: solid primary background
                let bg = if pressed {
                    Self::darken(theme.colors.primary, 0.2)
                } else if hovered {
                    Self::darken(theme.colors.primary, 0.1)
                } else {
                    theme.colors.primary
                };
                (bg, bg, 0.0) // No visible border when checked
            } else {
                // Unchecked state: transparent background with visible border
                let border = if hovered {
                    theme.colors.foreground // Full contrast on hover
                } else {
                    theme.colors.foreground.linear_multiply(0.5) // Visible but not harsh
                };
                (egui::Color32::TRANSPARENT, border, 2.0) // Thicker border for visibility
            };

            // Draw fill
            painter.rect_filled(rect, rounding, bg_color);

            // Draw border stroke for unchecked state
            if border_width > 0.0 {
                painter.rect_stroke(
                    rect,
                    rounding,
                    egui::Stroke::new(border_width, border_color),
                    egui::StrokeKind::Inside,
                );
            }

            // Draw focus ring on hover
            if hovered {
                theme.draw_focus_ring(&painter, rect, rounding, true);
            }

            // Draw checkmark or indeterminate line
            if *self.checked {
                // Draw bold checkmark matching shadcn style
                let check_color = theme.colors.primary_foreground;
                let center = rect.center();

                // Larger, more prominent checkmark
                let scale = visual_size * 0.35;

                // Checkmark shape: short line down-left, then longer line up-right
                let p1 = center + egui::vec2(-scale * 0.9, -scale * 0.1);
                let p2 = center + egui::vec2(-scale * 0.2, scale * 0.6);
                let p3 = center + egui::vec2(scale * 0.9, -scale * 0.5);

                // Thicker stroke for bold checkmark (2.5px)
                let stroke = egui::Stroke::new(2.5, check_color);
                painter.line_segment([p1, p2], stroke);
                painter.line_segment([p2, p3], stroke);
            } else if self.indeterminate {
                // Draw horizontal line for indeterminate state
                let line_color = theme.colors.primary_foreground;
                let center = rect.center();
                let line_width = visual_size * 0.5;

                painter.line_segment(
                    [
                        center + egui::vec2(-line_width / 2.0, 0.0),
                        center + egui::vec2(line_width / 2.0, 0.0),
                    ],
                    egui::Stroke::new(2.5, line_color),
                );
            }

            // Draw label if present
            let label_x = response.rect.min.x + touch_target + spacing;
            if let Some(ref label) = self.label {
                let label_y = if self.description.is_some() {
                    // Align with checkbox center when there's description
                    response.rect.min.y + touch_target / 2.0
                } else {
                    response.rect.center().y
                };

                painter.text(
                    egui::pos2(label_x, label_y),
                    egui::Align2::LEFT_CENTER,
                    label,
                    egui::FontId::proportional(theme.typography.body().size),
                    if self.enabled {
                        theme.colors.foreground
                    } else {
                        theme.colors.foreground.linear_multiply(0.5)
                    },
                );
            }

            // Draw description if present (below label)
            if let Some(ref description) = self.description {
                let desc_y = response.rect.min.y + touch_target / 2.0 + theme.typography.body().size / 2.0 + 6.0;
                painter.text(
                    egui::pos2(label_x, desc_y),
                    egui::Align2::LEFT_TOP,
                    description,
                    egui::FontId::proportional(theme.typography.small().size),
                    theme.colors.muted_foreground,
                );
            }
        }

        if !self.enabled {
            response.on_disabled_hover_text("Disabled")
        } else {
            response
        }
    }
}

impl Checkbox<'_> {
    /// Darken a color
    fn darken(color: egui::Color32, factor: f32) -> egui::Color32 {
        let [r, g, b, a] = color.to_array();
        let r = (r as f32 * (1.0 - factor)).max(0.0) as u8;
        let g = (g as f32 * (1.0 - factor)).max(0.0) as u8;
        let b = (b as f32 * (1.0 - factor)).max(0.0) as u8;
        egui::Color32::from_rgba_premultiplied(r, g, b, a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checkbox_creation() {
        let mut checked = false;
        let checkbox = Checkbox::new(&mut checked);
        assert!(!checkbox.indeterminate);
        assert!(checkbox.enabled);
    }

    #[test]
    fn test_checkbox_with_label() {
        let mut checked = false;
        let checkbox = Checkbox::new(&mut checked).label("Test");
        assert_eq!(checkbox.label, Some("Test".to_string()));
    }
}
