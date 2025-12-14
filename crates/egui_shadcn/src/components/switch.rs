//! Switch/Toggle component matching shadcn/ui exactly
//!
//! Provides a toggle switch for on/off states.
//!
//! Reference: <https://ui.shadcn.com/docs/components/switch>

use egui::{Response, Sense, Ui, Widget};
use crate::theme::ShadcnTheme;

/// Switch/Toggle widget matching shadcn/ui design
///
/// ## Example
/// ```rust,ignore
/// let mut enabled = false;
/// if Switch::new(&mut enabled).ui(ui).changed() {
///     // Handle change
/// }
///
/// // With label
/// if Switch::new(&mut enabled).label("Enable notifications").ui(ui).changed() {
///     // Handle change
/// }
/// ```
pub struct Switch<'a> {
    checked: &'a mut bool,
    label: Option<String>,
    enabled: bool,
}

impl<'a> Switch<'a> {
    /// Create a new switch bound to a boolean value
    pub fn new(checked: &'a mut bool) -> Self {
        Self {
            checked,
            label: None,
            enabled: true,
        }
    }

    /// Add a label to the switch
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set whether the switch is enabled
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

impl<'a> Widget for Switch<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(egui::Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        // shadcn switch dimensions - slightly larger for better visibility
        let width = 48.0;  // Wider track
        let visual_height = 26.0;  // Taller track
        let thumb_size = 22.0;  // Larger thumb
        let touch_target = 44.0; // Apple HIG minimum touch target
        let spacing = 8.0; // 8px spacing between switch and label

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

        // Layout: use touch_target for hit area height, switch is centered within
        let total_width = width + if self.label.is_some() { spacing + label_width } else { 0.0 };
        let (mut response, painter) = ui.allocate_painter(
            egui::vec2(total_width, touch_target),
            sense,
        );

        if response.clicked() && self.enabled {
            *self.checked = !*self.checked;
            response.mark_changed();
        }

        if ui.is_rect_visible(response.rect) {
            // Center the visual switch vertically within the touch target
            let visual_offset_y = (touch_target - visual_height) / 2.0;
            let track_rect = egui::Rect::from_min_size(
                response.rect.min + egui::vec2(0.0, visual_offset_y),
                egui::vec2(width, visual_height)
            );

            let hovered = response.hovered() && self.enabled;

            // Track colors based on state - matching shadcn exactly
            let track_color = if *self.checked {
                // Checked/ON state: solid primary color
                if hovered {
                    Self::darken(theme.colors.primary, 0.1)
                } else {
                    theme.colors.primary
                }
            } else {
                // Unchecked/OFF state: visible gray track
                // shadcn uses a medium gray that's clearly visible
                if hovered {
                    theme.colors.foreground.linear_multiply(0.35)
                } else {
                    theme.colors.foreground.linear_multiply(0.25) // More visible off state
                }
            };

            // Draw track - fully rounded pill shape
            let rounding = visual_height / 2.0; // Full pill shape

            // Draw track fill
            painter.rect_filled(track_rect, rounding, track_color);

            // Draw focus ring on hover with proper offset
            if hovered {
                theme.draw_focus_ring(&painter, track_rect, visual_height / 2.0, true);
            }

            // Calculate thumb position with animation (simplified - egui will smooth this)
            let thumb_padding = 2.0;
            let thumb_travel = width - thumb_size - 2.0 * thumb_padding;
            let thumb_x = if *self.checked {
                track_rect.min.x + thumb_padding + thumb_travel
            } else {
                track_rect.min.x + thumb_padding
            };

            let thumb_center = egui::pos2(
                thumb_x + thumb_size / 2.0,
                track_rect.center().y,
            );

            // Draw thumb shadow for depth (shadcn style)
            let shadow_offset = egui::vec2(0.0, 1.0);
            let shadow_color = egui::Color32::from_black_alpha(25);
            painter.circle(
                thumb_center + shadow_offset,
                thumb_size / 2.0 + 0.5,
                shadow_color,
                egui::Stroke::NONE,
            );

            // Draw thumb
            painter.circle(
                thumb_center,
                thumb_size / 2.0,
                theme.colors.background, // White/background colored thumb
                egui::Stroke::new(0.5, egui::Color32::from_black_alpha(10)), // Subtle border
            );

            // Draw label if present - vertically centered with switch
            if let Some(label) = self.label {
                let label_pos = egui::pos2(
                    track_rect.max.x + spacing,
                    track_rect.center().y, // Vertically center with switch
                );

                painter.text(
                    label_pos,
                    egui::Align2::LEFT_CENTER, // Center-align vertically
                    label,
                    egui::FontId::proportional(theme.typography.body().size),
                    if self.enabled {
                        theme.colors.foreground
                    } else {
                        // Disabled: use 50% foreground for sufficient contrast
                        theme.colors.foreground.linear_multiply(0.5)
                    },
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

impl Switch<'_> {
    /// Lighten a color
    fn lighten(color: egui::Color32, factor: f32) -> egui::Color32 {
        let [r, g, b, a] = color.to_array();
        let r = (r as f32 + (255.0 - r as f32) * factor).min(255.0) as u8;
        let g = (g as f32 + (255.0 - g as f32) * factor).min(255.0) as u8;
        let b = (b as f32 + (255.0 - b as f32) * factor).min(255.0) as u8;
        egui::Color32::from_rgba_premultiplied(r, g, b, a)
    }

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
    fn test_switch_creation() {
        let mut checked = false;
        let switch = Switch::new(&mut checked);
        assert!(switch.enabled);
    }

    #[test]
    fn test_switch_with_label() {
        let mut checked = false;
        let switch = Switch::new(&mut checked).label("Test");
        assert_eq!(switch.label, Some("Test".to_string()));
    }
}
