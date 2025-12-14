//! Select component ported from shadcn/ui
//!
//! A dropdown select for choosing from a list of options.
//!
//! Reference: <https://ui.shadcn.com/docs/components/select>

use egui::{Response, Ui, Sense, Id};
use crate::theme::ShadcnTheme;

/// Select component for dropdown selection
///
/// ## Example
/// ```rust,ignore
/// let mut selected = 0;
/// Select::new("fruit", &mut selected)
///     .placeholder("Select a fruit...")
///     .option("Apple")
///     .option("Banana")
///     .option("Cherry")
///     .show(ui);
/// ```
pub struct Select<'a> {
    id: &'a str,
    selected: &'a mut usize,
    options: Vec<SelectOption>,
    placeholder: String,
    width: f32,
    enabled: bool,
}

/// Configuration for a select option
struct SelectOption {
    label: String,
    enabled: bool,
}

impl<'a> Select<'a> {
    /// Create a new select bound to a selection index
    pub fn new(id: &'a str, selected: &'a mut usize) -> Self {
        Self {
            id,
            selected,
            options: Vec::new(),
            placeholder: "Select...".to_string(),
            width: 180.0,
            enabled: true,
        }
    }

    /// Set placeholder text shown when nothing is selected
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Add an option
    pub fn option(mut self, label: impl Into<String>) -> Self {
        self.options.push(SelectOption {
            label: label.into(),
            enabled: true,
        });
        self
    }

    /// Set the width of the select trigger
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Set whether the select is enabled
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Show the select
    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(egui::Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        let id = Id::new(self.id);
        let touch_target = 44.0; // Apple HIG minimum
        let chevron_width = 24.0;

        // Use simple boolean state in memory instead of Popup API
        let open_id = id.with("open");
        let is_open = ui.ctx().data(|d| d.get_temp::<bool>(open_id).unwrap_or(false));

        let sense = if self.enabled {
            Sense::click()
        } else {
            Sense::hover()
        };

        // Allocate trigger button
        let (response, painter) = ui.allocate_painter(
            egui::vec2(self.width, touch_target),
            sense,
        );

        // Toggle on click
        let mut new_open_state = is_open;
        if response.clicked() && self.enabled {
            new_open_state = !is_open;
        }

        // Draw trigger
        if ui.is_rect_visible(response.rect) {
            let hovered = response.hovered() && self.enabled;
            let rect = response.rect;

            // Background and border
            let bg_color = theme.colors.background;
            let border_color = if is_open {
                theme.colors.ring
            } else if hovered {
                theme.colors.ring
            } else {
                theme.colors.input
            };

            let rounding = theme.radii.md;

            painter.rect_filled(rect, rounding, bg_color);
            painter.rect_stroke(
                rect,
                rounding,
                egui::Stroke::new(1.0, border_color),
                egui::StrokeKind::Inside,
            );

            // Draw focus ring when open or hovered
            if hovered || is_open {
                theme.draw_focus_ring(&painter, rect, rounding, true);
            }

            // Text (selected value or placeholder)
            let text = if *self.selected < self.options.len() {
                self.options[*self.selected].label.clone()
            } else {
                self.placeholder.clone()
            };

            let text_color = if *self.selected < self.options.len() {
                if self.enabled {
                    theme.colors.foreground
                } else {
                    theme.colors.foreground.linear_multiply(0.5)
                }
            } else {
                theme.colors.foreground.linear_multiply(0.5) // Placeholder color
            };

            let text_rect = egui::Rect::from_min_max(
                rect.min + egui::vec2(12.0, 0.0),
                rect.max - egui::vec2(chevron_width + 8.0, 0.0),
            );

            painter.text(
                egui::pos2(text_rect.min.x, rect.center().y),
                egui::Align2::LEFT_CENTER,
                text,
                egui::FontId::proportional(theme.typography.body().size),
                text_color,
            );

            // Chevron icon
            let chevron_center = egui::pos2(
                rect.max.x - chevron_width / 2.0 - 4.0,
                rect.center().y,
            );
            let chevron_size = 4.0;
            let chevron_color = theme.colors.foreground.linear_multiply(0.5);

            // Draw chevron (down arrow)
            let p1 = chevron_center + egui::vec2(-chevron_size, -chevron_size / 2.0);
            let p2 = chevron_center + egui::vec2(0.0, chevron_size / 2.0);
            let p3 = chevron_center + egui::vec2(chevron_size, -chevron_size / 2.0);

            painter.line_segment([p1, p2], egui::Stroke::new(1.5, chevron_color));
            painter.line_segment([p2, p3], egui::Stroke::new(1.5, chevron_color));
        }

        // Show popup using new_open_state (which reflects the click toggle)
        if new_open_state {
            let popup_area_id = id.with("area");
            let area_response = egui::Area::new(popup_area_id)
                .order(egui::Order::Foreground)
                .fixed_pos(response.rect.left_bottom() + egui::vec2(0.0, 4.0))
                .show(ui.ctx(), |ui| {
                    let frame = egui::Frame::NONE
                        .fill(theme.colors.popover)
                        .stroke(egui::Stroke::new(1.0, theme.colors.border))
                        .corner_radius(theme.radii.md)
                        .shadow(theme.shadows.md)
                        .inner_margin(4.0);

                    frame.show(ui, |ui| {
                        ui.set_min_width(self.width - 8.0);

                        for (idx, option) in self.options.iter().enumerate() {
                            let is_selected = idx == *self.selected;
                            let item_enabled = self.enabled && option.enabled;

                            // 44px minimum for Apple HIG touch target
                            let item_response = ui.allocate_response(
                                egui::vec2(ui.available_width(), 44.0),
                                if item_enabled { Sense::click() } else { Sense::hover() },
                            );

                            if item_response.clicked() && item_enabled {
                                *self.selected = idx;
                                new_open_state = false; // Close on selection
                            }

                            if ui.is_rect_visible(item_response.rect) {
                                let hovered = item_response.hovered() && item_enabled;

                                // Hover background
                                if hovered {
                                    ui.painter().rect_filled(
                                        item_response.rect,
                                        theme.radii.sm,
                                        theme.colors.accent,
                                    );
                                }

                                // Text
                                let text_color = if item_enabled {
                                    if hovered {
                                        theme.colors.accent_foreground
                                    } else {
                                        theme.colors.popover_foreground
                                    }
                                } else {
                                    theme.colors.popover_foreground.linear_multiply(0.5)
                                };

                                ui.painter().text(
                                    egui::pos2(
                                        item_response.rect.min.x + 8.0,
                                        item_response.rect.center().y,
                                    ),
                                    egui::Align2::LEFT_CENTER,
                                    &option.label,
                                    egui::FontId::proportional(theme.typography.body().size),
                                    text_color,
                                );

                                // Check mark for selected
                                if is_selected {
                                    let check_pos = egui::pos2(
                                        item_response.rect.max.x - 20.0,
                                        item_response.rect.center().y,
                                    );
                                    let check_size = 5.0;

                                    let p1 = check_pos + egui::vec2(-check_size * 0.5, 0.0);
                                    let p2 = check_pos + egui::vec2(-check_size * 0.1, check_size * 0.4);
                                    let p3 = check_pos + egui::vec2(check_size * 0.5, -check_size * 0.4);

                                    ui.painter().line_segment(
                                        [p1, p2],
                                        egui::Stroke::new(2.0, text_color),
                                    );
                                    ui.painter().line_segment(
                                        [p2, p3],
                                        egui::Stroke::new(2.0, text_color),
                                    );
                                }
                            }
                        }
                    });
                });

            // Close popup when clicking outside (but not on the trigger itself)
            // Only check if we didn't just open this frame
            if is_open {
                // Use clicked_elsewhere which is more reliable
                if area_response.response.clicked_elsewhere() && !response.clicked() {
                    new_open_state = false;
                }
            }
        }

        // Save the open state for next frame
        ui.ctx().data_mut(|d| d.insert_temp(open_id, new_open_state));

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
    fn test_select_creation() {
        let mut selected = 0;
        let select = Select::new("test", &mut selected)
            .option("Option 1")
            .option("Option 2");
        assert_eq!(select.options.len(), 2);
    }
}
