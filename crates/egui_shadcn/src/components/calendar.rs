//! Calendar component ported from shadcn/ui
//!
//! A date calendar for selecting dates.
//!
//! Reference: <https://ui.shadcn.com/docs/components/calendar>

use egui::{Id, Response, Ui, Sense, Vec2, Pos2};
use chrono::Datelike;
use crate::theme::ShadcnTheme;

/// Calendar selection mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CalendarMode {
    /// Single date selection
    #[default]
    Single,
    /// Date range selection
    Range,
}

/// A selected date or range
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalendarSelection {
    /// No selection
    None,
    /// Single date selected
    Single(chrono::NaiveDate),
    /// Date range selected (start, end)
    Range(chrono::NaiveDate, chrono::NaiveDate),
}

impl Default for CalendarSelection {
    fn default() -> Self {
        Self::None
    }
}

/// Calendar component for date selection
///
/// ## Example
/// ```rust,ignore
/// let mut selected = CalendarSelection::None;
/// let mut view_date = chrono::Local::now().date_naive();
///
/// Calendar::new("my_calendar", &mut selected, &mut view_date)
///     .show_outside_days(true)
///     .show(ui);
/// ```
pub struct Calendar<'a> {
    id: Id,
    selection: &'a mut CalendarSelection,
    view_date: &'a mut chrono::NaiveDate,
    mode: CalendarMode,
    show_outside_days: bool,
    min_date: Option<chrono::NaiveDate>,
    max_date: Option<chrono::NaiveDate>,
}

impl<'a> Calendar<'a> {
    /// Create a new calendar
    pub fn new(
        id: impl std::hash::Hash,
        selection: &'a mut CalendarSelection,
        view_date: &'a mut chrono::NaiveDate,
    ) -> Self {
        Self {
            id: Id::new(id),
            selection,
            view_date,
            mode: CalendarMode::Single,
            show_outside_days: true,
            min_date: None,
            max_date: None,
        }
    }

    /// Set the selection mode
    pub fn mode(mut self, mode: CalendarMode) -> Self {
        self.mode = mode;
        self
    }

    /// Show days from previous/next months
    pub fn show_outside_days(mut self, show: bool) -> Self {
        self.show_outside_days = show;
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

    /// Show the calendar
    pub fn show(mut self, ui: &mut Ui) -> Response {
        let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        let cell_size = 32.0;
        let padding = 12.0;
        let today = chrono::Local::now().date_naive();

        let response = egui::Frame::NONE
            .inner_margin(egui::Margin::same(padding as i8))
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    // Navigation header
                    self.draw_header(ui, &theme, cell_size);

                    ui.add_space(8.0);

                    // Weekday headers
                    self.draw_weekday_headers(ui, &theme, cell_size);

                    ui.add_space(4.0);

                    // Day grid
                    self.draw_day_grid(ui, &theme, cell_size, today);
                });
            });

        response.response
    }

    fn draw_header(&mut self, ui: &mut Ui, theme: &ShadcnTheme, cell_size: f32) {
        // Total width is 7 cells (7 days)
        let total_width = cell_size * 7.0;
        let button_width = cell_size;
        let center_width = total_width - (button_width * 2.0);

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;

            // Previous month button
            let prev_response = self.draw_nav_button(ui, theme, cell_size, true);
            if prev_response.clicked() {
                *self.view_date = *self.view_date - chrono::Months::new(1);
            }

            // Month and Year dropdowns (centered in fixed-width area)
            let (center_rect, _) = ui.allocate_exact_size(Vec2::new(center_width, cell_size), Sense::hover());
            if ui.is_rect_visible(center_rect) {
                // Create a child UI in the center area for the dropdowns
                let mut child_ui = ui.new_child(egui::UiBuilder::new().max_rect(center_rect));
                child_ui.with_layout(egui::Layout::left_to_right(egui::Align::Center).with_main_justify(true), |ui| {
                    ui.horizontal(|ui| {
                        // Total width of dropdowns: 58 + 4 + 62 = 124
                        ui.add_space((center_width - 124.0) / 2.0); // Center the dropdowns

                        // Month dropdown (abbreviated names in button)
                        let months_short = [
                            "Jan", "Feb", "Mar", "Apr", "May", "Jun",
                            "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"
                        ];
                        // Full names for dropdown list
                        let months_full = [
                            "January", "February", "March", "April", "May", "June",
                            "July", "August", "September", "October", "November", "December"
                        ];
                        let current_month = self.view_date.month() as usize - 1;

                        let month_id = ui.id().with("month_dropdown");
                        let month_response = self.draw_dropdown_button(
                            ui, theme, months_short[current_month], 58.0, month_id
                        );

                        if month_response.clicked() {
                            let open_id = month_id.with("open");
                            let is_open = ui.ctx().data(|d| d.get_temp::<bool>(open_id).unwrap_or(false));
                            ui.ctx().data_mut(|d| d.insert_temp(open_id, !is_open));
                        }

                        // Show month dropdown popup (with full names)
                        self.show_month_dropdown(ui, theme, month_id, &month_response, &months_full);

                        ui.add_space(4.0);

                        // Year dropdown
                        let current_year = self.view_date.year();
                        let year_id = ui.id().with("year_dropdown");
                        let year_response = self.draw_dropdown_button(
                            ui, theme, &current_year.to_string(), 62.0, year_id
                        );

                        if year_response.clicked() {
                            let open_id = year_id.with("open");
                            let is_open = ui.ctx().data(|d| d.get_temp::<bool>(open_id).unwrap_or(false));
                            ui.ctx().data_mut(|d| d.insert_temp(open_id, !is_open));
                        }

                        // Show year dropdown popup
                        self.show_year_dropdown(ui, theme, year_id, &year_response, current_year);
                    });
                });
            }

            // Next month button
            let next_response = self.draw_nav_button(ui, theme, cell_size, false);
            if next_response.clicked() {
                *self.view_date = *self.view_date + chrono::Months::new(1);
            }
        });
    }

    fn draw_dropdown_button(&self, ui: &mut Ui, theme: &ShadcnTheme, text: &str, width: f32, _id: Id) -> Response {
        let height = 24.0;
        let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::click());

        if ui.is_rect_visible(rect) {
            // Hover background
            if response.hovered() {
                ui.painter().rect_filled(rect, theme.radii.sm, theme.colors.accent);
            }

            // Text color
            let text_color = if response.hovered() {
                theme.colors.accent_foreground
            } else {
                theme.colors.foreground
            };

            // Text aligned left with padding, leaving room for chevron
            let text_x = rect.min.x + 6.0;
            ui.painter().text(
                Pos2::new(text_x, rect.center().y),
                egui::Align2::LEFT_CENTER,
                text,
                egui::FontId::new(theme.typography.small().size, egui::FontFamily::Proportional),
                text_color,
            );

            // Small down chevron on the right
            let chevron_x = rect.max.x - 8.0;
            let chevron_y = rect.center().y;
            let chevron_size = 3.0;
            let stroke = egui::Stroke::new(1.5, text_color);

            ui.painter().line_segment(
                [Pos2::new(chevron_x - chevron_size, chevron_y - chevron_size * 0.5),
                 Pos2::new(chevron_x, chevron_y + chevron_size * 0.5)],
                stroke,
            );
            ui.painter().line_segment(
                [Pos2::new(chevron_x, chevron_y + chevron_size * 0.5),
                 Pos2::new(chevron_x + chevron_size, chevron_y - chevron_size * 0.5)],
                stroke,
            );
        }

        if response.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }

        response
    }

    fn show_month_dropdown(&mut self, ui: &mut Ui, theme: &ShadcnTheme, dropdown_id: Id, trigger: &Response, months: &[&str; 12]) {
        let open_id = dropdown_id.with("open");
        let is_open = ui.ctx().data(|d| d.get_temp::<bool>(open_id).unwrap_or(false));

        if !is_open {
            return;
        }

        let area_id = dropdown_id.with("area");
        let area_response = egui::Area::new(area_id)
            .order(egui::Order::Foreground)
            .fixed_pos(trigger.rect.left_bottom() + egui::vec2(0.0, 4.0))
            .show(ui.ctx(), |ui| {
                egui::Frame::NONE
                    .fill(theme.colors.popover)
                    .stroke(egui::Stroke::new(1.0, theme.colors.border))
                    .corner_radius(theme.radii.md)
                    .shadow(theme.shadows.sm)
                    .inner_margin(egui::Margin::same(4))
                    .show(ui, |ui| {
                        ui.set_max_height(200.0);
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for (i, month) in months.iter().enumerate() {
                                let is_current = self.view_date.month() as usize - 1 == i;
                                let item_response = self.draw_dropdown_item(ui, theme, month, is_current, 100.0);
                                if item_response.clicked() {
                                    // Update to selected month
                                    if let Some(new_date) = chrono::NaiveDate::from_ymd_opt(
                                        self.view_date.year(),
                                        (i + 1) as u32,
                                        1.min(self.view_date.day()),
                                    ) {
                                        *self.view_date = new_date;
                                    }
                                    ui.ctx().data_mut(|d| d.insert_temp(open_id, false));
                                }
                            }
                        });
                    });
            });

        // Close when clicking outside
        if area_response.response.clicked_elsewhere() && !trigger.clicked() {
            ui.ctx().data_mut(|d| d.insert_temp(open_id, false));
        }
    }

    fn show_year_dropdown(&mut self, ui: &mut Ui, theme: &ShadcnTheme, dropdown_id: Id, trigger: &Response, current_year: i32) {
        let open_id = dropdown_id.with("open");
        let is_open = ui.ctx().data(|d| d.get_temp::<bool>(open_id).unwrap_or(false));

        if !is_open {
            return;
        }

        // Show years: future years first (next 10), then past years (last 100)
        let future_years: Vec<i32> = ((current_year + 1)..=(current_year + 10)).rev().collect();
        let past_years: Vec<i32> = ((current_year - 100)..=current_year).rev().collect();

        let area_id = dropdown_id.with("area");
        let area_response = egui::Area::new(area_id)
            .order(egui::Order::Foreground)
            .fixed_pos(trigger.rect.left_bottom() + egui::vec2(0.0, 4.0))
            .show(ui.ctx(), |ui| {
                egui::Frame::NONE
                    .fill(theme.colors.popover)
                    .stroke(egui::Stroke::new(1.0, theme.colors.border))
                    .corner_radius(theme.radii.md)
                    .shadow(theme.shadows.sm)
                    .inner_margin(egui::Margin::same(4))
                    .show(ui, |ui| {
                        ui.set_max_height(200.0);
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            // Future years first (2026, 2027, ... 2035)
                            for year in future_years.iter().rev() {
                                let is_current = *year == current_year;
                                let item_response = self.draw_dropdown_item(ui, theme, &year.to_string(), is_current, 50.0);
                                if item_response.clicked() {
                                    if let Some(new_date) = chrono::NaiveDate::from_ymd_opt(
                                        *year,
                                        self.view_date.month(),
                                        1.min(self.view_date.day()),
                                    ) {
                                        *self.view_date = new_date;
                                    }
                                    ui.ctx().data_mut(|d| d.insert_temp(open_id, false));
                                }
                            }
                            // Then current and past years (2025, 2024, 2023, ...)
                            for year in past_years {
                                let is_current = year == current_year;
                                let item_response = self.draw_dropdown_item(ui, theme, &year.to_string(), is_current, 50.0);
                                if item_response.clicked() {
                                    if let Some(new_date) = chrono::NaiveDate::from_ymd_opt(
                                        year,
                                        self.view_date.month(),
                                        1.min(self.view_date.day()),
                                    ) {
                                        *self.view_date = new_date;
                                    }
                                    ui.ctx().data_mut(|d| d.insert_temp(open_id, false));
                                }
                            }
                        });
                    });
            });

        // Close when clicking outside
        if area_response.response.clicked_elsewhere() && !trigger.clicked() {
            ui.ctx().data_mut(|d| d.insert_temp(open_id, false));
        }
    }

    fn draw_dropdown_item(&self, ui: &mut Ui, theme: &ShadcnTheme, text: &str, is_selected: bool, width: f32) -> Response {
        let height = 28.0;
        let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::click());

        if ui.is_rect_visible(rect) {
            // Background
            let bg_color = if is_selected {
                Some(theme.colors.accent)
            } else if response.hovered() {
                Some(theme.colors.accent.gamma_multiply(0.5))
            } else {
                None
            };

            if let Some(bg) = bg_color {
                ui.painter().rect_filled(rect, theme.radii.sm, bg);
            }

            // Text
            let text_color = if is_selected {
                theme.colors.accent_foreground
            } else {
                theme.colors.foreground
            };

            ui.painter().text(
                Pos2::new(rect.min.x + 8.0, rect.center().y),
                egui::Align2::LEFT_CENTER,
                text,
                egui::FontId::proportional(13.0),
                text_color,
            );
        }

        if response.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }

        response
    }

    fn draw_nav_button(&self, ui: &mut Ui, theme: &ShadcnTheme, size: f32, is_prev: bool) -> Response {
        let (rect, response) = ui.allocate_exact_size(Vec2::splat(size), Sense::click());

        if ui.is_rect_visible(rect) {
            // Hover background (ghost button style)
            if response.hovered() {
                ui.painter().rect_filled(rect, theme.radii.md, theme.colors.accent);
            }

            // Draw chevron
            let cx = rect.center().x;
            let cy = rect.center().y;
            let chevron_size = 4.0;
            let color = if response.hovered() {
                theme.colors.accent_foreground
            } else {
                theme.colors.foreground
            };
            let stroke = egui::Stroke::new(1.5, color);

            if is_prev {
                // Left chevron (<)
                ui.painter().line_segment(
                    [Pos2::new(cx + chevron_size * 0.5, cy - chevron_size),
                     Pos2::new(cx - chevron_size * 0.5, cy)],
                    stroke,
                );
                ui.painter().line_segment(
                    [Pos2::new(cx - chevron_size * 0.5, cy),
                     Pos2::new(cx + chevron_size * 0.5, cy + chevron_size)],
                    stroke,
                );
            } else {
                // Right chevron (>)
                ui.painter().line_segment(
                    [Pos2::new(cx - chevron_size * 0.5, cy - chevron_size),
                     Pos2::new(cx + chevron_size * 0.5, cy)],
                    stroke,
                );
                ui.painter().line_segment(
                    [Pos2::new(cx + chevron_size * 0.5, cy),
                     Pos2::new(cx - chevron_size * 0.5, cy + chevron_size)],
                    stroke,
                );
            }
        }

        if response.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }

        response
    }

    fn draw_weekday_headers(&self, ui: &mut Ui, theme: &ShadcnTheme, cell_size: f32) {
        let weekdays = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            for day in weekdays {
                let (rect, _) = ui.allocate_exact_size(Vec2::splat(cell_size), Sense::hover());
                if ui.is_rect_visible(rect) {
                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        day,
                        egui::FontId::proportional(12.0),
                        theme.colors.muted_foreground,
                    );
                }
            }
        });
    }

    fn draw_day_grid(&mut self, ui: &mut Ui, theme: &ShadcnTheme, cell_size: f32, today: chrono::NaiveDate) {
        use chrono::NaiveDate;

        let year = self.view_date.year();
        let month = self.view_date.month();

        // First day of the month
        let first_of_month = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
        // Last day of the month
        let last_of_month = if month == 12 {
            NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap() - chrono::Days::new(1)
        } else {
            NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap() - chrono::Days::new(1)
        };

        // Day of week for first day (0 = Sunday)
        let first_weekday = first_of_month.weekday().num_days_from_sunday() as i32;

        // Start date for the grid (may be in previous month)
        let grid_start = first_of_month - chrono::Days::new(first_weekday as u64);

        // Draw 6 weeks (42 days)
        for week in 0..6 {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                for day in 0..7 {
                    let day_offset = week * 7 + day;
                    let current_date = grid_start + chrono::Days::new(day_offset as u64);
                    let is_current_month = current_date.month() == month;

                    self.draw_day_cell(
                        ui,
                        theme,
                        cell_size,
                        current_date,
                        today,
                        is_current_month,
                    );
                }
            });
        }
    }

    fn draw_day_cell(
        &mut self,
        ui: &mut Ui,
        theme: &ShadcnTheme,
        size: f32,
        date: chrono::NaiveDate,
        today: chrono::NaiveDate,
        is_current_month: bool,
    ) {
        let is_today = date == today;
        let is_selected = match self.selection {
            CalendarSelection::None => false,
            CalendarSelection::Single(d) => *d == date,
            CalendarSelection::Range(start, end) => date >= *start && date <= *end,
        };
        let is_range_start = matches!(self.selection, CalendarSelection::Range(start, _) if *start == date);
        let is_range_end = matches!(self.selection, CalendarSelection::Range(_, end) if *end == date);
        let is_disabled = self.is_date_disabled(date);

        // Don't show outside days if disabled
        if !is_current_month && !self.show_outside_days {
            let (_, _) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());
            return;
        }

        let (rect, response) = ui.allocate_exact_size(Vec2::splat(size), Sense::click());

        if ui.is_rect_visible(rect) {
            let cell_rect = rect.shrink(1.0);

            // Full circle rounding for selected dates (shadcn style)
            let circle_rounding = size / 2.0;

            // Determine background, text color, and whether to draw border for today
            let (bg_color, text_color, rounding, draw_today_border) = if is_selected {
                if is_range_start || is_range_end || matches!(self.selection, CalendarSelection::Single(_)) {
                    // Filled circle for selected date
                    (Some(theme.colors.primary), theme.colors.primary_foreground, circle_rounding, false)
                } else {
                    // Middle of range - subtle highlight, no rounding
                    (Some(theme.colors.accent), theme.colors.accent_foreground, 0.0, false)
                }
            } else if response.hovered() && !is_disabled {
                (Some(theme.colors.accent), theme.colors.accent_foreground, circle_rounding, is_today)
            } else if is_today {
                // Today: no fill, just border - subtle circle indicator
                (None, theme.colors.foreground, circle_rounding, true)
            } else {
                (None, if is_current_month { theme.colors.foreground } else { theme.colors.muted_foreground }, 0.0, false)
            };

            // Draw background
            if let Some(bg) = bg_color {
                ui.painter().rect_filled(cell_rect, rounding, bg);
            }

            // Draw today border (subtle outline)
            if draw_today_border {
                ui.painter().rect_stroke(
                    cell_rect,
                    rounding,
                    egui::Stroke::new(1.0, theme.colors.foreground),
                    egui::StrokeKind::Inside,
                );
            }

            // Draw day number
            let final_text_color = if is_disabled {
                theme.colors.muted_foreground.gamma_multiply(0.5)
            } else {
                text_color
            };

            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                date.day().to_string(),
                egui::FontId::proportional(14.0),
                final_text_color,
            );
        }

        // Handle click
        if response.clicked() && !is_disabled && (is_current_month || self.show_outside_days) {
            match self.mode {
                CalendarMode::Single => {
                    *self.selection = CalendarSelection::Single(date);
                }
                CalendarMode::Range => {
                    match self.selection {
                        CalendarSelection::None | CalendarSelection::Range(_, _) => {
                            *self.selection = CalendarSelection::Single(date);
                        }
                        CalendarSelection::Single(start) => {
                            if date < *start {
                                *self.selection = CalendarSelection::Range(date, *start);
                            } else {
                                *self.selection = CalendarSelection::Range(*start, date);
                            }
                        }
                    }
                }
            }

            // Update view if clicking outside current month
            if !is_current_month {
                *self.view_date = date;
            }
        }

        if response.hovered() && !is_disabled {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }
    }

    fn is_date_disabled(&self, date: chrono::NaiveDate) -> bool {
        if let Some(min) = self.min_date {
            if date < min {
                return true;
            }
        }
        if let Some(max) = self.max_date {
            if date > max {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calendar_selection() {
        let date = chrono::NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
        let selection = CalendarSelection::Single(date);
        assert!(matches!(selection, CalendarSelection::Single(_)));
    }
}
