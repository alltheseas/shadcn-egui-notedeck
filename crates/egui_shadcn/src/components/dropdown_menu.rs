//! Dropdown Menu component ported from shadcn/ui
//!
//! A menu of actions/commands triggered by a button.
//!
//! Reference: <https://ui.shadcn.com/docs/components/dropdown-menu>

use egui::{Response, Ui, Sense, Popup};
use crate::theme::ShadcnTheme;

/// Dropdown Menu component for action menus
///
/// ## Example
/// ```rust,ignore
/// DropdownMenu::new("actions")
///     .trigger(|ui| ui.button("Open Menu"))
///     .item("Copy", || println!("Copy"))
///     .item("Paste", || println!("Paste"))
///     .separator()
///     .item("Delete", || println!("Delete"))
///     .show(ui);
/// ```
pub struct DropdownMenu<'a> {
    id: &'a str,
    items: Vec<MenuItem>,
    trigger_text: Option<String>,
    width: f32,
}

/// A menu item type
enum MenuItem {
    Action {
        label: String,
        shortcut: Option<String>,
        enabled: bool,
        destructive: bool,
    },
    Separator,
    Label(String),
}

/// Result of showing a dropdown menu
pub struct DropdownMenuResponse {
    /// The response from the trigger button
    pub trigger_response: Response,
    /// Index of clicked item, if any
    pub clicked_item: Option<usize>,
}

impl<'a> DropdownMenu<'a> {
    /// Create a new dropdown menu
    pub fn new(id: &'a str) -> Self {
        Self {
            id,
            items: Vec::new(),
            trigger_text: None,
            width: 160.0,
        }
    }

    /// Set trigger button text (simple trigger)
    pub fn trigger_text(mut self, text: impl Into<String>) -> Self {
        self.trigger_text = Some(text.into());
        self
    }

    /// Add a menu item
    pub fn item(mut self, label: impl Into<String>) -> Self {
        self.items.push(MenuItem::Action {
            label: label.into(),
            shortcut: None,
            enabled: true,
            destructive: false,
        });
        self
    }

    /// Add a menu item with keyboard shortcut
    pub fn item_with_shortcut(mut self, label: impl Into<String>, shortcut: impl Into<String>) -> Self {
        self.items.push(MenuItem::Action {
            label: label.into(),
            shortcut: Some(shortcut.into()),
            enabled: true,
            destructive: false,
        });
        self
    }

    /// Add a destructive (red) menu item
    pub fn destructive_item(mut self, label: impl Into<String>) -> Self {
        self.items.push(MenuItem::Action {
            label: label.into(),
            shortcut: None,
            enabled: true,
            destructive: true,
        });
        self
    }

    /// Add a separator
    pub fn separator(mut self) -> Self {
        self.items.push(MenuItem::Separator);
        self
    }

    /// Add a label/header
    pub fn label(mut self, text: impl Into<String>) -> Self {
        self.items.push(MenuItem::Label(text.into()));
        self
    }

    /// Set the menu width
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Show the dropdown menu
    pub fn show(self, ui: &mut Ui) -> DropdownMenuResponse {
        let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(egui::Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        let id = egui::Id::new(self.id);
        let popup_id = id.with("popup");
        let is_open = Popup::is_id_open(ui.ctx(), popup_id);

        // Render trigger button
        let trigger_response = if let Some(text) = &self.trigger_text {
            let response = ui.add(
                egui::Button::new(text.as_str())
                    .min_size(egui::vec2(0.0, 44.0)) // Apple HIG touch target
            );
            if response.clicked() {
                Popup::toggle_id(ui.ctx(), popup_id);
            }
            response
        } else {
            // Default trigger - three dots icon
            let (response, painter) = ui.allocate_painter(
                egui::vec2(44.0, 44.0),
                Sense::click(),
            );

            if response.clicked() {
                Popup::toggle_id(ui.ctx(), popup_id);
            }

            if ui.is_rect_visible(response.rect) {
                let center = response.rect.center();
                let dot_radius = 2.0;
                let dot_spacing = 6.0;
                let dot_color = if response.hovered() {
                    theme.colors.foreground
                } else {
                    theme.colors.foreground.linear_multiply(0.7)
                };

                // Three vertical dots
                for i in -1..=1 {
                    painter.circle_filled(
                        center + egui::vec2(0.0, i as f32 * dot_spacing),
                        dot_radius,
                        dot_color,
                    );
                }

                // Background on hover
                if response.hovered() {
                    painter.rect_filled(
                        response.rect,
                        theme.radii.sm,
                        theme.colors.accent,
                    );
                    // Redraw dots on top
                    for i in -1..=1 {
                        painter.circle_filled(
                            center + egui::vec2(0.0, i as f32 * dot_spacing),
                            dot_radius,
                            theme.colors.accent_foreground,
                        );
                    }
                }
            }

            response
        };

        let mut clicked_item = None;
        let mut action_index = 0;

        // Show popup menu
        if is_open {
            let popup_response = egui::Area::new(popup_id)
                .order(egui::Order::Foreground)
                .fixed_pos(trigger_response.rect.left_bottom() + egui::vec2(0.0, 4.0))
                .show(ui.ctx(), |ui| {
                    let frame = egui::Frame::NONE
                        .fill(theme.colors.popover)
                        .stroke(egui::Stroke::new(1.0, theme.colors.border))
                        .corner_radius(theme.radii.md)
                        .shadow(theme.shadows.md)
                        .inner_margin(4.0);

                    frame.show(ui, |ui| {
                        ui.set_min_width(self.width);

                        for item in &self.items {
                            match item {
                                MenuItem::Action { label, shortcut, enabled, destructive } => {
                                    let current_index = action_index;
                                    action_index += 1;

                                    let item_response = ui.allocate_response(
                                        egui::vec2(ui.available_width(), 36.0),
                                        if *enabled { Sense::click() } else { Sense::hover() },
                                    );

                                    if item_response.clicked() && *enabled {
                                        clicked_item = Some(current_index);
                                        Popup::close_id(ui.ctx(), popup_id);
                                    }

                                    if ui.is_rect_visible(item_response.rect) {
                                        let hovered = item_response.hovered() && *enabled;

                                        // Hover background
                                        if hovered {
                                            ui.painter().rect_filled(
                                                item_response.rect,
                                                theme.radii.sm,
                                                if *destructive {
                                                    theme.colors.destructive.linear_multiply(0.1)
                                                } else {
                                                    theme.colors.accent
                                                },
                                            );
                                        }

                                        // Text color
                                        let text_color = if *destructive {
                                            if *enabled {
                                                theme.colors.destructive
                                            } else {
                                                theme.colors.destructive.linear_multiply(0.5)
                                            }
                                        } else if *enabled {
                                            if hovered {
                                                theme.colors.accent_foreground
                                            } else {
                                                theme.colors.popover_foreground
                                            }
                                        } else {
                                            theme.colors.popover_foreground.linear_multiply(0.5)
                                        };

                                        // Label
                                        ui.painter().text(
                                            egui::pos2(
                                                item_response.rect.min.x + 8.0,
                                                item_response.rect.center().y,
                                            ),
                                            egui::Align2::LEFT_CENTER,
                                            label,
                                            egui::FontId::proportional(theme.typography.body().size),
                                            text_color,
                                        );

                                        // Shortcut
                                        if let Some(shortcut) = shortcut {
                                            ui.painter().text(
                                                egui::pos2(
                                                    item_response.rect.max.x - 8.0,
                                                    item_response.rect.center().y,
                                                ),
                                                egui::Align2::RIGHT_CENTER,
                                                shortcut,
                                                egui::FontId::proportional(theme.typography.small().size),
                                                theme.colors.foreground.linear_multiply(0.5),
                                            );
                                        }
                                    }
                                }
                                MenuItem::Separator => {
                                    let separator_rect = ui.allocate_space(egui::vec2(ui.available_width(), 9.0)).1;
                                    ui.painter().line_segment(
                                        [
                                            egui::pos2(separator_rect.min.x + 4.0, separator_rect.center().y),
                                            egui::pos2(separator_rect.max.x - 4.0, separator_rect.center().y),
                                        ],
                                        egui::Stroke::new(1.0, theme.colors.border),
                                    );
                                }
                                MenuItem::Label(text) => {
                                    let label_response = ui.allocate_response(
                                        egui::vec2(ui.available_width(), 28.0),
                                        Sense::hover(),
                                    );

                                    if ui.is_rect_visible(label_response.rect) {
                                        ui.painter().text(
                                            egui::pos2(
                                                label_response.rect.min.x + 8.0,
                                                label_response.rect.center().y,
                                            ),
                                            egui::Align2::LEFT_CENTER,
                                            text,
                                            egui::FontId::proportional(theme.typography.small().size),
                                            theme.colors.foreground.linear_multiply(0.7),
                                        );
                                    }
                                }
                            }
                        }
                    });
                });

            // Close popup when clicking outside
            if popup_response.response.clicked_elsewhere() {
                Popup::close_id(ui.ctx(), popup_id);
            }
        }

        DropdownMenuResponse {
            trigger_response,
            clicked_item,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dropdown_menu_creation() {
        let menu = DropdownMenu::new("test")
            .item("Option 1")
            .separator()
            .item("Option 2");
        assert_eq!(menu.items.len(), 3);
    }
}
