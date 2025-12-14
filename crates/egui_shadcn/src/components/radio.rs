//! Radio Group component ported from shadcn/ui
//!
//! A set of radio buttons for selecting a single option from a list.
//!
//! Reference: <https://ui.shadcn.com/docs/components/radio-group>

use egui::{Response, Sense, Ui, Widget};
use crate::theme::ShadcnTheme;

/// A single radio button item
///
/// ## Example
/// ```rust,ignore
/// let mut selected = 0;
///
/// RadioGroup::new("options", &mut selected)
///     .option("Option 1")
///     .option("Option 2")
///     .option("Option 3")
///     .show(ui);
/// ```
pub struct RadioGroup<'a> {
    id: &'a str,
    selected: &'a mut usize,
    options: Vec<RadioOption>,
    enabled: bool,
}

/// Configuration for a radio option
struct RadioOption {
    label: String,
    description: Option<String>,
    enabled: bool,
}

impl<'a> RadioGroup<'a> {
    /// Create a new radio group bound to a selection index
    pub fn new(id: &'a str, selected: &'a mut usize) -> Self {
        Self {
            id,
            selected,
            options: Vec::new(),
            enabled: true,
        }
    }

    /// Add an option with just a label
    pub fn option(mut self, label: impl Into<String>) -> Self {
        self.options.push(RadioOption {
            label: label.into(),
            description: None,
            enabled: true,
        });
        self
    }

    /// Add an option with label and description
    pub fn option_with_description(
        mut self,
        label: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        self.options.push(RadioOption {
            label: label.into(),
            description: Some(description.into()),
            enabled: true,
        });
        self
    }

    /// Set whether the entire group is enabled
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Show the radio group
    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(egui::Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        let _id = egui::Id::new(self.id);
        let visual_size = 20.0; // Larger for better visibility
        let touch_target = 44.0; // Apple HIG minimum
        let dot_radius = 6.0; // Larger inner dot for clear selection state
        let border_width = 2.0; // Thicker border for visibility
        let spacing = 12.0; // Gap between options (gap-3)

        let mut changed = false;
        let mut group_response: Option<Response> = None;

        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = spacing;

            for (idx, option) in self.options.iter().enumerate() {
                let is_selected = idx == *self.selected;
                let item_enabled = self.enabled && option.enabled;

                let sense = if item_enabled {
                    Sense::click()
                } else {
                    Sense::hover()
                };

                // Calculate total width including label
                let label_galley = ui.painter().layout_no_wrap(
                    option.label.clone(),
                    egui::FontId::proportional(theme.typography.body().size),
                    theme.colors.foreground,
                );
                let label_width = label_galley.size().x;

                let total_width = touch_target + 8.0 + label_width;
                let height = if option.description.is_some() {
                    touch_target.max(44.0) // Ensure touch target even with description
                } else {
                    touch_target
                };

                let (response, painter) = ui.allocate_painter(
                    egui::vec2(total_width, height),
                    sense,
                );

                if response.clicked() && item_enabled {
                    *self.selected = idx;
                    changed = true;
                }

                if ui.is_rect_visible(response.rect) {
                    let hovered = response.hovered() && item_enabled;

                    // Center the visual radio within the touch target
                    let _visual_offset = (touch_target - visual_size) / 2.0;
                    let radio_center = egui::pos2(
                        response.rect.min.x + touch_target / 2.0,
                        response.rect.min.y + touch_target / 2.0,
                    );

                    // Outer circle colors - more visible border
                    let border_color = if is_selected {
                        theme.colors.primary
                    } else if hovered {
                        theme.colors.foreground // Full contrast on hover
                    } else {
                        theme.colors.foreground.linear_multiply(0.5) // Visible unselected state
                    };

                    // Apply disabled styling
                    let border_color = if !item_enabled {
                        border_color.linear_multiply(0.5)
                    } else {
                        border_color
                    };

                    // Draw outer circle with thicker border
                    painter.circle(
                        radio_center,
                        visual_size / 2.0,
                        egui::Color32::TRANSPARENT,
                        egui::Stroke::new(border_width, border_color),
                    );

                    // Draw inner dot when selected
                    if is_selected {
                        let dot_color = if item_enabled {
                            theme.colors.primary
                        } else {
                            theme.colors.primary.linear_multiply(0.5)
                        };
                        painter.circle_filled(radio_center, dot_radius, dot_color);
                    }

                    // Draw focus ring on hover
                    if hovered {
                        let ring_rect = egui::Rect::from_center_size(
                            radio_center,
                            egui::vec2(visual_size, visual_size),
                        );
                        theme.draw_focus_ring(&painter, ring_rect, visual_size / 2.0, true);
                    }

                    // Draw label - vertically centered
                    let label_x = response.rect.min.x + touch_target + 8.0;
                    let label_y = if option.description.is_some() {
                        response.rect.min.y + touch_target / 2.0 - label_galley.size().y / 2.0 - 6.0
                    } else {
                        response.rect.center().y - label_galley.size().y / 2.0
                    };

                    let label_color = if item_enabled {
                        theme.colors.foreground
                    } else {
                        theme.colors.foreground.linear_multiply(0.5)
                    };

                    // Store height before moving galley
                    let label_height = label_galley.size().y;

                    painter.galley(
                        egui::pos2(label_x, label_y),
                        label_galley,
                        label_color,
                    );

                    // Draw description if present
                    if let Some(ref desc) = option.description {
                        let desc_galley = ui.painter().layout_no_wrap(
                            desc.clone(),
                            egui::FontId::proportional(theme.typography.small().size),
                            theme.colors.foreground.linear_multiply(0.7),
                        );

                        let desc_color = if item_enabled {
                            theme.colors.foreground.linear_multiply(0.7)
                        } else {
                            theme.colors.foreground.linear_multiply(0.35)
                        };

                        painter.galley(
                            egui::pos2(label_x, label_y + label_height + 2.0),
                            desc_galley,
                            desc_color,
                        );
                    }
                }

                if let Some(ref mut gr) = group_response {
                    *gr = gr.union(response);
                } else {
                    group_response = Some(response);
                }
            }
        });

        let mut response = group_response.unwrap_or_else(|| {
            ui.allocate_response(egui::vec2(0.0, 0.0), Sense::hover())
        });

        if changed {
            response.mark_changed();
        }

        response
    }
}

/// Single radio button widget (for standalone use)
///
/// ## Example
/// ```rust,ignore
/// let mut selected = false;
/// if RadioButton::new(&mut selected, "Option A").ui(ui).changed() {
///     // Handle selection
/// }
/// ```
pub struct RadioButton<'a> {
    selected: &'a mut bool,
    label: Option<String>,
    enabled: bool,
}

impl<'a> RadioButton<'a> {
    /// Create a new radio button
    pub fn new(selected: &'a mut bool) -> Self {
        Self {
            selected,
            label: None,
            enabled: true,
        }
    }

    /// Add a label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set whether enabled
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

impl<'a> Widget for RadioButton<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(egui::Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        let visual_size = 20.0; // Larger for better visibility
        let touch_target = 44.0;
        let dot_radius = 6.0; // Larger inner dot
        let border_width = 2.0; // Thicker border
        let spacing = 8.0;

        let sense = if self.enabled {
            Sense::click()
        } else {
            Sense::hover()
        };

        // Calculate label width
        let label_width = if let Some(ref label_text) = self.label {
            ui.painter().layout_no_wrap(
                label_text.clone(),
                egui::FontId::proportional(theme.typography.body().size),
                theme.colors.foreground,
            ).size().x
        } else {
            0.0
        };

        let total_width = touch_target + if self.label.is_some() { spacing + label_width } else { 0.0 };
        let (mut response, painter) = ui.allocate_painter(
            egui::vec2(total_width, touch_target),
            sense,
        );

        if response.clicked() && self.enabled {
            *self.selected = !*self.selected;
            response.mark_changed();
        }

        if ui.is_rect_visible(response.rect) {
            let hovered = response.hovered() && self.enabled;

            let radio_center = egui::pos2(
                response.rect.min.x + touch_target / 2.0,
                response.rect.center().y,
            );

            // Colors - more visible border
            let border_color = if *self.selected {
                theme.colors.primary
            } else if hovered {
                theme.colors.foreground // Full contrast on hover
            } else {
                theme.colors.foreground.linear_multiply(0.5) // Visible unselected state
            };

            let border_color = if self.enabled {
                border_color
            } else {
                border_color.linear_multiply(0.5)
            };

            // Outer circle with thicker border
            painter.circle(
                radio_center,
                visual_size / 2.0,
                egui::Color32::TRANSPARENT,
                egui::Stroke::new(border_width, border_color),
            );

            // Inner dot
            if *self.selected {
                let dot_color = if self.enabled {
                    theme.colors.primary
                } else {
                    theme.colors.primary.linear_multiply(0.5)
                };
                painter.circle_filled(radio_center, dot_radius, dot_color);
            }

            // Focus ring
            if hovered {
                let ring_rect = egui::Rect::from_center_size(
                    radio_center,
                    egui::vec2(visual_size, visual_size),
                );
                theme.draw_focus_ring(&painter, ring_rect, visual_size / 2.0, true);
            }

            // Label
            if let Some(label) = self.label {
                let label_pos = egui::pos2(
                    response.rect.min.x + touch_target + spacing,
                    response.rect.center().y,
                );

                painter.text(
                    label_pos,
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
        }

        if !self.enabled {
            response.on_disabled_hover_text("Disabled")
        } else {
            response
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radio_group_creation() {
        let mut selected = 0;
        let group = RadioGroup::new("test", &mut selected)
            .option("Option 1")
            .option("Option 2");
        assert_eq!(group.options.len(), 2);
    }

    #[test]
    fn test_radio_button_creation() {
        let mut selected = false;
        let button = RadioButton::new(&mut selected).label("Test");
        assert!(button.label.is_some());
        assert!(button.enabled);
    }
}
