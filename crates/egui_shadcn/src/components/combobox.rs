//! Combobox component ported from shadcn/ui
//!
//! A searchable select component that combines an input field with a dropdown list.
//!
//! Reference: <https://ui.shadcn.com/docs/components/combobox>

use egui::{Id, Response, Ui, Sense, Vec2, Pos2};
use crate::theme::ShadcnTheme;

/// A single combobox option
#[derive(Debug, Clone, PartialEq)]
pub struct ComboboxOption {
    /// The value (used for selection)
    pub value: String,
    /// The display label
    pub label: String,
}

impl ComboboxOption {
    /// Create a new option with the same value and label
    pub fn new(value: impl Into<String>) -> Self {
        let v = value.into();
        Self {
            label: v.clone(),
            value: v,
        }
    }

    /// Create a new option with separate value and label
    pub fn with_label(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
        }
    }
}

/// Combobox component - searchable select
///
/// ## Example
/// ```rust,ignore
/// let options = vec![
///     ComboboxOption::new("apple"),
///     ComboboxOption::new("banana"),
///     ComboboxOption::new("cherry"),
/// ];
/// let mut selected: Option<String> = None;
///
/// Combobox::new("fruit_combo", &options, &mut selected)
///     .placeholder("Select a fruit...")
///     .show(ui);
/// ```
pub struct Combobox<'a> {
    id: Id,
    options: &'a [ComboboxOption],
    selected: &'a mut Option<String>,
    placeholder: String,
    search_placeholder: String,
    width: f32,
    empty_message: String,
}

impl<'a> Combobox<'a> {
    /// Create a new combobox
    pub fn new(
        id: impl std::hash::Hash,
        options: &'a [ComboboxOption],
        selected: &'a mut Option<String>,
    ) -> Self {
        Self {
            id: Id::new(id),
            options,
            selected,
            placeholder: "Select...".to_string(),
            search_placeholder: "Search...".to_string(),
            width: 200.0,
            empty_message: "No results found.".to_string(),
        }
    }

    /// Set the placeholder text when nothing is selected
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Set the search input placeholder
    pub fn search_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.search_placeholder = placeholder.into();
        self
    }

    /// Set the width of the combobox
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Set the message shown when no results match the search
    pub fn empty_message(mut self, message: impl Into<String>) -> Self {
        self.empty_message = message.into();
        self
    }

    /// Show the combobox
    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        // State management
        let open_id = self.id.with("open");
        let search_id = self.id.with("search");

        let is_open = ui.ctx().data(|d| d.get_temp::<bool>(open_id).unwrap_or(false));
        let search_text = ui.ctx().data(|d| d.get_temp::<String>(search_id).unwrap_or_default());

        // Get selected label for display
        let selected_label = self.selected.as_ref().and_then(|val| {
            self.options.iter().find(|o| &o.value == val).map(|o| o.label.clone())
        });

        // Trigger button (44px minimum for Apple HIG touch target)
        let button_height = 44.0;
        let (rect, response) = ui.allocate_exact_size(
            Vec2::new(self.width, button_height),
            Sense::click(),
        );

        if ui.is_rect_visible(rect) {
            self.draw_trigger_button(ui, &theme, rect, &response, &selected_label);
        }

        // Toggle on click
        let mut new_open_state = is_open;
        if response.clicked() {
            new_open_state = !is_open;
            // Clear search when opening
            if new_open_state {
                ui.ctx().data_mut(|d| d.insert_temp(search_id, String::new()));
            }
        }

        if response.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }

        // Dropdown popup
        if new_open_state {
            let area_id = self.id.with("area");
            let area_response = egui::Area::new(area_id)
                .order(egui::Order::Foreground)
                .fixed_pos(rect.left_bottom() + egui::vec2(0.0, 4.0))
                .show(ui.ctx(), |ui| {
                    egui::Frame::NONE
                        .fill(theme.colors.popover)
                        .stroke(egui::Stroke::new(1.0, theme.colors.border))
                        .corner_radius(theme.radii.md)
                        .shadow(theme.shadows.md)
                        .inner_margin(egui::Margin::same(8))
                        .show(ui, |ui| {
                            ui.set_min_width(self.width - 16.0);

                            // Force vertical layout
                            ui.vertical(|ui| {
                                // Search input
                                let mut current_search = ui.ctx().data(|d| {
                                    d.get_temp::<String>(search_id).unwrap_or_default()
                                });

                                // Search input field with border
                                let search_response = ui.horizontal(|ui| {
                                    // Search icon
                                    let (icon_rect, _) = ui.allocate_exact_size(Vec2::new(16.0, 16.0), Sense::hover());
                                    self.draw_search_icon(ui, &theme, icon_rect.center());

                                    // Text input
                                    ui.add(
                                        egui::TextEdit::singleline(&mut current_search)
                                            .hint_text(&self.search_placeholder)
                                            .frame(false)
                                            .desired_width(ui.available_width())
                                            .font(egui::FontId::proportional(theme.typography.small().size))
                                    )
                                }).inner;

                                // Request focus on first frame
                                if !is_open {
                                    search_response.request_focus();
                                }

                                // Save search text
                                ui.ctx().data_mut(|d| d.insert_temp(search_id, current_search.clone()));

                                ui.add_space(8.0);

                                // Separator
                                ui.add(egui::Separator::default().horizontal());

                                ui.add_space(4.0);

                                // Filter options
                                let search_lower = current_search.to_lowercase();
                                let filtered: Vec<_> = self.options.iter()
                                    .filter(|o| {
                                        search_lower.is_empty() ||
                                        o.label.to_lowercase().contains(&search_lower) ||
                                        o.value.to_lowercase().contains(&search_lower)
                                    })
                                    .collect();

                                // Options list in scroll area
                                egui::ScrollArea::vertical()
                                    .max_height(200.0)
                                    .show(ui, |ui| {
                                        if filtered.is_empty() {
                                            // Empty state
                                            ui.add_space(8.0);
                                            ui.label(
                                                egui::RichText::new(&self.empty_message)
                                                    .size(13.0)
                                                    .color(theme.colors.muted_foreground)
                                            );
                                            ui.add_space(8.0);
                                        } else {
                                            for option in filtered {
                                                let is_selected = self.selected.as_ref() == Some(&option.value);
                                                let item_response = self.draw_option_item(
                                                    ui, &theme, &option.label, is_selected
                                                );

                                                if item_response.clicked() {
                                                    *self.selected = Some(option.value.clone());
                                                    new_open_state = false;
                                                }
                                            }
                                        }
                                    });
                            });
                        });
                });

            // Close when clicking outside
            if is_open && area_response.response.clicked_elsewhere() && !response.clicked() {
                new_open_state = false;
            }
        }

        // Save open state
        ui.ctx().data_mut(|d| d.insert_temp(open_id, new_open_state));

        response
    }

    fn draw_trigger_button(
        &self,
        ui: &mut Ui,
        theme: &ShadcnTheme,
        rect: egui::Rect,
        response: &Response,
        selected_label: &Option<String>,
    ) {
        // Background and border - subtle hover like shadcn
        let bg_color = if response.hovered() {
            theme.colors.foreground.linear_multiply(0.05)
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

        // Text
        let text = selected_label.as_deref().unwrap_or(&self.placeholder);
        let text_color = if selected_label.is_some() {
            theme.colors.foreground
        } else {
            theme.colors.muted_foreground
        };

        ui.painter().text(
            Pos2::new(rect.min.x + 12.0, rect.center().y),
            egui::Align2::LEFT_CENTER,
            text,
            egui::FontId::proportional(theme.typography.small().size),
            text_color,
        );

        // Chevron down icon
        let chevron_x = rect.max.x - 16.0;
        let chevron_y = rect.center().y;
        let chevron_size = 4.0;
        let stroke = egui::Stroke::new(1.5, theme.colors.muted_foreground);

        ui.painter().line_segment(
            [
                Pos2::new(chevron_x - chevron_size, chevron_y - chevron_size * 0.5),
                Pos2::new(chevron_x, chevron_y + chevron_size * 0.5),
            ],
            stroke,
        );
        ui.painter().line_segment(
            [
                Pos2::new(chevron_x, chevron_y + chevron_size * 0.5),
                Pos2::new(chevron_x + chevron_size, chevron_y - chevron_size * 0.5),
            ],
            stroke,
        );
    }

    fn draw_search_input(&self, ui: &mut Ui, theme: &ShadcnTheme, search_text: &mut String) -> Response {
        // Use a horizontal layout with proper spacing
        let response = ui.horizontal(|ui| {
            ui.add_space(8.0);

            // Search icon
            let icon_rect = ui.allocate_exact_size(Vec2::new(16.0, 16.0), Sense::hover()).0;
            self.draw_search_icon(ui, theme, icon_rect.center());

            ui.add_space(4.0);

            // Text input
            let text_response = ui.add(
                egui::TextEdit::singleline(search_text)
                    .hint_text(&self.search_placeholder)
                    .frame(false)
                    .desired_width(ui.available_width() - 8.0)
                    .font(egui::FontId::proportional(theme.typography.small().size))
            );

            text_response
        });

        response.inner
    }

    fn draw_search_icon(&self, ui: &mut Ui, theme: &ShadcnTheme, center: Pos2) {
        let stroke = egui::Stroke::new(1.5, theme.colors.muted_foreground);
        let radius = 5.0;

        // Circle
        ui.painter().circle_stroke(
            Pos2::new(center.x - 1.0, center.y - 1.0),
            radius,
            stroke,
        );

        // Handle (diagonal line)
        let handle_start = Pos2::new(center.x + 2.5, center.y + 2.5);
        let handle_end = Pos2::new(center.x + 5.5, center.y + 5.5);
        ui.painter().line_segment([handle_start, handle_end], stroke);
    }

    fn draw_option_item(&self, ui: &mut Ui, theme: &ShadcnTheme, label: &str, is_selected: bool) -> Response {
        let height = 36.0; // Slightly smaller for dropdown items
        let width = ui.available_width();
        let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::click());

        if ui.is_rect_visible(rect) {
            // Background - subtle hover like shadcn
            let bg_color = if is_selected {
                Some(theme.colors.accent)
            } else if response.hovered() {
                Some(theme.colors.foreground.linear_multiply(0.08))
            } else {
                None
            };

            if let Some(bg) = bg_color {
                ui.painter().rect_filled(rect, theme.radii.sm, bg);
            }

            // Checkmark for selected item
            if is_selected {
                let check_x = rect.min.x + 12.0;
                let check_y = rect.center().y;
                let stroke = egui::Stroke::new(2.0, theme.colors.foreground);

                // Draw checkmark
                ui.painter().line_segment(
                    [
                        Pos2::new(check_x - 3.0, check_y),
                        Pos2::new(check_x - 1.0, check_y + 2.5),
                    ],
                    stroke,
                );
                ui.painter().line_segment(
                    [
                        Pos2::new(check_x - 1.0, check_y + 2.5),
                        Pos2::new(check_x + 4.0, check_y - 3.0),
                    ],
                    stroke,
                );
            }

            // Label
            let text_x = rect.min.x + 28.0;
            let text_color = if is_selected {
                theme.colors.accent_foreground
            } else {
                theme.colors.foreground
            };

            ui.painter().text(
                Pos2::new(text_x, rect.center().y),
                egui::Align2::LEFT_CENTER,
                label,
                egui::FontId::proportional(14.0),
                text_color,
            );
        }

        if response.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combobox_option() {
        let opt = ComboboxOption::new("test");
        assert_eq!(opt.value, "test");
        assert_eq!(opt.label, "test");

        let opt2 = ComboboxOption::with_label("val", "Label");
        assert_eq!(opt2.value, "val");
        assert_eq!(opt2.label, "Label");
    }
}
