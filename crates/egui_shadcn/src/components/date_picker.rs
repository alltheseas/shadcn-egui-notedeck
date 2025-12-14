//! DatePicker component ported from shadcn/ui
//!
//! A date picker combining a button trigger with a calendar popover.
//!
//! Reference: <https://ui.shadcn.com/docs/components/date-picker>

use egui::{Id, Response, Ui, Sense, Vec2, Pos2};
use crate::theme::ShadcnTheme;
use crate::components::calendar::{Calendar, CalendarSelection};

/// DatePicker component for date selection
///
/// ## Example
/// ```rust,ignore
/// let mut selected_date: Option<chrono::NaiveDate> = None;
///
/// DatePicker::new("my_datepicker", &mut selected_date)
///     .placeholder("Pick a date")
///     .show(ui);
/// ```
pub struct DatePicker<'a> {
    id: Id,
    selected: &'a mut Option<chrono::NaiveDate>,
    placeholder: String,
    format: String,
    min_date: Option<chrono::NaiveDate>,
    max_date: Option<chrono::NaiveDate>,
}

impl<'a> DatePicker<'a> {
    /// Create a new date picker
    pub fn new(id: impl std::hash::Hash, selected: &'a mut Option<chrono::NaiveDate>) -> Self {
        Self {
            id: Id::new(id),
            selected,
            placeholder: "Pick a date".to_string(),
            format: "%B %d, %Y".to_string(),
            min_date: None,
            max_date: None,
        }
    }

    /// Set the placeholder text when no date is selected
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Set the date format string (uses chrono format specifiers)
    pub fn format(mut self, format: impl Into<String>) -> Self {
        self.format = format.into();
        self
    }

    /// Set minimum selectable date
    pub fn min_date(mut self, date: chrono::NaiveDate) -> Self {
        self.min_date = Some(date);
        self
    }

    /// Set maximum selectable date
    pub fn max_date(mut self, date: chrono::NaiveDate) -> Self {
        self.max_date = Some(date);
        self
    }

    /// Show the date picker
    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        // Use simple boolean state in memory (same pattern as Popover)
        let open_id = self.id.with("open");
        let is_open = ui.ctx().data(|d| d.get_temp::<bool>(open_id).unwrap_or(false));

        // Trigger button
        let button_text = match self.selected {
            Some(date) => date.format(&self.format).to_string(),
            None => self.placeholder.clone(),
        };

        let text_color = if self.selected.is_some() {
            theme.colors.foreground
        } else {
            theme.colors.muted_foreground
        };

        // Draw the trigger button (outline style with calendar icon)
        let desired_width = 240.0;
        let button_height = 36.0;
        let (rect, response) = ui.allocate_exact_size(
            Vec2::new(desired_width, button_height),
            Sense::click(),
        );

        if ui.is_rect_visible(rect) {
            let is_hovered = response.hovered();

            // Background and border
            let bg_color = if is_hovered {
                theme.colors.accent
            } else {
                theme.colors.background
            };

            ui.painter().rect(
                rect,
                theme.radii.md,
                bg_color,
                egui::Stroke::new(1.0, theme.colors.input),
                egui::StrokeKind::Inside,
            );

            // Calendar icon on the left
            let icon_x = rect.min.x + 12.0;
            let icon_y = rect.center().y;
            let icon_color = theme.colors.muted_foreground;
            self.draw_calendar_icon(ui, Pos2::new(icon_x, icon_y), icon_color);

            // Text
            let text_x = rect.min.x + 36.0;
            ui.painter().text(
                Pos2::new(text_x, rect.center().y),
                egui::Align2::LEFT_CENTER,
                &button_text,
                egui::FontId::proportional(theme.typography.small().size),
                text_color,
            );
        }

        // Toggle on click
        let mut new_open_state = is_open;
        if response.clicked() {
            new_open_state = !is_open;
        }

        if response.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }

        // Show popup if open
        if new_open_state {
            let area_id = self.id.with("area");
            let area_response = egui::Area::new(area_id)
                .order(egui::Order::Foreground)
                .fixed_pos(rect.left_bottom() + egui::vec2(0.0, 4.0))
                .show(ui.ctx(), |ui| {
                    egui::Frame::NONE
                        .fill(theme.colors.popover)
                        .stroke(egui::Stroke::new(1.0, theme.colors.border))
                        .corner_radius(theme.radii.lg)
                        .shadow(theme.shadows.md)
                        .inner_margin(egui::Margin::same(4))
                        .show(ui, |ui| {
                            // Convert Option<NaiveDate> to CalendarSelection
                            let mut calendar_selection = match *self.selected {
                                Some(date) => CalendarSelection::Single(date),
                                None => CalendarSelection::None,
                            };

                            // View date state - stored in memory
                            let view_date_id = self.id.with("view_date");
                            let mut view_date = ui.ctx().data(|d| {
                                d.get_temp::<chrono::NaiveDate>(view_date_id)
                                    .unwrap_or_else(|| {
                                        self.selected.unwrap_or_else(|| chrono::Local::now().date_naive())
                                    })
                            });

                            let mut calendar = Calendar::new(
                                self.id.with("calendar"),
                                &mut calendar_selection,
                                &mut view_date,
                            );

                            if let Some(min) = self.min_date {
                                calendar = calendar.min_date(min);
                            }
                            if let Some(max) = self.max_date {
                                calendar = calendar.max_date(max);
                            }

                            calendar.show(ui);

                            // Update selected date from calendar selection
                            match calendar_selection {
                                CalendarSelection::Single(date) => {
                                    if *self.selected != Some(date) {
                                        *self.selected = Some(date);
                                        // Close popup on selection
                                        new_open_state = false;
                                    }
                                }
                                _ => {}
                            }

                            // Save view date
                            ui.ctx().data_mut(|d| d.insert_temp(view_date_id, view_date));
                        });
                });

            // Close when clicking outside (but not on trigger)
            // Only check if we were already open (not just opened this frame)
            if is_open {
                if area_response.response.clicked_elsewhere() && !response.clicked() {
                    new_open_state = false;
                }
            }
        }

        // Save the open state for next frame
        ui.ctx().data_mut(|d| d.insert_temp(open_id, new_open_state));

        response
    }

    fn draw_calendar_icon(&self, ui: &mut Ui, center: Pos2, color: egui::Color32) {
        let stroke = egui::Stroke::new(1.2, color);
        let size = 7.0;

        // Calendar outline (rounded rectangle)
        let rect = egui::Rect::from_center_size(center, Vec2::splat(size * 2.0));
        ui.painter().rect_stroke(rect, 2.0, stroke, egui::StrokeKind::Inside);

        // Top bar (header area)
        let top_y = rect.min.y + size * 0.5;
        ui.painter().line_segment(
            [Pos2::new(rect.min.x, top_y), Pos2::new(rect.max.x, top_y)],
            stroke,
        );

        // Two small lines for calendar "rings" at top
        let ring_y_top = rect.min.y - 2.0;
        let ring_y_bottom = rect.min.y + 2.0;
        let ring1_x = center.x - size * 0.4;
        let ring2_x = center.x + size * 0.4;

        ui.painter().line_segment(
            [Pos2::new(ring1_x, ring_y_top), Pos2::new(ring1_x, ring_y_bottom)],
            stroke,
        );
        ui.painter().line_segment(
            [Pos2::new(ring2_x, ring_y_top), Pos2::new(ring2_x, ring_y_bottom)],
            stroke,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datepicker_creation() {
        let mut date: Option<chrono::NaiveDate> = None;
        let picker = DatePicker::new("test", &mut date)
            .placeholder("Select...")
            .format("%Y-%m-%d");

        assert_eq!(picker.placeholder, "Select...");
        assert_eq!(picker.format, "%Y-%m-%d");
    }
}
